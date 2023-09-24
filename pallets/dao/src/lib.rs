#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::*,
		sp_runtime::{
			traits::{AccountIdConversion, Hash, Zero},
			DispatchResult,
		},
		traits::{Currency, ExistenceRequirement, NamedReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	/// The DAO's reserved currency identifier.
	pub const RESERVE_ID: [u8; 8] = *b"pall/dao";

	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Currency to be used for this pallet, to pay for membership cost,
		/// stake for randomness, and reward from randomness usage.
		type Currency: NamedReservableCurrency<Self::AccountId, ReserveIdentifier = [u8; 8]>;
		/// The Dao's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		/// The maximum number of members allowed in the DAO.
		#[pallet::constant]
		type MaxMembers: Get<u32>;
		// Type representing the weight of this pallet
		type WeightInfo: crate::WeightInfo;
	}

	type BalanceOf<T, I> =
		<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Encode, Decode, PartialEq, Eq, Clone, TypeInfo, MaxEncodedLen)]
	pub enum Phase {
		Commit,
		Reveal,
		Cooldown,
	}

	impl Default for Phase {
		fn default() -> Self {
			Phase::Cooldown
		}
	}

	// The pallet's runtime storage items.

	/// The current members of the DAO.
	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config<I>, I: 'static = ()> =
		CountedStorageMap<_, Identity, T::AccountId, (), OptionQuery>;

	/// The current cost of membership.
	/// Designed to be upgradable by the DAO.
	#[pallet::storage]
	#[pallet::getter(fn membership_cost)]
	pub type MembershipCost<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BalanceOf<T, I>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn current_phase)]
	pub type CurrentPhase<T: Config<I>, I: 'static = ()> = StorageValue<_, Phase, ValueQuery>;

	/// The commitments of the members of the DAO.
	#[pallet::storage]
	#[pallet::getter(fn commitments)]
	pub type Commitments<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Identity, T::AccountId, (T::Hash, [u8; 16], bool), OptionQuery>;

	/// The current random number, generated from the commitments.
	#[pallet::storage]
	#[pallet::getter(fn current_random)]
	pub type CurrentRandom<T: Config<I>, I: 'static = ()> = StorageValue<_, u128, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A new member has joined the DAO.
		MemberJoined(T::AccountId),
		/// A member has left the DAO.
		MemberLeft(T::AccountId),
		/// A member has committed to a number.
		Committed(T::AccountId, T::Hash, [u8; 16], BalanceOf<T, I>),
		/// Members can now commit to a new number.
		CommitPhaseStarted,
		/// Members can now reveal their commitments.
		RevealPhaseStarted,
		/// A new random number has been generated.
		CooldownPhaseStarted,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// The account is already a member of the DAO.
		AlreadyMember,
		/// The DAO is already at maximum capacity.
		MaximumCapacityReached,
		/// The membership cost was not paid, probably due to insufficient balance.
		MembershipCostNotPaid,
		/// The account is not a member of the DAO.
		NotAMember,
		/// Commit is not the current phase.
		NotCommitPhase,
		/// A member is trying to commit without any stake.
		StakeRequired,
		/// Failed to stake, probably due to insufficient balance.
		StakingFailed,
		/// Reveal is not the current phase.
		NotRevealPhase,
		/// Member has not previously committed to a number.
		NotCommitted,
		/// The revealed number does not match the committed value.
		CommitmentMismatch,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::join())]
		/// An account joins to the DAO, paying the cost of membership.
		pub fn join(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Members<T, I>>::contains_key(&who), <Error<T, I>>::AlreadyMember);
			ensure!(
				<Members<T, I>>::count() < T::MaxMembers::get(),
				<Error<T, I>>::MaximumCapacityReached
			);
			T::Currency::transfer(
				&who,
				&<Pallet<T, I>>::account_id(),
				<MembershipCost<T, I>>::get(),
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|_| <Error<T, I>>::MembershipCostNotPaid)?;
			<Members<T, I>>::insert(&who, ());
			Self::deposit_event(<Event<T, I>>::MemberJoined(who.clone()));
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::leave())]
		/// A member leaves the DAO.
		pub fn leave(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<Members<T, I>>::contains_key(&who), <Error<T, I>>::NotAMember);
			<Members<T, I>>::remove(&who);
			Self::deposit_event(<Event<T, I>>::MemberLeft(who.clone()));
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::commit())]
		/// Commit a number to be used for randomness generation,
		/// where `committed_value = BlakeTwo256((number, dummy).encode())`.
		/// The caller must be a member of the DAO, and the current phase must be `Commit`.
		/// If `stake` is non-zero, it will be reserved for the duration of the commitment.
		/// Otherwise, there must be already some stake reserved from previous phases.
		pub fn commit(
			origin: OriginFor<T>,
			committed_value: T::Hash,
			dummy: [u8; 16],
			stake: BalanceOf<T, I>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<Members<T, I>>::contains_key(&who), <Error<T, I>>::NotAMember);
			ensure!(<CurrentPhase<T, I>>::get() == Phase::Commit, <Error<T, I>>::NotCommitPhase);
			if stake.is_zero() {
				ensure!(
					!T::Currency::reserved_balance_named(&RESERVE_ID, &who).is_zero(),
					<Error<T, I>>::StakeRequired
				);
			} else {
				T::Currency::reserve_named(&RESERVE_ID, &who, stake)
					.map_err(|_| <Error<T, I>>::StakingFailed)?;
			}
			<Commitments<T, I>>::insert(&who, (committed_value, dummy, false));
			Self::deposit_event(<Event<T, I>>::Committed(who, committed_value, dummy, stake));

			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::reveal())]
		/// Reveal the number previously committed for randomness generation,
		/// where `committed_value = BlakeTwo256((number, dummy).encode())`.
		/// The caller must be a member of the DAO, and the current phase must be `Reveal`.
		pub fn reveal(origin: OriginFor<T>, number: u128) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<Members<T, I>>::contains_key(&who), <Error<T, I>>::NotAMember);
			ensure!(<CurrentPhase<T, I>>::get() == Phase::Reveal, <Error<T, I>>::NotCommitPhase);

			<Commitments<T, I>>::try_mutate_exists(&who, |opt| match opt {
				None => Err(<Error<T, I>>::NotCommitted.into()),
				Some((committed_value, dummy, revealed)) => {
					ensure!(
						T::Hashing::hash_of(&(number, dummy).encode()) == *committed_value,
						<Error<T, I>>::CommitmentMismatch
					);
					if !*revealed {
						<CurrentRandom<T, I>>::mutate(|random| *random ^= number);
					}
					*revealed = true;
					Ok::<(), DispatchError>(())
				},
			})?;

			Ok(())
		}
	}

	// Helper functions
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		/// The account id of the DAO Pallet.
		pub fn account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config<I>, I: 'static = ()> {
		pub membership_cost: BalanceOf<T, I>,
		#[serde(skip)]
		pub _config: PhantomData<(T, I)>,
	}

	#[pallet::genesis_build]
	impl<T: Config<I>, I: 'static> BuildGenesisConfig for GenesisConfig<T, I> {
		fn build(&self) {
			<MembershipCost<T, I>>::put(self.membership_cost);
			// Create Pallet account
			let account_id = <Pallet<T, I>>::account_id();
			let min = T::Currency::minimum_balance();
			if T::Currency::free_balance(&account_id) < min {
				let _ = T::Currency::make_free_balance_be(&account_id, min);
			}
		}
	}

	#[pallet::hooks]
	impl<T: Config<I>, I: 'static> Hooks<BlockNumberFor<T>> for Pallet<T, I> {
		fn on_initialize(n: BlockNumberFor<T>) -> Weight {
			// There is a commit phase of 10 blocks, a reveal phase of 10 blocks,
			// and a cooldown phase of 80 blocks.
			// We will have a new random number every 100 blocks, usually 10 minutes.
			// This is designed with the assuption of being an essential operation for the on-chain
			// applications to work properly, so it is correct to perform it on initialize.
			let m = n % 100u32.into();
			if m == 1u32.into() {
				// Start the commit phase.
				<CurrentPhase<T, I>>::set(Phase::Commit);
				let res = <Commitments<T, I>>::clear(T::MaxMembers::get(), None);
				Self::deposit_event(<Event<T, I>>::CommitPhaseStarted);
				T::DbWeight::get().reads_writes(res.loops as u64, 1 + res.backend as u64)
			} else if m == 11u32.into() {
				// Start the reveal phase.
				<CurrentPhase<T, I>>::set(Phase::Reveal);
				<CurrentRandom<T, I>>::set(0u128);
				Self::deposit_event(<Event<T, I>>::RevealPhaseStarted);
				T::DbWeight::get().writes(2_u64)
			} else if m == 21u32.into() {
				// Start the cooldown phase.
				<CurrentPhase<T, I>>::set(Phase::Cooldown);
				let mut slashed = 0u64;
				let mut reads = 0u64;
				<Commitments<T, I>>::iter().for_each(|(who, (_, _, revealed))| {
					if !revealed {
						let amount = T::Currency::reserved_balance_named(&RESERVE_ID, &who);
						T::Currency::slash_reserved_named(&RESERVE_ID, &who, amount / 4u32.into());
						slashed += 1;
					}
					reads += 1;
				});
				Self::deposit_event(<Event<T, I>>::CooldownPhaseStarted);
				T::DbWeight::get().reads_writes(slashed + reads, slashed + 1)
			} else {
				Zero::zero()
			}
		}
	}
}

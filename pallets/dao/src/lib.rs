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
		traits::{Currency, ExistenceRequirement, ReservableCurrency},
		PalletId,
	};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AccountIdConversion;

	#[pallet::pallet]
	pub struct Pallet<T, I = ()>(PhantomData<(T, I)>);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config<I: 'static = ()>: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self, I>>
			+ IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Currency to be used for this pallet, to pay for membership cost,
		/// stake for randomness, and reward from randomness usage.
		type Currency: ReservableCurrency<Self::AccountId>;
		/// The Dao's pallet id, used for deriving its sovereign account ID.
		#[pallet::constant]
		type PalletId: Get<PalletId>;
		// Type representing the weight of this pallet
		type WeightInfo: crate::WeightInfo;
	}

	type BalanceOf<T, I> =
		<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The pallet's runtime storage items.

	/// The current members of the DAO.
	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub type Members<T: Config<I>, I: 'static = ()> =
		StorageMap<_, Identity, T::AccountId, (), OptionQuery>;

	/// The current cost of membership.
	/// Designed to be upgradable by the DAO.
	#[pallet::storage]
	#[pallet::getter(fn membership_cost)]
	pub type MembershipCost<T: Config<I>, I: 'static = ()> =
		StorageValue<_, BalanceOf<T, I>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config<I>, I: 'static = ()> {
		/// A new member has joined the DAO.
		NewMember(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T, I = ()> {
		/// The account is already a member of the DAO.
		AlreadyMember,
		/// The subscription cost was not paid, probably due to insufficient balance.
		SubscriptionCostNotPaid,
		/// The account is not a member of the DAO.
		NotAMember,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config<I>, I: 'static> Pallet<T, I> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::subscribe())]
		/// Subscribe as a new member of the DAO.
		/// Pay the predetermined cost of membership.
		pub fn subscribe(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Members<T, I>>::contains_key(&who), <Error<T, I>>::AlreadyMember);
			T::Currency::transfer(
				&who,
				&<Pallet<T, I>>::account_id(),
				<MembershipCost<T, I>>::get(),
				ExistenceRequirement::KeepAlive,
			)
			.map_err(|_| <Error<T, I>>::SubscriptionCostNotPaid)?;
			<Members<T, I>>::insert(&who, ());
			Self::deposit_event(<Event<T, I>>::NewMember(who.clone()));
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::unsubscribe())]
		/// Unsubscribe as a member of the DAO.
		pub fn unsubscribe(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<Members<T, I>>::contains_key(&who), <Error<T, I>>::NotAMember);
			<Members<T, I>>::remove(&who);
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
}

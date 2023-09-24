//! Benchmarking setup for pallet-dao
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Dao;
use codec::Encode;
use frame_benchmarking::v2::*;
use frame_support::traits::{Currency, Hooks, NamedReservableCurrency};
use frame_system::RawOrigin;
use sp_runtime::{traits::Hash, Saturating};

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn join() {
		let caller: T::AccountId = account("caller", 0, 0);
		T::Currency::make_free_balance_be(
			&caller,
			<MembershipCost<T, ()>>::get().saturating_mul(2u32.into()),
		);

		assert!(
			T::Currency::free_balance(&caller).is_zero() == false,
			"Caller should have some free balance, GenesisConfig didn't work"
		);

		#[extrinsic_call]
		join(RawOrigin::Signed(caller.clone()));

		assert!(<Members<T, ()>>::contains_key(caller));
	}

	#[benchmark]
	fn leave() {
		let caller: T::AccountId = account("caller", 0, 0);
		<Members<T, ()>>::insert(&caller, ());

		#[extrinsic_call]
		leave(RawOrigin::Signed(caller.clone()));

		assert!(!<Members<T, ()>>::contains_key(caller));
	}

	#[benchmark]
	fn commit() {
		Dao::<T, ()>::on_initialize(1u32.into());

		let caller: T::AccountId = account("caller", 0, 0);
		<Members<T, ()>>::insert(&caller, ());

		T::Currency::make_free_balance_be(
			&caller,
			<MembershipCost<T, ()>>::get().saturating_mul(2u32.into()),
		);

		assert!(
			T::Currency::free_balance(&caller).is_zero() == false,
			"Caller should have some free balance, GenesisConfig didn't work"
		);

		let dummy = [0u8; 16];
		let committed_hash = T::Hashing::hash_of(&(10u128, dummy).encode());

		#[extrinsic_call]
		commit(RawOrigin::Signed(caller.clone()), committed_hash, dummy, 100u32.into());

		assert_eq!(<Commitments<T, ()>>::get(&caller), Some((committed_hash, dummy, false)));
		assert!(!T::Currency::reserved_balance_named(&RESERVE_ID, &caller).is_zero());
	}

	#[benchmark]
	fn reveal() {
		Dao::<T, ()>::on_initialize(1u32.into());

		let caller: T::AccountId = account("caller", 0, 0);
		<Members<T, ()>>::insert(&caller, ());

		T::Currency::make_free_balance_be(
			&caller,
			<MembershipCost<T, ()>>::get().saturating_mul(2u32.into()),
		);

		assert!(
			T::Currency::free_balance(&caller).is_zero() == false,
			"Caller should have some free balance, GenesisConfig didn't work"
		);

		let dummy = [0u8; 16];
		let committed_hash = T::Hashing::hash_of(&(10u128, dummy).encode());
		frame_support::assert_ok!(Dao::<T, ()>::commit(
			RawOrigin::Signed(caller.clone()).into(),
			committed_hash,
			dummy,
			100u32.into()
		));

		assert_eq!(<Commitments<T, ()>>::get(&caller), Some((committed_hash, dummy, false)));
		assert!(!T::Currency::reserved_balance_named(&RESERVE_ID, &caller).is_zero());

		Dao::<T, ()>::on_initialize(11u32.into());

		#[extrinsic_call]
		reveal(RawOrigin::Signed(caller.clone()), 10u128);

		assert_eq!(<CurrentRandom<T, ()>>::get(), 10);
	}

	impl_benchmark_test_suite!(Dao, crate::mock::new_test_ext(), crate::mock::Test);
}

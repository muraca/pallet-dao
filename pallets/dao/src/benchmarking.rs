//! Benchmarking setup for pallet-dao
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Dao;
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::Saturating;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn subscribe() {
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
		subscribe(RawOrigin::Signed(caller.clone()));

		assert!(<Members<T, ()>>::contains_key(caller));
	}

	#[benchmark]
	fn unsubscribe() {
		let caller: T::AccountId = account("caller", 0, 0);
		<Members<T, ()>>::insert(&caller, ());

		#[extrinsic_call]
		unsubscribe(RawOrigin::Signed(caller.clone()));

		assert!(!<Members<T, ()>>::contains_key(caller));
	}

	impl_benchmark_test_suite!(Dao, crate::mock::new_test_ext(), crate::mock::Test);
}

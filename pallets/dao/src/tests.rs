use crate::{mock::*, Commitments, CurrentRandom, Error, Members, MembershipCost, RESERVE_ID};
use codec::Encode;
use frame_support::{
	assert_noop, assert_ok,
	sp_runtime::traits::{Hash, Zero},
	traits::{Currency, Hooks, NamedReservableCurrency},
};

#[test]
fn genesis_works() {
	new_test_ext().execute_with(|| assert_eq!(Dao::membership_cost(), 1_000_000));
}

#[test]
fn join_and_leave_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 1_500_000);
		let dao_balance = Balances::free_balance(Dao::account_id());
		assert_ok!(Dao::join(RuntimeOrigin::signed(1)));
		assert_eq!(Balances::free_balance(Dao::account_id()), dao_balance + Dao::membership_cost());
		assert_noop!(Dao::join(RuntimeOrigin::signed(1)), <Error<Test, ()>>::AlreadyMember);
		assert_ok!(Dao::leave(RuntimeOrigin::signed(1)));
		assert_noop!(Dao::leave(RuntimeOrigin::signed(1)), <Error<Test, ()>>::NotAMember);
		assert_noop!(Dao::join(RuntimeOrigin::signed(1)), <Error<Test, ()>>::MembershipCostNotPaid);
	});
}

#[test]
fn commit_and_reveal_work() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&1, 1_500_000);
		Dao::on_initialize(1u32.into());
		<Members<Test, ()>>::insert(&1, ());
		Balances::make_free_balance_be(
			&1,
			<MembershipCost<Test, ()>>::get().saturating_mul(2u32.into()),
		);
		assert_eq!(Balances::free_balance(&1).is_zero(), false);
		let dummy = [9u8; 16];
		let committed_hash =
			<Test as frame_system::Config>::Hashing::hash_of(&(10u128, dummy).encode());
		assert_ok!(Dao::commit(RuntimeOrigin::signed(1), committed_hash, dummy, 10_000u32.into()));
		assert_eq!(<Commitments<Test, ()>>::get(&1), Some((committed_hash, dummy, false)));
		assert!(!Balances::reserved_balance_named(&RESERVE_ID, &1).is_zero());
		Dao::on_initialize(11u32.into());
		assert_ok!(Dao::reveal(RuntimeOrigin::signed(1), 10u128));
		assert_eq!(<CurrentRandom<Test, ()>>::get(), 10);
	});
}

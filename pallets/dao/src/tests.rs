use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};

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

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_lock_capital() {
    new_test_ext().execute_with(|| {
        assert_ok!(LockableModule::lock_capital(RuntimeOrigin::signed(1), 100));
    });
}

#[test]
fn test_extend_lock() {
    new_test_ext().execute_with(|| {
        assert_ok!(LockableModule::lock_capital(RuntimeOrigin::signed(1), 100));
        assert_ok!(LockableModule::extend_lock(RuntimeOrigin::signed(1), 100));
    });
}

#[test]
fn test_unlock_all() {
    new_test_ext().execute_with(|| {
        assert_ok!(LockableModule::lock_capital(RuntimeOrigin::signed(1), 100));
        assert_ok!(LockableModule::unlock_all(RuntimeOrigin::signed(1)));
    });
}

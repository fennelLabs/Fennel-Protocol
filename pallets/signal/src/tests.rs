use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_signal(Origin::signed(1), "TEST".as_bytes().to_vec()));
    });
}

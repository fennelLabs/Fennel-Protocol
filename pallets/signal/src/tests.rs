use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_signal(Origin::signed(1), "TEST".as_bytes().to_vec()));
    });
}

#[test]
fn test_send_service_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_service_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            "TEST".as_bytes().to_vec()
        ));
    });
}

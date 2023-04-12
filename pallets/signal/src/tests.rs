use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_send_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_send_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_whiteflag_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_update_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::update_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_update_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::update_whiteflag_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_revoke_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        assert_ok!(SignalModule::revoke_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec()
        ));
    });
}

#[test]
fn test_revoke_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_whiteflag_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        assert_ok!(SignalModule::revoke_whiteflag_rating_signal(
            Origin::signed(1),
            "TEST".as_bytes().to_vec()
        ));
    });
}

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

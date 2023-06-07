use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_set_signal_parameter() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::set_signal_parameter(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_send_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        System::assert_last_event(
            crate::Event::RatingSignalSent("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
    });
}

#[test]
fn test_send_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_update_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::update_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        System::assert_last_event(
            crate::Event::RatingSignalUpdated("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
    });
}

#[test]
fn test_update_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::update_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
    });
}

#[test]
fn test_revoke_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        System::assert_last_event(
            crate::Event::RatingSignalSent("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
        assert_ok!(SignalModule::revoke_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec()
        ));
        System::assert_last_event(
            crate::Event::RatingSignalRevoked("TEST".as_bytes().to_vec(), 1).into(),
        );
    });
}

#[test]
fn test_revoke_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        assert_ok!(SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0
        ));
        assert_ok!(SignalModule::revoke_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec()
        ));
    });
}

#[test]
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec()));
        System::assert_last_event(crate::Event::SignalSent("TEST".as_bytes().to_vec(), 1).into());
    });
}

#[test]
fn test_send_service_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_service_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            "TEST".as_bytes().to_vec()
        ));
        System::assert_last_event(
            crate::Event::ServiceSignalSent(
                "TEST".as_bytes().to_vec(),
                "TEST".as_bytes().to_vec(),
                1,
            )
            .into(),
        );
    });
}

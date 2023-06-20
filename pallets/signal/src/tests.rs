use crate::{mock::*, Error};
use frame_support::{assert_noop, traits::Currency};

#[test]
fn test_set_signal_parameter() {
    new_test_ext().execute_with(|| {
        SignalModule::set_signal_parameter(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec(), 0)
            .unwrap();
    });
}

#[test]
fn test_send_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_rating_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec(), 0)
            .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalSent("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
    });
}

#[test]
fn test_send_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::send_rating_signal(
                RuntimeOrigin::signed(1),
                "TEST".as_bytes().to_vec(),
                0
            ),
            Error::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn test_send_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0,
        )
        .unwrap();
    });
}

#[test]
fn test_send_whiteflag_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::send_whiteflag_rating_signal(
                RuntimeOrigin::signed(1),
                "TEST".as_bytes().to_vec(),
                0
            ),
            Error::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn test_update_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::update_rating_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec(), 0)
            .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalUpdated("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
    });
}

#[test]
fn test_update_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::update_rating_signal(
                RuntimeOrigin::signed(1),
                "TEST".as_bytes().to_vec(),
                0
            ),
            Error::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn test_update_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::update_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0,
        )
        .unwrap();
    });
}

#[test]
fn test_update_whiteflag_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::update_whiteflag_rating_signal(
                RuntimeOrigin::signed(1),
                "TEST".as_bytes().to_vec(),
                0
            ),
            Error::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn test_revoke_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_rating_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec(), 0)
            .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalSent("TEST".as_bytes().to_vec(), 0, 1).into(),
        );
        SignalModule::revoke_rating_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec())
            .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalRevoked("TEST".as_bytes().to_vec(), 1).into(),
        );
    });
}

#[test]
fn test_revoke_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            0,
        )
        .unwrap();
        SignalModule::revoke_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
        )
        .unwrap();
    });
}

#[test]
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        SignalModule::send_signal(RuntimeOrigin::signed(1), "TEST".as_bytes().to_vec()).unwrap();
        System::assert_last_event(crate::Event::SignalSent("TEST".as_bytes().to_vec(), 1).into());
    });
}

#[test]
fn test_send_service_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        SignalModule::send_service_signal(
            RuntimeOrigin::signed(1),
            "TEST".as_bytes().to_vec(),
            "TEST".as_bytes().to_vec(),
        )
        .unwrap();
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

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn test_set_signal_parameter() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        SignalModule::set_signal_parameter(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(crate::Event::SignalParameterSet(1).into());
    });
}

#[test]
fn test_send_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(crate::Event::RatingSignalSent(1).into());
        System::assert_last_event(crate::Event::RatingSignalSent(1).into());
    });
}

#[test]
fn test_send_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::send_rating_signal(
                RuntimeOrigin::signed(1),
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
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
        assert!(SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        )
        .is_ok());
        assert_ok!(SignalModule::update_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            1
        ));
        System::assert_last_event(crate::Event::RatingSignalUpdated(1).into());
    });
}

#[test]
fn test_revoke_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(crate::Event::RatingSignalSent(1).into());
        SignalModule::revoke_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
        System::assert_last_event(crate::Event::RatingSignalRevoked(1).into());
    });
}

#[test]
fn test_revoke_rating_signal_fails_if_none() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_noop!(
            SignalModule::revoke_rating_signal(
                RuntimeOrigin::signed(1),
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap()
            ),
            Error::<Test>::RatingSignalDoesNotExist
        );
    });
}

#[test]
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        SignalModule::send_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::SignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                1,
            )
            .into(),
        );
    });
}

#[test]
fn test_send_service_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        SignalModule::send_service_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::ServiceSignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                1,
            )
            .into(),
        );
    });
}

use crate::{mock::*, Error};
use frame_support::{assert_noop, traits::Currency};
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
        System::assert_last_event(
            crate::Event::SignalParameterSet(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
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
        System::assert_last_event(
            crate::Event::RatingSignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
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
fn test_send_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::WhiteflagRatingSignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
    });
}

#[test]
fn test_send_whiteflag_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::send_whiteflag_rating_signal(
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
        SignalModule::update_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalUpdated(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
    });
}

#[test]
fn test_update_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::update_rating_signal(
                RuntimeOrigin::signed(1),
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0
            ),
            Error::<Test>::InsufficientBalance
        );
    })
}

#[test]
fn test_update_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::update_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::WhiteflagRatingSignalUpdated(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
    });
}

#[test]
fn test_update_whiteflag_rating_signal_insufficient_balance() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            SignalModule::update_whiteflag_rating_signal(
                RuntimeOrigin::signed(1),
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
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
        SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
        SignalModule::revoke_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
        System::assert_last_event(
            crate::Event::RatingSignalRevoked(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                1,
            )
            .into(),
        );
    });
}

#[test]
fn test_revoke_whiteflag_rating_signal() {
    new_test_ext().execute_with(|| {
        let _ = Balances::deposit_creating(&1, 100);
        SignalModule::send_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0,
        )
        .unwrap();
        SignalModule::revoke_whiteflag_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
        )
        .unwrap();
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

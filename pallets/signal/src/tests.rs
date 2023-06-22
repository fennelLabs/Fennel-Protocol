use crate::mock::*;
use frame_support::assert_ok;
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn test_send_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        ));
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
fn test_update_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::update_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        ));
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
fn test_revoke_rating_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        ));
        System::assert_last_event(
            crate::Event::RatingSignalSent(
                BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
                0,
                1,
            )
            .into(),
        );
        assert_ok!(SignalModule::revoke_rating_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap()
        ));
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
fn test_send_signal() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(SignalModule::send_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap()
        ));
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
        assert_ok!(SignalModule::send_service_signal(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap()
        ));
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

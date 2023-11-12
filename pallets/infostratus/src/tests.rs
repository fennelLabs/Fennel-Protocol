use crate::{mock::*, Error};
use frame_support::{assert_ok, traits::Currency, assert_noop};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn test_create_submission_entry() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
    });
}

#[test]
fn test_create_existing_submission() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
        assert_noop!(
            Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()),
            Error::<Test>::SubmissionExists
        );
    });
}

#[test]
fn test_request_submission_assignment() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        let _ = Balances::deposit_creating(&2, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
        assert_ok!(Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionAssigned(BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap(), 2).into());
    });
}

#[test]
fn test_try_assign_nonexistent_submission() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        let _ = Balances::deposit_creating(&2, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
        assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 3, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()),
            Error::<Test>::SubmissionDoesNotExist
        );
    });
}

#[test]
fn test_request_submission_assignment_already_assigned() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        let _ = Balances::deposit_creating(&2, 100);
        let _ = Balances::deposit_creating(&3, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
        assert_ok!(Infostratus::request_submission_assignment(RuntimeOrigin::signed(2), 1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionAssigned(BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap(), 2).into());
        assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(3), 1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()),
            Error::<Test>::SubmissionAlreadyAssigned
        );
    });
}

#[test]
fn test_request_submission_assignment_own_submission() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(Infostratus::create_submission_entry(RuntimeOrigin::signed(1), BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()));
        System::assert_last_event(crate::Event::SubmissionSent(1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()).into());
        assert_noop!(
            Infostratus::request_submission_assignment(RuntimeOrigin::signed(1), 1, BoundedVec::<u8, ConstU32<1024>>::try_from("TEST".as_bytes().to_vec()).unwrap()),
            Error::<Test>::CannotAssignOwnSubmission
        );
    });
}
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::ConstU32;
use sp_runtime::BoundedVec;

#[test]
fn test_set_trust_parameter() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::set_trust_parameter(
            RuntimeOrigin::signed(1),
            BoundedVec::<u8, ConstU32<100>>::try_from("TEST".as_bytes().to_vec()).unwrap(),
            0
        ));
        System::assert_last_event(
            crate::Event::TrustParameterSet(1).into(),
        );
    });
}

#[test]
fn issue_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssued(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 1);
    });
}

#[test]
fn issue_trust_error() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssued(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_noop!(
            TrustModule::issue_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustExists
        );
    });
}

#[test]
fn request_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRequest(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRequestRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_requests(), 0);
    });
}

#[test]
fn request_trust_error() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRequest(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_noop!(
            TrustModule::request_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRequestExists
        );
    });
}

#[test]
fn cancel_request_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRequest(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRequestRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_requests(), 0);
    });
}

#[test]
fn cancel_request_trust_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRequestNotFound
        );
    });
}

#[test]
fn remove_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssued(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssuanceRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 0);
    });
}

#[test]
fn remove_trust_no_failure() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssued(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustIssuanceRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_trust_count(), 0);
    });
}

#[test]
fn remove_trust_raises_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TrustModule::remove_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustNotFound
        );
    });
}

#[test]
fn revoke_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevoked(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);
    });
}

#[test]
fn revoke_trust_error() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevoked(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_noop!(
            TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRevocationExists
        );
    });
}

#[test]
fn remove_revoked_trust() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevoked(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevocationRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);
    });
}

#[test]
fn remove_revoked_trust_no_failure() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevoked(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::TrustRevocationRemoved(1, 1).into());
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);
    });
}

#[test]
fn remove_revoked_trust_raises_error() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1),
            Error::<Test>::TrustRevocationNotFound
        );
    });
}

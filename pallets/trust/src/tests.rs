use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn issue_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);
    });
}

#[test]
fn issue_trust_error() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
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
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 0);
    });
}

#[test]
fn cancel_request_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::request_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(RuntimeOrigin::signed(1), 1));
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
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 0);
    });
}

#[test]
fn remove_trust_no_failure() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(RuntimeOrigin::signed(1), 1));
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
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);
    });
}

#[test]
fn revoke_trust_once() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);
    });
}

#[test]
fn remove_revoked_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);
    });
}

#[test]
fn remove_revoked_trust_no_failure() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);

        assert_ok!(TrustModule::remove_revoked_trust(RuntimeOrigin::signed(1), 1));
    });
}

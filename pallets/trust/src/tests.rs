use crate::mock::*;
use frame_support::assert_ok;

// SBP-M1 review: how about adding checks for events?

#[test]
fn issue_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);
    });
}

#[test]
fn issue_trust_once() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::issue_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);
    });
}

#[test]
fn request_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::request_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 0);
    });
}

#[test]
fn cancel_request_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::request_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 1);

        assert_ok!(TrustModule::cancel_trust_request(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_requests(), 0);

        assert_ok!(TrustModule::cancel_trust_request(Origin::signed(1), 1));
    });
}

#[test]
fn remove_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 0);
    });
}

#[test]
fn remove_trust_no_failure() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::issue_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 1);

        assert_ok!(TrustModule::remove_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_trust_count(), 0);

        assert_ok!(TrustModule::remove_trust(Origin::signed(1), 1));
    });
}

#[test]
fn revoke_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);
    });
}

#[test]
fn revoke_trust_once() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::revoke_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);
    });
}

#[test]
fn remove_revoked_trust() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);
    });
}

#[test]
fn remove_revoked_trust_no_failure() {
    new_test_ext().execute_with(|| {
        assert_ok!(TrustModule::revoke_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 1);

        assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), 1));
        assert_eq!(TrustModule::get_current_non_trust_count(), 0);

        assert_ok!(TrustModule::remove_revoked_trust(Origin::signed(1), 1));
    });
}

use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_issue_key() {
    new_test_ext().execute_with(|| {
        let luke = "Luke".as_bytes().to_vec();
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::announce_key(RuntimeOrigin::signed(1), luke, skywalker));
    });
}

#[test]
fn test_revoke_key() {
    new_test_ext().execute_with(|| {
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::revoke_key(RuntimeOrigin::signed(1), skywalker));
    });
}

#[test]
fn test_issue_encryption_key() {
    new_test_ext().execute_with(|| {
        let luke = [0; 32];
        assert_ok!(KeystoreModule::issue_encryption_key(RuntimeOrigin::signed(1), luke));
    });
}

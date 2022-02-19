use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn issue_key() {
    new_test_ext().execute_with(|| {
        let luke = "Luke".as_bytes().to_vec();
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::announce_key(Origin::signed(1), luke, skywalker));
    });
}

#[test]
fn announce_key() {
    new_test_ext().execute_with(|| {
        let luke = "Luke".as_bytes().to_vec();
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::announce_key(Origin::signed(1), luke, skywalker));
    });
}

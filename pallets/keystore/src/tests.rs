use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_issue_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let luke = "Luke".as_bytes().to_vec();
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::announce_key(
            RuntimeOrigin::signed(1),
            luke.clone(),
            skywalker.clone()
        ));
        System::assert_last_event(crate::Event::KeyExists(luke, skywalker, 1).into());
    });
}

#[test]
fn test_revoke_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(KeystoreModule::revoke_key(RuntimeOrigin::signed(1), skywalker.clone()));
        System::assert_last_event(crate::Event::KeyRevoked(skywalker, 1).into());
    });
}

#[test]
fn test_issue_encryption_key() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let luke = [0; 32];
        assert_ok!(KeystoreModule::issue_encryption_key(RuntimeOrigin::signed(1), luke.clone()));
        System::assert_last_event(crate::Event::EncryptionKeyIssued(luke, 1).into());
    });
}

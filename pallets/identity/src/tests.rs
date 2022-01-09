use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn issue_identity() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(1)));
    });
}

#[test]
fn issue_identity_increments_by_number_of_times_called() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(1)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(2)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(3)));

        assert_eq!(IdentityModule::identity_number(), 3);
    });
}

#[test]
fn issue_identity_registers_different_account_ids_with_new_identities() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(200)));

        assert_eq!(IdentityModule::identity_list(300).contains(&1), true);
        assert_eq!(IdentityModule::identity_list(200).contains(&2), true);
    });
}

#[test]
fn issue_identity_registers_same_account_id_with_multiple_new_identities() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));

        assert_eq!(IdentityModule::identity_list(300).contains(&1), true);
        assert_eq!(IdentityModule::identity_list(300).contains(&2), true);
    });
}

#[test]
fn revoke_identity() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::revoke_identity(Origin::signed(300), 1));

        assert_eq!(IdentityModule::revoked_identity_number(), 1);
    });
}

#[test]
fn revoke_identity_multiple_from_different_accounts() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(200)));
        
        assert_ok!(IdentityModule::revoke_identity(Origin::signed(300), 1));
        assert_ok!(IdentityModule::revoke_identity(Origin::signed(200), 2));

        assert_eq!(IdentityModule::revoked_identity_number(), 2);
    });
}

#[test]
fn revoke_identity_multiple_from_same_account() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        
        assert_ok!(IdentityModule::revoke_identity(Origin::signed(300), 2));
        assert_ok!(IdentityModule::revoke_identity(Origin::signed(300), 1));

        assert_eq!(IdentityModule::revoked_identity_number(), 2);
    });
}

#[test]
fn revoke_identity_from_non_owning_account() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            IdentityModule::revoke_identity(Origin::signed(300), 1),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn add_identity_trait() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::add_identity_trait(Origin::signed(300), 1, "name".as_bytes().to_vec(), "Luke Skywalker".as_bytes().to_vec()));
    });
}

#[test]
fn remove_identity_trait() {
    new_test_ext().execute_with(|| {
        assert_ok!(IdentityModule::create_identity(Origin::signed(300)));
        assert_ok!(IdentityModule::add_identity_trait(Origin::signed(300), 1, "name".as_bytes().to_vec(), "Luke Skywalker".as_bytes().to_vec()));
        assert_ok!(IdentityModule::remove_identity_trait(Origin::signed(300), 1, "name".as_bytes().to_vec()));
    });
}
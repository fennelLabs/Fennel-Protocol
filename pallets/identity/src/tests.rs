use crate::{mock::*};
use frame_support::{assert_ok};

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
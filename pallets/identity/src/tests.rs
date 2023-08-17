use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use sp_core::ConstU32;

#[test]
fn issue_identity() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(1)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 1).into());
    });
}

#[test]
fn issue_identity_increments_by_number_of_times_called() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(1)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 1).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(2)));
        System::assert_last_event(crate::Event::IdentityCreated(1, 2).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(3)));
        System::assert_last_event(crate::Event::IdentityCreated(2, 3).into());

        assert_eq!(IdentityModule::identity_number(), 3);
    });
}

#[test]
fn issue_identity_registers_different_account_ids_with_new_identities() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(200)));
        System::assert_last_event(crate::Event::IdentityCreated(1, 200).into());

        assert_eq!(IdentityModule::identity_list(0).unwrap(), 300);
        assert_eq!(IdentityModule::identity_list(1).unwrap(), 200);
    });
}

#[test]
fn issue_identity_registers_same_account_id_with_multiple_new_identities() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(1, 300).into());

        assert_eq!(IdentityModule::identity_list(0).unwrap(), 300);
        assert_eq!(IdentityModule::identity_list(1).unwrap(), 300);
    });
}

#[test]
fn revoke_identity() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::revoke_identity(RuntimeOrigin::signed(300), 0));
        System::assert_last_event(crate::Event::IdentityRevoked(0, 300).into());
    });
}

#[test]
fn revoke_identity_multiple_from_different_accounts() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(200)));
        System::assert_last_event(crate::Event::IdentityCreated(1, 200).into());

        assert_ok!(IdentityModule::revoke_identity(RuntimeOrigin::signed(300), 0));
        System::assert_last_event(crate::Event::IdentityRevoked(0, 300).into());
        assert_ok!(IdentityModule::revoke_identity(RuntimeOrigin::signed(200), 1));
        System::assert_last_event(crate::Event::IdentityRevoked(1, 200).into());
    });
}

#[test]
fn revoke_identity_multiple_from_same_account() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(1, 300).into());

        assert_ok!(IdentityModule::revoke_identity(RuntimeOrigin::signed(300), 1));
        System::assert_last_event(crate::Event::IdentityRevoked(1, 300).into());
        assert_ok!(IdentityModule::revoke_identity(RuntimeOrigin::signed(300), 0));
        System::assert_last_event(crate::Event::IdentityRevoked(0, 300).into());
    });
}

#[test]
fn revoke_identity_from_non_owning_account() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            IdentityModule::revoke_identity(RuntimeOrigin::signed(300), 1),
            Error::<Test>::IdentityNotOwned
        );
    });
}

#[test]
fn add_or_update_identity_trait() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let account_id = 300;
        let key = BoundedVec::<u8, ConstU32<1000>>::try_from("name".as_bytes().to_vec()).unwrap();

        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(account_id)));
        System::assert_last_event(
            crate::Event::IdentityCreated(0, account_id.try_into().unwrap()).into(),
        );

        let luke = BoundedVec::<u8, ConstU32<1000>>::try_from("Luke Skywalker".as_bytes().to_vec())
            .unwrap();
        assert_ok!(IdentityModule::add_or_update_identity_trait(
            RuntimeOrigin::signed(account_id),
            0,
            key.clone(),
            luke.clone()
        ));
        System::assert_last_event(
            crate::Event::IdentityUpdated(0, account_id.try_into().unwrap()).into(),
        );
        assert_eq!(IdentityModule::identity_trait_list(0, key.clone()), luke.clone());

        let anakin =
            BoundedVec::<u8, ConstU32<1000>>::try_from("Anakin Skywalker".as_bytes().to_vec())
                .unwrap();
        assert_ok!(IdentityModule::add_or_update_identity_trait(
            RuntimeOrigin::signed(300),
            0,
            key.clone(),
            anakin.clone()
        ));
        System::assert_last_event(
            crate::Event::IdentityUpdated(0, account_id.try_into().unwrap()).into(),
        );
        assert_eq!(IdentityModule::identity_trait_list(0, key.clone()), anakin.clone());
    });
}

#[test]
fn remove_identity_trait() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        assert_ok!(IdentityModule::create_identity(RuntimeOrigin::signed(300)));
        System::assert_last_event(crate::Event::IdentityCreated(0, 300).into());
        assert_ok!(IdentityModule::add_or_update_identity_trait(
            RuntimeOrigin::signed(300),
            0,
            BoundedVec::<u8, ConstU32<1000>>::try_from("name".as_bytes().to_vec()).unwrap(),
            BoundedVec::<u8, ConstU32<1000>>::try_from("Luke Skywalker".as_bytes().to_vec())
                .unwrap()
        ));
        System::assert_last_event(crate::Event::IdentityUpdated(0, 300).into());
        assert_ok!(IdentityModule::remove_identity_trait(
            RuntimeOrigin::signed(300),
            0,
            BoundedVec::<u8, ConstU32<1000>>::try_from("name".as_bytes().to_vec()).unwrap()
        ));
        System::assert_last_event(crate::Event::IdentityUpdated(0, 300).into());
    });
}

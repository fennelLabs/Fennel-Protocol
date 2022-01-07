use crate::{mock::*};
use frame_support::{assert_ok};

#[test]
fn issue_identity() {
	new_test_ext().execute_with(|| {
		assert_ok!(IdentityModule::create_identity(Origin::signed(1)));
		assert_eq!(IdentityModule::identity_list(1), 0);
		assert_eq!(IdentityModule::identity_number(), 1);
	});
}
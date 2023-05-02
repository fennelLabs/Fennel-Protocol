use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn test_send_certificate() {
    new_test_ext().execute_with(|| {
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
    });
}

#[test]
fn test_revoke_certificate() {
    new_test_ext().execute_with(|| {
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        assert_ok!(CertificateModule::revoke_certificate(RuntimeOrigin::signed(1), 1));
    });
}

#[test]
fn test_try_revoke_unowned_certificate() {
    new_test_ext().execute_with(|| {
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        assert_noop!(
            CertificateModule::revoke_certificate(RuntimeOrigin::signed(2), 1),
            Error::<Test>::CertificateNotOwned
        );
    });
}

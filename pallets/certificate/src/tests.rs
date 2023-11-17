use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency};

#[test]
fn test_send_certificate() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::CertificateSent(1, 1).into());
        assert_eq!(CertificateModule::certificate_list(1, 1), true);
    });
}

#[test]
fn test_send_existing_certificate() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::CertificateSent(1, 1).into());
        assert_noop!(
            CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1),
            Error::<Test>::CertificateExists
        );
        assert_eq!(CertificateModule::certificate_list(1, 1), true);
    });
}

#[test]
fn test_revoke_certificate() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::CertificateSent(1, 1).into());
        assert_ok!(CertificateModule::revoke_certificate(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::CertificateRevoked(1, 1).into());
        assert_eq!(CertificateModule::certificate_list(1, 1), false);
    });
}

#[test]
fn test_try_revoke_unowned_certificate() {
    new_test_ext().execute_with(|| {
        System::set_block_number(1);
        let _ = Balances::deposit_creating(&1, 100);
        let _ = Balances::deposit_creating(&2, 100);
        assert_ok!(CertificateModule::send_certificate(RuntimeOrigin::signed(1), 1));
        System::assert_last_event(crate::Event::CertificateSent(1, 1).into());
        assert_noop!(
            CertificateModule::revoke_certificate(RuntimeOrigin::signed(2), 1),
            Error::<Test>::CertificateNotOwned
        );
        assert_eq!(CertificateModule::certificate_list(1, 1), true);
    });
}

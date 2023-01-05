use crate::mock::*;
use frame_support::assert_ok;

#[test]
fn test_send_certificate() {
    new_test_ext().execute_with(|| {
        assert_ok!(CertificateModule::send_certificate(Origin::signed(1), 1));
    });
}

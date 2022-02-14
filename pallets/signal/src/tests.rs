use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn announce_key() {
    new_test_ext().execute_with(|| {
        let luke = "Luke".as_bytes().to_vec();
        let skywalker = "Skywalker".as_bytes().to_vec();
        assert_ok!(SignalModule::announce_key(Origin::signed(1), luke, skywalker));
    });
}

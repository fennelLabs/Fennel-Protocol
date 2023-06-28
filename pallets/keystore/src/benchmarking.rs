#![cfg(feature = "runtime-benchmarks")]
use super::*;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    benchmark_account(name, 0, 0)
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn announce_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "location".as_bytes().to_vec(),
        )
        .unwrap();
        let fingerprint = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "fingerprint".as_bytes().to_vec(),
        )
        .unwrap();

        #[extrinsic_call]
        _(origin.clone(), fingerprint.clone(), location.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &fingerprint), Some(location));

        Ok(())
    }

    #[benchmark]
    fn revoke_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let key_index = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "somekey".as_bytes().to_vec(),
        )
        .unwrap();

        #[extrinsic_call]
        _(origin.clone(), key_index.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &key_index), None);

        Ok(())
    }

    #[benchmark]
    fn issue_encryption_key() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let key = [0; 32];

        #[extrinsic_call]
        _(origin.clone(), key.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedEncryptionKeys::<T>::get(&origin_address), Some(key));

        Ok(())
    }

    impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);
}

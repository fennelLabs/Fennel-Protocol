#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as Keystore;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::format;

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

    #[benchmark(extra)]
    fn announce_a_whole_lotta_keys() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "location".as_bytes().to_vec(),
        )
        .unwrap();
        let fingerprint = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "fingerprint".as_bytes().to_vec(),
        )
        .unwrap();

        // Check how the extrinsic performs with a lot of keys in storage already.
        for i in 0..100_000 {
            let origin = get_origin::<T>("Anakin");
            let fingerprint = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("fingerprint{}", i).as_bytes().to_vec(),
            )
            .unwrap();
            let location = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                format!("location{}", i).as_bytes().to_vec(),
            )
            .unwrap();

            Keystore::<T>::announce_key(origin.into(), fingerprint, location)?;
        }

        #[extrinsic_call]
        announce_key(origin.clone(), fingerprint.clone(), location.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &fingerprint), Some(location));

        Ok(())
    }

    #[benchmark(extra)]
    fn announce_key_with_long_vectors() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");
        let location =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 1000]).unwrap();
        let fingerprint =
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 1000]).unwrap();

        #[extrinsic_call]
        announce_key(origin.clone(), fingerprint.clone(), location.clone());

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

        Keystore::<T>::announce_key(
            origin.clone().into(),
            key_index.clone(),
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 32]).unwrap(),
        )?;

        #[extrinsic_call]
        _(origin.clone(), key_index.clone());

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedKeys::<T>::get(&origin_address, &key_index), None);

        Ok(())
    }

    #[benchmark(extra)]
    fn revoke_one_of_many_keys() -> Result<(), BenchmarkError> {
        let origin = get_origin::<T>("Anakin");

        // Check how the extrinsic performs with a lot of keys in storage already.
        for i in 1..100_000 {
            let origin = get_origin::<T>("Anakin");
            Keystore::<T>::announce_key(
                origin.clone().into(),
                BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
                    format!("key{}", i).as_bytes().to_vec(),
                )
                .unwrap(),
                BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 32]).unwrap(),
            )?;
        }

        let key_index = BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(
            "somekey".as_bytes().to_vec(),
        )
        .unwrap();
        Keystore::<T>::announce_key(
            origin.clone().into(),
            key_index.clone(),
            BoundedVec::<u8, <T as pallet::Config>::MaxSize>::try_from(vec![0; 32]).unwrap(),
        )?;

        #[extrinsic_call]
        revoke_key(origin.clone(), key_index.clone());

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

    #[benchmark(extra)]
    fn issue_a_ton_of_encryption_keys() -> Result<(), BenchmarkError> {
        // Set up a load of encryption keys in storage
        // so we can see what impact it has on runtime.
        for _ in 0..100_000 {
            let origin = get_origin::<T>("Anakin");
            let key = [0; 32];

            Keystore::<T>::issue_encryption_key(origin.clone().into(), key.clone())?;
        }

        let origin = get_origin::<T>("Anakin");
        let key = [0; 32];

        #[extrinsic_call]
        issue_encryption_key(origin, key);

        let origin_address = get_account::<T>("Anakin");
        assert_eq!(IssuedEncryptionKeys::<T>::get(&origin_address), Some(key));

        Ok(())
    }

    impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);
}

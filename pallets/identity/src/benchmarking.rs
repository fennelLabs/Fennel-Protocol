#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as Identity;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use scale_info::prelude::format;
use scale_info::prelude::vec;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn create_identity() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");

        for _ in 0..1000 {
            Identity::<T>::create_identity(anakin.clone().into())?;
        }

        let previous_identity_num = IdentityNumber::<T>::get();

        #[extrinsic_call]
        _(anakin.clone());

        assert_eq!(IdentityNumber::<T>::get(), previous_identity_num + 1);

        Ok(())
    }

    #[benchmark]
    fn revoke_identity() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let identity_num: u32 = IdentityNumber::<T>::get().into();
        Identity::<T>::create_identity(anakin.clone().into())?;

        #[extrinsic_call]
        _(anakin.clone(), identity_num);

        // Check that the identity no longer exists in the IdentityList.
        assert!(!IdentityList::<T>::contains_key(identity_num));

        Ok(())
    }

    #[benchmark]
    fn revoke_identity_heavy_storage() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");

        for _ in 0..1000 {
            Identity::<T>::create_identity(anakin.clone().into())?;
        }

        #[extrinsic_call]
        revoke_identity(anakin.clone(), 301);

        // Check that the identity no longer exists in the IdentityList.
        assert!(!IdentityList::<T>::contains_key(301));

        Ok(())
    }

    #[benchmark]
    fn add_or_update_identity_trait() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize> = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = "Skywalker".as_bytes().to_vec().try_into().unwrap();

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        #[extrinsic_call]
        _(anakin.clone(), identity_index.into(), name.into(), value.into());

        assert!(IdentityList::<T>::contains_key(identity_index));

        Ok(())
    }

    #[benchmark]
    fn add_or_update_long_identity_trait() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        #[extrinsic_call]
        add_or_update_identity_trait(
            anakin.clone(),
            identity_index.into(),
            name.into(),
            value.into(),
        );

        assert!(IdentityList::<T>::contains_key(identity_index));

        Ok(())
    }

    #[benchmark]
    fn add_or_update_many_identity_traits() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        for i in 0..100_000 {
            let name: BoundedVec<u8, T::MaxSize> =
                format!("name{}", i).as_bytes().to_vec().try_into().unwrap();
            let value: BoundedVec<u8, T::MaxSize> =
                format!("value{}", i).as_bytes().to_vec().try_into().unwrap();

            Identity::<T>::add_or_update_identity_trait(
                anakin.clone().into(),
                identity_index.into(),
                name.into(),
                value.into(),
            )?;
        }

        let obiwan = get_origin::<T>("Obi-Wan");
        let new_identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(obiwan.clone().into())?;
        let name: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();

        #[extrinsic_call]
        add_or_update_identity_trait(obiwan, new_identity_index.into(), name.into(), value.into());

        assert!(IdentityList::<T>::contains_key(new_identity_index));

        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize> = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = "Skywalker".as_bytes().to_vec().try_into().unwrap();

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;
        Identity::<T>::add_or_update_identity_trait(
            anakin.clone().into(),
            identity_index.into(),
            name.clone(),
            value.into(),
        )?;

        #[extrinsic_call]
        _(anakin.clone(), identity_index.into(), name.into());

        assert!(IdentityList::<T>::contains_key(identity_index));

        Ok(())
    }

    #[benchmark]
    fn remove_identity_trait_heavy_storage() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        for i in 0..100_000 {
            let name: BoundedVec<u8, T::MaxSize> =
                format!("name{}", i).as_bytes().to_vec().try_into().unwrap();
            let value: BoundedVec<u8, T::MaxSize> =
                format!("value{}", i).as_bytes().to_vec().try_into().unwrap();

            Identity::<T>::add_or_update_identity_trait(
                anakin.clone().into(),
                identity_index.into(),
                name.into(),
                value.into(),
            )?;
        }

        let name: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();

        #[extrinsic_call]
        remove_identity_trait(anakin.clone(), identity_index.into(), name.into());

        assert!(IdentityList::<T>::contains_key(identity_index));

        Ok(())
    }

    #[benchmark]
    fn remove_long_identity_trait() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;
        Identity::<T>::add_or_update_identity_trait(
            anakin.clone().into(),
            identity_index.into(),
            name.clone(),
            value.into(),
        )?;

        #[extrinsic_call]
        remove_identity_trait(anakin.clone(), identity_index.into(), name.into());

        assert!(IdentityList::<T>::contains_key(identity_index));

        Ok(())
    }

    impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
}

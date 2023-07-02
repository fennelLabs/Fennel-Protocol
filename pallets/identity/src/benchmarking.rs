#![cfg(feature = "runtime-benchmarks")]
use super::*;
use crate::Pallet as Identity;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

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

        assert_eq!(RevokedIdentityNumber::<T>::get(), 1);

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

        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));

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
        add_or_update_identity_trait(anakin.clone(), identity_index.into(), name.into(), value.into());

        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));

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

        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));

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

        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));

        Ok(())
    }

    #[benchmark]
    fn sign_for_identity() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let value: BoundedVec<u8, T::MaxSize> =
            "I am your father.".as_bytes().to_vec().try_into().unwrap();

        let identity_index = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        #[extrinsic_call]
        _(anakin.clone(), identity_index.into(), value.into());

        assert_eq!(SignalCount::<T>::get(), identity_index + 1);

        Ok(())
    }

    #[benchmark(extra)]
    fn sign_for_identity_big_vector() -> Result<(), BenchmarkError> {
        let anakin = get_origin::<T>("Anakin");
        let value: BoundedVec<u8, T::MaxSize> = vec![0; 1000].try_into().unwrap();

        let identity_index = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

        #[extrinsic_call]
        sign_for_identity(anakin.clone(), identity_index.into(), value.into());

        assert_eq!(SignalCount::<T>::get(), identity_index + 1);

        Ok(())
    }

    impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
}

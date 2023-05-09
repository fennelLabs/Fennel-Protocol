use super::*;

#[allow(unused)]
use crate::Pallet as Identity;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::BoundedVec;
use frame_system::RawOrigin;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

benchmarks! {
    create_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let previous_identity_num = IdentityNumber::<T>::get();
    }: _(anakin.clone())
    verify {
        assert_eq!(IdentityNumber::<T>::get(), previous_identity_num + 1);
    }

    revoke_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        // create identity to be used for revoking
        let identity_num: u32 = IdentityNumber::<T>::get().into();
        Identity::<T>::create_identity(anakin.clone().into())?;
    }: _(anakin.clone(), identity_num)
    verify {
        // one revoked identity
        assert_eq!(RevokedIdentityNumber::<T>::get(), 1);
    }

    add_or_update_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize>  = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize>  = "Skywalker".as_bytes().to_vec().try_into().unwrap();

        // create identity to be used for placing traits

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), name.into(), value.into())
    verify {
        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));
    }


    remove_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let name: BoundedVec<u8, T::MaxSize> = "name".as_bytes().to_vec().try_into().unwrap();
        let value: BoundedVec<u8, T::MaxSize>  = "Skywalker".as_bytes().to_vec().try_into().unwrap();

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;
        Identity::<T>::add_or_update_identity_trait(anakin.clone().into(), identity_index.into(), name.clone(), value.into())?;

    }: _(anakin.clone(), identity_index.into(), name.into())
    verify {
        let key: T::AccountId = get_account::<T>("Anakin");
        assert!(IdentityList::<T>::contains_key(key));
    }


    sign_for_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let value: BoundedVec<u8, T::MaxSize>  = "I am your father.".as_bytes().to_vec().try_into().unwrap();

        let identity_index = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), value.into())
    verify {
        assert_eq!(SignalCount::<T>::get(), identity_index + 1);
    }
}

impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);

use super::*;

#[allow(unused)]
use crate::Pallet as Identity;
use codec::alloc::string::{String, ToString};
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_support::inherent::Vec;
use frame_system::RawOrigin;

pub fn from_str_to_vec(string: String) -> BoundedVec<u8, T::MaxSize> {
    string.as_bytes().to_vec()
}

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
        let name = from_str_to_vec("name".to_string());
        let value = from_str_to_vec("Skywalker".to_string());

        // create identity to be used for placing traits

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), name, value)


    remove_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let name = from_str_to_vec("name".to_string());
        let value = from_str_to_vec("Skywalker".to_string());

        let identity_index: u32 = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;
        Identity::<T>::add_or_update_identity_trait(anakin.clone().into(), identity_index.into(), name.clone(), value)?;

    }: _(anakin.clone(), identity_index.into(), name)


    sign_for_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let value = from_str_to_vec("I am your father.".to_string());

        let identity_index = IdentityNumber::<T>::get();
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), value.clone())
    verify {
        assert_eq!(SignalCount::<T>::get(), identity_index + 1);
    }
}

impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);

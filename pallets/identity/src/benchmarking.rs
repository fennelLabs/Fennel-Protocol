use super::*;

#[allow(unused)]
use crate::Pallet as Identity;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use frame_support::inherent::Vec;
use codec::alloc::string::{ToString, String};

pub fn from_str_to_vec(string: String) -> Vec<u8> {
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
        let identity_index: u32 = s as u32 % 5_u32;
    }: _(anakin.clone())
    verify {
        assert_eq!(IdentityNumber::<T>::get(), identity_index + 1);
    }

    revoke_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let identity_index: u32 = s as u32 % 5_u32;

        // create identity to be used for revoking
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into())
    verify {
        assert_eq!(RevokedIdentityNumber::<T>::get(), identity_index + 1);
    }

    add_or_update_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let name = from_str_to_vec("name".to_string());
        let value = from_str_to_vec("Skywalker".to_string());
        let identity_index: u32 = s as u32 % 5_u32;

        // create identity to be used for placing traits
        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), name, value)


    remove_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let name = from_str_to_vec("name".to_string());
        let value = from_str_to_vec("Skywalker".to_string());
        let identity_index: u32 = s as u32 % 5_u32;

        Identity::<T>::create_identity(anakin.clone().into())?;
        Identity::<T>::add_or_update_identity_trait(anakin.clone().into(), identity_index.into(), name.clone(), value)?;

    }: _(anakin.clone(), identity_index.into(), name)


    sign_for_identity {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let value = from_str_to_vec("I am your father.".to_string());
        let identity_index: u32 = s as u32 % 5_u32;

        Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), identity_index.into(), value.clone())
    verify {
        assert_eq!(SignalCount::<T>::get(), identity_index + 1);
    }
}

impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
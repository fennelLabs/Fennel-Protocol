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
    }: _(anakin.clone())

    add_or_update_identity_trait {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");

        // create identity to be used for placing traits
        Identity::<T>::create_identity(anakin.clone().into())?;

        let name = from_str_to_vec(s.to_string() + "_name");
        let value = from_str_to_vec(s.to_string() + "_Skywalker");

    }: _(anakin.clone(), 0_u32.into(), name, value)

}

impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);
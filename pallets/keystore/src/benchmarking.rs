use super::*;

#[allow(unused)]
use crate::Pallet as Keystore;
use codec::alloc::string::{String, ToString};
use frame_benchmarking::{
    account as benchmark_account, benchmarks, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_support::inherent::Vec;
use frame_system::RawOrigin;

pub fn from_str_to_vec(string: String) -> Vec<u8> {
    string.as_bytes().to_vec()
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    benchmark_account(name, 0, 0)
}

benchmarks! {
    revoke_key {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        let value = from_str_to_vec("I am your father.".to_string());

    }: _(anakin.clone(), value.clone())
    verify {
        assert_eq!(1, 1);
    }
}

impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);

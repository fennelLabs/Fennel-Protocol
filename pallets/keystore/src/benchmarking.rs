use super::*;

#[allow(unused)]
use crate::Pallet as Signal;
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
    announce_key {
        let s in 0 .. 100;
        let origin = get_origin::<T>("Anakin");
        let location = from_str_to_vec("location".to_string());
        let fingerprint = from_str_to_vec("fingerprint".to_string());

    }: _(origin.clone(), fingerprint.clone(), location.clone())

    issue_key {
        let s in 0 .. 100;
        let origin = get_origin::<T>("Anakin");
        let location = from_str_to_vec("location".to_string());
        let fingerprint = from_str_to_vec("fingerprint".to_string());

    }: _(origin.clone(), fingerprint.clone(), location.clone())

    revoke_key {
        let s in 0 .. 100;
        let origin = get_origin::<T>("Anakin");
        let key_index = from_str_to_vec("somekey".to_string());

    }: _(origin.clone(), key_index.clone())
}

impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);

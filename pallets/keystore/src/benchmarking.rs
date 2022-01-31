use super::*;

#[allow(unused)]
use crate::Pallet as Identity;
use codec::alloc::string::{String, ToString};
use frame_benchmarking::{
    account as benchmark_account, benchmarks, impl_benchmark_test_suite, whitelisted_caller,
};
use frame_support::inherent::Vec;
use frame_system::RawOrigin;

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
    
   
}

impl_benchmark_test_suite!(Identity, crate::mock::new_test_ext(), crate::mock::Test);

//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Trust;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    send_certificate {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)
}

impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);

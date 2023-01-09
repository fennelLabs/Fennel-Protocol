//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Trust;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    send_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    update_rating_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target, 0)

    send_signal {
        let target = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    send_service_signal {
        let service = "TEST".as_bytes().to_vec();
        let url = "TEST".as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), service, url)
}

impl_benchmark_test_suite!(Signal, crate::mock::new_test_ext(), crate::mock::Test);

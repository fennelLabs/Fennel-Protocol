//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Trust;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
    issue_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    revoke_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    remove_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    remove_revoked_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    request_trust {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)

    cancel_trust_request {
        let target: T::AccountId = whitelisted_caller();
        let caller: T::AccountId = whitelisted_caller();
    }: _(RawOrigin::Signed(caller), target)
}

impl_benchmark_test_suite!(Trust, crate::mock::new_test_ext(), crate::mock::Test);

use super::*;

#[allow(unused)]
use crate::Pallet as Lockable;
use frame_benchmarking::{account as benchmark_account, benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

benchmarks! {
    lock_capital {
        let anakin = get_origin::<T>("Anakin");
    }: _(anakin.clone(), 10u32.into())

    extend_lock {
        let anakin = get_origin::<T>("Anakin");
    }: _(anakin.clone(), 10u32.into())

    unlock_all {
        let anakin = get_origin::<T>("Anakin");
    }: _(anakin.clone())
}

impl_benchmark_test_suite!(Lockable, crate::mock::new_test_ext(), crate::mock::Test);

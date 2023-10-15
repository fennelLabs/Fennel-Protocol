//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Certificate;

use frame_benchmarking::{account as benchmark_account, v2::*};
use frame_support::{sp_runtime::traits::Bounded, traits::Currency};
use frame_system::RawOrigin;
use scale_info::prelude::vec;

pub fn get_account<T: Config>(name: &'static str) -> T::AccountId {
    let account: T::AccountId = benchmark_account(name, 0, 0);
    account
}

pub fn get_origin<T: Config>(name: &'static str) -> RawOrigin<T::AccountId> {
    RawOrigin::Signed(get_account::<T>(name))
}

#[benchmarks]
mod benchmarks {
    use super::*;
    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[benchmark]
    fn send_certificate() -> Result<(), BenchmarkError> {
        let target = get_account::<T>("James");
        let caller = get_origin::<T>("Spock");
        let caller_account: T::AccountId = get_account::<T>("Spock");

        T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

        #[extrinsic_call]
        _(caller, target);

        let caller_account_id: T::AccountId = get_account::<T>("Spock");
        let target_account_id: T::AccountId = get_account::<T>("James");
        assert!(CertificateList::<T>::contains_key(caller_account_id, target_account_id));

        Ok(())
    }

    #[benchmark]
    fn send_certificate_heavy_storage() -> Result<(), BenchmarkError> {
        let caller = get_origin::<T>("Leonard");
        let caller_account: T::AccountId = get_account::<T>("Leonard");

        T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

        // Generate a larger set of certificates.
        for i in 0..10000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
        }

        let target = get_account::<T>("Montgomery");
        #[extrinsic_call]
        send_certificate(caller, target);

        let caller_account_id: T::AccountId = get_account::<T>("Leonard");
        let target_account_id: T::AccountId = get_account::<T>("Montgomery");
        assert!(CertificateList::<T>::contains_key(caller_account_id, target_account_id));

        Ok(())
    }

    #[benchmark]
    fn revoke_certificate() -> Result<(), BenchmarkError> {
        let caller = get_origin::<T>("Leonard");
        let caller_account: T::AccountId = get_account::<T>("Leonard");
        let target = get_account::<T>("Montgomery");

        T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;

        #[extrinsic_call]
        _(caller, target);

        let caller_account_id: T::AccountId = get_account::<T>("Leonard");
        let target_account_id: T::AccountId = get_account::<T>("Montgomery");
        assert!(!CertificateList::<T>::contains_key(caller_account_id, target_account_id));

        Ok(())
    }

    #[benchmark]
    fn revoke_certificate_heavy_storage() -> Result<(), BenchmarkError> {
        let caller = get_origin::<T>("Leonard");
        let caller_account: T::AccountId = get_account::<T>("Leonard");

        T::Currency::make_free_balance_be(&caller_account, BalanceOf::<T>::max_value());

        // Generate a larger set of certificates.
        for i in 0..10000 {
            let target: T::AccountId = benchmark_account("target", i, 0);
            Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
        }

        let target = get_account::<T>("Montgomery");
        Certificate::<T>::send_certificate(caller.clone().into(), target.clone())?;
        #[extrinsic_call]
        revoke_certificate(caller, target);

        let caller_account_id: T::AccountId = get_account::<T>("Leonard");
        let target_account_id: T::AccountId = get_account::<T>("Montgomery");
        assert!(!CertificateList::<T>::contains_key(caller_account_id, target_account_id));

        Ok(())
    }

    impl_benchmark_test_suite!(Certificate, crate::mock::new_test_ext(), crate::mock::Test);
}

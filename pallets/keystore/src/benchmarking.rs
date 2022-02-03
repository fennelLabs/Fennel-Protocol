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

/*benchmarks! {
    retrieve_key {
        let s in 0 .. 100;
        let anakin = get_origin::<T>("Anakin");
        //let identity_index: u32 = s as u32 % 5_u32;
        let location = from_str_to_vec("Tatooine".to_string());
        // create identity to be used for revoking
        //Identity::<T>::create_identity(anakin.clone().into())?;

    }: _(anakin.clone(), location)
    verify {
        assert_eq!(1, 1);
    }
}*/

impl_benchmark_test_suite!(Keystore, crate::mock::new_test_ext(), crate::mock::Test);


//! Autogenerated weights for pallet_signal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-18, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kusanagi`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain
// fennel-local
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_signal
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/signal/src/weights.rs
// --template=scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_signal.
pub trait WeightInfo {
    fn set_signal_parameter() -> Weight;
	fn send_rating_signal() -> Weight;
	fn update_rating_signal() -> Weight;
	fn revoke_rating_signal() -> Weight;
    fn send_whiteflag_rating_signal() -> Weight;
	fn send_signal() -> Weight;
    fn update_whiteflag_rating_signal() -> Weight;
	fn send_service_signal() -> Weight;
    fn revoke_whiteflag_rating_signal() -> Weight;
}

/// Weight functions for pallet_trust.
pub struct SubstrateWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeights<T> {
    fn set_signal_parameter() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn send_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn send_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn update_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn update_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn revoke_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn revoke_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn send_signal() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(12_000_000)
	}
	fn send_service_signal() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(12_000_000)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn set_signal_parameter() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn send_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn send_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn update_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn update_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn revoke_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

    fn revoke_whiteflag_rating_signal() -> Weight {
        Weight::from_ref_time(7_000_000)
    }

	// Storage: TrustModule TrustIssuance (r:1 w:1)
	// Storage: TrustModule CurrentIssued (r:1 w:1)
	fn send_signal() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(12_000_000)
	}
	fn send_service_signal() -> Weight {
		// Minimum execution time: 12_000 nanoseconds.
		Weight::from_ref_time(12_000_000)
	}
}

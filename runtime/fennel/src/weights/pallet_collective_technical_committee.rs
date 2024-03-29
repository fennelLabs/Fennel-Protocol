//! Autogenerated weights for `pallet_collective`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-27, STEPS: `100`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_collective
// --extrinsic=*
// --steps=100
// --repeat=100
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32) -> Weight;
    fn execute(b: u32, m: u32) -> Weight;
    fn propose_execute(b: u32, m: u32) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight;
    fn vote(m: u32) -> Weight;
    fn close_early_disapproved(m: u32, p: u32) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight;
    fn close_disapproved(m: u32, p: u32) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32) -> Weight;
    fn disapprove_proposal(p: u32) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
    /// Storage: TechnicalCommittee Members (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:99 w:99)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Prime (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `m` is `[0, 100]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `p` is `[0, 100]`.
    /// The range of component `m` is `[0, 100]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `p` is `[0, 100]`.
    fn set_members(m: u32, _n: u32, p: u32) -> Weight {
        Weight::from_parts(14_000_000, 10056)
            // Standard Error: 9_418
            .saturating_add(Weight::from_parts(3_379_570, 0).saturating_mul(m.into()))
            // Standard Error: 9_418
            .saturating_add(Weight::from_parts(6_399_549, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
            .saturating_add(Weight::from_parts(0, 1966).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 4360).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn execute(b: u32, m: u32) -> Weight {
        Weight::from_parts(15_003_467, 1555)
            // Standard Error: 16
            .saturating_add(Weight::from_parts(1_600, 0).saturating_mul(b.into()))
            // Standard Error: 174
            .saturating_add(Weight::from_parts(10_571, 0).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn propose_execute(b: u32, m: u32) -> Weight {
        Weight::from_parts(17_920_696, 3535)
            // Standard Error: 13
            .saturating_add(Weight::from_parts(1_141, 0).saturating_mul(b.into()))
            // Standard Error: 139
            .saturating_add(Weight::from_parts(12_192, 0).saturating_mul(m.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalCount (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[2, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[2, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_parts(22_174_770, 3752)
            // Standard Error: 124
            .saturating_add(Weight::from_parts(8_479, 0).saturating_mul(b.into()))
            // Standard Error: 1_290
            .saturating_add(Weight::from_parts(3_719, 0).saturating_mul(m.into()))
            // Standard Error: 1_290
            .saturating_add(Weight::from_parts(195_512, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[5, 100]`.
    /// The range of component `m` is `[5, 100]`.
    fn vote(m: u32) -> Weight {
        Weight::from_parts(32_840_542, 4272)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_disapproved(m: u32, p: u32) -> Weight {
        Weight::from_parts(34_821_808, 3842)
            // Standard Error: 1_245
            .saturating_add(Weight::from_parts(212_236, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_parts(36_651_519, 4011)
            // Standard Error: 209
            .saturating_add(Weight::from_parts(5_651, 0).saturating_mul(b.into()))
            // Standard Error: 2_164
            .saturating_add(Weight::from_parts(259_627, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Prime (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_disapproved(m: u32, p: u32) -> Weight {
        Weight::from_parts(28_072_648, 3862)
            // Standard Error: 670
            .saturating_add(Weight::from_parts(34_601, 0).saturating_mul(m.into()))
            // Standard Error: 655
            .saturating_add(Weight::from_parts(176_075, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Prime (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_approved(b: u32, m: u32, p: u32) -> Weight {
        Weight::from_parts(39_156_257, 4031)
            // Standard Error: 131
            .saturating_add(Weight::from_parts(4_530, 0).saturating_mul(b.into()))
            // Standard Error: 1_361
            .saturating_add(Weight::from_parts(231_117, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `p` is `[1, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn disapprove_proposal(p: u32) -> Weight {
        Weight::from_parts(16_452_775, 1711)
            // Standard Error: 338
            .saturating_add(Weight::from_parts(150_933, 0).saturating_mul(p.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    /// Storage: TechnicalCommittee Members (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:99 w:99)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Prime (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `m` is `[0, 100]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `p` is `[0, 100]`.
    /// The range of component `m` is `[0, 100]`.
    /// The range of component `n` is `[0, 100]`.
    /// The range of component `p` is `[0, 100]`.
    fn set_members(m: u32, _n: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + m * (3232 ±0) + p * (3190 ±0)`
        //  Estimated: `10056 + m * (1966 ±5) + p * (4360 ±5)`
        // Minimum execution time: 14_000_000 picoseconds.
        Weight::from_parts(14_000_000, 10056)
            // Standard Error: 9_418
            .saturating_add(Weight::from_parts(3_379_570, 0).saturating_mul(m.into()))
            // Standard Error: 9_418
            .saturating_add(Weight::from_parts(6_399_549, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
            .saturating_add(Weight::from_parts(0, 1966).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 4360).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn execute(b: u32, m: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `71 + m * (32 ±0)`
        //  Estimated: `1555 + m * (32 ±0)`
        // Minimum execution time: 15_000_000 picoseconds.
        Weight::from_parts(15_003_467, 1555)
            // Standard Error: 16
            .saturating_add(Weight::from_parts(1_600, 0).saturating_mul(b.into()))
            // Standard Error: 174
            .saturating_add(Weight::from_parts(10_571, 0).saturating_mul(m.into()))
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[1, 100]`.
    fn propose_execute(b: u32, m: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `71 + m * (32 ±0)`
        //  Estimated: `3535 + m * (32 ±0)`
        // Minimum execution time: 18_000_000 picoseconds.
        Weight::from_parts(17_920_696, 3535)
            // Standard Error: 13
            .saturating_add(Weight::from_parts(1_141, 0).saturating_mul(b.into()))
            // Standard Error: 139
            .saturating_add(Weight::from_parts(12_192, 0).saturating_mul(m.into()))
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalCount (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalCount (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[2, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[2, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `362 + m * (32 ±0) + p * (36 ±0)`
        //  Estimated: `3752 + m * (33 ±0) + p * (36 ±0)`
        // Minimum execution time: 24_000_000 picoseconds.
        Weight::from_parts(22_174_770, 3752)
            // Standard Error: 124
            .saturating_add(Weight::from_parts(8_479, 0).saturating_mul(b.into()))
            // Standard Error: 1_290
            .saturating_add(Weight::from_parts(3_719, 0).saturating_mul(m.into()))
            // Standard Error: 1_290
            .saturating_add(Weight::from_parts(195_512, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 33).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// The range of component `m` is `[5, 100]`.
    /// The range of component `m` is `[5, 100]`.
    fn vote(m: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `808 + m * (64 ±0)`
        //  Estimated: `4272 + m * (64 ±0)`
        // Minimum execution time: 21_000_000 picoseconds.
        Weight::from_parts(32_840_542, 4272)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 64).saturating_mul(m.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_disapproved(m: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `395 + m * (64 ±0) + p * (36 ±0)`
        //  Estimated: `3842 + m * (65 ±0) + p * (36 ±0)`
        // Minimum execution time: 25_000_000 picoseconds.
        Weight::from_parts(34_821_808, 3842)
            // Standard Error: 1_245
            .saturating_add(Weight::from_parts(212_236, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `629 + b * (1 ±0) + m * (64 ±0) + p * (41 ±0)`
        //  Estimated: `4011 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
        // Minimum execution time: 36_000_000 picoseconds.
        Weight::from_parts(36_651_519, 4011)
            // Standard Error: 209
            .saturating_add(Weight::from_parts(5_651, 0).saturating_mul(b.into()))
            // Standard Error: 2_164
            .saturating_add(Weight::from_parts(259_627, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Prime (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_disapproved(m: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `414 + m * (64 ±0) + p * (36 ±0)`
        //  Estimated: `3862 + m * (65 ±0) + p * (36 ±0)`
        // Minimum execution time: 28_000_000 picoseconds.
        Weight::from_parts(28_072_648, 3862)
            // Standard Error: 670
            .saturating_add(Weight::from_parts(34_601, 0).saturating_mul(m.into()))
            // Standard Error: 655
            .saturating_add(Weight::from_parts(176_075, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 65).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 36).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Voting (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee Members (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Members (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Prime (r:1 w:0)
    /// Proof Skipped: TechnicalCommittee Prime (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee ProposalOf (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    /// The range of component `b` is `[2, 1024]`.
    /// The range of component `m` is `[4, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn close_approved(b: u32, m: u32, p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `649 + b * (1 ±0) + m * (64 ±0) + p * (41 ±0)`
        //  Estimated: `4031 + b * (1 ±0) + m * (66 ±0) + p * (40 ±0)`
        // Minimum execution time: 39_000_000 picoseconds.
        Weight::from_parts(39_156_257, 4031)
            // Standard Error: 131
            .saturating_add(Weight::from_parts(4_530, 0).saturating_mul(b.into()))
            // Standard Error: 1_361
            .saturating_add(Weight::from_parts(231_117, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(b.into()))
            .saturating_add(Weight::from_parts(0, 66).saturating_mul(m.into()))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(p.into()))
    }
    /// Storage: TechnicalCommittee Proposals (r:1 w:1)
    /// Proof Skipped: TechnicalCommittee Proposals (max_values: Some(1), max_size: None, mode:
    /// Measured) Storage: TechnicalCommittee Voting (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee Voting (max_values: None, max_size: None, mode: Measured)
    /// Storage: TechnicalCommittee ProposalOf (r:0 w:1)
    /// Proof Skipped: TechnicalCommittee ProposalOf (max_values: None, max_size: None, mode:
    /// Measured) The range of component `p` is `[1, 100]`.
    /// The range of component `p` is `[1, 100]`.
    fn disapprove_proposal(p: u32) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `226 + p * (32 ±0)`
        //  Estimated: `1711 + p * (32 ±0)`
        // Minimum execution time: 14_000_000 picoseconds.
        Weight::from_parts(16_452_775, 1711)
            // Standard Error: 338
            .saturating_add(Weight::from_parts(150_933, 0).saturating_mul(p.into()))
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 32).saturating_mul(p.into()))
    }
}

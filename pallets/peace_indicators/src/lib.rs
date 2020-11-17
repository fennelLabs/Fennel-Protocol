#![cfg_attr(not(feature = "std"), no_std)]

use crate::dispatch::Vec;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, storage::IterableStorageMap, traits::Get,
};
use frame_system::{ensure_root, ensure_signed};
use codec::{Encode, Decode};
use frame_support::sp_runtime::RuntimeDebug;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        // Learn more about declaring storage items:
        // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
        PeaceIndicators get(fn indicators): map hasher(blake2_128_concat) u32 => Vec<u8>;
        Aggregator get(fn aggregate_attests): map hasher(blake2_128_concat) u32 => EipAttestation<T>;
        Attestation get(fn attestations): double_map hasher(blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => bool; 
        IndicatorSize get(fn size): Option<u32>;
    }
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug)]
pub struct EipAttestation<T: Trait> {
    positive_count: u32,
    negative_count: u32,
    who_positive: Vec<T::AccountId>, 
    who_negative: Vec<T::AccountId>
}

impl<T: Trait> Default for EipAttestation<T> {
    fn default() -> EipAttestation<T> {
        EipAttestation {
            positive_count: 0,
            negative_count: 0,
            who_positive: Vec::new(),
            who_negative: Vec::new(),
        }
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        IndicatorStored(u32),
        InvestigationUnderway(u32),
        AttestedBy(AccountId, u32, bool),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Removes all current indicators and instantiates the set with new indicators
        /// Drains the aggregate data
        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn sudo_submit_new_indicator_set(origin, indicators: Vec<Vec<u8>>) -> dispatch::DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://substrate.dev/docs/en/knowledgebase/runtime/origin
            let _who = ensure_root(origin)?;
            Self::clear_all();

            for (idx, indicator) in indicators.iter().enumerate() {
                // Update storage.
                PeaceIndicators::insert(idx as u32, indicator);
                Self::deposit_event(RawEvent::IndicatorStored(idx as u32));
            }
            IndicatorSize::put(indicators.len() as u32);
            Ok(())
        }

        #[weight = 10_000 + T::DbWeight::get().writes(1)]
        pub fn raise_investigation(origin, indicator: u32) -> dispatch::DispatchResult {
            let _who = ensure_root(origin)?;
            Self::deposit_event(RawEvent::InvestigationUnderway(indicator));
            Ok(())
        }

        #[weight = 100 + T::DbWeight::get().writes(1)]
        pub fn attest_positive(origin, indicator: u32) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            <Attestation<T>>::insert(&who, indicator, true);
            Self::increment_aggregator(indicator, true, &who);
            Self::deposit_event(RawEvent::AttestedBy(who, indicator, true));
            Ok(())
        }

        #[weight = 100 + T::DbWeight::get().writes(1)]
        pub fn attest_negative(origin, indicator: u32) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            <Attestation<T>>::insert(&who, indicator, false);
            Self::increment_aggregator(indicator, false, &who);
            Self::deposit_event(RawEvent::AttestedBy(who, indicator, false));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    fn increment_aggregator(indicator: u32, affirmation: bool, who: &T::AccountId) {
        if <Aggregator<T>>::contains_key(indicator) {
            <Aggregator<T>>::mutate(indicator, |q| {
                if affirmation {
                    q.positive_count += 1;
                    q.who_positive.push(who.clone());
                } else {
                    q.negative_count += 1;
                    q.who_negative.push(who.clone());
                }
            });
        } else {
            <Aggregator<T>>::insert(indicator, EipAttestation::default());
            <Aggregator<T>>::mutate(indicator, |q| {
                if affirmation {
                    q.positive_count += 1;
                    q.who_positive.push(who.clone());
                } else {
                    q.negative_count += 1;
                    q.who_negative.push(who.clone());
                }
            });
        }
    }

    fn clear_all() {
        let mut i = 0;
        for _ in PeaceIndicators::iter() {
            i += 1;
        }
        for idx in 0..i {
            PeaceIndicators::remove(idx);
            <Aggregator<T>>::remove(idx);
        }
    }
}

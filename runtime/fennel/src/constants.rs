use frame_support::{
    parameter_types,
    sp_runtime::{Perbill, Permill},
    traits::{ConstU128, ConstU32},
    PalletId,
};

pub type Balance = u128;
pub type BlockNumber = u32;

pub const TOKEN_DECIMALS: u8 = 8;

pub const MILLISECS_PER_BLOCK: u64 = 12000;

pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

pub mod currency {
    use super::Balance;

    pub const UNITS: Balance = 10u128.saturating_pow(super::TOKEN_DECIMALS as u32);
    pub const DOLLARS: Balance = UNITS;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;

    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * 20 * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
    }
}

pub type Fifty = ConstU32<50>;
pub type Hundred = ConstU32<100>;

pub type SchedulerMaxScheduledPerBlock = Fifty;

pub type PreimageBaseDeposit = ConstU128<{ currency::deposit(10, 64) }>;
pub type PreimageByteDeposit = ConstU128<{ currency::deposit(0, 1) }>;

parameter_types! {
    pub const CouncilMaxProposals: u32 = 100;
    pub const CouncilMaxMembers: u32 = 100;
}

pub type CouncilMotionDuration = ConstU32<{ 5 * DAYS }>;

parameter_types! {
    pub const TCMaxProposals: u32 = 100;
    pub const TCMaxMembers: u32 = 100;
}

pub type TCMotionDuration = ConstU32<{ 5 * DAYS }>;

pub type LaunchPeriod = ConstU32<{ 5 * MINUTES }>;
pub type VotingPeriod = ConstU32<{ 5 * MINUTES }>;
pub type FastTrackVotingPeriod = ConstU32<{ 5 * MINUTES }>;
pub type EnactmentPeriod = ConstU32<{ 10 * MINUTES }>;
pub type CooloffPeriod = ConstU32<{ 5 * MINUTES }>;
pub type MinimumDeposit = ConstU128<{ 100 * currency::deposit(5, 0) }>;
pub type SpendPeriod = ConstU32<{ 10 * MINUTES }>;
pub type DemocracyMaxVotes = ConstU32<100>;
pub type DemocracyMaxProposals = Hundred;

pub const TREASURY_PALLET_ID: PalletId = PalletId(*b"py/trsry");

parameter_types! {

    pub const TreasuryPalletId: PalletId = TREASURY_PALLET_ID;

    pub const ProposalBondPercent: Permill = Permill::from_percent(5);

    pub const Burn: Permill = Permill::zero();
}

pub type MaxApprovals = ConstU32<64>;

pub type ProposalBondMinimum = ConstU128<{ 100 * currency::DOLLARS }>;

pub type ProposalBondMaximum = ConstU128<{ 1_000 * currency::DOLLARS }>;

parameter_types! {
    pub const NeverDepositIntoId: PalletId = PalletId(*b"NeverDep");
    pub const MessagesMaxPayloadSizeBytes: u32 = 1024 * 50;
}

impl Clone for MessagesMaxPayloadSizeBytes {
    fn clone(&self) -> Self {
        MessagesMaxPayloadSizeBytes {}
    }
}

parameter_types! {
    pub const Ss58Prefix: u16 = 42;
}

parameter_types! {
    pub const MaxItemizedPageSizeBytes: u32 = 64 * 1024;
    pub const MaxPaginatedPageSizeBytes: u32 = 1024;
    pub const MaxItemizedBlobSizeBytes: u32 = 1024;
}

impl Default for MaxItemizedPageSizeBytes {
    fn default() -> Self {
        Self
    }
}

impl Default for MaxPaginatedPageSizeBytes {
    fn default() -> Self {
        Self
    }
}

impl Clone for MaxItemizedBlobSizeBytes {
    fn clone(&self) -> Self {
        MaxItemizedBlobSizeBytes {}
    }
}

impl Eq for MaxItemizedBlobSizeBytes {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for MaxItemizedBlobSizeBytes {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl sp_std::fmt::Debug for MaxItemizedBlobSizeBytes {
    #[cfg(feature = "std")]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }

    #[cfg(not(feature = "std"))]
    fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
        Ok(())
    }
}

parameter_types! {
    pub const CapacityPerToken: Perbill = Perbill::from_percent(2);
}

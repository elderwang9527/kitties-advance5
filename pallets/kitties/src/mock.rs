use crate as pallet_kitties;
pub use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, StorageInfo},
	weights::{
		constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
		IdentityFee, Weight,
	},
	StorageValue,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
pub type Balance = u128;

use pallet_randomness_collective_flip;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		KittiesModule: pallet_kitties::{Pallet, Call, Storage, Event<T>},
		Randomness: pallet_randomness_collective_flip::{Pallet, Storage},
		Balances: pallet_balances,
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitties::Config for Test {
	type Event = Event;
	type Randomness = Randomness;
	type KittyIndex = u32;
	type KtReserve = KtReserve;
	type Currency = Balances;
	type MaxLength = ConstU32<64>;
}

impl pallet_randomness_collective_flip::Config for Test {}

parameter_types! {
	// pub const BlockHashCount: BlockNumber = 2400;
	// pub const Version: RuntimeVersion = VERSION;
	// /// We allow for 2 seconds of compute with a 6 second average block time.
	// pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
	// 	::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
	// pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
	// 	::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	// pub const SS58Prefix: u8 = 42;
	pub const KtReserve: u64 =1_000;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

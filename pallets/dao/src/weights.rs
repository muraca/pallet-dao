
//! Autogenerated weights for `pallet_dao`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-24, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `castigo.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_dao
// --extrinsic
// *
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output
// pallets/dao/src/weights.rs
// --template
// .maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_dao`.
pub trait WeightInfo {
	fn join() -> Weight;
	fn leave() -> Weight;
	fn commit() -> Weight;
	fn reveal() -> Weight;
}

/// Weights for `pallet_dao` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Dao::Members` (r:1 w:1)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CounterForMembers` (r:1 w:1)
	/// Proof: `Dao::CounterForMembers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Dao::MembershipCost` (r:1 w:0)
	/// Proof: `Dao::MembershipCost` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn join() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `6196`
		// Minimum execution time: 51_000_000 picoseconds.
		Weight::from_parts(52_000_000, 6196)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:1)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CounterForMembers` (r:1 w:1)
	/// Proof: `Dao::CounterForMembers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn leave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119`
		//  Estimated: `3497`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3497)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:0)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentPhase` (r:1 w:0)
	/// Proof: `Dao::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Dao::Commitments` (r:0 w:1)
	/// Proof: `Dao::Commitments` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn commit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `4714`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4714)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:0)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentPhase` (r:1 w:0)
	/// Proof: `Dao::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Dao::Commitments` (r:1 w:1)
	/// Proof: `Dao::Commitments` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentRandom` (r:1 w:1)
	/// Proof: `Dao::CurrentRandom` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn reveal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3546`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 3546)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Dao::Members` (r:1 w:1)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CounterForMembers` (r:1 w:1)
	/// Proof: `Dao::CounterForMembers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Dao::MembershipCost` (r:1 w:0)
	/// Proof: `Dao::MembershipCost` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn join() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `249`
		//  Estimated: `6196`
		// Minimum execution time: 51_000_000 picoseconds.
		Weight::from_parts(52_000_000, 6196)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:1)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CounterForMembers` (r:1 w:1)
	/// Proof: `Dao::CounterForMembers` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn leave() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119`
		//  Estimated: `3497`
		// Minimum execution time: 13_000_000 picoseconds.
		Weight::from_parts(13_000_000, 3497)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:0)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentPhase` (r:1 w:0)
	/// Proof: `Dao::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Dao::Commitments` (r:0 w:1)
	/// Proof: `Dao::Commitments` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn commit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240`
		//  Estimated: `4714`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(31_000_000, 4714)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Dao::Members` (r:1 w:0)
	/// Proof: `Dao::Members` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentPhase` (r:1 w:0)
	/// Proof: `Dao::CurrentPhase` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Dao::Commitments` (r:1 w:1)
	/// Proof: `Dao::Commitments` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Dao::CurrentRandom` (r:1 w:1)
	/// Proof: `Dao::CurrentRandom` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn reveal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `3546`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(15_000_000, 3546)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}

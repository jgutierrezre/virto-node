// Copyright Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::generic`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm6`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("statemine-dev"), DB CACHE: 1024

// Executed Command:
// ./artifacts/polkadot-parachain
// benchmark
// pallet
// --template=./templates/xcm-bench-template.hbs
// --chain=statemine-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_xcm_benchmarks::generic
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/assets/statemine/src/weights/xcm/pallet_xcm_benchmarks_generic.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::generic`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 340_875_000 picoseconds.
		Weight::from_parts(349_790_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_898_000 picoseconds.
		Weight::from_parts(3_994_000, 0)
	}
	// Storage: PolkadotXcm Queries (r:1 w:0)
	// Proof Skipped: PolkadotXcm Queries (max_values: None, max_size: None, mode: Measured)
	pub fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `69`
		//  Estimated: `3534`
		// Minimum execution time: 10_760_000 picoseconds.
		Weight::from_parts(11_070_000, 3534)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	pub fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_340_000 picoseconds.
		Weight::from_parts(13_787_000, 0)
	}
	pub fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_205_000 picoseconds.
		Weight::from_parts(4_328_000, 0)
	}
	pub fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_675_000 picoseconds.
		Weight::from_parts(2_761_000, 0)
	}
	pub fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_648_000 picoseconds.
		Weight::from_parts(2_723_000, 0)
	}
	pub fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_675_000 picoseconds.
		Weight::from_parts(2_866_000, 0)
	}
	pub fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_584_000 picoseconds.
		Weight::from_parts(3_679_000, 0)
	}
	pub fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_686_000 picoseconds.
		Weight::from_parts(2_750_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 24_994_000 picoseconds.
		Weight::from_parts(25_553_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: PolkadotXcm AssetTraps (r:1 w:1)
	// Proof Skipped: PolkadotXcm AssetTraps (max_values: None, max_size: None, mode: Measured)
	pub fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `126`
		//  Estimated: `3591`
		// Minimum execution time: 15_960_000 picoseconds.
		Weight::from_parts(16_240_000, 3591)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	pub fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_659_000 picoseconds.
		Weight::from_parts(2_732_000, 0)
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 28_083_000 picoseconds.
		Weight::from_parts(28_531_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: PolkadotXcm VersionNotifyTargets (r:0 w:1)
	// Proof Skipped: PolkadotXcm VersionNotifyTargets (max_values: None, max_size: None, mode: Measured)
	pub fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_884_000 picoseconds.
		Weight::from_parts(4_996_000, 0)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 383_046_000 picoseconds.
		Weight::from_parts(397_796_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 118_811_000 picoseconds.
		Weight::from_parts(121_240_000, 0)
	}
	pub fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_406_000 picoseconds.
		Weight::from_parts(12_637_000, 0)
	}
	pub fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_758_000 picoseconds.
		Weight::from_parts(2_831_000, 0)
	}
	pub fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_703_000 picoseconds.
		Weight::from_parts(2_798_000, 0)
	}
	pub fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_892_000 picoseconds.
		Weight::from_parts(2_930_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 28_960_000 picoseconds.
		Weight::from_parts(29_596_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_303_000 picoseconds.
		Weight::from_parts(5_444_000, 0)
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: PolkadotXcm SupportedVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SupportedVersion (max_values: None, max_size: None, mode: Measured)
	// Storage: PolkadotXcm VersionDiscoveryQueue (r:1 w:1)
	// Proof Skipped: PolkadotXcm VersionDiscoveryQueue (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: PolkadotXcm SafeXcmVersion (r:1 w:0)
	// Proof Skipped: PolkadotXcm SafeXcmVersion (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem HostConfiguration (r:1 w:0)
	// Proof Skipped: ParachainSystem HostConfiguration (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: ParachainSystem PendingUpwardMessages (r:1 w:1)
	// Proof Skipped: ParachainSystem PendingUpwardMessages (max_values: Some(1), max_size: None, mode: Measured)
	pub fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `75`
		//  Estimated: `3540`
		// Minimum execution time: 25_221_000 picoseconds.
		Weight::from_parts(25_842_000, 3540)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	pub fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_734_000 picoseconds.
		Weight::from_parts(2_787_000, 0)
	}
	pub fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_625_000 picoseconds.
		Weight::from_parts(2_692_000, 0)
	}
	pub fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_638_000 picoseconds.
		Weight::from_parts(2_705_000, 0)
	}
	pub fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_666_000 picoseconds.
		Weight::from_parts(2_738_000, 0)
	}
	pub fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_857_000 picoseconds.
		Weight::from_parts(2_966_000, 0)
	}
}

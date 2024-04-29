// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_xyk_liquidity_mining`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-04-08, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-xyk-liquidity-mining
// --output=xyk_lm.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_xyk_liquidity_mining`.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xyk_liquidity_mining::weights::WeightInfo for HydraWeight<T> {
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:3 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:4 w:4)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	fn create_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3099`
		//  Estimated: `11402`
		// Minimum execution time: 345_032_000 picoseconds.
		Weight::from_parts(347_056_000, 11402)
			.saturating_add(T::DbWeight::get().reads(19))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn update_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4314`
		//  Estimated: `6156`
		// Minimum execution time: 121_473_000 picoseconds.
		Weight::from_parts(122_923_000, 6156)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn terminate_global_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3811`
		//  Estimated: `6196`
		// Minimum execution time: 203_724_000 picoseconds.
		Weight::from_parts(204_983_000, 6196)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::FarmSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::FarmSequencer` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	fn create_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1548`
		//  Estimated: `3670`
		// Minimum execution time: 55_477_000 picoseconds.
		Weight::from_parts(56_139_000, 3670)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn update_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4796`
		//  Estimated: `6156`
		// Minimum execution time: 149_365_000 picoseconds.
		Weight::from_parts(150_999_000, 6156)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn stop_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4630`
		//  Estimated: `6156`
		// Minimum execution time: 143_827_000 picoseconds.
		Weight::from_parts(144_970_000, 6156)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:0)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn terminate_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4677`
		//  Estimated: `6156`
		// Minimum execution time: 114_877_000 picoseconds.
		Weight::from_parts(115_998_000, 6156)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:4)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:0)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::DepositSequencer` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::DepositSequencer` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::CollectionMaxSupply` (r:1 w:0)
	/// Proof: `Uniques::CollectionMaxSupply` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:0 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	fn deposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5965`
		//  Estimated: `16488`
		// Minimum execution time: 256_115_000 picoseconds.
		Weight::from_parts(257_670_000, 16488)
			.saturating_add(T::DbWeight::get().reads(24))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:4 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:0)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	fn redeposit_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6894`
		//  Estimated: `11322`
		// Minimum execution time: 194_931_000 picoseconds.
		Weight::from_parts(196_752_000, 11322)
			.saturating_add(T::DbWeight::get().reads(16))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Uniques::Asset` (r:1 w:0)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:3 w:3)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6769`
		//  Estimated: `8739`
		// Minimum execution time: 205_147_000 picoseconds.
		Weight::from_parts(207_113_000, 8739)
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: `Uniques::Asset` (r:1 w:1)
	/// Proof: `Uniques::Asset` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::Deposit` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::Deposit` (`max_values`: None, `max_size`: Some(413), added: 2888, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:2 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:5 w:5)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:3 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:2 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:4 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:1 w:0)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Class` (r:1 w:1)
	/// Proof: `Uniques::Class` (`max_values`: None, `max_size`: Some(190), added: 2665, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::Account` (r:0 w:1)
	/// Proof: `Uniques::Account` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `Uniques::ItemPriceOf` (r:0 w:1)
	/// Proof: `Uniques::ItemPriceOf` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn withdraw_shares() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6442`
		//  Estimated: `13905`
		// Minimum execution time: 439_033_000 picoseconds.
		Weight::from_parts(442_308_000, 13905)
			.saturating_add(T::DbWeight::get().reads(28))
			.saturating_add(T::DbWeight::get().writes(16))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::ActiveYieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::ActiveYieldFarm` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::YieldFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::YieldFarm` (`max_values`: None, `max_size`: Some(226), added: 2701, mode: `MaxEncodedLen`)
	/// Storage: `XYKWarehouseLM::GlobalFarm` (r:1 w:1)
	/// Proof: `XYKWarehouseLM::GlobalFarm` (`max_values`: None, `max_size`: Some(205), added: 2680, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:1 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:2 w:2)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Oracles` (r:1 w:0)
	/// Proof: `EmaOracle::Oracles` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:1 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn resume_yield_farm() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5088`
		//  Estimated: `6156`
		// Minimum execution time: 145_447_000 picoseconds.
		Weight::from_parts(147_388_000, 6156)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
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

//! Autogenerated weights for pallet_lbp
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-06, STEPS: 10, REPEAT: 30, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet-lbp
// --output=lbp.rs
// --extrinsic=*

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_lbp.
pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn update_pool_data() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn sell() -> Weight;
	fn buy() -> Weight;
	fn router_execution_sell(c: u32, e: u32) -> Weight;
	fn router_execution_buy(c: u32, e: u32) -> Weight;
}

/// Weights for pallet_lbp using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn create_pool() -> Weight {
        // Minimum execution time: 141_654 nanoseconds.
        Weight::from_ref_time(143_331_000 as u64)            .saturating_add(T::DbWeight::get().reads(12 as u64))
            .saturating_add(T::DbWeight::get().writes(8 as u64))
    }
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:2)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
    fn update_pool_data() -> Weight {
        // Minimum execution time: 30_269 nanoseconds.
        Weight::from_ref_time(30_677_000 as u64)            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn add_liquidity() -> Weight {
        // Minimum execution time: 98_867 nanoseconds.
        Weight::from_ref_time(100_102_000 as u64)            .saturating_add(T::DbWeight::get().reads(8 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:0)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:0 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
    fn remove_liquidity() -> Weight {
        // Minimum execution time: 125_051 nanoseconds.
        Weight::from_ref_time(126_556_000 as u64)            .saturating_add(T::DbWeight::get().reads(10 as u64))
            .saturating_add(T::DbWeight::get().writes(8 as u64))
    }
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn sell() -> Weight {
        // Minimum execution time: 217_207 nanoseconds.
        Weight::from_ref_time(218_401_000 as u64)            .saturating_add(T::DbWeight::get().reads(12 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn buy() -> Weight {
        // Minimum execution time: 211_853 nanoseconds.
        Weight::from_ref_time(213_114_000 as u64)            .saturating_add(T::DbWeight::get().reads(12 as u64))
            .saturating_add(T::DbWeight::get().writes(7 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
    fn router_execution_sell(c: u32, e: u32, ) -> Weight {
        // Minimum execution time: 66_725 nanoseconds.
        Weight::from_ref_time(67_159_000 as u64)            // Standard Error: 592_525
            .saturating_add(Weight::from_ref_time(2_278_417 as u64).saturating_mul(c as u64))
            // Standard Error: 1_300_761
            .saturating_add(Weight::from_ref_time(151_794_260 as u64).saturating_mul(e as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((9 as u64).saturating_mul(e as u64)))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(e as u64)))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
    fn router_execution_buy(c: u32, e: u32, ) -> Weight {
        // Minimum execution time: 118_181 nanoseconds.
        Weight::from_ref_time(118_740_000 as u64)            // Standard Error: 769_691
            .saturating_add(Weight::from_ref_time(3_767_843 as u64).saturating_mul(c as u64))
            // Standard Error: 2_541_567
            .saturating_add(Weight::from_ref_time(124_213_432 as u64).saturating_mul(e as u64))
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().reads((9 as u64).saturating_mul(e as u64)))
            .saturating_add(T::DbWeight::get().writes((7 as u64).saturating_mul(e as u64)))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:2 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:1 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
    fn create_pool() -> Weight {
        // Minimum execution time: 141_654 nanoseconds.
        Weight::from_ref_time(143_331_000 as u64)            .saturating_add(RocksDbWeight::get().reads(12 as u64))
            .saturating_add(RocksDbWeight::get().writes(8 as u64))
    }
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:1 w:2)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
    fn update_pool_data() -> Weight {
        // Minimum execution time: 30_269 nanoseconds.
        Weight::from_ref_time(30_677_000 as u64)            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:0)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn add_liquidity() -> Weight {
        // Minimum execution time: 98_867 nanoseconds.
        Weight::from_ref_time(100_102_000 as u64)            .saturating_add(RocksDbWeight::get().reads(8 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
	// Storage: LBP PoolData (r:1 w:1)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:4 w:4)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:0)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: LBP FeeCollectorWithAsset (r:0 w:1)
	// Proof: LBP FeeCollectorWithAsset (max_values: None, max_size: Some(69), added: 2544, mode: MaxEncodedLen)
    fn remove_liquidity() -> Weight {
        // Minimum execution time: 125_051 nanoseconds.
        Weight::from_ref_time(126_556_000 as u64)            .saturating_add(RocksDbWeight::get().reads(10 as u64))
            .saturating_add(RocksDbWeight::get().writes(8 as u64))
    }
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn sell() -> Weight {
        // Minimum execution time: 217_207 nanoseconds.
        Weight::from_ref_time(218_401_000 as u64)            .saturating_add(RocksDbWeight::get().reads(12 as u64))
            .saturating_add(RocksDbWeight::get().writes(7 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    fn buy() -> Weight {
        // Minimum execution time: 211_853 nanoseconds.
        Weight::from_ref_time(213_114_000 as u64)            .saturating_add(RocksDbWeight::get().reads(12 as u64))
            .saturating_add(RocksDbWeight::get().writes(7 as u64))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
    fn router_execution_sell(c: u32, e: u32, ) -> Weight {
        // Minimum execution time: 66_725 nanoseconds.
        Weight::from_ref_time(67_159_000 as u64)            // Standard Error: 592_525
            .saturating_add(Weight::from_ref_time(2_278_417 as u64).saturating_mul(c as u64))
            // Standard Error: 1_300_761
            .saturating_add(Weight::from_ref_time(151_794_260 as u64).saturating_mul(e as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().reads((9 as u64).saturating_mul(e as u64)))
            .saturating_add(RocksDbWeight::get().writes((7 as u64).saturating_mul(e as u64)))
    }
	// Storage: LBP PoolData (r:1 w:0)
	// Proof: LBP PoolData (max_values: None, max_size: Some(163), added: 2638, mode: MaxEncodedLen)
	// Storage: Tokens Accounts (r:5 w:5)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Tokens Locks (r:1 w:1)
	// Proof: Tokens Locks (max_values: None, max_size: Some(1261), added: 3736, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:2 w:0)
	// Proof: AssetRegistry Assets (max_values: None, max_size: Some(87), added: 2562, mode: MaxEncodedLen)
	// Storage: System Account (r:3 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
    fn router_execution_buy(c: u32, e: u32, ) -> Weight {
        // Minimum execution time: 118_181 nanoseconds.
        Weight::from_ref_time(118_740_000 as u64)            // Standard Error: 769_691
            .saturating_add(Weight::from_ref_time(3_767_843 as u64).saturating_mul(c as u64))
            // Standard Error: 2_541_567
            .saturating_add(Weight::from_ref_time(124_213_432 as u64).saturating_mul(e as u64))
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().reads((9 as u64).saturating_mul(e as u64)))
            .saturating_add(RocksDbWeight::get().writes((7 as u64).saturating_mul(e as u64)))
    }
}

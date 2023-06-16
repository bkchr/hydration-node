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

//! Autogenerated weights for pallet_duster
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --pallet=pallet-duster
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// duster.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_duster::weights::WeightInfo;

/// Weights for pallet_duster using the hydraDX node and recommended hardware.
pub struct HydraWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	// Storage: Duster AccountBlacklist (r:1 w:0)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
	// Storage: AssetRegistry Assets (r:1 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: Tokens Accounts (r:2 w:2)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Duster DustAccount (r:1 w:0)
	// Proof Skipped: Duster DustAccount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: Duster RewardAccount (r:1 w:0)
	// Proof Skipped: Duster RewardAccount (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:0 w:1)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
    fn dust_account() -> Weight {
        // Minimum execution time: 89_223 nanoseconds.
        Weight::from_ref_time(89_896_000 as u64)            .saturating_add(T::DbWeight::get().reads(7 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
	// Storage: Duster AccountBlacklist (r:0 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
    fn add_nondustable_account() -> Weight {
        // Minimum execution time: 22_009 nanoseconds.
        Weight::from_ref_time(22_568_000 as u64)            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
	// Storage: Duster AccountBlacklist (r:1 w:1)
	// Proof Skipped: Duster AccountBlacklist (max_values: None, max_size: None, mode: Measured)
    fn remove_nondustable_account() -> Weight {
        // Minimum execution time: 27_278 nanoseconds.
        Weight::from_ref_time(27_648_000 as u64)            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

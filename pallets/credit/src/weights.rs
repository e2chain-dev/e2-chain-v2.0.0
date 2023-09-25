// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_credit
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-05, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_credit
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/credit/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_credit.
pub trait WeightInfo {
    fn update_credit_setting() -> Weight;
    fn add_or_update_credit_data() -> Weight;
    fn burn_for_add_credit() -> Weight;
    fn force_modify_credit_history() -> Weight;
    fn update_nft_class_credit() -> Weight;
    fn update_sum_of_credit_nft_burn_history() -> Weight;
    fn burn_nft() -> Weight;
    fn set_switch_campaign() -> Weight;
    fn set_not_switch_accounts() -> Weight;
    fn set_dpr_price() -> Weight;
}

/// Weights for pallet_credit using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn update_credit_setting() -> Weight {
        Weight::from_all(12_688_000 as u64).saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    fn add_or_update_credit_data() -> Weight {
        Weight::from_all(17_815_000 as u64)
            .saturating_add(T::DbWeight::get().reads(2 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    fn burn_for_add_credit() -> Weight {
        Weight::from_all(61_294_000 as u64)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(5 as u64))
    }
    fn force_modify_credit_history() -> Weight {
        Weight::from_all(15_923_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    fn update_nft_class_credit() -> Weight {
        Weight::from_all(14_770_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    fn update_sum_of_credit_nft_burn_history() -> Weight {
        Weight::from_all(14_770_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
    fn burn_nft() -> Weight {
        Weight::from_all(57_653_000 as u64)
            .saturating_add(T::DbWeight::get().reads(5 as u64))
            .saturating_add(T::DbWeight::get().writes(4 as u64))
    }
    fn set_switch_campaign() -> Weight {
        Weight::from_all(7_484_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(3 as u64))
    }
    fn set_not_switch_accounts() -> Weight {
        Weight::from_all(6_173_000 as u64)
            .saturating_add(T::DbWeight::get().reads(1 as u64))
            .saturating_add(T::DbWeight::get().writes(2 as u64))
    }
    fn set_dpr_price() -> Weight {
        Weight::from_all(16_112_000 as u64)
            .saturating_add(T::DbWeight::get().reads(3 as u64))
            .saturating_add(T::DbWeight::get().writes(1 as u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn update_credit_setting() -> Weight {
        Weight::from_all(12_688_000 as u64).saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    fn add_or_update_credit_data() -> Weight {
        Weight::from_all(17_815_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(2 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    fn burn_for_add_credit() -> Weight {
        Weight::from_all(61_294_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(5 as u64))
    }
    fn force_modify_credit_history() -> Weight {
        Weight::from_all(15_923_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    fn update_nft_class_credit() -> Weight {
        Weight::from_all(14_770_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    fn update_sum_of_credit_nft_burn_history() -> Weight {
        Weight::from_all(14_770_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
    fn burn_nft() -> Weight {
        Weight::from_all(57_653_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(5 as u64))
            .saturating_add(RocksDbWeight::get().writes(4 as u64))
    }
    fn set_switch_campaign() -> Weight {
        Weight::from_all(7_484_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(3 as u64))
    }
    fn set_not_switch_accounts() -> Weight {
        Weight::from_all(6_173_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(1 as u64))
            .saturating_add(RocksDbWeight::get().writes(2 as u64))
    }
    fn set_dpr_price() -> Weight {
        Weight::from_all(16_112_000 as u64)
            .saturating_add(RocksDbWeight::get().reads(3 as u64))
            .saturating_add(RocksDbWeight::get().writes(1 as u64))
    }
}

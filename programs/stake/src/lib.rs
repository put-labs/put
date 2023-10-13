#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::integer_arithmetic)]
#[deprecated(
    since = "1.8.0",
    note = "Please use `put_sdk::stake::program::id` or `put_program::stake::program::id` instead"
)]
pub use put_sdk::stake::program::{check_id, id};
use put_sdk::{
    feature_set::{self, FeatureSet},
    genesis_config::GenesisConfig,
    native_token::LAMPORTS_PER_PUT,
};

pub mod config;
pub mod stake_instruction;
pub mod stake_state;

pub fn add_genesis_accounts(genesis_config: &mut GenesisConfig) -> u128 {
    config::add_genesis_account(genesis_config)
}

/// The minimum stake amount that can be delegated, in lamports.
/// NOTE: This is also used to calculate the minimum balance of a stake account, which is the
/// rent exempt reserve _plus_ the minimum stake delegation.
#[inline(always)]
pub fn get_minimum_delegation(feature_set: &FeatureSet) -> u128 {
    if feature_set.is_active(&feature_set::stake_raise_minimum_delegation_to_1_sol::id()) {
        const MINIMUM_DELEGATION_SOL: u128 = 1;
        MINIMUM_DELEGATION_SOL * LAMPORTS_PER_PUT
    } else {
        #[allow(deprecated)]
        put_sdk::stake::MINIMUM_STAKE_DELEGATION
    }
}

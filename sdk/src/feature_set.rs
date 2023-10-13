//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core PUT code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://spl.put.com/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `put-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contirbutor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `put_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    put_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

// pub mod pico_inflation {
//     // 2
//     put_sdk::declare_id!("8hSvtxtTXVVDuuSjP9vYDSc3m5hARjBNoeUqTKfLQWPo");
//     //put_sdk::declare_id!("GkQfmYTzcfCNGpM8JCUE3RPGdTjuXrWSeqCARQ2LMHSn");
    
// }

// pub mod full_inflation {
//     pub mod devnet_and_testnet {
//         // 3
//         put_sdk::declare_id!("HomCY7ZJfDygeVFN8sV6PeT9KXarwzAsx4FMKYwTvLSq");
//         //put_sdk::declare_id!("D7U2bTPzobmwFDR4JPacwx9Cr8xDn9Jqi9rVQNTUChXu");
        
//     }
// }



pub mod curve25519_syscall_enabled {
    put_sdk::declare_id!("7rcw5UtqgDTBBv2EcynNfYckgdAaH1MAsCjKgXMkN7Ri");
}


pub mod libsecp256k1_fail_on_bad_count2 {
    put_sdk::declare_id!("54KAoNiUERNoWWUhTWWwXgym94gzoXFVnHyQwPA18V9A");
}


pub mod credits_auto_rewind {
    put_sdk::declare_id!("BUS12ciZ5gCoFafUHWW8qaFMMtwFQGVxjsDheWLdqBE2");
}


pub mod stake_deactivate_delinquent_instruction {
    put_sdk::declare_id!("437r62HoAdUb63amq3D7ENnBLDhHT2xY8eFkLJYVKK4x");
}

pub mod stake_redelegate_instruction {
    put_sdk::declare_id!("3EPmAX94PvVJCjMeFfRFvj4avqCPL8vv3TGsZQg7ydMx");
}



pub mod check_slice_translation_size {
    put_sdk::declare_id!("GmC19j9qLn2RFk5NduX6QXaDhVpGncVVBzyM8e9WMz2F");
}

pub mod stake_split_uses_rent_sysvar {
    put_sdk::declare_id!("FQnc7U4koHqWgRvFaBJjZnV8VPg6L6wWK33yJeDp4yvV");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    put_sdk::declare_id!("St8k9dVXP97xT6faW24YmRSYConLbhsMJA4TJTBLmMT");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    put_sdk::declare_id!("8199Q2gMD2kwgfopK5qqVWuDbegLgpuFUFHCcUJQDN8b");
}

pub mod reject_callx_r10 {
    put_sdk::declare_id!("3NKRSwpySNwD3TvP5pHnRmkAQRsdkXWRr1WaQh8p4PWX");
}

pub mod drop_redundant_turbine_path {
    put_sdk::declare_id!("4Di3y24QFLt5QEUPZtbnjyfQKfm6ZMTfa6Dw1psfoMKU");
}

pub mod executables_incur_cpi_data_cost {
    put_sdk::declare_id!("7GUcYgq4tVtaqNCKT3dho9r4665Qp5TxCZ27Qgjx3829");
}

pub mod fix_recent_blockhashes {
    put_sdk::declare_id!("6iyggb5MTcsvdcugX7bEKbHV8c6jdLbpHwkncrgLMhfo");
}

pub mod update_rewards_from_cached_accounts {
    put_sdk::declare_id!("28s7i3htzhahXQKqmS2ExzbEoUypg9krwvtK2M9UWXh9");
}

pub mod spl_token_v3_4_0 {
    put_sdk::declare_id!("Ftok4njE8b7tDffYkC5bAbCaQv5sL6jispYrprzatUwN");
}

pub mod spl_associated_token_account_v1_1_0 {
    put_sdk::declare_id!("FaTa17gVKoqbh38HcfiQonPsAaQViyDCCSg71AubYZw8");
}

pub mod default_units_per_instruction {
    put_sdk::declare_id!("J2QdYx8crLbTVK8nur1jeLsmc3krDbfjoxoea2V1Uy5Q");
}

pub mod stake_allow_zero_undelegated_amount {
    put_sdk::declare_id!("sTKz343FM8mqtyGvYWvbLpTThw3ixRM4Xk8QvZ985mw");
}

pub mod require_static_program_ids_in_transaction {
    put_sdk::declare_id!("8FdwgyHFEjhAdjWfV2vfqk7wA1g9X3fQpKH7SBpEv3kC");
}

pub mod stake_raise_minimum_delegation_to_1_sol {
    // This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    put_sdk::declare_id!("9onWzzvCzNC2jfhxxeqRgs5q7nFAAKpCUvkj6T6GJK9i");
}

pub mod stake_minimum_delegation_for_rewards {
    put_sdk::declare_id!("ELjxSXwNsyXGfAh8TqX8ih22xeT8huF6UngQirbLKYKH");
}

pub mod add_set_compute_unit_price_ix {
    put_sdk::declare_id!("98std1NSHqXi9WYvFShfVepRdCoq1qvsp8fsR2XZtG8g");
}

pub mod disable_deploy_of_alloc_free_syscall {
    put_sdk::declare_id!("79HWsX9rpnnJBPcdNURVqygpMAfxdrAirzAGAVmf92im");
}

pub mod include_account_index_in_rent_error {
    put_sdk::declare_id!("2R72wpcQ7qV7aTJWUumdn8u5wmmTyXbK7qzEy7YSAgyY");
}

pub mod add_shred_type_to_shred_seed {
    put_sdk::declare_id!("Ds87KVeqhbv7Jw8W6avsS1mqz3Mw5J3pRTpPoDQ2QdiJ");
}

pub mod warp_timestamp_with_a_vengeance {
    put_sdk::declare_id!("3BX6SBeEBibHaVQXywdkcgyUk6evfYZkHdztXiDtEpFS");
}

pub mod separate_nonce_from_blockhash {
    put_sdk::declare_id!("Gea3ZkK2N4pHuVZVxWcnAtS6UEDdyumdYt4pFcKjA3ar");
}

pub mod enable_durable_nonce {
    put_sdk::declare_id!("4EJQtF2pkRyawwcTVfQutzq4Sa5hRhibF6QAK1QXhtEX");
}

pub mod vote_state_update_credit_per_dequeue {
    put_sdk::declare_id!("CveezY6FDLVBToHDcvJRmtMouqzsmj4UXYh5ths5G5Uv");
}

pub mod quick_bail_on_panic {
    put_sdk::declare_id!("DpJREPyuMZ5nDfU6H3WTqSqUFSXAfw8u7xqmWtEwJDcP");
}

pub mod nonce_must_be_authorized {
    put_sdk::declare_id!("HxrEu1gXuH7iD3Puua1ohd5n4iUKJyFNtNxk9DVJkvgr");
}

pub mod nonce_must_be_advanceable {
    put_sdk::declare_id!("3u3Er5Vc2jVcwz4xr2GJeSAXT3fAj6ADHZ4BJMZiScFd");
}

pub mod vote_authorize_with_seed {
    put_sdk::declare_id!("6tRxEYKuy2L5nnv5bgn7iT28MxUbYxp5h7F3Ncf1exrT");
}

pub mod cap_accounts_data_size_per_block {
    put_sdk::declare_id!("qywiJyZmqTKspFg2LeuUHqcA5nNvBgobqb9UprywS9N");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    put_sdk::declare_id!("HH3MUYReL2BvqqA3oEcAa7txju5GY6G4nxJ51zvsEjEZ");
}

pub mod enable_bpf_loader_extend_program_ix {
    put_sdk::declare_id!("8Zs9W7D9MpSEtUWSQdGniZk2cNmV22y6FLJwCx53asme");
}

pub mod enable_early_verification_of_account_modifications {
    put_sdk::declare_id!("7Vced912WrRnfjaiKRiNBcbuFw7RrnLv3E3z95Y4GTNc");
}

pub mod skip_rent_rewrites {
    put_sdk::declare_id!("CGB2jM8pwZkeeiXQ66kBMyBR6Np61mggL7XUsmLjVcrw");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    put_sdk::declare_id!("812kqX67odAp5NFwM8D2N24cku7WTm9CHUTFUXaDkWPn");
}

pub mod cap_bpf_program_instruction_accounts {
    put_sdk::declare_id!("9k5ijzTbYPtjzu8wj2ErH9v45xecHzQ1x4PMYMMxFgdM");
}

pub mod loosen_cpi_size_restriction {
    put_sdk::declare_id!("GDH5TVdbTPUpRnXaRyQqiKUa7uZAbZ28Q2N9bhbKoMLm");
}

pub mod use_default_units_in_fee_calculation {
    put_sdk::declare_id!("8sKQrMQoUHtQSUP83SPG4ta2JDjSAiWs7t5aJ9uEd6To");
}

pub mod compact_vote_state_updates {
    put_sdk::declare_id!("86HpNqzutEZwLcPxS6EHDcMNYWk6ikhteg9un7Y2PBKE");
}

pub mod incremental_snapshot_only_incremental_hash_calculation {
    put_sdk::declare_id!("25vqsfjk7Nv1prsQJmA4Xu1bN61s8LXCBGUPp8Rfy1UF");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    put_sdk::declare_id!("B9cdB55u4jQsDNsdTK525yE9dmSc5Ga7YBaBrDFvEhM9");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    put_sdk::declare_id!("CpkdQmspsaZZ8FVAouQTtTWZkc8eeQ7V3uj7dWz543rZ");
}

pub mod account_hash_ignore_slot {
    put_sdk::declare_id!("SVn36yVApPLYsa8koK3qUcy14zXDnqkNYWyUh1f4oK1");
}

pub mod set_exempt_rent_epoch_max {
    put_sdk::declare_id!("5wAGiy15X1Jb2hkHnPDCM8oB9V42VNA9ftNVFK84dEgv");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    put_sdk::declare_id!("FKAcEvNgSY79RpqsPNUV5gDyumopH4cEHqUxyfm8b8Ap");
}

pub mod stop_sibling_instruction_search_at_parent {
    put_sdk::declare_id!("EYVpEP7uzH1CoXzbD6PubGhYmnxRXPeq3PPsm1ba3gpo");
}

pub mod vote_state_update_root_fix {
    put_sdk::declare_id!("G74BkWBzmsByZ1kxHy44H3wjwp5hp7JbrGRuDpco22tY");
}

pub mod cap_accounts_data_allocations_per_transaction {
    put_sdk::declare_id!("9gxu85LYRAcZL38We8MYJ4A9AwgBBPtVBAqebMcT1241");
}

pub mod epoch_accounts_hash {
    put_sdk::declare_id!("5GpmAKxaGsWWbPp4bNXFLJxZVvG92ctxf7jQnzTQjF3n");
}

pub mod remove_deprecated_request_unit_ix {
    put_sdk::declare_id!("EfhYd3SafzGT472tYQDUc4dPd2xdEfKs5fwkowUgVt4W");
}

pub mod disable_rehash_for_rent_epoch {
    put_sdk::declare_id!("DTVTkmw3JSofd8CJVJte8PXEbxNQ2yZijvVr3pe2APPj");
}

pub mod increase_tx_account_lock_limit {
    put_sdk::declare_id!("9LZdXeKGeBV6hRLdxS1rHbHoEUsKqesCC2ZAPTPKJAbK");
}

pub mod limit_max_instruction_trace_length {
    put_sdk::declare_id!("GQALDaC48fEhZGWRj9iL5Q889emJKcj3aCvHF7VCbbF4");
}

pub mod check_syscall_outputs_do_not_overlap {
    put_sdk::declare_id!("3uRVPBpyEJRo1emLCrq38eLRFGcu6uKSpUXqGvU8T7SZ");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    put_sdk::declare_id!("5x3825XS7M2A3Ekbn5VGGkvFoAg5qrRWkTrY4bARP1GL");
}

pub mod enable_alt_bn128_syscall {
    put_sdk::declare_id!("A16q37opZdQMCbe5qJ6xpBB9usykfv8jZaMkxvZQi4GJ");
}

pub mod enable_program_redeployment_cooldown {
    put_sdk::declare_id!("J4HFT8usBxpcF63y46t1upYobJgChmKyZPm5uTBRg25Z");
}

pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    put_sdk::declare_id!("noRuG2kzACwgaY7TVmLRnUNPLKNVQE1fb7X55YWBehp");
}

pub mod enable_turbine_fanout_experiments {
    put_sdk::declare_id!("D31EFnLgdiysi84Woo3of4JMu7VmasUS3Z7j9HYXCeLY");
}

pub mod disable_turbine_fanout_experiments {
    put_sdk::declare_id!("Gz1aLrbeQ4Q6PTSafCZcGWZXz91yVRi7ASFzFEr1U4sa");
}

pub mod drop_merkle_shreds {
    put_sdk::declare_id!("84zy5N23Q9vTZuLc9h1HWUtyM9yCFV2SCmyP9W9C3yHZ");
}

pub mod keep_merkle_shreds {
    put_sdk::declare_id!("HyNQzc7TMNmRhpVHXqDGjpsHzeQie82mDQXSF9hj7nAH");
}

pub mod move_serialized_len_ptr_in_cpi {
    put_sdk::declare_id!("74CoWuBmt3rUVUrCb2JiSTvh6nXyBWUsK4SaMj3CtE3T");
}

pub mod update_hashes_per_tick {
    put_sdk::declare_id!("3uFHb9oKdGfgZGJK9EHaAXN4USvnQtAFC13Fh5gGFS5B");
}

pub mod enable_big_mod_exp_syscall {
    put_sdk::declare_id!("EBq48m8irRKuE7ZnMTLvLg2UuGSqhe8s8oMqnmja1fJw");
}

pub mod disable_builtin_loader_ownership_chains {
    put_sdk::declare_id!("4UDcAfQ6EcA6bdcadkeHpkarkhZGJ7Bpq7wTAiRMjkoi");
}

pub mod cap_transaction_accounts_data_size {
    put_sdk::declare_id!("DdLwVYuvDz26JohmgSbA7mjpJFgX5zP2dkp8qsF2C33V");
}

pub mod remove_congestion_multiplier_from_fee_calculation {
    put_sdk::declare_id!("A8xyMHZovGXFkorFqEmVH2PKGLiBip5JD7jt4zsUWo4H");
}

pub mod enable_request_heap_frame_ix {
    put_sdk::declare_id!("Hr1nUA9b7NJ6eChS26o7Vi8gYYDDwWD3YeBfzJkTbU86");
}

pub mod prevent_rent_paying_rent_recipients {
    put_sdk::declare_id!("Fab5oP3DmsLYCiQZXdjyqT3ukFFPrsmqhXU4WU1AWVVF");
}

pub mod delay_visibility_of_program_deployment {
    put_sdk::declare_id!("GmuBvtFb2aHfSfMXpuFeWZGHyDeCLPS79s48fmCWCfM5");
}

pub mod apply_cost_tracker_during_replay {
    put_sdk::declare_id!("2ry7ygxiYURULZCrypHhveanvP5tzZ4toRwVp89oCNSj");
}
pub mod bpf_account_data_direct_mapping {
    put_sdk::declare_id!("9gwzizfABsKUereT6phZZxbTzuAnovkgwpVVpdcSxv9h");
}

pub mod add_set_tx_loaded_accounts_data_size_instruction {
    put_sdk::declare_id!("G6vbf1UBok8MWb8m25ex86aoQHeKTzDKzuZADHkShqm6");
}

pub mod switch_to_new_elf_parser {
    put_sdk::declare_id!("Cdkc8PPTeTNUPoZEfCY5AyetUrEdkZtNPMgz58nqyaHD");
}

pub mod round_up_heap_size {
    put_sdk::declare_id!("CE2et8pqgyQMP2mQRg3CgvX8nJBKUArMu3wfiQiQKY1y");
}

pub mod remove_bpf_loader_incorrect_program_id {
    put_sdk::declare_id!("2HmTkCj9tXuPE4ueHzdD7jPeMf9JGCoZh5AsyoATiWEe");
}

pub mod include_loaded_accounts_data_size_in_fee_calculation {
    put_sdk::declare_id!("EaQpmC6GtRssaZ3PCUM5YksGqUdMLeZ46BQXYtHYakDS");
}

pub mod native_programs_consume_cu {
    put_sdk::declare_id!("8pgXCMNXC8qyEFypuwpXyRxLXZdpM4Qo72gJ6k87A6wL");
}

pub mod simplify_writable_program_account_check {
    put_sdk::declare_id!("5ZCcFAzJ1zsFKe1KSZa9K92jhx7gkcKj97ci2DBo1vwj");
}

pub mod stop_truncating_strings_in_syscalls {
    put_sdk::declare_id!("16FMCmgLzCNNz6eTwGanbyN2ZxvTBSLuQ6DZhgeMshg");
}

pub mod clean_up_delegation_errors {
    put_sdk::declare_id!("Bj2jmUsM2iRhfdLLDSTkhM5UQRQvQHm57HSmPibPtEyu");
}

pub mod vote_state_add_vote_latency {
    put_sdk::declare_id!("7axKe5BTYBDD87ftzWbk5DfzWMGyRvqmWTduuo22Yaqy");
}

pub mod checked_arithmetic_in_fee_validation {
    put_sdk::declare_id!("5Pecy6ie6XGm22pc9d4P9W5c31BugcFBuy6hsP2zkETv");
}

pub mod reduce_stake_warmup_cooldown {
    use put_program::{epoch_schedule::EpochSchedule, stake_history::Epoch};
    put_sdk::declare_id!("GwtDQBghCTBgmX2cpEGNPxTEBUTQRaDMGTr5qychdGMj");

    pub trait NewWarmupCooldownRateEpoch {
        fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch>;
    }
    impl NewWarmupCooldownRateEpoch for super::FeatureSet {
        fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch> {
            self.activated_slot(&id())
                .map(|slot| epoch_schedule.get_epoch(slot))
        }
    }
}


lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [

        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
       
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsec256k1_verify if count appears wrong"),
        
        (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),

        (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        (check_slice_translation_size::id(), "check size when translating slices"),
        (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        (spl_token_v3_4_0::id(), "SPL Token Program version 3.4.0 release #24740"),
        (spl_associated_token_account_v1_1_0::id(), "SPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (stake_raise_minimum_delegation_to_1_sol::id(), "Raise minimum stake delegation to 1.0 SOL #24357"),
        (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated sol_alloc_free_ syscall"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        (drop_merkle_shreds::id(), "drop merkle shreds #29711"),
        (keep_merkle_shreds::id(), "keep merkle shreds #29711"),
        (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        (remove_congestion_multiplier_from_fee_calculation::id(), "Remove congestion multiplier from transaction fee calculation #29881"),
        (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        (delay_visibility_of_program_deployment::id(), "delay visibility of program upgrades #30085"),
        (apply_cost_tracker_during_replay::id(), "apply cost tracker to blocks during replay #29595"),
        (add_set_tx_loaded_accounts_data_size_instruction::id(), "add compute budget instruction for setting account data size per transaction #30366"),
        (switch_to_new_elf_parser::id(), "switch to new ELF parser #30497"),
        (round_up_heap_size::id(), "round up heap size when calculating heap cost #30679"),
        (remove_bpf_loader_incorrect_program_id::id(), "stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"),
        (include_loaded_accounts_data_size_in_fee_calculation::id(), "include transaction loaded accounts data size in base fee calculation #30657"),
        (native_programs_consume_cu::id(), "Native program should consume compute units #30620"),
        (simplify_writable_program_account_check::id(), "Simplify checks performed for writable upgradeable program accounts #30559"),
        (stop_truncating_strings_in_syscalls::id(), "Stop truncating strings in syscalls #31029"),
        (clean_up_delegation_errors::id(), "Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"),
        (vote_state_add_vote_latency::id(), "replace Lockout with LandedVote (including vote latency) in vote state #31264"),
        (checked_arithmetic_in_fee_validation::id(), "checked arithmetic in fee validation #31273"),
        (bpf_account_data_direct_mapping::id(), "use memory regions to map account data into the rbpf vm instead of copying the data"),
        (reduce_stake_warmup_cooldown::id(), "reduce stake warmup cooldown from 25% to 9%"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };

    pub static ref INIT_FEATURE_NAMES: HashSet<Pubkey> = [
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    // Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        // FullInflationFeaturePair {
        //     vote_id: full_inflation::mainnet::certusone::vote::id(),
        //     enable_id: full_inflation::mainnet::certusone::enable::id(),
        // },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let  hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        // if self.is_active(&full_inflation::devnet_and_testnet::id()) {
        //     hash_set.insert(full_inflation::devnet_and_testnet::id());
        // }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }
}

#[derive(AbiExample, Debug, Clone)]
pub struct InitFeatureSet {
    pub init: HashSet<Pubkey>,
}
impl Default for InitFeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            init: INIT_FEATURE_NAMES.iter().cloned().collect(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::devnet_and_testnet::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::devnet_and_testnet::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // // Normal sequence: vote_id then enable_id
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::mainnet::certusone::enable::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );

        // // Backwards sequence: enable_id and then vote_id
        // let mut feature_set = FeatureSet::default();
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        // assert!(feature_set.full_inflation_features_enabled().is_empty());
        // feature_set
        //     .active
        //     .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        // assert_eq!(
        //     feature_set.full_inflation_features_enabled(),
        //     [full_inflation::mainnet::certusone::enable::id()]
        //         .iter()
        //         .cloned()
        //         .collect()
        // );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}

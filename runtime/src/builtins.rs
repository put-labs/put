use {
    put_program_runtime::invoke_context::ProcessInstructionWithContext,
    put_sdk::{
        bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable, pubkey::Pubkey,
    },
};

/// Transitions of built-in programs at epoch bondaries when features are activated.
pub struct BuiltinPrototype {
    pub feature_id: Option<Pubkey>,
    pub program_id: Pubkey,
    pub name: &'static str,
    pub entrypoint: ProcessInstructionWithContext,
}

impl std::fmt::Debug for BuiltinPrototype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut builder = f.debug_struct("BuiltinPrototype");
        builder.field("program_id", &self.program_id);
        builder.field("name", &self.name);
        builder.field("feature_id", &self.feature_id);
        builder.finish()
    }
}

#[cfg(RUSTC_WITH_SPECIALIZATION)]
impl put_frozen_abi::abi_example::AbiExample for BuiltinPrototype {
    fn example() -> Self {
        // BuiltinPrototype isn't serializable by definition.
        put_program_runtime::declare_process_instruction!(entrypoint, 0, |_invoke_context| {
            // Do nothing
            Ok(())
        });
        Self {
            feature_id: None,
            program_id: Pubkey::default(),
            name: "",
            entrypoint,
        }
    }
}

pub static BUILTINS: &[BuiltinPrototype] = &[
    BuiltinPrototype {
        feature_id: None,
        program_id: put_system_program::id(),
        name: "system_program",
        entrypoint: put_system_program::system_processor::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_vote_program::id(),
        name: "vote_program",
        entrypoint: put_vote_program::vote_processor::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_stake_program::id(),
        name: "stake_program",
        entrypoint: put_stake_program::stake_instruction::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_config_program::id(),
        name: "config_program",
        entrypoint: put_config_program::config_processor::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader_deprecated::id(),
        name: "put_bpf_loader_deprecated_program",
        entrypoint: put_bpf_loader_program::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader::id(),
        name: "put_bpf_loader_program",
        entrypoint: put_bpf_loader_program::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader_upgradeable::id(),
        name: "put_bpf_loader_upgradeable_program",
        entrypoint: put_bpf_loader_program::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_sdk::compute_budget::id(),
        name: "compute_budget_program",
        entrypoint: put_compute_budget_program::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_address_lookup_table_program::id(),
        name: "address_lookup_table_program",
        entrypoint: put_address_lookup_table_program::processor::process_instruction,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: put_zk_token_sdk::zk_token_proof_program::id(),
        name: "zk_token_proof_program",
        entrypoint: put_zk_token_proof_program::process_instruction,
    },
];

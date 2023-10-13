use {
    crate::{
        parse_address_lookup_table::parse_address_lookup_table,
        parse_bpf_loader::parse_bpf_upgradeable_loader, parse_config::parse_config,
        parse_nonce::parse_nonce, parse_stake::parse_stake, parse_sysvar::parse_sysvar,
        parse_token::{parse_token, ppl_token_ids}, parse_vote::parse_vote,
	parse_name::parse_name, parse_nft::parse_nft,
    },
    inflector::Inflector,
    serde_json::Value,
    put_sdk::{
        instruction::InstructionError, pubkey::Pubkey, stake, system_program, sysvar, vote,
    },
    std::collections::HashMap,
    thiserror::Error,
};

lazy_static! {
    static ref ADDRESS_LOOKUP_PROGRAM_ID: Pubkey = put_address_lookup_table_program::id();
    static ref BPF_UPGRADEABLE_LOADER_PROGRAM_ID: Pubkey = put_sdk::bpf_loader_upgradeable::id();
    static ref CONFIG_PROGRAM_ID: Pubkey = put_config_program::id();
    static ref STAKE_PROGRAM_ID: Pubkey = stake::program::id();
    static ref SYSTEM_PROGRAM_ID: Pubkey = system_program::id();
    static ref SYSVAR_PROGRAM_ID: Pubkey = sysvar::id();
    static ref VOTE_PROGRAM_ID: Pubkey = vote::program::id();
    static ref NFT_PROGRAM_ID: Pubkey = ppl_nft::id();
    static ref NAME_PROGRAM_ID: Pubkey = ppl_name::id();
    pub static ref PARSABLE_PROGRAM_IDS: HashMap<Pubkey, ParsableAccount> = {
        let mut m = HashMap::new();
        m.insert(
            *ADDRESS_LOOKUP_PROGRAM_ID,
            ParsableAccount::AddressLookupTable,
        );
        m.insert(
            *BPF_UPGRADEABLE_LOADER_PROGRAM_ID,
            ParsableAccount::BpfUpgradeableLoader,
        );
        m.insert(*CONFIG_PROGRAM_ID, ParsableAccount::Config);
        m.insert(*SYSTEM_PROGRAM_ID, ParsableAccount::Nonce);
        for ppl_token_id in ppl_token_ids() {
            m.insert(ppl_token_id, ParsableAccount::PplToken);
        }
        m.insert(*STAKE_PROGRAM_ID, ParsableAccount::Stake);
        m.insert(*SYSVAR_PROGRAM_ID, ParsableAccount::Sysvar);
        m.insert(*VOTE_PROGRAM_ID, ParsableAccount::Vote);
        m.insert(*NFT_PROGRAM_ID, ParsableAccount::PplNft);
        m.insert(*NAME_PROGRAM_ID, ParsableAccount::PplName);
        m
    };
}

#[derive(Error, Debug)]
pub enum ParseAccountError {
    #[error("{0:?} account not parsable")]
    AccountNotParsable(ParsableAccount),

    #[error("Program not parsable")]
    ProgramNotParsable,

    #[error("Additional data required to parse: {0}")]
    AdditionalDataMissing(String),

    #[error("Instruction error")]
    InstructionError(#[from] InstructionError),

    #[error("Serde json error")]
    SerdeJsonError(#[from] serde_json::error::Error),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParsedAccount {
    pub program: String,
    pub parsed: Value,
    pub space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParsableAccount {
    AddressLookupTable,
    BpfUpgradeableLoader,
    Config,
    Nonce,
    PplToken,
    Stake,
    Sysvar,
    Vote,
    PplNft,
    PplName,
}

#[derive(Clone, Default)]
pub struct AccountAdditionalData {
    pub ppl_token_decimals: Option<u8>,
    pub ppl_token_symbol: Option<String>,
    pub ppl_token_name: Option<String>,
    pub ppl_token_icon: Option<String>,
}

pub fn parse_account_data(
    pubkey: &Pubkey,
    program_id: &Pubkey,
    data: &[u8],
    additional_data: Option<AccountAdditionalData>,
) -> Result<ParsedAccount, ParseAccountError> {
    let program_name = PARSABLE_PROGRAM_IDS
        .get(program_id)
        .ok_or(ParseAccountError::ProgramNotParsable)?;
    let additional_data = additional_data.unwrap_or_default();
    let parsed_json = match program_name {
        ParsableAccount::AddressLookupTable => {
            serde_json::to_value(parse_address_lookup_table(data)?)?
        }
        ParsableAccount::BpfUpgradeableLoader => {
            serde_json::to_value(parse_bpf_upgradeable_loader(data)?)?
        }
        ParsableAccount::Config => serde_json::to_value(parse_config(data, pubkey)?)?,
        ParsableAccount::Nonce => serde_json::to_value(parse_nonce(data)?)?,
        ParsableAccount::PplToken => {
            serde_json::to_value(parse_token(data, additional_data)?)?
        }
        ParsableAccount::Stake => serde_json::to_value(parse_stake(data)?)?,
        ParsableAccount::Sysvar => serde_json::to_value(parse_sysvar(data, pubkey)?)?,
        ParsableAccount::Vote => serde_json::to_value(parse_vote(data)?)?,
        ParsableAccount::PplNft => serde_json::to_value(parse_nft(data)?)?,
        ParsableAccount::PplName => serde_json::to_value(parse_name(data)?)?,
    };
    Ok(ParsedAccount {
        program: format!("{program_name:?}").to_kebab_case(),
        parsed: parsed_json,
        space: data.len() as u64,
    })
}

#[cfg(test)]
mod test {
    use {
        super::*,
        put_sdk::nonce::{
            state::{Data, Versions},
            State,
        },
        vote::{
            program::id as vote_program_id,
            state::{VoteState, VoteStateVersions},
        },
        
    };

    #[test]
    fn test_parse_account_data() {
        let account_pubkey = put_sdk::pubkey::new_rand();
        let other_program = put_sdk::pubkey::new_rand();
        let data = vec![0; 4];
        assert!(parse_account_data(&account_pubkey, &other_program, &data, None).is_err());

        let vote_state = VoteState::default();
        let mut vote_account_data: Vec<u8> = vec![0; VoteState::size_of()];
        let versioned = VoteStateVersions::new_current(vote_state);
        VoteState::serialize(&versioned, &mut vote_account_data).unwrap();
        let parsed = parse_account_data(
            &account_pubkey,
            &vote_program_id(),
            &vote_account_data,
            None,
        )
        .unwrap();
        assert_eq!(parsed.program, "vote".to_string());
        assert_eq!(parsed.space, VoteState::size_of() as u64);

        let nonce_data = Versions::new(State::Initialized(Data::default()));
        let nonce_account_data = bincode::serialize(&nonce_data).unwrap();
        let parsed = parse_account_data(
            &account_pubkey,
            &system_program::id(),
            &nonce_account_data,
            None,
        )
        .unwrap();
        assert_eq!(parsed.program, "nonce".to_string());
        assert_eq!(parsed.space, State::size() as u64);
    }
}

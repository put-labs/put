use {
    crate::{
        parse_account_data::{ParsableAccount, ParseAccountError},
        StringAmount,
    },
    put_sdk::pubkey::Pubkey,

};
use ppl_nft::state::{AccountState, MetaAccount, NftMint};
use ppl_nft::put_program::program_pack::Pack;

fn nft_program_id() -> Pubkey {
    Pubkey::new_from_array(ppl_nft::id().to_bytes())
}

// Check if the provided program id as a known SPL Token program id
pub fn is_known_nft_token_id(program_id: &Pubkey) -> bool {
    *program_id == nft_program_id()
}



pub fn parse_nft(
    data: &[u8],
) -> Result<TokenAccountType, ParseAccountError> {
    if data.len() == MetaAccount::get_packed_len() {
        let account = MetaAccount::unpack(data)
            .map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::PplNft))?;
        Ok(TokenAccountType::Account(UiNFTAccount {
            mint: account.mint.to_string(),
            owner: account.owner.to_string(),
            state: account.state.into(),
            close_authority: match account.close_authority {
                Option::Some(pubkey) => Some(pubkey.to_string()),
                Option::None => None,
            },
            token_id: account.token_id,
            token_uri: account.token_uri
        }))
        //todo need mint unpack?
    } else {
        Err(ParseAccountError::AccountNotParsable(
            ParsableAccount::PplNft,
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
#[allow(clippy::large_enum_variant)]
pub enum TokenAccountType {
    Account(UiNFTAccount),
    Mint(UiMint),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiNFTAccount {
    /// The mint associated with this account
    pub mint: String, //32
    /// The owner of this account.
    pub owner: String, //32
    /// The account's state
    pub state: UiAccountState, //1
    /// Optional authority to close the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_authority: Option<String>,// 33
    /// The mint's token_id of nft
    pub token_id: u64, // 8
    /// The suffix of the nft
    pub token_uri: String, // 200

}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiAccountState {
    Uninitialized,
    Initialized,
    Frozen,
}

impl From<AccountState> for UiAccountState {
    fn from(state: AccountState) -> Self {
        match state {
            AccountState::Uninitialized => UiAccountState::Uninitialized,
            AccountState::Initialized => UiAccountState::Initialized,
            AccountState::Frozen => UiAccountState::Frozen,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiMint {
    pub mint_authority: Option<String>,
    pub supply: StringAmount,
    pub decimals: u8,
    pub is_initialized: bool,
    pub freeze_authority: Option<String>,
}

pub fn get_token_account_mint(data: &[u8]) -> Option<Pubkey> {
    if data.len() == NftMint::get_packed_len() {
        Some(Pubkey::try_from(data).unwrap())
    } else {
        None
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_token() {
        let mint_pubkey = Pubkey::from([2; 32]);
        let owner_pubkey = Pubkey::from([3; 32]);
        let mut account_data = vec![0; MetaAccount::get_packed_len()];
        let mut account = MetaAccount::unpack_unchecked(&account_data).unwrap();
        account.mint = mint_pubkey;
        account.owner = owner_pubkey;
        account.state = AccountState::Initialized;
        account.close_authority = Option::Some(owner_pubkey);
        account.token_id = 1;
        account.token_uri = "www.baidu.com".to_string();
        MetaAccount::pack(account, &mut account_data).unwrap();

        let invalid_account_data = vec![0; 178];
        assert!(parse_nft(&invalid_account_data).is_err());
        assert_eq!(
            parse_nft(&account_data).unwrap(),
            TokenAccountType::Account(UiNFTAccount {
                mint: mint_pubkey.to_string(),
                owner: owner_pubkey.to_string(),
                state: UiAccountState::Initialized,
                close_authority:  Option::Some(owner_pubkey.to_string()),
                token_id: 1,
                token_uri: "www.baidu.com".to_string()
            }),
        );
    }
}
use {
    crate::parse_instruction::{
        check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
    },
    ppl_token::{
        instruction::{AuthorityType, TokenInstruction},
        put_program::{instruction::Instruction as PplTokenInstruction, program_option::COption},
    },
    put_account_decoder::parse_token::{pubkey_from_ppl_token, token_amount_to_ui_amount},
    put_sdk::{
        instruction::{AccountMeta, CompiledInstruction, Instruction},
        message::AccountKeys,
    },
    serde_json::{json, Map, Value},
};

pub fn parse_token(
    instruction: &CompiledInstruction,
    account_keys: &AccountKeys,
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    let token_instruction = TokenInstruction::unpack(&instruction.data)
        .map_err(|_| ParseInstructionError::InstructionNotParsable(ParsableProgram::PplToken))?;
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::PplToken,
            ));
        }
    }
    match token_instruction {
        TokenInstruction::InitializeMint {
            decimals,
            mint_authority,
            freeze_authority,
        } => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "decimals": decimals,
                "mintAuthority": mint_authority.to_string(),
                "rentSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            if let COption::Some(freeze_authority) = freeze_authority {
                map.insert(
                    "freezeAuthority".to_string(),
                    json!(freeze_authority.to_string()),
                );
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeMint".to_string(),
                info: value,
            })
        }
        TokenInstruction::InitializeAccount => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeAccount".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                    "owner": account_keys[instruction.accounts[2] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[3] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::InitializeAccount2 { owner } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeAccount2".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                    "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                    "owner": owner.to_string(),
                    "rentSysvar": account_keys[instruction.accounts[2] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::InitializeMultisig { m } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut signers: Vec<String> = vec![];
            for i in instruction.accounts[2..].iter() {
                signers.push(account_keys[*i as usize].to_string());
            }
            Ok(ParsedInstructionEnum {
                instruction_type: "initializeMultisig".to_string(),
                info: json!({
                    "multisig": account_keys[instruction.accounts[0] as usize].to_string(),
                    "rentSysvar": account_keys[instruction.accounts[1] as usize].to_string(),
                    "signers": signers,
                    "m": m,
                }),
            })
        }
        TokenInstruction::Transfer { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "destination": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: value,
            })
        }
        TokenInstruction::Approve { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "delegate": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "approve".to_string(),
                info: value,
            })
        }
        TokenInstruction::Revoke => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                1,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "revoke".to_string(),
                info: value,
            })
        }
        TokenInstruction::SetAuthority {
            authority_type,
            new_authority,
        } => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let owned = match authority_type {
                AuthorityType::MintTokens | AuthorityType::FreezeAccount | AuthorityType::UpdateMeta => "mint",
                AuthorityType::AccountOwner | AuthorityType::CloseAccount => "account",
            };
            let mut value = json!({
                owned: account_keys[instruction.accounts[0] as usize].to_string(),
                "authorityType": Into::<UiAuthorityType>::into(authority_type),
                "newAuthority": match new_authority {
                    COption::Some(authority) => Some(authority.to_string()),
                    COption::None => None,
                },
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                1,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "setAuthority".to_string(),
                info: value,
            })
        }
        TokenInstruction::MintTo { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "account": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "mintAuthority",
                "multisigMintAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "mintTo".to_string(),
                info: value,
            })
        }
        TokenInstruction::Burn { amount } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "amount": amount.to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "burn".to_string(),
                info: value,
            })
        }
        TokenInstruction::CloseAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "destination": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "closeAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::FreezeAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "freezeAuthority",
                "multisigFreezeAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "freezeAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::ThawAccount => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "freezeAuthority",
                "multisigFreezeAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "thawAccount".to_string(),
                info: value,
            })
        }
        TokenInstruction::TransferChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "destination": account_keys[instruction.accounts[2] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                3,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "transferChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::ApproveChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 4)?;
            let mut value = json!({
                "source": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "delegate": account_keys[instruction.accounts[2] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                3,
                account_keys,
                &instruction.accounts,
                "owner",
                "multisigOwner",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "approveChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::MintToChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "account": account_keys[instruction.accounts[1] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "mintAuthority",
                "multisigMintAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "mintToChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::BurnChecked { amount, decimals } => {
            check_num_token_accounts(&instruction.accounts, 3)?;
            let mut value = json!({
                "account": account_keys[instruction.accounts[0] as usize].to_string(),
                "mint": account_keys[instruction.accounts[1] as usize].to_string(),
                "tokenAmount": token_amount_to_ui_amount(amount, decimals),
            });
            let map = value.as_object_mut().unwrap();
            parse_signers(
                map,
                2,
                account_keys,
                &instruction.accounts,
                "authority",
                "multisigAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "burnChecked".to_string(),
                info: value,
            })
        }
        TokenInstruction::SyncNative => {
            check_num_token_accounts(&instruction.accounts, 1)?;
            Ok(ParsedInstructionEnum {
                instruction_type: "syncNative".to_string(),
                info: json!({
                    "account": account_keys[instruction.accounts[0] as usize].to_string(),
                }),
            })
        }
        TokenInstruction::UpdateSymbol { new_symbol } => {
            check_num_token_accounts(&instruction.accounts, 2)?;

            let mut value = json!({
                "tokenMetaAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                "mintMetaAuthorityAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                "symbol": new_symbol,
            });
            let map = value.as_object_mut().unwrap();
            // todo  oa
            parse_signers(
                map,
                0,
                account_keys,
                &instruction.accounts,
                "mintMetaAuthority",
                "multisigMintMetaAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateSymbol".to_string(),
                info: value,
            })
        }
        TokenInstruction::UpdateName { new_name } => {
            check_num_token_accounts(&instruction.accounts, 2)?;

            let mut value = json!({
                "tokenMetaAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                "mintMetaAuthorityAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                "name":new_name,
            });
            let map = value.as_object_mut().unwrap();
            // todo  oa
            parse_signers(
                map,
                0,
                account_keys,
                &instruction.accounts,
                "mintMetaAuthority",
                "multisigMintMetaAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateName".to_string(),
                info: value,
            })
        }
        TokenInstruction::UpdateIcon { new_icon } => {
            check_num_token_accounts(&instruction.accounts, 2)?;

            let mut value = json!({
                "tokenMetaAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                "mintMetaAuthorityAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                "icon":new_icon,
            });
            let map = value.as_object_mut().unwrap();
            // todo  oa
            parse_signers(
                map,
                0,
                account_keys,
                &instruction.accounts,
                "mintMetaAuthority",
                "multisigMintMetaAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "UpdateIcon".to_string(),
                info: value,
            })
        }
        TokenInstruction::CreateMintMetaAccount {} => {
            check_num_token_accounts(&instruction.accounts, 3)?;

            let mut value = json!({
                "feePayAccount": account_keys[instruction.accounts[0] as usize].to_string(),
                "tokenMintAccount": account_keys[instruction.accounts[1] as usize].to_string(),
                "tokenMintMetaAccount": account_keys[instruction.accounts[2] as usize].to_string(),
            });
            let map = value.as_object_mut().unwrap();
            // todo  oa
            parse_signers(
                map,
                1,
                account_keys,
                &instruction.accounts,
                "mintAuthority",
                "multisigMintAuthority",
            );
            Ok(ParsedInstructionEnum {
                instruction_type: "CreateMintMetaAcount".to_string(),
                info: value,
            })
        }
        TokenInstruction::InitMintMetaAccount { symbol, name, icon } => {
            check_num_token_accounts(&instruction.accounts, 2)?;
            let mut value = json!({
                "mint": account_keys[instruction.accounts[0] as usize].to_string(),
                "mintMeta": account_keys[instruction.accounts[1] as usize].to_string(),
                "symbol":symbol,
                "name":name,
                "icon":icon,
            });
            let _map = value.as_object_mut().unwrap();
            Ok(ParsedInstructionEnum {
                instruction_type: "InitMintMetaAccount".to_string(),
                info: value,
            })
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiAuthorityType {
    MintTokens,
    FreezeAccount,
    AccountOwner,
    CloseAccount,
    UpdateMeta,
}

impl From<AuthorityType> for UiAuthorityType {
    fn from(authority_type: AuthorityType) -> Self {
        match authority_type {
            AuthorityType::MintTokens => UiAuthorityType::MintTokens,
            AuthorityType::FreezeAccount => UiAuthorityType::FreezeAccount,
            AuthorityType::AccountOwner => UiAuthorityType::AccountOwner,
            AuthorityType::CloseAccount => UiAuthorityType::CloseAccount,
            AuthorityType::UpdateMeta => UiAuthorityType::UpdateMeta,
        }
    }
}

fn parse_signers(
    map: &mut Map<String, Value>,
    last_nonsigner_index: usize,
    account_keys: &AccountKeys,
    accounts: &[u8],
    owner_field_name: &str,
    multisig_field_name: &str,
) {
    if accounts.len() > last_nonsigner_index + 1 {
        let mut signers: Vec<String> = vec![];
        for i in accounts[last_nonsigner_index + 1..].iter() {
            signers.push(account_keys[*i as usize].to_string());
        }
        map.insert(
            multisig_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
        map.insert("signers".to_string(), json!(signers));
    } else {
        map.insert(
            owner_field_name.to_string(),
            json!(account_keys[accounts[last_nonsigner_index] as usize].to_string()),
        );
    }
}

fn check_num_token_accounts(accounts: &[u8], num: usize) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::PplToken)
}

pub fn ppl_token_instruction(instruction: PplTokenInstruction) -> Instruction {
    Instruction {
        program_id: pubkey_from_ppl_token(&instruction.program_id),
        accounts: instruction
            .accounts
            .iter()
            .map(|meta| AccountMeta {
                pubkey: pubkey_from_ppl_token(&meta.pubkey),
                is_signer: meta.is_signer,
                is_writable: meta.is_writable,
            })
            .collect(),
        data: instruction.data,
    }
}

#[cfg(test)]
mod test {

    use {
        super::*,
        ppl_token::{
            instruction::*,
            put_program::{
                instruction::CompiledInstruction as PplTokenCompiledInstruction, message::Message,
                pubkey::Pubkey as PplTokenPubkey,
            },
        },
        put_sdk::{instruction::CompiledInstruction, pubkey::Pubkey},
        std::str::FromStr,
    };

    fn convert_pubkey(pubkey: Pubkey) -> PplTokenPubkey {
        PplTokenPubkey::from_str(&pubkey.to_string()).unwrap()
    }

    fn convert_compiled_instruction(
        instruction: &PplTokenCompiledInstruction,
    ) -> CompiledInstruction {
        CompiledInstruction {
            program_id_index: instruction.program_id_index,
            accounts: instruction.accounts.clone(),
            data: instruction.data.clone(),
        }
    }

    pub(super) fn convert_account_keys(message: &Message) -> Vec<Pubkey> {
        message
            .account_keys
            .iter()
            .map(pubkey_from_ppl_token)
            .collect()
    }

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_parse_token() {
        let mint_pubkey = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();
        let freeze_authority = Pubkey::new_unique();
        let rent_sysvar = put_sdk::sysvar::rent::id();

        // Test InitializeMint variations
        let initialize_mint_ix = initialize_mint(
            &ppl_token::id(),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(mint_authority),
            Some(&convert_pubkey(freeze_authority)),
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_mint_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeMint".to_string(),
                info: json!({
                    "mint": mint_pubkey.to_string(),
                    "decimals": 2,
                    "mintAuthority": mint_authority.to_string(),
                    "freezeAuthority": freeze_authority.to_string(),
                    "rentSysvar": rent_sysvar.to_string(),
                })
            }
        );

        let initialize_mint_ix = initialize_mint(
            &ppl_token::id(),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(mint_authority),
            None,
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_mint_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeMint".to_string(),
                info: json!({
                   "mint": mint_pubkey.to_string(),
                   "decimals": 2,
                   "mintAuthority": mint_authority.to_string(),
                   "rentSysvar": rent_sysvar.to_string(),
                })
            }
        );

        // Test InitializeAccount
        let account_pubkey = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let initialize_account_ix = initialize_account(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(owner),
        )
        .unwrap();
        let message = Message::new(&[initialize_account_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeAccount".to_string(),
                info: json!({
                   "account": account_pubkey.to_string(),
                   "mint": mint_pubkey.to_string(),
                   "owner": owner.to_string(),
                   "rentSysvar": rent_sysvar.to_string(),
                })
            }
        );

        // Test InitializeAccount2
        let initialize_account_ix = initialize_account2(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(owner),
        )
        .unwrap();
        let message = Message::new(&[initialize_account_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeAccount2".to_string(),
                info: json!({
                   "account": account_pubkey.to_string(),
                   "mint": mint_pubkey.to_string(),
                   "owner": owner.to_string(),
                   "rentSysvar": rent_sysvar.to_string(),
                })
            }
        );

        // Test InitializeMultisig
        let multisig_pubkey = Pubkey::new_unique();
        let multisig_signer0 = Pubkey::new_unique();
        let multisig_signer1 = Pubkey::new_unique();
        let multisig_signer2 = Pubkey::new_unique();
        let initialize_multisig_ix = initialize_multisig(
            &ppl_token::id(),
            &convert_pubkey(multisig_pubkey),
            &[
                &convert_pubkey(multisig_signer0),
                &convert_pubkey(multisig_signer1),
                &convert_pubkey(multisig_signer2),
            ],
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_multisig_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "initializeMultisig".to_string(),
                info: json!({
                    "multisig": multisig_pubkey.to_string(),
                    "m": 2,
                    "rentSysvar": rent_sysvar.to_string(),
                    "signers": vec![
                        multisig_signer0.to_string(),
                        multisig_signer1.to_string(),
                        multisig_signer2.to_string(),
                    ],
                })
            }
        );

        // Test Transfer, incl multisig
        let recipient = Pubkey::new_unique();
        let transfer_ix = transfer(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(owner),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "destination": recipient.to_string(),
                    "authority": owner.to_string(),
                    "amount": "42",
                })
            }
        );
        let transfer_ix = transfer(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(multisig_pubkey),
            &[
                &convert_pubkey(multisig_signer0),
                &convert_pubkey(multisig_signer1),
            ],
            42,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transfer".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "destination": recipient.to_string(),
                    "multisigAuthority": multisig_pubkey.to_string(),
                    "signers": vec![
                        multisig_signer0.to_string(),
                        multisig_signer1.to_string(),
                    ],
                    "amount": "42",
                })
            }
        );

        // Test Approve, incl multisig
        let approve_ix = approve(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(owner),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "approve".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "delegate": recipient.to_string(),
                    "owner": owner.to_string(),
                    "amount": "42",
                })
            }
        );

        let approve_ix = approve(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(multisig_pubkey),
            &[
                &convert_pubkey(multisig_signer0),
                &convert_pubkey(multisig_signer1),
            ],
            42,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "approve".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "delegate": recipient.to_string(),
                    "multisigOwner": multisig_pubkey.to_string(),
                    "signers": vec![
                        multisig_signer0.to_string(),
                        multisig_signer1.to_string(),
                    ],
                    "amount": "42",
                })
            }
        );

        // Test Revoke
        let revoke_ix = revoke(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(owner),
            &[],
        )
        .unwrap();
        let message = Message::new(&[revoke_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "revoke".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "owner": owner.to_string(),
                })
            }
        );

        // Test SetOwner
        let new_freeze_authority = Pubkey::new_unique();
        let set_authority_ix = set_authority(
            &ppl_token::id(),
            &convert_pubkey(mint_pubkey),
            Some(&convert_pubkey(new_freeze_authority)),
            AuthorityType::FreezeAccount,
            &convert_pubkey(freeze_authority),
            &[],
        )
        .unwrap();
        let message = Message::new(&[set_authority_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "setAuthority".to_string(),
                info: json!({
                    "mint": mint_pubkey.to_string(),
                    "newAuthority": new_freeze_authority.to_string(),
                    "authority": freeze_authority.to_string(),
                    "authorityType": "freezeAccount".to_string(),
                })
            }
        );

        let set_authority_ix = set_authority(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            None,
            AuthorityType::CloseAccount,
            &convert_pubkey(owner),
            &[],
        )
        .unwrap();
        let message = Message::new(&[set_authority_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        let new_authority: Option<String> = None;
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "setAuthority".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "newAuthority": new_authority,
                    "authority": owner.to_string(),
                    "authorityType": "closeAccount".to_string(),
                })
            }
        );

        // Test MintTo
        let mint_to_ix = mint_to(
            &ppl_token::id(),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_authority),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[mint_to_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "mintTo".to_string(),
                info: json!({
                    "mint": mint_pubkey.to_string(),
                    "account": account_pubkey.to_string(),
                    "mintAuthority": mint_authority.to_string(),
                    "amount": "42",
                })
            }
        );

        // Test Burn
        let burn_ix = burn(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(owner),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[burn_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "burn".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "authority": owner.to_string(),
                    "amount": "42",
                })
            }
        );

        // Test CloseAccount
        let close_account_ix = close_account(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(owner),
            &[],
        )
        .unwrap();
        let message = Message::new(&[close_account_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "closeAccount".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "destination": recipient.to_string(),
                    "owner": owner.to_string(),
                })
            }
        );

        // Test FreezeAccount
        let freeze_account_ix = freeze_account(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(freeze_authority),
            &[],
        )
        .unwrap();
        let message = Message::new(&[freeze_account_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "freezeAccount".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "freezeAuthority": freeze_authority.to_string(),
                })
            }
        );

        // Test ThawAccount
        let thaw_account_ix = thaw_account(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(freeze_authority),
            &[],
        )
        .unwrap();
        let message = Message::new(&[thaw_account_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "thawAccount".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "freezeAuthority": freeze_authority.to_string(),
                })
            }
        );

        // Test TransferChecked, incl multisig
        let transfer_ix = transfer_checked(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(owner),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transferChecked".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "destination": recipient.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "authority": owner.to_string(),
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                   }
                })
            }
        );

        let transfer_ix = transfer_checked(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(multisig_pubkey),
            &[
                &convert_pubkey(multisig_signer0),
                &convert_pubkey(multisig_signer1),
            ],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "transferChecked".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "destination": recipient.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "multisigAuthority": multisig_pubkey.to_string(),
                    "signers": vec![
                        multisig_signer0.to_string(),
                        multisig_signer1.to_string(),
                    ],
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                   }
                })
            }
        );

        // Test ApproveChecked, incl multisig
        let approve_ix = approve_checked(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(owner),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "approveChecked".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "delegate": recipient.to_string(),
                    "owner": owner.to_string(),
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                    }
                })
            }
        );

        let approve_ix = approve_checked(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(recipient),
            &convert_pubkey(multisig_pubkey),
            &[
                &convert_pubkey(multisig_signer0),
                &convert_pubkey(multisig_signer1),
            ],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "approveChecked".to_string(),
                info: json!({
                    "source": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "delegate": recipient.to_string(),
                    "multisigOwner": multisig_pubkey.to_string(),
                    "signers": vec![
                        multisig_signer0.to_string(),
                        multisig_signer1.to_string(),
                    ],
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                    }
                })
            }
        );

        // Test MintToChecked
        let mint_to_ix = mint_to_checked(
            &ppl_token::id(),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_authority),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[mint_to_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "mintToChecked".to_string(),
                info: json!({
                    "mint": mint_pubkey.to_string(),
                    "account": account_pubkey.to_string(),
                    "mintAuthority": mint_authority.to_string(),
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                    }
                })
            }
        );

        // Test BurnChecked
        let burn_ix = burn_checked(
            &ppl_token::id(),
            &convert_pubkey(account_pubkey),
            &convert_pubkey(mint_pubkey),
            &convert_pubkey(owner),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[burn_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "burnChecked".to_string(),
                info: json!({
                    "account": account_pubkey.to_string(),
                    "mint": mint_pubkey.to_string(),
                    "authority": owner.to_string(),
                    "tokenAmount": {
                        "uiAmount": 0.42,
                        "decimals": 2,
                        "amount": "42",
                        "uiAmountString": "0.42",
                    }
                })
            }
        );

        // Test SyncNative
        let sync_native_ix =
            sync_native(&ppl_token::id(), &convert_pubkey(account_pubkey)).unwrap();
        let message = Message::new(&[sync_native_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "syncNative".to_string(),
                info: json!({
                   "account": account_pubkey.to_string(),
                })
            }
        );
    }

    #[test]
    #[allow(clippy::same_item_push)]
    fn test_token_ix_not_enough_keys() {
        let mut keys: Vec<Pubkey> = vec![];
        for _ in 0..10 {
            keys.push(put_sdk::pubkey::new_rand());
        }

        let (_, _) =
            Pubkey::find_program_address(&[b"MintMeta", &keys[0].to_bytes()], &ppl_token::id());

        // Test InitializeMint variations
        let initialize_mint_ix = initialize_mint(
            &ppl_token::id(),
            &convert_pubkey(keys[0]),
            &convert_pubkey(keys[1]),
            Some(&convert_pubkey(keys[2])),
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_mint_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..1], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        let initialize_mint_ix = initialize_mint(
            &ppl_token::id(),
            &convert_pubkey(keys[0]),
            &convert_pubkey(keys[1]),
            None,
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_mint_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..1], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test InitializeAccount
        let initialize_account_ix = initialize_account(
            &ppl_token::id(),
            &convert_pubkey(keys[0]),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
        )
        .unwrap();
        let message = Message::new(&[initialize_account_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..3], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test InitializeMultisig
        let initialize_multisig_ix = initialize_multisig(
            &ppl_token::id(),
            &convert_pubkey(keys[0]),
            &[
                &convert_pubkey(keys[1]),
                &convert_pubkey(keys[2]),
                &convert_pubkey(keys[3]),
            ],
            2,
        )
        .unwrap();
        let message = Message::new(&[initialize_multisig_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..4], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 3].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test Transfer, incl multisig
        let transfer_ix = transfer(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        let transfer_ix = transfer(
            &ppl_token::id(),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[4]),
            &[&convert_pubkey(keys[0]), &convert_pubkey(keys[1])],
            42,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..4], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 3].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test Approve, incl multisig
        let approve_ix = approve(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        let approve_ix = approve(
            &ppl_token::id(),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[4]),
            &[&convert_pubkey(keys[0]), &convert_pubkey(keys[1])],
            42,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..4], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 3].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test Revoke
        let revoke_ix = revoke(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[0]),
            &[],
        )
        .unwrap();
        let message = Message::new(&[revoke_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..1], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test SetAuthority
        let set_authority_ix = set_authority(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            Some(&convert_pubkey(keys[2])),
            AuthorityType::FreezeAccount,
            &convert_pubkey(keys[0]),
            &[],
        )
        .unwrap();
        let message = Message::new(&[set_authority_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..1], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test MintTo
        let mint_to_ix = mint_to(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[mint_to_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test Burn
        let burn_ix = burn(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
        )
        .unwrap();
        let message = Message::new(&[burn_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test CloseAccount
        let close_account_ix = close_account(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
        )
        .unwrap();
        let message = Message::new(&[close_account_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test FreezeAccount
        let freeze_account_ix = freeze_account(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
        )
        .unwrap();
        let message = Message::new(&[freeze_account_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test ThawAccount
        let thaw_account_ix = thaw_account(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
        )
        .unwrap();
        let message = Message::new(&[thaw_account_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test TransferChecked, incl multisig
        let transfer_ix = transfer_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[0]),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..3], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        let transfer_ix = transfer_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[4]),
            &convert_pubkey(keys[5]),
            &[&convert_pubkey(keys[0]), &convert_pubkey(keys[1])],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[transfer_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..5], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 3].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test ApproveChecked, incl multisig
        let approve_ix = approve_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[0]),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..3], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        let approve_ix = approve_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[3]),
            &convert_pubkey(keys[4]),
            &convert_pubkey(keys[5]),
            &[&convert_pubkey(keys[0]), &convert_pubkey(keys[1])],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[approve_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..5], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 3].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test MintToChecked
        let mint_to_ix = mint_to_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[mint_to_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test BurnChecked
        let burn_ix = burn_checked(
            &ppl_token::id(),
            &convert_pubkey(keys[1]),
            &convert_pubkey(keys[2]),
            &convert_pubkey(keys[0]),
            &[],
            42,
            2,
        )
        .unwrap();
        let message = Message::new(&[burn_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys[0..2], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());

        // Test SyncNative
        let sync_native_ix = sync_native(&ppl_token::id(), &convert_pubkey(keys[0])).unwrap();
        let message = Message::new(&[sync_native_ix], None);
        let mut compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&[], None)).is_err());
        compiled_instruction.accounts =
            compiled_instruction.accounts[0..compiled_instruction.accounts.len() - 1].to_vec();
        assert!(parse_token(&compiled_instruction, &AccountKeys::new(&keys, None)).is_err());
    }
}

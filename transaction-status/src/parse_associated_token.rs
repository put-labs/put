use {
    crate::parse_instruction::{
        check_num_accounts, ParsableProgram, ParseInstructionError, ParsedInstructionEnum,
    },
    serde_json::json,
    put_sdk::{instruction::CompiledInstruction, message::AccountKeys, pubkey::Pubkey},
};

// A helper function to convert ppl_associated_token_account::id() as ppl_sdk::pubkey::Pubkey
// to put_sdk::pubkey::Pubkey
pub fn ppl_associated_token_id() -> Pubkey {
    Pubkey::new_from_array(ppl_associated_token_account::id().to_bytes())
}

pub fn parse_associated_token(
    instruction: &CompiledInstruction,
    account_keys: &AccountKeys,
) -> Result<ParsedInstructionEnum, ParseInstructionError> {
    match instruction.accounts.iter().max() {
        Some(index) if (*index as usize) < account_keys.len() => {}
        _ => {
            // Runtime should prevent this from ever happening
            return Err(ParseInstructionError::InstructionKeyMismatch(
                ParsableProgram::PplAssociatedTokenAccount,
            ));
        }
    }
    check_num_associated_token_accounts(&instruction.accounts, 7)?;
    Ok(ParsedInstructionEnum {
        instruction_type: "create".to_string(),
        info: json!({
            "source": account_keys[instruction.accounts[0] as usize].to_string(),
            "account": account_keys[instruction.accounts[1] as usize].to_string(),
            "wallet": account_keys[instruction.accounts[2] as usize].to_string(),
            "mint": account_keys[instruction.accounts[3] as usize].to_string(),
            "systemProgram": account_keys[instruction.accounts[4] as usize].to_string(),
            "tokenProgram": account_keys[instruction.accounts[5] as usize].to_string(),
            "rentSysvar": account_keys[instruction.accounts[6] as usize].to_string(),
        }),
    })
}

fn check_num_associated_token_accounts(
    accounts: &[u8],
    num: usize,
) -> Result<(), ParseInstructionError> {
    check_num_accounts(accounts, num, ParsableProgram::PplAssociatedTokenAccount)
}

#[cfg(test)]
mod test {
    use ppl_associated_token_account::create_associated_token_account;
    use {
        super::*,
        put_account_decoder::parse_token::pubkey_from_ppl_token,
        ppl_associated_token_account::{
            get_associated_token_address,
            put_program::{
                instruction::CompiledInstruction as PplAssociatedTokenCompiledInstruction,
                message::Message, pubkey::Pubkey as PplAssociatedTokenPubkey, sysvar,
            },
        },
    };

    fn convert_pubkey(pubkey: Pubkey) -> PplAssociatedTokenPubkey {
        PplAssociatedTokenPubkey::new_from_array(pubkey.to_bytes())
    }

    fn convert_compiled_instruction(
        instruction: &PplAssociatedTokenCompiledInstruction,
    ) -> CompiledInstruction {
        CompiledInstruction {
            program_id_index: instruction.program_id_index,
            accounts: instruction.accounts.clone(),
            data: instruction.data.clone(),
        }
    }

    fn convert_account_keys(message: &Message) -> Vec<Pubkey> {
        message
            .account_keys
            .iter()
            .map(pubkey_from_ppl_token)
            .collect()
    }

    #[test]
    fn test_parse_associated_token() {
        let funder = Pubkey::new_unique();
        let wallet_address = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let associated_account_address =
            get_associated_token_address(&convert_pubkey(wallet_address), &convert_pubkey(mint));
        #[allow(deprecated)]
        let create_ix = create_associated_token_account(
            &convert_pubkey(funder),
            &convert_pubkey(wallet_address),
            &convert_pubkey(mint),
        );
        let message = Message::new(&[create_ix], None);
        let compiled_instruction = convert_compiled_instruction(&message.instructions[0]);
        assert_eq!(
            parse_associated_token(
                &compiled_instruction,
                &AccountKeys::new(&convert_account_keys(&message), None)
            )
            .unwrap(),
            ParsedInstructionEnum {
                instruction_type: "create".to_string(),
                info: json!({
                    "source": funder.to_string(),
                    "account": associated_account_address.to_string(),
                    "wallet": wallet_address.to_string(),
                    "mint": mint.to_string(),
                    "systemProgram": put_sdk::system_program::id().to_string(),
                    "tokenProgram": ppl_token::id().to_string(),
                    "rentSysvar": sysvar::rent::id().to_string(),
                })
            }
        );
    }
}

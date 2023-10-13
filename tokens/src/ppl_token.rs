use {
    crate::{
        args::{DistributeTokensArgs, PplTokenArgs},
        commands::{get_fee_estimate_for_messages, Allocation, Error, FundingSource},
    },
    console::style,
    put_account_decoder::parse_token::{
        pubkey_from_ppl_token, real_number_string, real_number_string_trimmed, ppl_token_pubkey,
    },
    put_rpc_client::rpc_client::RpcClient,
    put_sdk::{instruction::Instruction, message::Message, native_token::lamports_to_put},
    put_transaction_status::parse_token::ppl_token_instruction,
    ppl_associated_token_account::{
        create_associated_token_account, get_associated_token_address
    },
    ppl_token::{
        put_program::program_pack::Pack,
        state::{Account as PplTokenAccount, Mint},
    },
    
};

pub fn update_token_args(client: &RpcClient, args: &mut Option<PplTokenArgs>) -> Result<(), Error> {
    if let Some(ppl_token_args) = args {
        let sender_account = client
            .get_account(&ppl_token_args.token_account_address)
            .unwrap_or_default();
	    ppl_token_args.mint = PplTokenAccount::unpack(&sender_account.data)?.mint;
        update_decimals(client, args)?;
    }
    Ok(())
}

pub fn update_decimals(client: &RpcClient, args: &mut Option<PplTokenArgs>) -> Result<(), Error> {
    if let Some(ppl_token_args) = args {
        let mint_account = client.get_account(&ppl_token_args.mint).unwrap_or_default();
        let mint = Mint::unpack(&mint_account.data)?;
        ppl_token_args.decimals = mint.decimals;
    }
    Ok(())
}

pub fn ppl_token_amount(amount: f64, decimals: u8) -> u128 {
    (amount * 10_usize.pow(decimals as u32) as f64) as u128
}

pub fn build_ppl_token_instructions(
    allocation: &Allocation,
    args: &DistributeTokensArgs,
    do_create_associated_token_account: bool,
) -> Vec<Instruction> {
    let ppl_token_args = args
        .ppl_token_args
        .as_ref()
        .expect("ppl_token_args must be some");
    let wallet_address = allocation.recipient.parse().unwrap();
    let associated_token_address =
        get_associated_token_address(&wallet_address, &ppl_token_args.mint);
    let mut instructions = vec![];
    if do_create_associated_token_account {
        let create_associated_token_account_instruction = create_associated_token_account(
            &ppl_token_pubkey(&args.fee_payer.pubkey()),
            &wallet_address,
            &ppl_token_pubkey(&ppl_token_args.mint),
        );
        instructions.push(ppl_token_instruction(
            create_associated_token_account_instruction,
        ));
    }
    let ppl_instruction = ppl_token::instruction::transfer_checked(
        &ppl_token::id(),
        &ppl_token_pubkey(&ppl_token_args.token_account_address),
        &ppl_token_pubkey(&ppl_token_args.mint),
        &associated_token_address,
        &ppl_token_pubkey(&args.sender_keypair.pubkey()),
        &[],
        allocation.amount,
        ppl_token_args.decimals,
    )
    .unwrap();
    instructions.push(ppl_token_instruction(ppl_instruction));
    instructions
}

pub fn check_ppl_token_balances(
    messages: &[Message],
    allocations: &[Allocation],
    client: &RpcClient,
    args: &DistributeTokensArgs,
    created_accounts: u64,
) -> Result<(), Error> {
    let ppl_token_args = args
        .ppl_token_args
        .as_ref()
        .expect("ppl_token_args must be some");
    let allocation_amount: u128 = allocations.iter().map(|x| x.amount).sum();
    let fees = get_fee_estimate_for_messages(messages, client)?;

    let token_account_rent_exempt_balance =
        client.get_minimum_balance_for_rent_exemption(PplTokenAccount::LEN)?;
    let account_creation_amount = created_accounts as u128 * token_account_rent_exempt_balance;
    let fee_payer_balance = client.get_balance(&args.fee_payer.pubkey())?;
    if fee_payer_balance < fees + account_creation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::FeePayer].into(),
            lamports_to_put(fees + account_creation_amount).to_string(),
        ));
    }
    let source_token_account = client
        .get_account(&ppl_token_args.token_account_address)
        .unwrap_or_default();
    let source_token = PplTokenAccount::unpack(&source_token_account.data)?;
    if source_token.amount < allocation_amount {
        return Err(Error::InsufficientFunds(
            vec![FundingSource::PplTokenAccount].into(),
            real_number_string_trimmed(allocation_amount, ppl_token_args.decimals),
        ));
    }
    Ok(())
}

pub fn print_token_balances(
    client: &RpcClient,
    allocation: &Allocation,
    ppl_token_args: &PplTokenArgs,
) -> Result<(), Error> {
    let address = allocation.recipient.parse().unwrap();
    let expected = allocation.amount;
    let associated_token_address = get_associated_token_address(
        &ppl_token_pubkey(&address),
        &ppl_token_pubkey(&ppl_token_args.mint),
    );
    let recipient_account = client
        .get_account(&pubkey_from_ppl_token(&associated_token_address))
        .unwrap_or_default();
    let (actual, difference) = if let Ok(recipient_token) =
        PplTokenAccount::unpack(&recipient_account.data)
    {
        let actual_ui_amount = real_number_string(recipient_token.amount, ppl_token_args.decimals);
        let delta_string =
            real_number_string(recipient_token.amount - expected, ppl_token_args.decimals);
        (
            style(format!("{:>24}", actual_ui_amount)),
            format!("{:>24}", delta_string),
        )
    } else {
        (
            style("Associated token account not yet created".to_string()).yellow(),
            "".to_string(),
        )
    };
    println!(
        "{:<44}  {:>24}  {:>24}  {:>24}",
        allocation.recipient,
        real_number_string(expected, ppl_token_args.decimals),
        actual,
        difference,
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    // The following unit tests were written for v1.4 using the ProgramTest framework, passing its
    // BanksClient into the `put-tokens` methods. 
    // These tests were removed rather than rewritten to avoid accruing technical debt. Once a new
    // rpc/client framework is implemented, they should be restored.
    //
    // async fn test_process_ppl_token_allocations()
    // async fn test_process_ppl_token_transfer_amount_allocations()
    // async fn test_check_ppl_token_balances()
    //
}

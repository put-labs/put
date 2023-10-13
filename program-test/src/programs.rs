use put_sdk::{
    account::{Account, AccountSharedData},
    bpf_loader_upgradeable::UpgradeableLoaderState,
    pubkey::Pubkey,
    rent::Rent,
};

mod ppl_token {
    put_sdk::declare_id!("PutToken11111111111111111111111111111111111");
}
mod ppl_memo_1_0 {
    put_sdk::declare_id!("8jjXD6tToXM9zJiLG44t1P4nhrf8zrVLy3T39CvBEgaa");
}
mod ppl_memo_3_0 {
    put_sdk::declare_id!("PutMemo111111111111111111111111111111111111");
}
mod ppl_associated_token_account {
    put_sdk::declare_id!("PutATA1111111111111111111111111111111111111");
}

static PPL_PROGRAMS: &[(Pubkey, &[u8])] = &[
    (ppl_token::ID, include_bytes!("programs/ppl_token.so")),
    (
        ppl_memo_1_0::ID,
        include_bytes!("programs/ppl_memo.so"),
    ),
    (
        ppl_memo_3_0::ID,
        include_bytes!("programs/ppl_memo.so"),
    ),
    (
        ppl_associated_token_account::ID,
        include_bytes!("programs/ppl_associated_token_account.so"),
    ),
];

pub fn ppl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    PPL_PROGRAMS
        .iter()
        .map(|(program_id, elf)| {
            (
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(elf.len()).min(1),
                    data: elf.to_vec(),
                    owner: put_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                }),
            )
        })
        .collect()
}

static SPL_PROGRAMS: &[(Pubkey, Pubkey, &[u8])] = &[
    (
        ppl_token::ID,
        put_sdk::bpf_loader::ID,
        include_bytes!("programs/ppl_token.so"),
    ),
    (
        ppl_memo_1_0::ID,
        put_sdk::bpf_loader_upgradeable::ID,
        include_bytes!("programs/ppl_token.so"),
    ),
    (
        ppl_memo_1_0::ID,
        put_sdk::bpf_loader::ID,
        include_bytes!("programs/ppl_memo.so"),
    ),
    (
        ppl_memo_3_0::ID,
        put_sdk::bpf_loader::ID,
        include_bytes!("programs/ppl_memo.so"),
    ),
    (
        ppl_associated_token_account::ID,
        put_sdk::bpf_loader::ID,
        include_bytes!("programs/ppl_associated_token_account.so"),
    ),
];

pub fn spl_programs(rent: &Rent) -> Vec<(Pubkey, AccountSharedData)> {
    SPL_PROGRAMS
        .iter()
        .flat_map(|(program_id, loader_id, elf)| {
            let mut accounts = vec![];
            let data = if *loader_id == put_sdk::bpf_loader_upgradeable::ID {
                let (programdata_address, _) =
                    Pubkey::find_program_address(&[program_id.as_ref()], loader_id);
                let mut program_data = bincode::serialize(&UpgradeableLoaderState::ProgramData {
                    slot: 0,
                    upgrade_authority_address: Some(Pubkey::default()),
                })
                .unwrap();
                program_data.extend_from_slice(elf);
                accounts.push((
                    programdata_address,
                    AccountSharedData::from(Account {
                        lamports: rent.minimum_balance(program_data.len()).max(1),
                        data: program_data,
                        owner: *loader_id,
                        executable: false,
                        rent_epoch: 0,
                    }),
                ));
                bincode::serialize(&UpgradeableLoaderState::Program {
                    programdata_address,
                })
                .unwrap()
            } else {
                elf.to_vec()
            };
            accounts.push((
                *program_id,
                AccountSharedData::from(Account {
                    lamports: rent.minimum_balance(data.len()).max(1),
                    data,
                    owner: *loader_id,
                    executable: true,
                    rent_epoch: 0,
                }),
            ));
            accounts.into_iter()
        })
        .collect()
}

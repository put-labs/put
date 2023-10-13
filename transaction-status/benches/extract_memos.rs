#![feature(test)]

extern crate test;

use {
    put_sdk::{instruction::CompiledInstruction, message::Message, pubkey::Pubkey},
    put_transaction_status::extract_memos::{ppl_memo_id_v1, ExtractMemos},
    test::Bencher,
};

#[bench]
fn bench_extract_memos(b: &mut Bencher) {
    let mut account_keys: Vec<Pubkey> = (0..64).map(|_| Pubkey::new_unique()).collect();
    account_keys[62] = ppl_memo_id_v1();
    account_keys[63] = ppl_memo_id_v1();
    let memo = "Test memo";

    let instructions: Vec<_> = (0..20)
        .map(|i| CompiledInstruction {
            program_id_index: 62 + (i % 2),
            accounts: vec![],
            data: memo.as_bytes().to_vec(),
        })
        .collect();

    let message = Message {
        account_keys,
        instructions,
        ..Message::default()
    };

    b.iter(|| message.extract_memos());
}

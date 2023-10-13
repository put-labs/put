use {
    crate::{inline_ppl_associated_token_account, inline_ppl_token, inline_ppl_token_2022},
    put_sdk::pubkey::Pubkey,
};

lazy_static! {
    /// Vector of static token & mint IDs
    pub static ref STATIC_IDS: Vec<Pubkey> = vec![
        inline_ppl_associated_token_account::id(),
        // inline_ppl_associated_token_account::program_v1_1_0::id(),
        inline_ppl_token::id(),
        inline_ppl_token::native_mint::id(),
        inline_ppl_token_2022::id(),
    ];
}

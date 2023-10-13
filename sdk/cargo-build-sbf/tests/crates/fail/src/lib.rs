//! Example Rust-based SBF noop program

use put_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

put_program::entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // error to make build fail: no return value
}

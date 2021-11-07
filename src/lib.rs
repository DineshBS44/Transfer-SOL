use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;

    **source_info.try_borrow_mut_lamports()? -= 100; // 100 lamports are withdrawn from source account

    **destination_info.try_borrow_mut_lamports()? += 100; // 100 lamports are deposited to destination account

    Ok(())
}

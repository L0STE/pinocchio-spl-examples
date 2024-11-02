use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_token::instructions::InitilizeAccount3;

pub fn initialize_account_3(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [token, mint, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Not sure about this one. *const or *mut ?
    let owner = unsafe { *(data.as_ptr() as *const Pubkey) };

    InitilizeAccount3 {
        token,
        mint,
        owner: &owner,
    }
    .invoke()?;

    Ok(())
}

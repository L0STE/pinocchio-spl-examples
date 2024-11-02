use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::InitilizeAccount;

pub fn initialize_account(accounts: &[AccountInfo]) -> ProgramResult {
    let [token, mint, owner, rent_sysvar, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    InitilizeAccount {
        token,
        mint,
        owner,
        rent_sysvar,
    }
    .invoke()?;

    Ok(())
}

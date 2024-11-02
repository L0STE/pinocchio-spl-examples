use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::CloseAccount;

pub fn close_account(accounts: &[AccountInfo]) -> ProgramResult {
    let [account, destination, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    CloseAccount {
        account,
        destination,
        authority,
    }
    .invoke()?;

    Ok(())
}

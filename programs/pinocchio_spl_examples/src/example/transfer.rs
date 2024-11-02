use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::Transfer;

pub fn transfer(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [from, to, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    Transfer {
        from,
        to,
        authority,
        amount,
    }
    .invoke()?;

    Ok(())
}

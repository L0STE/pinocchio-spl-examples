use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::Burn;

pub fn burn(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [token, mint, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    Burn {
        token,
        mint,
        authority,
        amount,
    }
    .invoke()?;

    Ok(())
}

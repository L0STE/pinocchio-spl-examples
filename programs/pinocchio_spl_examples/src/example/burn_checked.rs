use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::BurnChecked;

pub fn burn_checked(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [token, mint, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8)) };

    BurnChecked {
        token,
        mint,
        authority,
        amount,
        decimals,
    }
    .invoke()?;

    Ok(())
}

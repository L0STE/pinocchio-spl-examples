use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::TransferChecked;

pub fn transfer_checked(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [from, to, authority, mint, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };
    let decimals = unsafe { *(data.as_ptr().add(8)) };

    TransferChecked {
        from,
        to,
        authority,
        mint,
        amount,
        decimals,
    }
    .invoke()?;

    Ok(())
}

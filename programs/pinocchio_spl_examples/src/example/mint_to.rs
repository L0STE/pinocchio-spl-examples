use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::instructions::MintTo;

pub fn mint_to(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint, token, mint_authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    MintTo {
        mint,
        token,
        mint_authority,
        amount,
    }
    .invoke()?;

    Ok(())
}

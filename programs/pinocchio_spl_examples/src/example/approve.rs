use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError
};

use pinocchio_token::instructions::Approve;

pub fn approve(
    accounts: &[AccountInfo],
    data: &[u8]
) -> ProgramResult {

    let [
        token,
        delegate,
        authority,
        _token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let amount = unsafe { *(data.as_ptr() as *const u64) };

    Approve {
        token,
        delegate,
        authority,
        amount,
    }.invoke()?;

    Ok(())
}
use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use pinocchio_token::instructions::{AuthorityType, SetAuthority};

pub fn set_authority(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [account, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let authority_type = unsafe { *(data.as_ptr() as *const AuthorityType) };

    let new_authority = unsafe { *(data.as_ptr().add(1) as *const Pubkey) };

    SetAuthority {
        account,
        authority,
        authority_type,
        new_authority: Some(&new_authority),
    }
    .invoke()?;

    Ok(())
}

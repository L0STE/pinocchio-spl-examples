use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError, msg,
};

use pinocchio_token::instructions::{SetAuthority, AuthorityType};

pub fn set_authority(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [account, authority, _token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let authority_type = unsafe { *(data.as_ptr() as *const AuthorityType) };

    msg!("authority_type: {:?}", authority_type);

    let new_authority = Some( unsafe { *(data.as_ptr().add(1) as *const [u8; 32]) });

    SetAuthority { 
        account,
        authority,
        authority_type,
        new_authority,
    }.invoke()?;
   
    Ok(())
}

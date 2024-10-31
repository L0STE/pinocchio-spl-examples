use pinocchio::{
    account_info::AccountInfo, ProgramResult, program_error::ProgramError,
};

use pinocchio_token::state::{TokenAccount, AccountState};

pub fn token_getters(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [token] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let token_account = &TokenAccount::from_account_info(token);

    let mint = unsafe { *(data.as_ptr() as *const [u8; 32]) };
    assert_eq!(token_account.mint(), mint);

    // Note: Authority is the owner of the token account
    let authority = unsafe { *(data.as_ptr().add(32) as *const [u8; 32]) }; 
    assert_eq!(token_account.authority(), authority);

    let amount = unsafe { *(data.as_ptr().add(64) as *const u64) };
    assert_eq!(token_account.amount(), amount);

    let has_delegate = unsafe { *(data.as_ptr().add(72) as *const bool) };
    assert_eq!(token_account.has_delegate(), has_delegate);

    if has_delegate {
        let delegate = unsafe { *(data.as_ptr().add(73) as *const [u8; 32]) };
        assert_eq!(token_account.delegate(), Some(delegate));
    } else {
        assert_eq!(token_account.delegate(), None);
    }

    let state = unsafe { *(data.as_ptr().add(105) as *const AccountState) };
    assert_eq!(token_account.state(), state);

    let is_native = unsafe { *(data.as_ptr().add(106) as *const bool) };
    assert_eq!(token_account.is_native(), is_native);

    if is_native {
        let native_amount = unsafe { *(data.as_ptr().add(107) as *const u64) };
        assert_eq!(token_account.native_amount(), Some(native_amount));
    } else {
        assert_eq!(token_account.native_amount(), None);
    }

    let delegated_amount = unsafe { *(data.as_ptr().add(115) as *const u64) };
    assert_eq!(token_account.delegated_amount(), delegated_amount);

    let has_close_authority = unsafe { *(data.as_ptr().add(123) as *const bool) };
    assert_eq!(token_account.has_close_authority(), has_close_authority);

    if has_close_authority {
        let close_authority = unsafe { *(data.as_ptr().add(124) as *const [u8; 32]) };
        assert_eq!(token_account.close_authority(), Some(close_authority));
    } else {
        assert_eq!(token_account.close_authority(), None);
    }
   
    Ok(())
}

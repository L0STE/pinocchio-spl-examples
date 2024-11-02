use pinocchio::{account_info::AccountInfo, program_error::ProgramError, ProgramResult};

use pinocchio_token::state::Mint;

pub fn mint_getters(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [mint] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let mint_account = &Mint::from_account_info(mint);

    let has_mint_authority = unsafe { *(data.as_ptr() as *const bool) };
    assert_eq!(mint_account.has_mint_authority(), has_mint_authority);

    if has_mint_authority {
        let mint_authority = unsafe { *(data.as_ptr().add(1) as *const [u8; 32]) };
        assert_eq!(mint_account.mint_authority(), Some(mint_authority));
    } else {
        assert_eq!(mint_account.mint_authority(), None);
    }

    let supply = unsafe { *(data.as_ptr().add(33) as *const u64) };
    assert_eq!(mint_account.supply(), supply);

    let decimals = unsafe { *(data.as_ptr().add(41)) };
    assert_eq!(mint_account.decimals(), decimals);

    let is_initialized = unsafe { *(data.as_ptr().add(42) as *const bool) };
    assert_eq!(mint_account.is_initialized(), is_initialized);

    let has_freeze_authority = unsafe { *(data.as_ptr().add(43) as *const bool) };
    assert_eq!(mint_account.has_freeze_authority(), has_freeze_authority);

    if has_freeze_authority {
        let freeze_authority = unsafe { *(data.as_ptr().add(44) as *const [u8; 32]) };
        assert_eq!(mint_account.freeze_authority(), Some(freeze_authority));
    } else {
        assert_eq!(mint_account.freeze_authority(), None);
    }

    Ok(())
}

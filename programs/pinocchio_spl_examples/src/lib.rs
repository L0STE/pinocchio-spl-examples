#[cfg(test)]
mod tests;

mod example;
use example::{
    approve::approve, approve_checked::approve_checked, burn::burn, burn_checked::burn_checked,
    close_account::close_account, freeze_account::freeze_account,
    initialize_account::initialize_account, initialize_account_2::initialize_account_2,
    initialize_account_3::initialize_account_3, initialize_mint::initialize_mint,
    initialize_mint_2::initialize_mint_2, mint_getters::mint_getters, mint_to::mint_to,
    mint_to_checked::mint_to_checked, revoke::revoke, set_authority::set_authority,
    sync_native::sync_native, thaw_account::thaw_account, token_getters::token_getters,
    transfer::transfer, transfer_checked::transfer_checked, TestInstruction,
};

use pinocchio::account_info::AccountInfo;
use pinocchio::entrypoint;
use pinocchio::program_error::ProgramError;
use pinocchio::pubkey::Pubkey;
use pinocchio::ProgramResult;

entrypoint!(process_instruction);

pub const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match TestInstruction::try_from(discriminator)? {
        TestInstruction::ApproveChecked => approve_checked(accounts, data),
        TestInstruction::Approve => approve(accounts, data),
        TestInstruction::BurnChecked => burn_checked(accounts, data),
        TestInstruction::Burn => burn(accounts, data),
        TestInstruction::CloseAccount => close_account(accounts),
        TestInstruction::FreezeAccount => freeze_account(accounts),
        TestInstruction::InitializeAccount => initialize_account(accounts),
        TestInstruction::InitializeAccount2 => initialize_account_2(accounts, data),
        TestInstruction::InitializeAccount3 => initialize_account_3(accounts, data),
        TestInstruction::InitializeMint2 => initialize_mint_2(accounts, data),
        TestInstruction::InitializeMint => initialize_mint(accounts, data),
        TestInstruction::Transfer => transfer(accounts, data),
        TestInstruction::Revoke => revoke(accounts),
        TestInstruction::MintTo => mint_to(accounts, data),
        TestInstruction::ThawAccount => thaw_account(accounts),
        TestInstruction::TransferChecked => transfer_checked(accounts, data),
        TestInstruction::MintToChecked => mint_to_checked(accounts, data),
        TestInstruction::SetAuthority => set_authority(accounts, data),
        TestInstruction::SyncNative => sync_native(accounts),

        TestInstruction::TokenGetters => token_getters(accounts, data),
        TestInstruction::MintGetters => mint_getters(accounts, data),
    }
}

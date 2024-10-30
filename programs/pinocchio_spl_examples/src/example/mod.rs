use pinocchio::program_error::ProgramError;

pub mod initialize_mint_2;
pub mod initialize_mint;
pub mod mint_to_checked;
pub mod mint_to;
pub mod revoke;
pub mod set_authority;
pub mod sync_native;
pub mod thaw_account;
pub mod transfer_checked;
pub mod transfer;
pub mod approve_checked;
pub mod approve;
pub mod burn_checked;
pub mod burn;
pub mod close_account;
pub mod freeze_account;
pub mod initialize_account_2;
pub mod initialize_account_3;
pub mod initialize_account;

#[derive(Clone, Copy, Debug)]
pub enum TestInstruction {
    InitializeMint2,
    InitializeMint,
    MintToChecked,
    MintTo,
    Revoke,
    // SetAuthority, -> todo after fixing the issue
    SyncNative,
    ThawAccount,
    TransferChecked,
    Transfer,
    ApproveChecked,
    Approve,
    BurnChecked,
    Burn,
    CloseAccount,
    FreezeAccount,
    InitializeAccount2,
    InitializeAccount3,
    InitializeAccount,
}

impl TryFrom<&u8> for TestInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TestInstruction::InitializeMint),
            1 => Ok(TestInstruction::InitializeAccount),
            3 => Ok(TestInstruction::Transfer),
            4 => Ok(TestInstruction::Approve),
            5 => Ok(TestInstruction::Revoke),
            7 => Ok(TestInstruction::MintTo),
            8 => Ok(TestInstruction::Burn),
            9 => Ok(TestInstruction::CloseAccount),
            10 => Ok(TestInstruction::FreezeAccount),
            11 => Ok(TestInstruction::ThawAccount),
            12 => Ok(TestInstruction::TransferChecked),
            13 => Ok(TestInstruction::ApproveChecked),
            14 => Ok(TestInstruction::MintToChecked),
            15 => Ok(TestInstruction::BurnChecked),
            16 => Ok(TestInstruction::InitializeAccount2),
            17 => Ok(TestInstruction::SyncNative),
            18 => Ok(TestInstruction::InitializeAccount3),
            20 => Ok(TestInstruction::InitializeMint2),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
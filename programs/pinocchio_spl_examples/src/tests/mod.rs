#[cfg(test)]
mod tests {
    use mollusk_svm::{program, result::Check, Mollusk};

    use solana_sdk::{
        account::{AccountSharedData, ReadableAccount, WritableAccount}, instruction::{AccountMeta, Instruction}, program_option::COption, program_pack::Pack, pubkey::Pubkey,
    };

    use spl_token::state::AccountState;

    #[test] 
    #[ignore = "working"]
    fn initialize_mint() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        let (rent_sysvar, rent_sysvar_account) = (solana_sdk::sysvar::rent::ID, program::create_program_account_loader_v3(&solana_sdk::sysvar::rent::ID));
        
        // Accounts
        let mint = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();

        // Data
        let data = [
            vec![0], 
            vec![6], 
            mint_authority.to_bytes().to_vec(), 
        ].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(rent_sysvar, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let mint_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN);

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(mint_authority),      // 32 + 4 = 36
                supply: 0,                                          // 8
                decimals: 6,                                        // 1
                is_initialized: true,                               // 1
                freeze_authority: COption::Some(mint_authority),    // 32 + 4 = 36
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let checks = vec![
            Check::success(),
            Check::account(&mint)
                .data(mint_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (mint, AccountSharedData::new(mint_lamports, spl_token::state::Mint::LEN, &spl_token::ID)),
                (rent_sysvar, rent_sysvar_account),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    fn initialize_account() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        let (rent_sysvar, rent_sysvar_account) = (solana_sdk::sysvar::rent::ID, program::create_program_account_loader_v3(&solana_sdk::sysvar::rent::ID));
        
        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let owner = Pubkey::new_unique();

        // Data
        let data = [1];

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(owner, false),
                AccountMeta::new_readonly(rent_sysvar, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let check = [
            Check::success(), 
            Check::account(&token)
                .data(token_account.data())
                .build()
        ];

        let token_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN);

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, AccountSharedData::new(token_lamports, spl_token::state::Account::LEN, &spl_token::ID)),
                (mint, mint_account),
                (owner, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (rent_sysvar, rent_sysvar_account),
                (token_program, token_program_account),
            ],
            &check,
        );
    }

    #[test]
    #[ignore = "working"]
    fn transfer() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let from = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Data
        let data = [vec![3], 1_000_000u64.to_le_bytes().to_vec()].concat();

        let mut from_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            from_account.data_as_mut_slice(),
        ).unwrap();

        let mut to_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: Pubkey::default(),
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            to_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_from_account_data = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_from_account_data.data_as_mut_slice(),
        ).unwrap();

        let mut new_to_account_data = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: Pubkey::default(),
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_to_account_data.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(from, false),
                AccountMeta::new(to, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let check = [
            Check::success(),
            Check::account(&from)
                .data(new_from_account_data.data())
                .build(),
            Check::account(&to)
                .data(new_to_account_data.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (from, from_account),
                (to, to_account),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &check,
        );
    }

    #[test]
    #[ignore = "working"]
    fn approve() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Data
        let data = [vec![4], 1_000_000u64.to_le_bytes().to_vec()].concat();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(delegate),
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(delegate, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let check = [
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (delegate, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &check,
        );
    }

    // Check is correct, but it's impossible to check cause the way that revoke works is that it only overwrite the COption flag to None instead of zeroing out the Authority as well. 
    #[test]
    #[ignore = "working"]
    fn revoke() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();

        // Data
        let data = [5];

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,                                   
                owner: authority,                     
                amount: 1_000_000,                   
                delegate: COption::Some(delegate),     
                state: AccountState::Initialized,       
                is_native: COption::None,              
                delegated_amount: 0,                   
                close_authority: COption::None,        
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,     
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let check = [
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &check,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn mint_to() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let mint = Pubkey::new_unique();
        let token = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();

        // Data
        let data = [vec![7], 1_000_000u64.to_le_bytes().to_vec()].concat();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(mint_authority),
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: mint_authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: mint_authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint, false),
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (mint, mint_account),
                (token, token_account),
                (mint_authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn burn() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Data
        let data = [vec![8], 1_000_000u64.to_le_bytes().to_vec()].concat();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (mint, mint_account),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn close_account() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let destination = Pubkey::new_unique();
        let authority = Pubkey::new_unique(); 

        // Data
        let data = [9];

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new(destination, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(&[])
                .lamports(0)
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (destination, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn freeze_account() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let authority = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let freeze_authority = Pubkey::new_unique(); 

        // Data
        let data = [10];

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(freeze_authority),
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(freeze_authority),
                state: AccountState::Frozen,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(freeze_authority),
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::Some(freeze_authority),
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(freeze_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (mint, mint_account),
                (freeze_authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn thaw_account() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let authority = Pubkey::new_unique(); 
        let mint = Pubkey::new_unique();
        let freeze_authority = Pubkey::new_unique(); 

        // Data
        let data = [11];

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(freeze_authority),
                state: AccountState::Frozen,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(freeze_authority),
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(freeze_authority),
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::Some(freeze_authority),
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(freeze_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (mint, mint_account),
                (freeze_authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    pub fn transfer_checked() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let from = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        let authority = Pubkey::new_unique(); 

        // Data
        let data = [vec![12], 1_000_000u64.to_le_bytes().to_vec(), vec![6]].concat();

        let mut from_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            from_account.data_as_mut_slice(),
        ).unwrap();

        let mut to_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: Pubkey::default(),
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            to_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(from, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new(to, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (from, from_account),
                (mint, mint_account),
                (to, to_account),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err());
    }

    #[test]
    #[ignore = "working"]
    pub fn approve_checked() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let delegate = Pubkey::new_unique();
        let authority = Pubkey::new_unique(); 

        // Data
        let data = [vec![13], 1_000_000u64.to_le_bytes().to_vec(), vec![6]].concat();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::Some(delegate),
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 1_000_000,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(delegate, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (mint, mint_account),
                (delegate, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn mint_to_checked() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let mint = Pubkey::new_unique();
        let token = Pubkey::new_unique();
        let mint_authority = Pubkey::new_unique();

        // Data
        let data = [vec![14], 1_000_000u64.to_le_bytes().to_vec(), vec![6]].concat();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::Some(mint_authority),
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: mint_authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: mint_authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint, false),
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint_authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (mint, mint_account),
                (token, token_account),
                (mint_authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    pub fn burn_checked() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        let authority = Pubkey::new_unique();

        // Data
        let data = [vec![15], 1_000_000u64.to_le_bytes().to_vec(), vec![6]].concat();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: authority,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 1_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(authority, true),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(new_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, token_account),
                (mint, mint_account),
                (authority, AccountSharedData::new(1_000_000_000, 0, &Pubkey::default())),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    fn initialize_account_2() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        let (rent_sysvar, rent_sysvar_account) = (solana_sdk::sysvar::rent::ID, program::create_program_account_loader_v3(&solana_sdk::sysvar::rent::ID));
        
        // Accounts
        let token = Pubkey::new_unique();
        let owner = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        // Data
        let data = [vec![16], owner.to_bytes().to_vec()].concat();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner,
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(rent_sysvar, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let token_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN);

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, AccountSharedData::new(token_lamports, spl_token::state::Account::LEN, &spl_token::ID)),
                (mint, mint_account),
                (rent_sysvar, rent_sysvar_account),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]    
    #[ignore = "working"]
    fn sync_native() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));
        
        // Accounts
        let native_token = Pubkey::new_unique();

        // Data
        let data = [17];

        let native_token_minimum_balance = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN);

        let mut native_token_account = AccountSharedData::new(
            native_token_minimum_balance + 2_000_000,
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: native_token,
                owner: Pubkey::default(),
                amount: 1_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::Some(native_token_minimum_balance),
                delegated_amount: 0,
                close_authority: COption::None,
            },
            native_token_account.data_as_mut_slice(),
        ).unwrap();

        let mut new_native_token_account = AccountSharedData::new(
            native_token_minimum_balance,
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: native_token,
                owner: Pubkey::default(),
                amount: 2_000_000,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::Some(native_token_minimum_balance),
                delegated_amount: 0,
                close_authority: COption::None,
            },
            new_native_token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(native_token, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let checks = vec![
            Check::success(),
            Check::account(&native_token)
                .data(new_native_token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (native_token, native_token_account),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    fn initialize_account_3() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));
        
        // Accounts
        let token = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        // Data
        let data = [vec![18], Pubkey::default().to_bytes().to_vec()].concat();

        let mut mint_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_account.data_as_mut_slice(),
        ).unwrap();

        let mut token_account = AccountSharedData::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint,
                owner: Pubkey::default(),
                amount: 0,
                delegate: COption::None,
                state: AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            token_account.data_as_mut_slice(),
        ).unwrap();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(token, false),
                AccountMeta::new_readonly(mint, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let token_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Account::LEN);

        let checks = vec![
            Check::success(),
            Check::account(&token)
                .data(&token_account.data())
                .build(),
        ];

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (token, AccountSharedData::new(token_lamports, spl_token::state::Account::LEN, &spl_token::ID)),
                (mint, mint_account),
                (token_program, token_program_account),
            ],
            &checks,
        );
    }

    #[test]
    #[ignore = "working"]
    fn initialize_mint_2() {
        let program_id = Pubkey::new_from_array(five8_const::decode_32_const("22222222222222222222222222222222222222222222"));

        let mut mollusk = Mollusk::new(&program_id, "../../target/deploy/pinocchio_spl_examples");

        mollusk.add_program(&spl_token::ID, "src/tests/spl_token-3.5.0", &mollusk_svm::program::loader_keys::LOADER_V3);
        let (token_program, token_program_account) = (spl_token::ID, program::create_program_account_loader_v3(&spl_token::ID));

        // Accounts
        let mint = Pubkey::new_unique();

        // Data
        let data = [vec![20], vec![6], Pubkey::new_unique().to_bytes().to_vec(), Pubkey::new_unique().to_bytes().to_vec()].concat();

        // Instruction
        let instruction = Instruction::new_with_bytes(
            program_id,
            &data,
            vec![
                AccountMeta::new(mint, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );

        let mint_lamports = mollusk
            .sysvars
            .rent
            .minimum_balance(spl_token::state::Mint::LEN);

        let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
            &instruction,
            &vec![
                (mint, AccountSharedData::new(mint_lamports, spl_token::state::Mint::LEN, &spl_token::ID)),
                (token_program, token_program_account),
            ],
        );

        assert!(!result.program_result.is_err());
    }
}
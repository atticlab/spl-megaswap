//! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeAssetInput {
    pub weight: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct DepositInput {
    pub amount: u64,
}


/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub enum Instruction {
    /// Initializes asset input
    ///
    /// Inputs:
    ///  InitializeAssetInput
    ///  
    /// Accounts:
    ///   -           pool               Pool this asset will belong to
    ///   - mut       asset              New asset account to initialize
    ///   - mut       token              Token account to store assets, owner should be asset authority    
    ///   - sysvar    rent               Sysvar rent to check rent exempt balance on asset and token
    InitializeAsset,

    /// Initializes pool of assets
    ///
    /// Inputs:
    ///  InitializeAssetInput
    ///  
    /// Accounts:
    ///   - mut           pool               New pool to initialize    
    ///   - mut           pool_mint          New pool mint to initialize  
    ///   ?- mut          pool_token_account
    ///   - read, sysvar  rent               Rent sysvar to check pool and pool_mint accounts balance    
    ///   - read          program_token      Token program used to initialize the pool_mint
    ///   - mut           [asset]            Accounts of initialized assets with the same pool address  
    ///   -  
    InitializePool,


    /// Deposit all vs one? How to deposit one into empty?
    /// 
    /// Accounts:
    /// ?-           asset
    /// ?- mut,cpi   asset_token_account
    /// ?-           user_authority
    /// ?- mut,cpi   user_token_account_input
    /// ?-           pool_mint         
    /// ?- mut       user_pool_token_account    user's pool token account
    Deposit(DepositInput),

    /// if one  
    Withdraw,

    Swap,

    // ? like in balancer?
    UpdateWeight,

    // delegate amount with throw away signature?
}

/// Create `InitializeAsset` instruction
#[allow(clippy::too_many_arguments)]
pub fn initialize_asset(
    rent: &Pubkey,
    pool: &Pubkey,
    asset: &Pubkey,
    token: &Pubkey,
    input: InitializeAssetInput,
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let mut data = Instruction::InitializeAsset.try_to_vec()?;
    let mut input = input.try_to_vec()?;
    data.append(&mut input);
    let accounts = vec![
        AccountMeta::new_readonly(*rent, false),
        AccountMeta::new_readonly(*pool, false), // makes sure prepare in same transaction
        AccountMeta::new(*asset, false),
        AccountMeta::new(*token, false),
    ];
    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

/// Create `InitializePool` instruction
#[allow(clippy::too_many_arguments)]
pub fn initialize_pool(
    rent: &Pubkey,
    program_token: &Pubkey,
    pool: &Pubkey,
    pool_mint: &Pubkey,
    assets: &[Pubkey],
) -> Result<solana_program::instruction::Instruction, ProgramError> {
    let data = Instruction::InitializePool.try_to_vec()?;
    let mut accounts = vec![
        AccountMeta::new_readonly(*rent, false),
        AccountMeta::new_readonly(*program_token, false), // makes sure prepare in same transaction
        AccountMeta::new_readonly(*pool, false),          // makes sure prepare in same transaction
        AccountMeta::new(*pool_mint, false),
    ];

    for asset in assets {
        accounts.push(AccountMeta::new(*asset, false));
    }

    Ok(solana_program::instruction::Instruction {
        program_id: crate::id(),
        accounts,
        data,
    })
}

//! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeAssetInput {
    pub weight: u64,
}

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ToPrimitive)]
pub enum Instruction {
    /// Initializes asset input
    ///
    /// Inputs:
    ///  InitializeAssetInput
    ///  
    /// Accounts:
    ///   -           rent               Sysvar rent to check rent exempt balance on asset and token
    ///   -           pool               Pool this asset will belong to
    ///   - writable  asset              New asset account to initialize
    ///   - writable  token              Token account to store assets, owner should be asset authority    
    InitializeAsset,

    /// Initializes pool of assets
    ///
    /// Inputs:
    ///  InitializeAssetInput
    ///  
    /// Accounts:
    ///   -            rent               Rent sysvar to check pool and pool_mint accounts balance
    ///   -            program_token      Token program used to initialize the pool_mint
    ///   -            pool               New pool to initialize    
    ///   - writable   pool_mint          New pool mint to initialize
    ///   - writable   [asset]            Accounts of initialized assets with the same pool address
    InitializePool,

    Deposit,
    Withdraw,
    Swap,
    UpdateWeight,
}

// // /// Create `Prepare` instruction
// #[allow(clippy::too_many_arguments)]
// pub fn prepare(
//     _swap: &Pubkey,
//     swap_authority: &Pubkey,
//     signer: &Pubkey,
//     token_user_ab: &Pubkey,
//     token_user_bc: &Pubkey,
//     token_swap_ab: &Pubkey,
//     token_swap_bc: &Pubkey,
//     swap_ab: &Pubkey,
//     token_swap_ab_total: &Pubkey,
//     token_swap_ab_b_total: &Pubkey,
//     swap_bc: &Pubkey,
//     token_swap_bc_total: &Pubkey,
//     token_swap_bc_b_total: &Pubkey,
//     input: Prepare,
// ) -> Result<solana_program::instruction::Instruction, ProgramError> {
//     let mut data = Instruction::Prepare.try_to_vec()?;
//     let mut input = input.try_to_vec()?;
//     data.append(&mut input);
//     let accounts = vec![
//         AccountMeta::new_readonly(spl_token::id(), false),
//         AccountMeta::new_readonly(*_swap, true), // makes sure prepare in same transaction
//         AccountMeta::new_readonly(*swap_authority, false),
//         AccountMeta::new_readonly(*signer, true),
//         AccountMeta::new(*token_user_ab, false),
//         AccountMeta::new(*token_user_bc, false),
//         AccountMeta::new(*token_swap_ab, false),
//         AccountMeta::new(*token_swap_bc, false),
//         AccountMeta::new_readonly(*swap_ab, false),
//         AccountMeta::new_readonly(*token_swap_ab_total, false),
//         AccountMeta::new_readonly(*token_swap_ab_b_total, false),
//         AccountMeta::new_readonly(*swap_bc, false),
//         AccountMeta::new_readonly(*token_swap_bc_total, false),
//         AccountMeta::new_readonly(*token_swap_bc_b_total, false),
//     ];
//     Ok(solana_program::instruction::Instruction {
//         program_id: crate::id(),
//         accounts,
//         data,
//     })
// }

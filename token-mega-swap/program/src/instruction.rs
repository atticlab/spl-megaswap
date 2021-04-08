//! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::ToPrimitive;
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct InitializeAssetInput {
}

/// Instructions
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, ToPrimitive)]
pub enum Instruction {
    ///   Initializes a new mega swap.  
    ///   AB and BC token balances must satisfy constraints. Use math module get satisfying value.
    ///   Must be executed before `spl_token_swap::instruction::Initialize` in same transaction.
    ///
    ///   Accounts:
    ///   - `[]`                 program_token      
    ///   - `[writable, signer]` swap                    New Token-swap to prepare.
    ///   - `[]                  swap_authority          derived from `create_program_address(&[Token-swap account])`
    ///   - `[signer]`           signer                  User authority
    ///   - `[writable]`         token_user_ab           Token owned by user Authority.
    ///   - `[writable]`         token_user_bc           Token owned by user authority.
    ///   - `[writable]`         token_swap_ab           Must be zero, owned by swap authority.
    ///   - `[writable]`         token_swap_bc           Must be zero, owned by swap authority.
    //    - `[]`                 swap_ab                 Swap from A to B
    ///   - `[]`                 token_swap_ab_total          
    ///   - `[]`                 token_swap_ab_b_total      
    //    -  []                  swap_bc                 Swap from B to C
    ///   - `[]`                 token_swap_bc_total   
    ///   - `[]`                 token_swap_bc_b_total
    InitializeAsset,

    ///   Swap the tokens in the pool. Accepts `spl_token_swap::instruction::Swap` input.
    ///
    ///   Swaps A to C via AB and BC pools.
    ///
    /// Accounts:
    /// []                  program_token
    /// []                  program_token_swap
    /// []                  swap_ab_authority
    /// []                  token_mint_ab
    /// [writable]          token_swap_ab
    /// [writable]          token_swap_ab_a
    /// [writable]          token_swap_ab_b
    /// []                  swap_bc_authority
    /// []                  token_mint_bc
    /// [writable]          token_swap_bc
    /// [writable]          token_swap_bc_b
    /// [writable]          token_swap_bc_c
    /// [writable]          token_swap_bc_supply
    /// [writable]          token_swap_bc_fee
    /// []                  swap_abc_authority
    /// []                  token_mint_abc
    /// []                  token_swap_abc
    /// [writable]          token_swap_abc_ab
    /// [writable]          token_swap_abc_bc
    /// [writable]          token_swap_abc_fee
    /// [writable]          token_user_a
    /// [writable]          token_user_ab_temp
    /// [writable]          token_user_bc_temp
    /// [writable]          token_user_c
    /// [signer]            signer    
    Swap,
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

//! Instruction types

use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use num_derive::{ToPrimitive, FromPrimitive};
use solana_program::{instruction::AccountMeta, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive, FromPrimitive)]
pub enum PoolVersion {
    Uninitialized,
    InitializedV1,
} 

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema, ToPrimitive, FromPrimitive)]
pub enum AssetVersion {
    Uninitialized,
    InitializedV1,
} 

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct PoolState {
    pub version: PoolVersion,
    /// Mint issuing pool tokens
    pub pool_mint: Pubkey,    
    /// ISSUE: may be we should do some alphanumeric sorting of seeds?
    /// Pubkey generated using program address derivation with all asset accounts as seed. 
    pub assets_hash: Pubkey,
    /// Sum of all asset weights
    pub weight_total :	u64,
    // fee_account	TBD	
    // fees	TBD	
    // authority_fee	Pubkey	
    // authority_merge	Pubkey	
    // authority_weights	Pubkey	
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone, BorshSchema)]
pub struct AssetState {
    pub version: AssetVersion,
    /// Reference to the pool
    pub pool: Pubkey,
    /// Account storing tokens
    pub token_account:	Pubkey,	
    /// This asset weight
    pub weight:	u64,
    // weight_valid_until	u64	Cutoff timestamp of weight validity
}


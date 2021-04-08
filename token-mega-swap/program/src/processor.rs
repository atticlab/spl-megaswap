//! Program state processor
#[allow(unused_imports)]
use borsh::{BorshDeserialize, BorshSerialize};
use num_traits::ToPrimitive;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    program_pack::Pack, pubkey::Pubkey,
};
use spl_token::state::Account;

use super::borsh::*;
use crate::{error::PoolError, instruction::{InitializeAssetInput, Instruction}, invoke::{self}};

/// Program state handler.
pub struct Processor {}
impl Processor {
    #[allow(clippy::too_many_arguments)]
    fn prepare<'a>(
        _program_id: &Pubkey,
        program_token: &AccountInfo<'a>,
        swap: &AccountInfo<'a>,
        _swap_authority: &AccountInfo<'a>,
        signer: &AccountInfo<'a>,
        token_user_ab: &AccountInfo<'a>,
        token_user_bc: &AccountInfo<'a>,
        token_swap_ab: &AccountInfo<'a>,
        token_swap_bc: &AccountInfo<'a>,
        swap_ab: &AccountInfo<'a>,
        token_swap_ab_total: &AccountInfo<'a>,
        token_swap_ab_b_total: &AccountInfo<'a>,
        swap_bc: &AccountInfo<'a>,
        token_swap_bc_total: &AccountInfo<'a>,
        token_swap_bc_b_total: &AccountInfo<'a>,
        input: &InitializeAssetInput,
    ) -> ProgramResult {
        todo!()
        // if !signer.is_signer || !swap.is_signer {
        //     return Err(PoolError::TokenProviderAndSwapMustBeSigners.into());
        // }

        // let token_swap_ab_data = swap_ab.try_borrow_data()?;
        // let token_swap_ab_data = SwapVersion::unpack(&token_swap_ab_data[..])?;
        // let token_swap_bc_data = swap_bc.try_borrow_data()?;
        // let token_swap_bc_data = SwapVersion::unpack(&token_swap_bc_data[..])?;

        // let token_swap_ab_total =
        //     Account::unpack_from_slice(&token_swap_ab_total.try_borrow_data().unwrap()[..])?;

        // let token_swap_bc_total =
        //     Account::unpack_from_slice(&token_swap_bc_total.try_borrow_data().unwrap()[..])?;

        // let token_swap_ab_b_total =
        //     Account::unpack_from_slice(&token_swap_ab_b_total.try_borrow_data().unwrap()[..])?;

        // let token_swap_bc_b_total =
        //     Account::unpack_from_slice(&token_swap_bc_b_total.try_borrow_data().unwrap()[..])?;

        // if &token_swap_ab_total.mint != token_swap_ab_data.pool_mint()
        //     || &token_swap_bc_total.mint != token_swap_bc_data.pool_mint()
        // {
        //     return Err(PoolError::OperationMustBeOnTwoPoolTokens.into());
        // }

        // if let Some(liquidity_pool_bc_amount) = crate::math::prepare_amounts(
        //     token_swap_ab_b_total.amount,
        //     input.liquidity_pool_ab_exact,
        //     token_swap_ab_total.amount,
        //     token_swap_bc_total.amount,
        //     token_swap_bc_b_total.amount,
        // )
        // .map(|x| x.to_u64())
        // .flatten()
        // {
        //     if liquidity_pool_bc_amount > input.liquidity_pool_bc_max {
        //         return Err(PoolError::TooSmallAmountOfBcLiquidity.into());
        //     }
        //     invoke::token_transfer(
        //         &program_token,
        //         &token_user_ab,
        //         &token_swap_ab,
        //         &signer,
        //         input.liquidity_pool_ab_exact,
        //     )?;

        //     invoke::token_transfer(
        //         &program_token,
        //         &token_user_bc,
        //         &token_swap_bc,
        //         &signer,
        //         liquidity_pool_bc_amount,
        //     )?;

        //     Ok(())
        // } else {
        //     Err(PoolError::TooUnbalancedInput.into())
        // }
    }


    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = Instruction::try_from_slice(&input[0..1])?;
        match instruction {
            Instruction::InitializeAsset => {
                msg!("Instruction: InitializeAsset");
                match accounts {
                    [program_token, swap, swap_authority, signer, token_user_ab, token_user_bc, token_swap_ab, token_swap_bc, swap_ab, token_swap_ab_total, token_swap_ab_b_total, swap_bc, token_swap_bc_total, token_swap_bc_b_total, ..] =>
                    {
                        let input = super::instruction::InitializeAssetInput::deserialize_const(&input[1..])?;
                        Self::prepare(
                            program_id,
                            program_token,
                            swap,
                            swap_authority,
                            signer,
                            token_user_ab,
                            token_user_bc,
                            token_swap_ab,
                            token_swap_bc,
                            swap_ab,
                            token_swap_ab_total,
                            token_swap_ab_b_total,
                            swap_bc,
                            token_swap_bc_total,
                            token_swap_bc_b_total,
                            &input,
                        )
                    }
                    _ => Err(ProgramError::NotEnoughAccountKeys),
                }
            }
            _ => todo!()
        }
    }
}

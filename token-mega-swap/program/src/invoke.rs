// use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke};

// /// Transfer tokens with authority of transaction signer
// pub fn token_transfer<'a>(
//     token_program: &AccountInfo<'a>,
//     source_account: &AccountInfo<'a>,
//     destination_account: &AccountInfo<'a>,
//     signer_account: &AccountInfo<'a>,
//     amount: u64,
// ) -> ProgramResult {
//     invoke(
//         &spl_token::instruction::transfer(
//             token_program.key,
//             source_account.key,
//             destination_account.key,
//             signer_account.key,
//             &[signer_account.key],
//             amount,
//         )
//         .unwrap(),
//         &[
//             token_program.clone().to_owned(),
//             signer_account.clone().to_owned(),
//             source_account.clone().to_owned(),
//             destination_account.clone().to_owned(),
//         ],
//     )
// }

// /// Transfer tokens with authority of transaction signer
// #[allow(clippy::too_many_arguments)]
// pub fn deposit_single_token_type_exact_amount_in<'a>(
//     program_token_swap: &AccountInfo<'a>,
//     program_token: &AccountInfo<'a>,
//     swap_account: &AccountInfo<'a>,
//     swap_authority: &AccountInfo<'a>,
//     user_transfer_authority: &AccountInfo<'a>,
//     source_token: &AccountInfo<'a>,
//     swap_token_a: &AccountInfo<'a>,
//     swap_token_b: &AccountInfo<'a>,
//     pool_mint: &AccountInfo<'a>,
//     destination: &AccountInfo<'a>,
//     instruction: spl_token_swap::instruction::DepositSingleTokenTypeExactAmountIn,
// ) -> ProgramResult {
//     let instruction = spl_token_swap::instruction::deposit_single_token_type_exact_amount_in(
//         program_token_swap.key,
//         program_token.key,
//         swap_account.key,
//         swap_authority.key,
//         user_transfer_authority.key,
//         source_token.key,
//         swap_token_a.key,
//         swap_token_b.key,
//         pool_mint.key,
//         destination.key,
//         instruction,
//     )
//     .unwrap();
//     invoke(
//         &instruction,
//         &[
//             program_token_swap.clone().to_owned(),
//             program_token.clone().to_owned(),
//             swap_account.clone().to_owned(),
//             swap_authority.clone().to_owned(),
//             user_transfer_authority.clone().to_owned(),
//             source_token.clone().to_owned(),
//             swap_token_a.clone().to_owned(),
//             swap_token_b.clone().to_owned(),
//             pool_mint.clone().to_owned(),
//             destination.clone().to_owned(),
//         ],
//     )
// }

// #[allow(clippy::too_many_arguments)]
// pub fn withdraw_single_token_type_exact_amount_out<'a>(
//     program_token: &AccountInfo<'a>,
//     program_token_swap: &AccountInfo<'a>,
//     token_swap: &AccountInfo<'a>,
//     authority: &AccountInfo<'a>,
//     user_transfer_authority: &AccountInfo<'a>,
//     pool_mint: &AccountInfo<'a>,
//     fee_account: &AccountInfo<'a>,
//     pool_token_source: &AccountInfo<'a>,
//     swap_token_a: &AccountInfo<'a>,
//     swap_token_b: &AccountInfo<'a>,
//     destination: &AccountInfo<'a>,
//     instruction: spl_token_swap::instruction::WithdrawSingleTokenTypeExactAmountOut,
// ) -> ProgramResult {
//     let instruction = spl_token_swap::instruction::withdraw_single_token_type_exact_amount_out(
//         program_token_swap.key,
//         program_token.key,
//         token_swap.key,
//         authority.key,
//         user_transfer_authority.key,
//         pool_mint.key,
//         fee_account.key,
//         pool_token_source.key,
//         swap_token_a.key,
//         swap_token_b.key,
//         destination.key,
//         instruction,
//     )
//     .unwrap();
//     invoke(
//         &instruction,
//         &[
//             program_token.clone(),
//             program_token_swap.clone(),
//             token_swap.clone(),
//             authority.clone(),
//             user_transfer_authority.clone(),
//             pool_mint.clone(),
//             fee_account.clone(),
//             pool_token_source.clone(),
//             swap_token_a.clone(),
//             swap_token_b.clone(),
//             destination.clone(),
//         ],
//     )
// }

// /// swap tokens with authority of transaction
// #[allow(clippy::too_many_arguments)]
// pub fn swap<'a>(
//     swap_program: &AccountInfo<'a>,
//     token_program_id: &AccountInfo<'a>,
//     swap_pubkey: &AccountInfo<'a>,
//     authority_pubkey: &AccountInfo<'a>,
//     user_transfer_authority_pubkey: &AccountInfo<'a>,
//     source_pubkey: &AccountInfo<'a>,
//     swap_source_pubkey: &AccountInfo<'a>,
//     swap_destination_pubkey: &AccountInfo<'a>,
//     destination_pubkey: &AccountInfo<'a>,
//     pool_mint_pubkey: &AccountInfo<'a>,
//     pool_fee_pubkey: &AccountInfo<'a>,
//     instruction: spl_token_swap::instruction::Swap,
// ) -> ProgramResult {
//     let instruction = spl_token_swap::instruction::swap(
//         swap_program.key,
//         token_program_id.key,
//         swap_pubkey.key,
//         authority_pubkey.key,
//         user_transfer_authority_pubkey.key,
//         source_pubkey.key,
//         swap_source_pubkey.key,
//         swap_destination_pubkey.key,
//         destination_pubkey.key,
//         pool_mint_pubkey.key,
//         pool_fee_pubkey.key,
//         None,
//         instruction,
//     )
//     .unwrap();
//     let accounts = &[
//         swap_program.clone(),
//         token_program_id.clone(),
//         swap_pubkey.clone(),
//         authority_pubkey.clone(),
//         user_transfer_authority_pubkey.clone(),
//         source_pubkey.clone(),
//         swap_source_pubkey.clone(),
//         swap_destination_pubkey.clone(),
//         destination_pubkey.clone(),
//         pool_mint_pubkey.clone(),
//         pool_fee_pubkey.clone(),
//     ];
//     // validator
//     let mut t = vec![];

//     for a in accounts {
//         t.push(a.key);
//     }

//     let len = t.len();
//     t.sort();
//     t.dedup();

//     if len != t.len() {
//         panic!("dup account!")
//     }

//     msg!("ASDSADSADSDSADQ!@#@!#@!");
//     invoke(&instruction, accounts)
// }

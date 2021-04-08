// #![cfg(feature = "test-bpf")]

// use solana_program::pubkey::Pubkey;
// use solana_program_test::*;
// use solana_sdk::signature::{Keypair, Signer};

// use spl_token_swap::instruction::Swap;

// use crate::{id, instruction::Prepare, processor, test::*};

// pub fn program_test() -> ProgramTest {
//     let mut program_test = ProgramTest::new(
//         "spl_token_mega_swap",
//         id(),
//         processor!(processor::Processor::process_instruction),
//     );

//     program_test.add_program("spl_token", spl_token::id(), None);
//     program_test.add_program("spl_token_swap", spl_token_swap::id(), None);

//     program_test
// }

// struct A;
// struct B;
// struct C;
// struct AB;
// struct BC;
// struct ABC;

// #[tokio::test]
// async fn flow() {
//     // arrange
//     let (mut banks_client, payer, recent_blockhash) = program_test().start().await;
//     let rent = banks_client.get_rent().await.unwrap();
//     let bank = BankInfo {
//         hash: recent_blockhash,
//         rent,
//     };

//     let mint_a = create_mint::<A>(&bank, &payer, &payer.pubkey());
//     let mint_a = mint_a.process_transaction(&mut banks_client).await;

//     let mint_b = create_mint::<B>(&bank, &payer, &payer.pubkey());
//     let mint_b = mint_b.process_transaction(&mut banks_client).await;

//     let mint_c = create_mint::<C>(&bank, &payer, &payer.pubkey());
//     let mint_c = mint_c.process_transaction(&mut banks_client).await;

//     let user_a = create_token_account(&bank, &payer, &mint_a, &payer.pubkey(), 1_000_000);
//     let user_a = user_a.process_transaction(&mut banks_client).await;

//     let user_b = create_token_account(&bank, &payer, &mint_b, &payer.pubkey(), 1_000_000);
//     let user_b = user_b.process_transaction(&mut banks_client).await;

//     let user_c = create_token_account(&bank, &payer, &mint_c, &payer.pubkey(), 0);
//     let user_c = user_c.process_transaction(&mut banks_client).await;

//     let swap_ab = ProgramAuthority::new(Keypair::new(), spl_token_swap::id());

//     let swap_ab_a = create_token_account(&bank, &payer, &mint_a, &swap_ab.authority, 1_000_000);
//     let swap_ab_a = swap_ab_a.process_transaction(&mut banks_client).await;

//     let swap_ab_b = create_token_account(&bank, &payer, &mint_b, &swap_ab.authority, 1_000_000);
//     let swap_ab_b = swap_ab_b.process_transaction(&mut banks_client).await;

//     let mint_ab = create_mint::<AB>(&bank, &payer, &swap_ab.authority);
//     let mint_ab = mint_ab.process_transaction(&mut banks_client).await;

//     let fee_ab = create_token_account(&bank, &payer, &mint_ab, &payer.pubkey(), 0);
//     let fee_ab = fee_ab.process_transaction(&mut banks_client).await;

//     let supply_ab = create_token_account(&bank, &payer, &mint_ab, &payer.pubkey(), 0);
//     let supply_ab = supply_ab.process_transaction(&mut banks_client).await;

//     let swap_ab = create_constant_product(
//         &bank, &payer, swap_ab_a, swap_ab_b, mint_ab, supply_ab, fee_ab, swap_ab,
//     );
//     let swap_ab = swap_ab.process_transaction(&mut banks_client).await;

//     let swap_bc = ProgramAuthority::new(Keypair::new(), spl_token_swap::id());

//     let swap_bc_b = create_token_account(&bank, &payer, &mint_b, &swap_bc.authority, 1_000_000);
//     let swap_bc_b = swap_bc_b.process_transaction(&mut banks_client).await;

//     let swap_bc_c = create_token_account(&bank, &payer, &mint_c, &swap_bc.authority, 1_000_000);
//     let swap_bc_c = swap_bc_c.process_transaction(&mut banks_client).await;

//     let mint_bc = create_mint::<BC>(&bank, &payer, &swap_bc.authority);
//     let mint_bc = mint_bc.process_transaction(&mut banks_client).await;

//     let fee_bc = create_token_account(&bank, &payer, &mint_bc, &payer.pubkey(), 0);
//     let fee_bc = fee_bc.process_transaction(&mut banks_client).await;

//     let supply_bc = create_token_account(&bank, &payer, &mint_bc, &payer.pubkey(), 0);
//     let supply_bc = supply_bc.process_transaction(&mut banks_client).await;

//     let swap_bc = create_constant_product(
//         &bank, &payer, swap_bc_b, swap_bc_c, mint_bc, supply_bc, fee_bc, swap_bc,
//     );
//     let swap_bc = swap_bc.process_transaction(&mut banks_client).await;

//     let account_ab = create_token_account(&bank, &payer, &swap_ab.mint, &payer.pubkey(), 0);
//     let account_ab = account_ab.process_transaction(&mut banks_client).await;

//     let account_bc = create_token_account(&bank, &payer, &swap_bc.mint, &payer.pubkey(), 0);
//     let account_bc = account_bc.process_transaction(&mut banks_client).await;

//     let a_ab = create_deposit_transaction(
//         &bank,
//         &payer,
//         &swap_ab,
//         &user_a,
//         &account_ab,
//         Amount::new(500_000),
//     );
//     banks_client.process_transaction(a_ab).await.unwrap();

//     let c_bc = create_deposit_transaction(
//         &bank,
//         &payer,
//         &swap_bc,
//         &user_b,
//         &account_bc,
//         Amount::new(500_000),
//     );
//     banks_client.process_transaction(c_bc).await.unwrap();

//     let swap_abc = ProgramAuthority::new(Keypair::new(), spl_token_swap::id());
//     let mint_abc = create_mint::<ABC>(&bank, &payer, &swap_abc.authority);
//     let mint_abc = mint_abc.process_transaction(&mut banks_client).await;

//     let fee_abc = create_token_account(&bank, &payer, &mint_abc, &payer.pubkey(), 0);
//     let fee_abc = fee_abc.process_transaction(&mut banks_client).await;

//     let supply_abc = create_token_account(&bank, &payer, &mint_abc, &payer.pubkey(), 0);
//     let supply_abc = supply_abc.process_transaction(&mut banks_client).await;

//     let swap_account_ab =
//         create_token_account(&bank, &payer, &swap_ab.mint, &swap_abc.authority, 0);
//     let swap_account_ab = swap_account_ab.process_transaction(&mut banks_client).await;

//     let swap_account_bc =
//         create_token_account(&bank, &payer, &swap_bc.mint, &swap_abc.authority, 0);
//     let swap_account_bc = swap_account_bc.process_transaction(&mut banks_client).await;

//     // prepare
//     let swap_abc = create_megaswap(
//         &bank,
//         &payer,
//         &account_ab,
//         &account_bc,
//         swap_account_ab,
//         swap_account_bc,
//         &swap_ab,
//         &swap_bc,
//         mint_abc,
//         supply_abc,
//         fee_abc,
//         swap_abc,
//         Prepare {
//             liquidity_pool_ab_exact: 100_000,
//             liquidity_pool_bc_max: 200_000,
//         },
//     );

//     let swap_abc = swap_abc.process_transaction(&mut banks_client).await;

//     // mega swap
//     let mega_swap = do_mega_swap(
//         &bank,
//         &payer,
//         &swap_ab,
//         &swap_abc,
//         &swap_bc,
//         &user_a,
//         &account_ab,
//         &account_bc,
//         &user_c,
//         Swap {
//             amount_in: 1_000,
//             minimum_amount_out: 100,
//         },
//     );

//     banks_client.process_transaction(mega_swap).await.unwrap();
//     let balance = get_token_balance(&mut banks_client, &user_c.pubkey()).await;
//     assert!(balance > 100);
// }

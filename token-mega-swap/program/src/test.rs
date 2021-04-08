// //! Helper function for testing.
// #![cfg(feature = "test-bpf")]

// use std::marker::PhantomData;

// use solana_program::{program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction};
// use solana_program_test::*;
// use solana_sdk::{
//     signature::{Keypair, Signer},
//     transaction::Transaction,
// };

// /// derived alternative of Keypair not on elliptic curve
// #[derive(Debug)]
// pub struct ProgramAuthority {
//     pub account: Keypair,
//     pub authority: Pubkey,
//     pub bump_seed: u8,
// }

// impl ProgramAuthority {
//     pub fn new(account: Keypair, program_id: Pubkey) -> Self {
//         let (authority, bump_seed) =
//             Pubkey::find_program_address(&[&account.pubkey().to_bytes()[..32]], &program_id);
//         Self {
//             account,
//             authority,
//             bump_seed,
//         }
//     }

//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// pub struct Mint<T> {
//     pub account: Keypair,
//     pub phantom_data: PhantomData<T>,
// }

// pub struct Token<T> {
//     pub account: Keypair,
//     pub phantom_data: PhantomData<T>,
// }

// impl<T> Token<T> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// impl<T> Mint<T> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.account.pubkey()
//     }
// }

// pub struct BankInfo {
//     pub rent: Rent,
//     pub hash: solana_program::hash::Hash,
// }

// pub struct CreateMintTransaction<T> {
//     transaction: Transaction,
//     mint: Mint<T>,
// }

// impl<T> CreateMintTransaction<T> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> Mint<T> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.mint
//     }
// }

// pub struct CreateTokenTransaction<T> {
//     transaction: Transaction,
//     token: Token<T>,
// }

// impl<T> CreateTokenTransaction<T> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> Token<T> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.token
//     }
// }

// pub fn create_mint<M: Send + Sync>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     owner: &Pubkey,
// ) -> CreateMintTransaction<M> {
//     let account = Keypair::new();
//     let rent = bank.rent.minimum_balance(spl_token::state::Mint::LEN);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &account.pubkey(),
//                 rent,
//                 spl_token::state::Mint::LEN as u64,
//                 &spl_token::id(),
//             ),
//             spl_token::instruction::initialize_mint(
//                 &spl_token::id(),
//                 &account.pubkey(),
//                 &owner,
//                 None,
//                 0,
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, &account], bank.hash);
//     CreateMintTransaction {
//         transaction,
//         mint: Mint {
//             account,
//             phantom_data: PhantomData::default(),
//         },
//     }
// }

// pub fn create_token_account<T>(
//     bank: &BankInfo,
//     payer: &Keypair,
//     mint: &Mint<T>,
//     owner: &Pubkey,
//     amount: u64,
// ) -> CreateTokenTransaction<T> {
//     let account = Keypair::new();
//     let rent = bank.rent.minimum_balance(spl_token::state::Account::LEN);
//     let mut instructions = vec![
//         system_instruction::create_account(
//             &payer.pubkey(),
//             &account.pubkey(),
//             rent,
//             spl_token::state::Account::LEN as u64,
//             &spl_token::id(),
//         ),
//         spl_token::instruction::initialize_account(
//             &spl_token::id(),
//             &account.pubkey(),
//             &mint.pubkey(),
//             &owner,
//         )
//         .unwrap(),
//     ];

//     if amount > 0 {
//         instructions.push(
//             spl_token::instruction::mint_to(
//                 &spl_token::id(),
//                 &mint.pubkey(),
//                 &account.pubkey(),
//                 &payer.pubkey(),
//                 &[&payer.pubkey()],
//                 amount,
//             )
//             .unwrap(),
//         )
//     }

//     let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
//     transaction.sign(&[payer, &account], bank.hash);
//     CreateTokenTransaction {
//         transaction,
//         token: Token {
//             account,
//             phantom_data: PhantomData::default(),
//         },
//     }
// }

// pub struct CreateSwapTransaction<A, B, AB> {
//     transaction: Transaction,
//     swap: TokenSwap<A, B, AB>,
// }

// pub struct TokenSwap<A, B, AB> {
//     pub swap: ProgramAuthority,
//     token_a: Token<A>,
//     token_b: Token<B>,
//     pub mint: Mint<AB>,
//     supply: Token<AB>,
//     fee: Token<AB>,
// }

// impl<A, B, AB> TokenSwap<A, B, AB> {
//     pub fn pubkey(&self) -> Pubkey {
//         self.swap.pubkey()
//     }
// }

// pub struct Amount<T> {
//     pub amount: u64,
//     pub phantom_data: PhantomData<T>,
// }

// impl<T> Amount<T> {
//     pub fn new(amount: u64) -> Self {
//         Self {
//             amount,
//             phantom_data: Default::default(),
//         }
//     }
// }

// impl<A, B, AB> CreateSwapTransaction<A, B, AB> {
//     pub async fn process_transaction(self, banks_client: &mut BanksClient) -> TokenSwap<A, B, AB> {
//         banks_client
//             .process_transaction(self.transaction)
//             .await
//             .unwrap();
//         self.swap
//     }
// }

// pub fn create_deposit_transaction<A, B, AB>(
//     banks: &BankInfo,
//     payer: &Keypair,
//     swap: &TokenSwap<A, B, AB>,
//     token_a: &Token<A>,
//     token_ab: &Token<AB>,
//     amount: Amount<A>,
// ) -> Transaction {
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             spl_token_swap::instruction::deposit_single_token_type_exact_amount_in(
//                 &spl_token_swap::id(),
//                 &spl_token::id(),
//                 &swap.swap.account.pubkey(),
//                 &swap.swap.authority,
//                 &payer.pubkey(),
//                 &token_a.pubkey(),
//                 &swap.token_a.pubkey(),
//                 &swap.token_b.pubkey(),
//                 &swap.mint.pubkey(),
//                 &token_ab.pubkey(),
//                 spl_token_swap::instruction::DepositSingleTokenTypeExactAmountIn {
//                     source_token_amount: amount.amount,
//                     minimum_pool_token_amount: 1,
//                 },
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer], banks.hash);
//     transaction
// }

// #[allow(clippy::too_many_arguments)]
// pub fn create_constant_product<A, B, AB>(
//     banks: &BankInfo,
//     payer: &Keypair,
//     token_a: Token<A>,
//     token_b: Token<B>,
//     mint_ab: Mint<AB>,
//     supply: Token<AB>,
//     fee: Token<AB>,
//     authority: ProgramAuthority,
// ) -> CreateSwapTransaction<A, B, AB> {
//     let rent = banks
//         .rent
//         .minimum_balance(spl_token_swap::state::SwapVersion::LATEST_LEN);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &authority.account.pubkey(),
//                 rent,
//                 spl_token_swap::state::SwapVersion::LATEST_LEN as u64,
//                 &spl_token_swap::id(),
//             ),
//             spl_token_swap::instruction::initialize(
//                 &spl_token_swap::id(),
//                 &spl_token::id(),
//                 &authority.account.pubkey(),
//                 &authority.authority,
//                 &token_a.pubkey(),
//                 &token_b.pubkey(),
//                 &mint_ab.pubkey(),
//                 &fee.pubkey(),
//                 &supply.pubkey(),
//                 authority.bump_seed,
//                 spl_token_swap::curve::fees::Fees::default(),
//                 spl_token_swap::curve::base::SwapCurve::default(),
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, &authority.account], banks.hash);
//     CreateSwapTransaction {
//         transaction,
//         swap: TokenSwap {
//             token_a,
//             token_b,
//             mint: mint_ab,
//             supply,
//             fee,
//             swap: authority,
//         },
//     }
// }

// #[allow(clippy::too_many_arguments)]
// pub fn do_mega_swap<A, B, C, AB, BC, ABC>(
//     banks: &BankInfo,
//     payer: &Keypair,
//     swap_ab: &TokenSwap<A, B, AB>,
//     swap_abc: &TokenSwap<AB, BC, ABC>,
//     swap_bc: &TokenSwap<B, C, BC>,
//     token_user_a: &Token<A>,
//     token_user_ab_temp: &Token<AB>,
//     token_user_bc_temp: &Token<BC>,
//     token_user_c: &Token<C>,
//     input: spl_token_swap::instruction::Swap,
// ) -> Transaction {
//     let mut transaction = Transaction::new_with_payer(
//         &[crate::instruction::swap(
//             &swap_ab.swap.authority,
//             &swap_ab.mint.pubkey(),
//             &swap_ab.pubkey(),
//             &swap_ab.token_a.pubkey(),
//             &swap_ab.token_b.pubkey(),
//             &swap_bc.swap.authority,
//             &swap_bc.mint.pubkey(),
//             &swap_bc.swap.pubkey(),
//             &swap_bc.token_a.pubkey(),
//             &swap_bc.token_b.pubkey(),
//             &swap_bc.supply.pubkey(),
//             &swap_bc.fee.pubkey(),
//             &swap_abc.swap.authority,
//             &swap_abc.mint.pubkey(),
//             &swap_abc.pubkey(),
//             &swap_abc.token_a.pubkey(),
//             &swap_abc.token_b.pubkey(),
//             &swap_abc.fee.pubkey(),
//             &token_user_a.pubkey(),
//             &token_user_ab_temp.pubkey(),
//             &token_user_bc_temp.pubkey(),
//             &token_user_c.pubkey(),
//             &payer.pubkey(),
//             input,
//         )
//         .unwrap()],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer], banks.hash);
//     transaction
// }

// #[allow(clippy::too_many_arguments)]
// pub fn create_megaswap<A, B, C, AB, BC, ABC>(
//     banks: &BankInfo,
//     payer: &Keypair,

//     token_user_ab: &Token<AB>,
//     token_user_bc: &Token<BC>,

//     token_swap_ab: Token<AB>,
//     token_swap_bc: Token<BC>,

//     swap_ab: &TokenSwap<A, B, AB>,
//     swap_bc: &TokenSwap<B, C, BC>,

//     mint_abc: Mint<ABC>,
//     supply: Token<ABC>,
//     fee: Token<ABC>,
//     authority: ProgramAuthority,
//     input: crate::instruction::Prepare,
// ) -> CreateSwapTransaction<AB, BC, ABC> {
//     let rent = banks
//         .rent
//         .minimum_balance(spl_token_swap::state::SwapVersion::LATEST_LEN);
//     let mut transaction = Transaction::new_with_payer(
//         &[
//             system_instruction::create_account(
//                 &payer.pubkey(),
//                 &authority.account.pubkey(),
//                 rent,
//                 spl_token_swap::state::SwapVersion::LATEST_LEN as u64,
//                 &spl_token_swap::id(),
//             ),
//             crate::instruction::prepare(
//                 &authority.account.pubkey(),
//                 &authority.authority,
//                 &payer.pubkey(),
//                 &token_user_ab.pubkey(),
//                 &token_user_bc.pubkey(),
//                 &token_swap_ab.pubkey(),
//                 &token_swap_bc.pubkey(),
//                 &swap_ab.swap.pubkey(),
//                 &swap_ab.supply.pubkey(),
//                 &swap_ab.token_b.pubkey(),
//                 &swap_bc.swap.pubkey(),
//                 &swap_bc.supply.pubkey(),
//                 &swap_bc.token_a.pubkey(),
//                 input,
//             )
//             .unwrap(),
//             spl_token_swap::instruction::initialize(
//                 &spl_token_swap::id(),
//                 &spl_token::id(),
//                 &authority.account.pubkey(),
//                 &authority.authority,
//                 &token_swap_ab.pubkey(),
//                 &token_swap_bc.pubkey(),
//                 &mint_abc.pubkey(),
//                 &fee.pubkey(),
//                 &supply.pubkey(),
//                 authority.bump_seed,
//                 spl_token_swap::curve::fees::Fees::default(),
//                 spl_token_swap::curve::base::SwapCurve::default(),
//             )
//             .unwrap(),
//         ],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[payer, &authority.account], banks.hash);
//     CreateSwapTransaction {
//         transaction,
//         swap: TokenSwap {
//             token_a: token_swap_ab,
//             token_b: token_swap_bc,
//             mint: mint_abc,
//             supply,
//             fee,
//             swap: authority,
//         },
//     }
// }

// pub async fn get_token_balance(banks_client: &mut BanksClient, token: &Pubkey) -> u64 {
//     let token_account = banks_client.get_account(*token).await.unwrap().unwrap();
//     let account_info: spl_token::state::Account =
//         spl_token::state::Account::unpack_from_slice(token_account.data.as_slice()).unwrap();
//     account_info.amount
// }

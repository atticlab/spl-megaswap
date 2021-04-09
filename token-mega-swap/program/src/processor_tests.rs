// #![cfg(feature = "test-bpf")]

use solana_program::pubkey::Pubkey;
use solana_program_test::*;
use solana_sdk::signature::{Keypair, Signer};


use crate::{id, instruction::*, processor, test::{Amount, BankInfo, ProgramAuthority, create_asset, create_mint, create_token_account}};

pub fn program_test() -> ProgramTest {
    let mut program_test = ProgramTest::new(
        "spl_token_mega_swap",
        id(),
        processor!(processor::Processor::process_instruction),
    );
    program_test
}

struct A;
struct B;

#[tokio::test]
async fn flow() {
    // arrange
    let (mut blockchain, payer, recent_blockhash) = program_test().start().await;
    let rent = blockchain.get_rent().await.unwrap();
    let bank = BankInfo {
        hash: recent_blockhash,
        rent,
    };

    
    let mint = create_mint::<A>(&bank, &payer, &payer.pubkey());
    let mint = mint.process_transaction(&mut blockchain).await;
    
    let pool = ProgramAuthority::new(Keypair::new(), crate::id());        
    let mut assets = Vec::new();
    for _ in 0..2u8 {
        let asset = ProgramAuthority::new(Keypair::new(), crate::id());        
        let token = create_token_account(&bank, &payer, &mint, &asset.authority, 1_000_000);
        let token = token.process_transaction(&mut blockchain).await;
        let asset = create_asset(&bank, &payer, &pool.pubkey(), asset, &token, InitializeAssetInput { weight : 1});
        let asset = asset.process_transaction(&mut blockchain).await;
        assets.push(asset);
    }

    
        
}

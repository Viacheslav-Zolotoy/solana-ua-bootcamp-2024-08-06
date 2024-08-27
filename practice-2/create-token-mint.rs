
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    native_token::LAMPORTS_PER_SOL,
};
use spl_token::{instruction::initialize_mint, id, state::Mint};
use solana_transaction_status::UiTransactionEncoding;
use std::env;
use std::str::FromStr;

fn main() {

    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");

    
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid keypair bytes");

    
    let connection = RpcClient::new("https://api.devnet.solana.com");

    
    println!("🔑 Our public key is: {}", sender.pubkey());

    
    let mint_keypair = Keypair::new();
    let rent_exemption = connection
        .get_minimum_balance_for_rent_exemption(Mint::LEN)
        .unwrap();

    let create_account_instruction = solana_sdk::system_instruction::create_account(
        &sender.pubkey(),
        &mint_keypair.pubkey(),
        rent_exemption,
        Mint::LEN as u64,
        &id(),
    );

    let initialize_mint_instruction = initialize_mint(
        &id(),
        &mint_keypair.pubkey(),
        &sender.pubkey(),
        None,
        2,
    )
    .unwrap();

    
    let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
        &[create_account_instruction, initialize_mint_instruction],
        Some(&sender.pubkey()),
    );

    
    let recent_blockhash = connection.get_latest_blockhash().unwrap();
    transaction.sign(&[&sender, &mint_keypair], recent_blockhash);

    
    let signature = connection
        .send_and_confirm_transaction(&transaction)
        .expect("Transaction failed");

    println!("✅ Transaction confirmed, signature: {}!", signature);

    
    let explorer_link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint_keypair.pubkey()
    );
    println!("✅ Token Mint: {}", explorer_link);
}

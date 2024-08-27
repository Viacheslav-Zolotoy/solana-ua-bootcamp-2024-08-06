use std::env;
use solana_sdk::{signer::Signer, transaction::Transaction, signature::Keypair, pubkey::Pubkey, system_program};
use solana_client::rpc_client::RpcClient;
use spl_associated_token_account::{create_associated_token_account, get_associated_token_address};
use spl_token::instruction::TokenInstruction;

fn main() {
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let sender = Keypair::from_base58_string(&private_key);

    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint_account = Pubkey::from_str("8DDUMgYdoAdeZgNr8igRrE6tMyd4XkUyMvzQGoGtcGP2").unwrap();
    let recipient = Pubkey::from_str("BhTmJoMAZgxfd3Cm9n6QY3V7M7a7dHaNRWo7mZ5mKoLe").unwrap();

    let token_account = get_associated_token_address(&recipient, &token_mint_account);

    let tx = Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &sender.pubkey(),
            &recipient,
            &token_mint_account,
        )],
        Some(&sender.pubkey()),
        &[&sender],
        connection.get_latest_blockhash().unwrap(),
    );

    connection.send_and_confirm_transaction(&tx).unwrap();

    println!("Token Account: {}", token_account);

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        token_account
    );

    println!("✅ Created token account: {}", link);
}
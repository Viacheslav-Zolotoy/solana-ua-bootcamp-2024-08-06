use std::env;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use spl_token::instruction::mint_to;
use solana_program::program_pack::Pack;

fn main() {
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let sender = Keypair::from_base58_string(&private_key);

    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());

    let minor_units_per_major_units = 25_u64.pow(2);

    let token_mint_account = Pubkey::from_str("8DDUMgYdoAdeZgNr8igRrE6tMyd4XkUyMvzQGoGtcGP2").unwrap();
    let recipient_associated_token_account = Pubkey::from_str("GvRxdoYB9BUZifXMd1Ws3qmm62pExvYNrNKs8owpvtCT").unwrap();

    let mint_to_ix = mint_to(
        &spl_token::id(),
        &token_mint_account,
        &recipient_associated_token_account,
        &sender.pubkey(),
        &[],
        10 * minor_units_per_major_units,
    ).unwrap();

    let mut transaction = Transaction::new_with_payer(
        &[mint_to_ix],
        Some(&sender.pubkey()),
    );

    let recent_blockhash = connection.get_latest_blockhash().unwrap();
    transaction.sign(&[&sender], recent_blockhash);

    let transaction_signature = connection.send_and_confirm_transaction(&transaction).unwrap();

    println!("✅ Success! Mint Token Transaction: {}", transaction_signature);
}
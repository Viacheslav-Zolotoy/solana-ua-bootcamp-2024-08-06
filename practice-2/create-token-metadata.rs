use std::env;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};
use solana_client::rpc_client::RpcClient;
use solana_program::borsh::BorshSerialize;
use mpl_token_metadata::instruction::create_metadata_accounts_v3;
use mpl_token_metadata::state::{Creator, DataV2};

fn main() {
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");
    let sender = Keypair::from_base58_string(&private_key);

    let connection = RpcClient::new("https://api.devnet.solana.com".to_string());

    let token_metadata_program_id = Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap();

    let token_mint_account = Pubkey::from_str("6FtPKgVijL78LpkVuDUnMNQGhQxJquNSjgUgckRfq7Yw").unwrap();

    let metadata_data = DataV2 {
        name: "Solana UA Bootcamp 2024-08-06".to_string(),
        symbol: "UAB-2".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let metadata_pda = Pubkey::find_program_address(
        &[
            b"metadata",
            &token_metadata_program_id.to_bytes(),
            &token_mint_account.to_bytes(),
        ],
        &token_metadata_program_id,
    ).0;

    let mut transaction = Transaction::new_with_payer(
        &[create_metadata_accounts_v3(
            mpl_token_metadata::id(),
            metadata_pda,
            token_mint_account,
            sender.pubkey(),
            sender.pubkey(),
            sender.pubkey(),
            metadata_data.name.clone(),
            metadata_data.symbol.clone(),
            metadata_data.uri.clone(),
            metadata_data.creators.clone(),
            metadata_data.seller_fee_basis_points,
            false, // update authority is signer
            true,  // is mutable
            metadata_data.collection.clone(),
            metadata_data.uses.clone(),
            metadata_data.collection.clone(),
            metadata_data.uses.clone(),
        )],
        Some(&sender.pubkey()),
    );

    transaction.sign(&[&sender], connection.get_latest_blockhash().unwrap());

    connection.send_and_confirm_transaction(&transaction).unwrap();

    println!("✅ Metadata created for token mint account!");
}
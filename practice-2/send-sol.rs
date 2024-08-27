use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signature, Signer},
    system_instruction,
    transaction::Transaction,
    pubkey::Pubkey,
    system_program,
    message::Message,
    instruction::Instruction,
};
use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    
    let private_key = env::var("SECRET_KEY").expect("Add SECRET_KEY to .env!");

    
    let as_array: Vec<u8> = serde_json::from_str(&private_key).expect("Invalid SECRET_KEY format");
    let sender = Keypair::from_bytes(&as_array).expect("Invalid keypair bytes");

    
    let connection = RpcClient::new("https://api.devnet.solana.com");

    
    println!("🔑 Our public key is: {}", sender.pubkey());

    
    let recipient = Pubkey::from_str("4JTLH4HXF1nXNAfJvnGT9HUBmzCzsCfRvpbBjKFSQHdL").expect("Invalid recipient address");
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    
    let send_sol_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recipient,
        (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64,
    );

    
    let mut transaction = Transaction::new_with_payer(
        &[send_sol_instruction],
        Some(&sender.pubkey()),
    );

    
    transaction.sign(&[&sender], connection.get_latest_blockhash().unwrap());

    
    let signature = connection.send_and_confirm_transaction(&transaction).expect("Transaction failed");
    println!("✅ Transaction confirmed, signature: {}!", signature);

    
    let memo_program = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr").expect("Invalid Memo Program address");
    
    let memo_text = "Hello Kumeka team!";
    
    
    let add_memo_instruction = Instruction {
        program_id: memo_program,
        accounts: vec![solana_sdk::instruction::AccountMeta::new(sender.pubkey(), true)],
        data: memo_text.as_bytes().to_vec(),
    };
    
    
    transaction.add_instruction(add_memo_instruction);
    println!("📝 memo is: {}", memo_text);
}
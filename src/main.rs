mod db;
mod wallet;
mod crypto;
mod transaction;
mod tor;
mod cli;

use anyhow::Result;
use bitcoin::Network;

#[tokio::main]
async fn main() -> Result<()> {
    // Setup
    // let network = Network::Regtest;
    // let db_client = db::connect_db().await?;
    // db::create_tables(&db_client).await?;
    
    // // Generate wallet
    // let wallet = wallet::Wallet::new(network);
    // let (secret_key, tweaked_pubkey) = wallet.generate_taproot_keypair();
    // let address = wallet.get_taproot_address(tweaked_pubkey);
    
    // println!("Generated Taproot address: {}", address);
    
    // Encrypt and store private key
    // let encryption_key = [0u8; 32]; // In real app, derive from user password
    // let encrypted_key = crypto::encrypt_key(&encryption_key, &secret_key.secret_bytes())?;
    

    // db_client.execute(
    //     "INSERT INTO addresses (private_key, public_key, address) VALUES ($1, $2, $3)",
    //     &[&hex::encode(&encrypted_key),  // Convert binary to hex string
    //     &tweaked_pubkey.to_string(), 
    //     &address.to_string()]
    // ).await?;
    

    // // Create and sign transaction (simplified example)
    // let tx_builder = transaction::TransactionBuilder::new(network);
    // let tx = tx_builder.create_transaction(
    //     vec![], // Would populate with UTXOs
    //     vec![], // Would add outputs
    //     &address,
    //     bitcoin::Amount::from_sat(1000),
    // );
    
    // // Broadcast via Tor
    // let txid = tor::broadcast_via_tor(
    //     "127.0.0.1:9050",
    //     "yourbitcoinnode.onion:8332",
    //     &hex::encode(&bitcoin::consensus::serialize(&tx)),
    // ).await?;
    
    // println!("Transaction broadcasted: {}", txid);
    
    // Ok(())
    cli::interactive_cli().await
}
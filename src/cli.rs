use clap::{Parser, Subcommand, ValueEnum, CommandFactory};
use std::io::{self, Write};
use crate::{wallet, db, crypto};
use bitcoin::Network;
use anyhow::Result;

#[derive(Parser)]
#[command(
    version,
    about = "Bitcoin Self-Custody Wallet CLI",
    long_about = r#"
A secure Bitcoin wallet with Taproot support

Examples:
  Generate a regtest wallet: wallet-cli generate --network regtest
  Send 0.5 BTC:            wallet-cli send --to bcrt1q... --amount 0.5
  Check balance:           wallet-cli balance
"#
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum NetworkType {
    Regtest,
    Testnet,
    Mainnet,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate a new Taproot wallet
    #[command(arg_required_else_help = true)]
    Generate {
        /// Network to use (regtest, testnet, or mainnet)
        #[arg(value_enum, short, long, default_value = "regtest")]
        network: NetworkType,
    },
    
    /// Send bitcoin to an address
    #[command(arg_required_else_help = true)]
    Send {
        /// Recipient Bitcoin address
        #[arg(short, long)]
        to: String,
        
        /// Amount to send in BTC
        #[arg(short, long)]
        amount: f64,
        
        /// Optional: Fee rate in sat/vB
        #[arg(short, long, default_value = "1.0")]
        fee_rate: f64,
    },
    
    /// Show wallet balance and UTXOs
    Balance,
    
    /// List all addresses in wallet
    Addresses,
}

pub async fn interactive_cli() -> Result<()> {
    let db_client = db::connect_db().await?;
    let mut rl = rustyline::Editor::<()>::new()?;

    println!("Bitcoin Self-Custody Wallet (type 'help' or 'exit')");
    
    loop {
        let input = rl.readline(">> ")?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }

        match input {
            "exit" | "quit" => break,
            "help" => {
                Cli::command().print_help()?;
                println!("\nAdditional interactive commands:");
                println!("  help     - Show this help");
                println!("  exit     - Quit the wallet");
                println!("  clear    - Clear the screen");
                continue;
            },
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
                continue;
            },
            _ => {}
        }

        let args = shell_words::split(input)?;
        match Cli::try_parse_from(args) {
            Ok(cli) => match cli.command {
                Commands::Generate { network } => {
                    let network = match network {
                        NetworkType::Regtest => Network::Regtest,
                        NetworkType::Testnet => Network::Testnet,
                        NetworkType::Mainnet => Network::Bitcoin,
                    };

                    let wallet = wallet::Wallet::new(network);
                    let (secret_key, tweaked_pubkey) = wallet.generate_taproot_keypair();
                    let address = wallet.get_taproot_address(tweaked_pubkey);

                    let encrypted_key = crypto::encrypt_key(&[0; 32], &secret_key.secret_bytes())?;

                    db_client.execute(
                    "INSERT INTO addresses (private_key, public_key, address) VALUES ($1, $2, $3)",
                    &[&hex::encode(&encrypted_key),  // Convert binary to hex string
                    &tweaked_pubkey.to_string(), 
                    &address.to_string()]
                    ).await?;

                    println!("Generated new wallet:");
                    println!("Address: {}", address);
                },
                Commands::Send { to, amount, fee_rate } => {
                    println!("Sending {} BTC to {} with fee rate {} sat/vB", amount, to, fee_rate);
                    // let recipient = Address::from_str(&to)?.require_network(network)?;
                    // let amount = Amount::from_btc(amount)?;
                    // let fee_rate = Amount::from_sat((fee_rate * 100_000_000.0) as u64);

                    // // Get UTXOs from database
                    // let utxos = db_client.query(
                    //     "SELECT txid, vout, amount FROM utxos WHERE spent = false",
                    //     &[]
                    // ).await?;

                    // if utxos.is_empty() {
                    //     println!("No spendable funds available");
                    //     continue;
                    // }

                    // let tx_builder = transaction::TransactionBuilder::new(network);
                    // let unsigned_tx = tx_builder.create_transaction(
                    //     utxos.iter().map(|row| {
                    //         (
                    //             OutPoint::from_str(row.get::<_, String>(0))?,
                    //             Amount::from_sat(row.get::<_, i64>(2) as u64)
                    //         )
                    //     }).collect::<Result<Vec<_>>>()?,
                    //     vec![(recipient, amount)],
                    //     &get_change_address(&db_client).await?,
                    //     fee_rate
                    // );

                    // // Sign transaction
                    // let signed_tx = tx_builder.sign_taproot_transaction(
                    //     unsigned_tx,
                    //     &get_private_keys(&db_client).await?,
                    //     &get_public_keys(&db_client).await?
                    // );

                    // // Broadcast via Tor
                    // let txid = tor::broadcast_transaction(&signed_tx).await?;
                    // println!("Transaction broadcasted: {}", txid);

                    // // Update database
                    // db_client.execute(
                    //     "INSERT INTO transactions (txid, raw_transaction, status) VALUES ($1, $2, 'broadcasted')",
                    //     &[&txid.to_string(), &hex::encode(&bitcoin::consensus::serialize(&signed_tx))]
                    // ).await?;
                },
                Commands::Balance => {
                    println!("Wallet balance:");
                    let balance = db_client.query_one(
                        "SELECT COALESCE(SUM(amount), 0) FROM utxos WHERE spent = false",
                        &[]
                    ).await?.get::<_, i64>(0);

                    // println!("Available balance: {:.8} BTC", Amount::from_sat(balance as u64).to_btc());
                },
                Commands::Addresses => {
                    println!("Wallet addresses:");
                    let addresses = db_client.query(
                        "SELECT address FROM addresses ORDER BY created_at DESC",
                        &[]
                    ).await?;

                    println!("Wallet addresses:");
                    for (i, row) in addresses.iter().enumerate() {
                        println!("{}. {}", i + 1, row.get::<_, String>(0));
                    }
                },
            },
            Err(e) => eprintln!("Error: {}\nType 'help' for commands", e),
        }
    }

    println!("Goodbye!");
    Ok(())
}
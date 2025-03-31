use tokio_postgres::{Client, NoTls, Error};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};
use chrono::{DateTime, Utc};

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub id: i32,
    pub private_key: String,
    pub public_key: String,
    pub address: String,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i32,
    pub txid: String,
    pub raw_transaction: String,
    pub status: String,
    #[serde_as(as = "DisplayFromStr")]
    pub created_at: DateTime<Utc>,
}

pub async fn create_tables(client: &Client) -> Result<(), Error> {
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS addresses (
            id SERIAL PRIMARY KEY,
            private_key TEXT NOT NULL,
            public_key TEXT NOT NULL,
            address TEXT UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT now()
        );
        
        CREATE TABLE IF NOT EXISTS transactions (
            id SERIAL PRIMARY KEY,
            txid TEXT UNIQUE NOT NULL,
            raw_transaction TEXT NOT NULL,
            status TEXT CHECK (status IN ('pending', 'broadcasted', 'confirmed')),
            created_at TIMESTAMP DEFAULT now()
        );
        "#
    ).await
}

pub async fn connect_db() -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=new_password_here dbname=bitcoin_wallet", 
        NoTls
    ).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    Ok(client)
}
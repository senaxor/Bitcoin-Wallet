# Bitcoin Self-Custody Wallet
![Rust](https://img.shields.io/badge/lang-Rust-orange)
![Bitcoin](https://img.shields.io/badge/network-regtest-blue)
![PostgreSQL](https://img.shields.io/badge/db-PostgreSQL-336791)

A secure Bitcoin wallet implementing Taproot addresses with self-custody features, designed for development and testing in regtest mode.

## 🔧 Setup Instructions

### 1. Bitcoin Core (Regtest)
```bash
# Install Bitcoin Core
sudo apt-add-repository ppa:bitcoin/bitcoin
sudo apt-get update
sudo apt-get install bitcoind

# Configure ~/.bitcoin/bitcoin.conf
echo "regtest=1
txindex=1
server=1
rpcuser=yourusername
rpcpassword=yourpassword
[regtest]
listen=1
bind=127.0.0.1
rpcport=18443
proxy=127.0.0.1:9050" > ~/.bitcoin/bitcoin.conf

# Start node
bitcoind -regtest -daemon

## Features

- Taproot (P2TR) address generation
- Secure key storage with AES-256-GCM encryption
- PostgreSQL backend
- Regtest support for development
- CLI interface

## Requirements

- Rust 1.85
- Bitcoin Core (regtest mode)
- PostgreSQL
- Tor (optional)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/senaxor/bitcoin_wallet.git
cd bitcoin_wallet
cargo run
# Bitcoin Self-Custody Wallet

A secure Bitcoin wallet supporting Taproot addresses, built with Rust.

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
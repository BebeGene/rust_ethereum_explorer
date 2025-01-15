# Ethereum Block Explorer
This is a simple Ethereum block explorer built using Rust. The application fetches block data from the Ethereum mainnet using the [Infura](https://infura.io/) API and displays information about the latest block, including the block hash, parent hash, timestamp, and the last 5 transactions.

## Features
* Fetch block data from the Ethereum mainnet using Infura's JSON-RPC API.
  Display block information (e.g., block number, hash, parent hash, timestamp).* Display the last 5 transactions in the block, including:
  - Transaction hash
  - From address
  - To address
  - Transaction value
  - Gas used
  - Gas price

## Prerequisites
* Install [Rust](https://www.rust-lang.org/)
* [Infura API Key](https://infura.io/): You'll need an Infura project ID to access Ethereum data. Add that to main.rs script on line 32 "// Replace with your Infura Project ID"
* Clone repo

## Run
* Add your Infura ID for ETH Mainnet in main.rs line 32 "// Replace with your Infura Project ID"
* cd ethereum_explorer
* cargo run

## Screenshots

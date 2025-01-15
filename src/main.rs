// Fetches block data for last 5 transactions for ETH Mainnet using Infura
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct BlockData {
    result: Option<Block>, // Wraps the block data inside the 'result' field as returned by Infura
}

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    number: Option<String>,  // Changes to Option to avoid missing field issues
    hash: Option<String>,    // Makes hash optional to avoid missing field issues
    parent_hash: Option<String>, // Makes parent_hash optional
    timestamp: String,
    transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    hash: String,
    from: String,
    to: Option<String>,
    value: String,
    gas: String,
    gas_price: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let infura_url = "https://mainnet.infura.io/v3/YOUR_INFURA_ID"; // Replace with your Infura Project ID
    let client = Client::new(); // Creates a new reqwest client

    let block_number = "latest"; // Uses "latest" to fetch the most recent block, change this if a specific block number is needed

    let block_data = fetch_block_data(&client, infura_url, block_number).await?; // Fetches block data using the Infura API

    // Checks if block data exists and prints it
    if let Some(block) = block_data.result {
        print_block_data(&block); // Prints the block data
    } else {
        eprintln!("No block data found."); // Prints an error message if no block data is found
    }

    Ok(())
}

async fn fetch_block_data(client: &Client, url: &str, block_number: &str) -> Result<BlockData, Box<dyn Error>> {
    let request_payload = json!({
        "jsonrpc": "2.0", // Specifies the version of JSON-RPC
        "id": 1, // Sets the ID for the request
        "method": "eth_getBlockByNumber", // Requests the block by number
        "params": [
            block_number, // Passes the block number (e.g., "latest")
            true // True means return full transaction data
        ]
    });

    let response = client
        .post(url) // Sends a POST request to the Infura API
        .json(&request_payload) // Sends the request payload as JSON
        .send()
        .await?; // Awaits the response

    let body = response.text().await?; // Gets the body of the response as text

    // Prints the raw JSON body for debugging purposes
    // Uncomment this line to check the response format
    // println!("{}", body);

    let block_data: BlockData = serde_json::from_str(&body)?; // Deserializes the response into BlockData

    Ok(block_data) // Returns the block data
}

fn print_block_data(block: &Block) {
    println!("Block Number: {}", block.number.as_ref().unwrap_or(&"Unknown".to_string())); // Prints the block number
    println!("Block Hash: {}", block.hash.as_ref().unwrap_or(&"Unknown".to_string())); // Prints the block hash
    println!("Parent Block Hash: {}", block.parent_hash.as_ref().unwrap_or(&"Unknown".to_string())); // Prints the parent block hash
    println!("Timestamp: {}", block.timestamp); // Prints the timestamp

    println!("\nLast 5 Transactions:"); // Prints the header for the transactions section
    let transactions = &block.transactions; // Gets the list of transactions
    let last_5_transactions = transactions.iter().rev().take(5).collect::<Vec<_>>(); // Reverses the transaction list and takes the last 5 transactions

    for transaction in last_5_transactions {
        println!("  Tx Hash: {}", transaction.hash); // Prints the transaction hash
        println!("  From: {}", transaction.from); // Prints the "from" address

        // Borrows `transaction.to` using `.as_ref()` and prints it
        println!("  To: {}", transaction.to.as_ref().unwrap_or(&"None".to_string())); // Prints the "to" address (if available)

        println!("  Value: {} Wei", transaction.value); // Prints the value of the transaction in Wei
        println!("  Gas: {}", transaction.gas); // Prints the gas used for the transaction

        // Borrows `transaction.gasPrice` using `.as_ref()` and prints it
        println!("  Gas Price: {}", transaction.gas_price.as_ref().unwrap_or(&"None".to_string())); // Prints the gas price (if available)
        println!(); // Adds a blank line between transactions
    }
}

use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init();

    println!("🚀 Avalanche X-Chain (Exchange Chain) Example");
    println!("=============================================");

    // X-Chain API endpoints
    // Mainnet: https://api.avax.network/ext/bc/X
    // Fuji Testnet: https://api.avax-test.network/ext/bc/X
    let x_chain_url = "https://api.avax-test.network/ext/bc/X";
    let client = Client::new();

    println!("✅ Connected to X-Chain");
    println!("   Network: Fuji Testnet");
    println!("   API Endpoint: {}", x_chain_url);

    // 1. Get blockchain ID
    println!("\n🔍 Getting X-Chain Blockchain ID...");
    let blockchain_id = get_blockchain_id(&client, x_chain_url).await?;
    println!("   Blockchain ID: {}", blockchain_id);

    // 2. Get network information
    println!("\n🌐 Getting Network Information...");
    let network_info = get_network_info(&client, x_chain_url).await?;
    println!(
        "   Network Name: {}",
        network_info
            .get("networkName")
            .unwrap_or(&Value::String("Unknown".to_string()))
    );
    println!(
        "   Network ID: {}",
        network_info
            .get("networkID")
            .unwrap_or(&Value::String("Unknown".to_string()))
    );

    // 3. Get asset information (AVAX)
    println!("\n💰 Getting AVAX Asset Information...");
    let asset_info = get_asset_description(&client, x_chain_url, "AVAX").await?;
    if let Some(asset) = asset_info {
        println!(
            "   Asset ID: {}",
            asset
                .get("assetID")
                .unwrap_or(&Value::String("Unknown".to_string()))
        );
        println!(
            "   Name: {}",
            asset
                .get("name")
                .unwrap_or(&Value::String("Unknown".to_string()))
        );
        println!(
            "   Symbol: {}",
            asset
                .get("symbol")
                .unwrap_or(&Value::String("Unknown".to_string()))
        );
        println!(
            "   Denomination: {}",
            asset
                .get("denomination")
                .unwrap_or(&Value::String("Unknown".to_string()))
        );
    }

    // 4. Get UTXOs for an address (example)
    println!("\n📦 Getting UTXOs for Example Address...");
    let example_address = "X-fuji1wst8jt3z3fmytcehzr8s6x9e2nvw7ztv5h8l3d"; // Example X-Chain address
    let utxos = get_utxos(&client, x_chain_url, example_address).await?;
    println!("   Address: {}", example_address);
    println!("   UTXO Count: {}", utxos.len());

    for (i, utxo) in utxos.iter().take(3).enumerate() {
        println!(
            "   UTXO {}: {:?}",
            i + 1,
            utxo.get("utxoID")
                .unwrap_or(&Value::String("Unknown".to_string()))
        );
    }

    // 5. Get balance for an address
    println!("\n💳 Getting Balance for Example Address...");
    let balance = get_balance(&client, x_chain_url, example_address, "AVAX").await?;
    println!("   Address: {}", example_address);
    println!("   AVAX Balance: {} nAVAX", balance);
    println!("   AVAX Balance: {} AVAX", balance as f64 / 1_000_000_000.0);

    // 6. Get recent transactions
    println!("\n📋 Getting Recent Transactions...");
    let recent_txs = get_recent_transactions(&client, x_chain_url).await?;
    println!("   Recent Transaction Count: {}", recent_txs.len());

    for (i, tx) in recent_txs.iter().take(3).enumerate() {
        if let Some(tx_id) = tx.get("txID") {
            println!("   Transaction {}: {}", i + 1, tx_id);
        }
    }

    // 7. Get transaction details (if any recent transactions exist)
    if let Some(first_tx) = recent_txs.first() {
        if let Some(tx_id) = first_tx.get("txID") {
            println!("\n🔍 Getting Transaction Details...");
            let tx_details = get_transaction(&client, x_chain_url, tx_id.as_str().unwrap()).await?;
            if let Some(tx) = tx_details {
                println!("   Transaction ID: {}", tx_id);
                println!(
                    "   Type: {}",
                    tx.get("typeName")
                        .unwrap_or(&Value::String("Unknown".to_string()))
                );
                if let Some(block_timestamp) = tx.get("blockTimestamp") {
                    println!("   Block Timestamp: {}", block_timestamp);
                }
            }
        }
    }

    println!("\n✅ X-Chain example completed successfully!");
    println!("💡 Note: X-Chain uses UTXO model and Avalanche's native API");
    println!("   - Not EVM-compatible like C-Chain");
    println!("   - Uses Avalanche's custom transaction format");
    println!("   - Supports asset transfers and atomic swaps");

    Ok(())
}

async fn get_blockchain_id(client: &Client, url: &str) -> eyre::Result<String> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getBlockchainID",
            "params": {}
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    Ok(result["result"]["blockchainID"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string())
}

async fn get_network_info(client: &Client, url: &str) -> eyre::Result<Value> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getNetworkID",
            "params": {}
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    Ok(result["result"].clone())
}

async fn get_asset_description(
    client: &Client,
    url: &str,
    asset_symbol: &str,
) -> eyre::Result<Option<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getAssetDescription",
            "params": {
                "assetID": asset_symbol
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    if result["result"].is_null() {
        Ok(None)
    } else {
        Ok(Some(result["result"].clone()))
    }
}

async fn get_utxos(client: &Client, url: &str, address: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getUTXOs",
            "params": {
                "addresses": [address]
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    if let Some(utxos) = result["result"]["utxos"].as_array() {
        Ok(utxos.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_balance(
    client: &Client,
    url: &str,
    address: &str,
    asset_id: &str,
) -> eyre::Result<u64> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getBalance",
            "params": {
                "address": address,
                "assetID": asset_id
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    Ok(result["result"]["balance"]
        .as_str()
        .unwrap_or("0")
        .parse()
        .unwrap_or(0))
}

async fn get_recent_transactions(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getTx",
            "params": {
                "txID": "latest"
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    if let Some(txs) = result["result"].as_array() {
        Ok(txs.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_transaction(client: &Client, url: &str, tx_id: &str) -> eyre::Result<Option<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "avm.getTx",
            "params": {
                "txID": tx_id
            }
        }))
        .send()
        .await?;

    let result: Value = response.json().await?;
    if result["result"].is_null() {
        Ok(None)
    } else {
        Ok(Some(result["result"].clone()))
    }
}

use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    
    println!("🚀 Avalanche P-Chain (Platform Chain) Example");
    println!("=============================================");
    
    // P-Chain API endpoints
    // Mainnet: https://api.avax.network/ext/bc/P
    // Fuji Testnet: https://api.avax-test.network/ext/bc/P
    let p_chain_url = "https://api.avax-test.network/ext/bc/P";
    let client = Client::new();
    
    println!("✅ Connected to P-Chain");
    println!("   Network: Fuji Testnet");
    println!("   API Endpoint: {}", p_chain_url);
    
    // 1. Get blockchain ID
    println!("\n🔍 Getting P-Chain Blockchain ID...");
    let blockchain_id = get_blockchain_id(&client, p_chain_url).await?;
    println!("   Blockchain ID: {}", blockchain_id);
    
    // 2. Get network information
    println!("\n🌐 Getting Network Information...");
    let network_info = get_network_info(&client, p_chain_url).await?;
    println!("   Network Name: {}", network_info.get("networkName").unwrap_or(&Value::String("Unknown".to_string())));
    println!("   Network ID: {}", network_info.get("networkID").unwrap_or(&Value::String("Unknown".to_string())));
    
    // 3. Get current validators
    println!("\n👥 Getting Current Validators...");
    let validators = get_current_validators(&client, p_chain_url).await?;
    println!("   Validator Count: {}", validators.len());
    
    for (i, validator) in validators.iter().take(3).enumerate() {
        if let Some(node_id) = validator.get("nodeID") {
            println!("   Validator {}: {}", i + 1, node_id);
        }
        if let Some(stake_amount) = validator.get("stakeAmount") {
            println!("      Stake Amount: {} nAVAX", stake_amount);
        }
    }
    
    // 4. Get pending validators
    println!("\n⏳ Getting Pending Validators...");
    let pending_validators = get_pending_validators(&client, p_chain_url).await?;
    println!("   Pending Validator Count: {}", pending_validators.len());
    
    // 5. Get current supply
    println!("\n💰 Getting Current Supply...");
    let supply = get_current_supply(&client, p_chain_url).await?;
    println!("   Current Supply: {} nAVAX", supply);
    println!("   Current Supply: {} AVAX", supply as f64 / 1_000_000_000.0);
    
    // 6. Get subnet information
    println!("\n🌐 Getting Subnet Information...");
    let subnets = get_subnets(&client, p_chain_url).await?;
    println!("   Subnet Count: {}", subnets.len());
    
    for (i, subnet) in subnets.iter().take(3).enumerate() {
        if let Some(subnet_id) = subnet.get("id") {
            println!("   Subnet {}: {}", i + 1, subnet_id);
        }
        if let Some(control_keys) = subnet.get("controlKeys") {
            println!("      Control Keys: {}", control_keys.as_array().map(|arr| arr.len()).unwrap_or(0));
        }
    }
    
    // 7. Get blockchain information
    println!("\n🔗 Getting Blockchain Information...");
    let blockchains = get_blockchains(&client, p_chain_url).await?;
    println!("   Blockchain Count: {}", blockchains.len());
    
    for blockchain in blockchains.iter().take(5) {
        if let Some(id) = blockchain.get("id") {
            println!("   Blockchain: {}", id);
        }
        if let Some(name) = blockchain.get("name") {
            println!("      Name: {}", name);
        }
        if let Some(subnet_id) = blockchain.get("subnetID") {
            println!("      Subnet ID: {}", subnet_id);
        }
    }
    
    // 8. Get staking information
    println!("\n🏆 Getting Staking Information...");
    let staking_info = get_staking_info(&client, p_chain_url).await?;
    if let Some(min_stake) = staking_info.get("minStake") {
        println!("   Minimum Stake: {} nAVAX", min_stake);
    }
    if let Some(max_stake) = staking_info.get("maxStake") {
        println!("   Maximum Stake: {} nAVAX", max_stake);
    }
    
    // 9. Get recent blocks
    println!("\n📋 Getting Recent Blocks...");
    let recent_blocks = get_recent_blocks(&client, p_chain_url).await?;
    println!("   Recent Block Count: {}", recent_blocks.len());
    
    for (i, block) in recent_blocks.iter().take(3).enumerate() {
        if let Some(block_id) = block.get("id") {
            println!("   Block {}: {}", i + 1, block_id);
        }
        if let Some(timestamp) = block.get("timestamp") {
            println!("      Timestamp: {}", timestamp);
        }
    }
    
    // 10. Get platform status
    println!("\n📊 Getting Platform Status...");
    let status = get_platform_status(&client, p_chain_url).await?;
    println!("   Platform Status: {:?}", status);
    
    println!("\n✅ P-Chain example completed successfully!");
    println!("💡 Note: P-Chain manages the Avalanche platform:");
    println!("   - Coordinates validators and subnets");
    println!("   - Handles staking and delegation");
    println!("   - Manages blockchain creation and management");
    println!("   - Not EVM-compatible");
    
    Ok(())
}

async fn get_blockchain_id(client: &Client, url: &str) -> eyre::Result<String> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getBlockchainID",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    Ok(result["result"]["blockchainID"].as_str().unwrap_or("Unknown").to_string())
}

async fn get_network_info(client: &Client, url: &str) -> eyre::Result<Value> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getNetworkID",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    Ok(result["result"].clone())
}

async fn get_current_validators(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getCurrentValidators",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(validators) = result["result"]["validators"].as_array() {
        Ok(validators.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_pending_validators(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getPendingValidators",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(validators) = result["result"]["validators"].as_array() {
        Ok(validators.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_current_supply(client: &Client, url: &str) -> eyre::Result<u64> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getCurrentSupply",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    Ok(result["result"]["supply"].as_str().unwrap_or("0").parse().unwrap_or(0))
}

async fn get_subnets(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getSubnets",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(subnets) = result["result"]["subnets"].as_array() {
        Ok(subnets.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_blockchains(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getBlockchains",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(blockchains) = result["result"]["blockchains"].as_array() {
        Ok(blockchains.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_staking_info(client: &Client, url: &str) -> eyre::Result<Value> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getStakingInfo",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    Ok(result["result"].clone())
}

async fn get_recent_blocks(client: &Client, url: &str) -> eyre::Result<Vec<Value>> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getBlocks",
            "params": {
                "numBlocks": 10
            }
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(blocks) = result["result"]["blocks"].as_array() {
        Ok(blocks.clone())
    } else {
        Ok(vec![])
    }
}

async fn get_platform_status(client: &Client, url: &str) -> eyre::Result<Value> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "platform.getStatus",
            "params": {}
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    Ok(result["result"].clone())
}

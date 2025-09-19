use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{BlockNumberOrTag};
use reqwest::{Client, Url};
use serde_json::{json, Value};
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init();
    
    println!("🚀 Avalanche Multi-Chain Example");
    println!("================================");
    println!("This example demonstrates connecting to all three Avalanche chains:");
    println!("• C-Chain (Contract Chain) - EVM compatible");
    println!("• X-Chain (Exchange Chain) - UTXO model");
    println!("• P-Chain (Platform Chain) - Platform management");
    println!();
    
    // Test all three chains
    test_c_chain().await?;
    test_x_chain().await?;
    test_p_chain().await?;
    
    println!("\n🎉 All chain examples completed successfully!");
    println!("💡 Summary:");
    println!("   ✅ C-Chain: EVM-compatible, uses alloy-rs");
    println!("   ✅ X-Chain: UTXO model, uses Avalanche API");
    println!("   ✅ P-Chain: Platform management, uses Avalanche API");
    
    Ok(())
}

async fn test_c_chain() -> eyre::Result<()> {
    println!("🔗 Testing C-Chain (Contract Chain)");
    println!("-----------------------------------");
    
    let c_chain_url = "https://api.avax-test.network/ext/bc/C/rpc";
    let url = Url::parse(c_chain_url)?;
    let provider = Arc::new(ProviderBuilder::new().connect_http(url));
    
    // Get basic chain information
    let chain_id = provider.get_chain_id().await?;
    let latest_block = provider.get_block_number().await?;
    let gas_price = provider.get_gas_price().await?;
    
    println!("   ✅ Connected to C-Chain");
    println!("      Chain ID: {}", chain_id);
    println!("      Latest Block: {}", latest_block);
    println!("      Gas Price: {} wei", gas_price);
    
    // Get block details
    if let Some(block) = provider
        .get_block_by_number(BlockNumberOrTag::Number(latest_block))
        .await?
    {
        println!("      Block Hash: {:?}", block.hash());
        println!("      Transaction Count: {}", block.transactions.len());
    }
    
    // Test balance check
    let example_address = Address::from_str("0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC")?;
    let balance = provider.get_balance(example_address).await?;
    println!("      Example Balance: {} AVAX", balance / U256::from(10_u64.pow(18)));
    
    println!("   ✅ C-Chain test completed\n");
    Ok(())
}

async fn test_x_chain() -> eyre::Result<()> {
    println!("🔗 Testing X-Chain (Exchange Chain)");
    println!("-----------------------------------");
    
    let x_chain_url = "https://api.avax-test.network/ext/bc/X";
    let client = Client::new();
    
    // Get blockchain ID
    let blockchain_id = make_rpc_call(&client, x_chain_url, "avm.getBlockchainID", json!({})).await?;
    println!("   ✅ Connected to X-Chain");
    println!("      Blockchain ID: {}", blockchain_id["blockchainID"]);
    
    // Get network info
    let network_id = make_rpc_call(&client, x_chain_url, "avm.getNetworkID", json!({})).await?;
    println!("      Network ID: {}", network_id["networkID"]);
    
    // Get asset info
    let asset_info = make_rpc_call(&client, x_chain_url, "avm.getAssetDescription", json!({"assetID": "AVAX"})).await?;
    if !asset_info.is_null() {
        println!("      AVAX Asset ID: {}", asset_info["assetID"]);
        println!("      AVAX Symbol: {}", asset_info["symbol"]);
    }
    
    // Test UTXO query
    let example_address = "X-fuji1wst8jt3z3fmytcehzr8s6x9e2nvw7ztv5h8l3d";
    let utxos = make_rpc_call(&client, x_chain_url, "avm.getUTXOs", json!({"addresses": [example_address]})).await?;
    println!("      Example Address UTXOs: {}", utxos["utxos"].as_array().map(|arr| arr.len()).unwrap_or(0));
    
    println!("   ✅ X-Chain test completed\n");
    Ok(())
}

async fn test_p_chain() -> eyre::Result<()> {
    println!("🔗 Testing P-Chain (Platform Chain)");
    println!("-----------------------------------");
    
    let p_chain_url = "https://api.avax-test.network/ext/bc/P";
    let client = Client::new();
    
    // Get blockchain ID
    let blockchain_id = make_rpc_call(&client, p_chain_url, "platform.getBlockchainID", json!({})).await?;
    println!("   ✅ Connected to P-Chain");
    println!("      Blockchain ID: {}", blockchain_id["blockchainID"]);
    
    // Get network info
    let network_id = make_rpc_call(&client, p_chain_url, "platform.getNetworkID", json!({})).await?;
    println!("      Network ID: {}", network_id["networkID"]);
    
    // Get current supply
    let supply = make_rpc_call(&client, p_chain_url, "platform.getCurrentSupply", json!({})).await?;
    let supply_amount: u64 = supply["supply"].as_str().unwrap_or("0").parse().unwrap_or(0);
    println!("      Current Supply: {} AVAX", supply_amount as f64 / 1_000_000_000.0);
    
    // Get validators
    let validators = make_rpc_call(&client, p_chain_url, "platform.getCurrentValidators", json!({})).await?;
    let validator_count = validators["validators"].as_array().map(|arr| arr.len()).unwrap_or(0);
    println!("      Active Validators: {}", validator_count);
    
    // Get subnets
    let subnets = make_rpc_call(&client, p_chain_url, "platform.getSubnets", json!({})).await?;
    let subnet_count = subnets["subnets"].as_array().map(|arr| arr.len()).unwrap_or(0);
    println!("      Subnets: {}", subnet_count);
    
    // Get blockchains
    let blockchains = make_rpc_call(&client, p_chain_url, "platform.getBlockchains", json!({})).await?;
    let blockchain_count = blockchains["blockchains"].as_array().map(|arr| arr.len()).unwrap_or(0);
    println!("      Blockchains: {}", blockchain_count);
    
    println!("   ✅ P-Chain test completed\n");
    Ok(())
}

async fn make_rpc_call(client: &Client, url: &str, method: &str, params: Value) -> eyre::Result<Value> {
    let response = client
        .post(url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    if let Some(error) = result.get("error") {
        return Err(eyre::eyre!("RPC Error: {}", error));
    }
    
    Ok(result["result"].clone())
}

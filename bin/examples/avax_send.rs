use alloy::primitives::U64;
use alloy::primitives::{Address, B256, Bytes, Signature, U256};
use alloy::providers::Provider;
use alloy::rpc::types::BlockNumberOrTag;
use alloy::rpc::types::eth::Block;
use alloy::rpc::types::eth::BlockId;
use alloy::rpc::types::eth::{Transaction, TransactionRequest};
use alloy_primitives::TxKind;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::Http;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Connect to Avalanche Fuji testnet
    let reqwest_url = reqwest::Url::parse("https://api.avax-test.network/ext/bc/C/rpc")?;
    let provider = Arc::new(ProviderBuilder::new().connect_http(reqwest_url));

    // Replace with the recipient's address
    let to_address = Address::from_str("0xRecipientAddressHere")?;

    // Amount to send (in wei, 1 AVAX = 10^18 wei)
    let value = U256::from_str_radix("10000000000000000", 10)?; // 0.01 AVAX

    // Get the current nonce for the sender
    let nonce = provider.get_transaction_count(to_address).await?;

    // Estimate gas price
    let gas_price = provider.get_gas_price().await?;

    // Build the transaction
    let tx = TransactionRequest {
        from: Some(to_address),
        to: Some(TxKind::Call(to_address)),
        value: Some(value),
        gas: Some(21000),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        input: alloy::rpc::types::eth::TransactionInput::default(),
        ..Default::default()
    };

    // Sign the transaction
    let chain_id = provider.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);
    println!("Transaction request: {:?}", tx);
    println!("From address: {:?}", to_address);
    println!("To address: {:?}", to_address);
    println!("Value: {:?}", value);
    println!("Nonce: {:?}", nonce);
    println!("Gas price: {:?}", gas_price);

    // Note: Transaction signing and sending requires additional setup
    // For now, just print the transaction details
    println!("Transaction prepared successfully!");
    println!("To send this transaction, you would need to:");
    println!("1. Sign the transaction with a private key");
    println!("2. Serialize it to RLP format");
    println!("3. Send it via send_raw_transaction");

    Ok(())
}

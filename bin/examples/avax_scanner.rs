use alloy::primitives::U64;
use alloy::providers::Provider;
use alloy::rpc::types::BlockNumberOrTag;
use alloy::rpc::types::eth::Block;
use alloy::rpc::types::eth::BlockId;
use alloy_primitives::TxKind;
use alloy_provider::ProviderBuilder;
use alloy_transport_http::Http;
use std::sync::Arc;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Connect to Avalanche Fuji testnet
    let reqwest_url = reqwest::Url::parse("https://api.avax-test.network/ext/bc/C/rpc")?;
    let provider = Arc::new(ProviderBuilder::new().connect_http(reqwest_url));

    loop {
        // Get latest block number
        let latest_block_num: u64 = provider.get_block_number().await?;
        println!("Latest block number: {}", latest_block_num);

        // Get full block by number, including transactions
        let block: Option<Block> = provider
            .get_block_by_number(BlockNumberOrTag::Number(latest_block_num))
            .await?;

        if let Some(block) = block {
            println!("Block hash: {:?}", block.hash());
            println!("Transaction count: {}", block.transactions.len());

            for tx_hash in block.transactions.hashes() {
                println!("Tx hash: {:?}", tx_hash);
            }
        } else {
            println!("Block not found");
        }

        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    // INSERT_YOUR_CODE

    // Example: Send AVAX (native token) to another wallet
    // NOTE: This is a simplified example and assumes you have a private key and enough balance.
    // In production, handle secrets securely and use proper error handling.

    use alloy::primitives::{Address, U256};
    use alloy::rpc::types::eth::TransactionRequest;
    use std::str::FromStr;

    // Example transaction preparation (without signing)
    let to_address = Address::from_str("0xRecipientAddressHere")?;
    let from_address = Address::from_str("0xSenderAddressHere")?;
    let value = U256::from_str_radix("10000000000000000", 10)?; // 0.01 AVAX

    // Get the current nonce for the sender
    let nonce = provider.get_transaction_count(from_address).await?;

    // Estimate gas price
    let gas_price = provider.get_gas_price().await?;

    // Build the transaction
    let tx = TransactionRequest {
        from: Some(from_address),
        to: Some(TxKind::Call(to_address)),
        value: Some(value),
        gas: Some(21000),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        input: alloy::rpc::types::eth::TransactionInput::default(),
        ..Default::default()
    };

    // Get chain ID
    let chain_id = provider.get_chain_id().await?;

    // Print transaction details (signing and sending requires additional setup)
    println!("Transaction prepared successfully!");
    println!("From: {:?}", from_address);
    println!("To: {:?}", to_address);
    println!("Value: {:?}", value);
    println!("Nonce: {:?}", nonce);
    println!("Gas price: {:?}", gas_price);
    println!("Chain ID: {:?}", chain_id);
    println!("To send this transaction, you would need to:");
    println!("1. Sign the transaction with a private key");
    println!("2. Serialize it to RLP format");
    println!("3. Send it via send_raw_transaction");

    Ok(())
}

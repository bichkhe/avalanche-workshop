use alloy::primitives::{Address, TxKind, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::BlockNumberOrTag;
use alloy::rpc::types::eth::{Block, TransactionRequest};
use reqwest::Url;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    pretty_env_logger::init();

    println!("🚀 Avalanche C-Chain (Contract Chain) Example");
    println!("=============================================");

    // Connect to Avalanche C-Chain (EVM-compatible)
    // Mainnet: https://api.avax.network/ext/bc/C/rpc
    // Fuji Testnet: https://api.avax-test.network/ext/bc/C/rpc
    let c_chain_url = "https://api.avax-test.network/ext/bc/C/rpc";
    let url = Url::parse(c_chain_url)?;
    let provider = Arc::new(ProviderBuilder::new().connect_http(url));

    // Get chain information
    let chain_id = provider.get_chain_id().await?;
    println!("✅ Connected to C-Chain");
    println!("   Chain ID: {}", chain_id);
    println!("   Network: Fuji Testnet");

    // Get latest block information
    let latest_block_number = provider.get_block_number().await?;
    println!("   Latest Block: {}", latest_block_number);

    // Get block details
    let block: Option<Block> = provider
        .get_block_by_number(BlockNumberOrTag::Number(latest_block_number))
        .await?;

    if let Some(ref block) = block {
        println!("   Block Hash: {:?}", block.hash());
        println!("   Block Timestamp: {}", block.header.inner.timestamp);
        println!("   Gas Used: {}", block.header.inner.gas_used);
        println!("   Gas Limit: {}", block.header.inner.gas_limit);
        println!("   Transaction Count: {}", block.transactions.len());
    }

    // Get network information
    let gas_price = provider.get_gas_price().await?;
    println!("   Current Gas Price: {} wei", gas_price);

    // Example: Check balance of an address
    // let example_address = Address::from_str("0x8db97C7cEcE249c2b98bDC0226Cc4C2A57BF52FC")?; // Example address
    let example_address = Address::from_str("0x3656DD5F2f1DEA6535E239A8daFCAbC1A02574FC")?;
    let balance = provider.get_balance(example_address).await?;
    println!("   Example Address Balance: {} wei", balance);
    println!(
        "   Example Address Balance: {} AVAX",
        U256::from(balance) / U256::from(10_u64.pow(18))
    );

    // Example: Get transaction count (nonce)
    let nonce = provider.get_transaction_count(example_address).await?;
    println!("   Example Address Nonce: {}", nonce);

    // Example: Prepare a transaction (without signing)
    println!("\n📝 Transaction Preparation Example:");
    let recipient = Address::from_str("0x742d35Cc6634C0532925a3b8D4C9db96C4b4d8b6")?;
    let value = U256::from_str_radix("1000000000000000000", 10)?; // 1 AVAX in wei

    let tx = TransactionRequest {
        from: Some(example_address),
        to: Some(TxKind::Call(recipient)),
        value: Some(value),
        gas: Some(21000),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        input: alloy::rpc::types::eth::TransactionInput::default(),
        ..Default::default()
    };

    println!("   From: {:?}", example_address);
    println!("   To: {:?}", recipient);
    println!("   Value: {} AVAX", value / U256::from(10_u64.pow(18)));
    println!("   Gas: {}", tx.gas.unwrap());
    println!("   Gas Price: {} wei", gas_price);
    println!("   Nonce: {}", nonce);

    // Example: Get recent transactions from the latest block
    println!("\n🔍 Recent Transactions:");
    if let Some(block) = block {
        let tx_hashes = block.transactions.hashes();
        for (i, tx_hash) in tx_hashes.take(5).enumerate() {
            println!("   {}. {:?}", i + 1, tx_hash);
        }
    }

    println!("\n✅ C-Chain example completed successfully!");
    println!("💡 Note: To send transactions, you would need to:");
    println!("   1. Sign the transaction with a private key");
    println!("   2. Serialize it to RLP format");
    println!("   3. Send it via send_raw_transaction");

    Ok(())
}

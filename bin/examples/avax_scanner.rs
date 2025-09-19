use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{BlockNumberOrTag, eth::Block};
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
}

use ethers::providers::{Middleware, Provider};

use crate::business::chain_service::eth_call_service::{get_block_by_block_hash, get_block_by_block_number, latest_block_number};
use crate::business::model::enums::block_status_enum::BlockStatusEnum;

#[tokio::test]
async fn test_eth_block_number() {
    let resp = latest_block_number().await;
    println!("test_eth_block_number {:?}", resp);
}

#[tokio::test]
async fn test_get_block_by_block_number() {
    let resp = get_block_by_block_number(BlockStatusEnum::Latest).await;
    println!("get_block_by_hash {:?}", resp);
}

#[tokio::test]
async fn test_get_block_by_block_hash() {
    let block_hash = "0x148c90e2a8e841394e3acde75e481a0f0832db3756f1a69c7d6bd903e18a3167".to_string();
    let resp = get_block_by_block_hash(Some(block_hash)).await;
    println!("get_block_by_hash {:?}", resp);
}




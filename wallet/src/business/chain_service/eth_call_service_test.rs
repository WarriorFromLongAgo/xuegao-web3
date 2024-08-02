use ethers::providers::{Middleware, Provider};

use crate::business::chain_service::eth_call_service::{get_block_by_block_hash, get_block_by_block_number, get_gas_price, get_max_priority_fee_per_gas, get_tx_by_tx_hash, get_tx_count_by_address, get_tx_receipt_by_tx_hash, latest_block_number};
use crate::business::model::enums::block_status_enum::BlockStatusEnum;

#[tokio::test]
async fn test_eth_block_number() {
    let resp = latest_block_number().await;
    println!("latest_block_number {:?}", resp);
}


#[tokio::test]
async fn test_get_gas_price() {
    let resp = get_gas_price().await;
    println!("get_gas_price {:?}", resp);
}

#[tokio::test]
async fn test_get_max_priority_fee_per_gas() {
    let resp = get_max_priority_fee_per_gas().await;
    println!("get_max_priority_fee_per_gas {:?}", resp);
}

#[tokio::test]
async fn test_get_block_by_block_number() {
    let resp = get_block_by_block_number(BlockStatusEnum::Latest).await;
    println!("get_block_by_block_number {:?}", resp);
}

#[tokio::test]
async fn test_get_block_by_block_hash() {
    let block_hash = "0x148c90e2a8e841394e3acde75e481a0f0832db3756f1a69c7d6bd903e18a3167".to_string();
    let resp = get_block_by_block_hash(Some(block_hash)).await;
    println!("get_block_by_block_hash {:?}", resp);
}

#[tokio::test]
async fn test_get_tx_by_tx_hash() {
    let tx_hash = "0x51669aadf3019d3fbc00822bb16f66abacecc4dc2fceed54cff447c150043cac".to_string();
    let resp = get_tx_by_tx_hash(tx_hash).await;
    println!("get_block_by_hash {:?}", resp);
}

#[tokio::test]
async fn test_get_tx_receipt_by_tx_hash() {
    let tx_hash = "0x51669aadf3019d3fbc00822bb16f66abacecc4dc2fceed54cff447c150043cac".to_string();
    let resp = get_tx_receipt_by_tx_hash(tx_hash).await;
    println!("test_get_tx_by_tx_hash {:?}", resp);
}

#[tokio::test]
async fn test_get_tx_count_by_address() {
    let tx_hash = "0xac561284b659341fa3199d9f44bd702c061f96b0".to_string();
    let resp = get_tx_count_by_address(tx_hash, BlockStatusEnum::Latest).await;
    println!("get_tx_count {:?}", resp);
}
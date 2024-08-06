// 提现 job


use std::str::FromStr;

use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer, WalletError};
use ethers::types::{Address, TransactionRequest};
use serde::Serialize;

pub async fn withdraw_job() {

    // 私钥，用于创建钱包
    let private_key = "your-private-key-here";
    let wallet_result: Result<LocalWallet, WalletError> = private_key.parse();
    if (wallet_result.is_err()) {
        eprintln!("[xuegao-web3][withdraw_job][withdraw_job][wallet_result err={:?}]", wallet_result.err());
        return;
    }
    let wallet = wallet_result.unwrap();

    // 配置交易信息
    let to_address_result: Result<H160, FromHexError> = Address::from_str("0xrecipientAddressHere");
    if to_address_result.is_err() {
        eprintln!("[xuegao-web3][withdraw_job][to_address_result err={:?}]", to_address_result.err());
        return;
    }
    let to_address = to_address_result.unwrap();


    let value = U256::from(1000000000000000000u64); // 发送 1 ETH
    let tx = TransactionRequest::pay(to_address, value)
        .from(wallet.address())
        .nonce(U256::from(0u64))
        .gas_price(U256::from(1000000000u64))
        .gas(U256::from(21000u64));

    // 签名交易
    match wallet.sign_transaction(&tx).await {
        Ok(signed_tx) => {
            let signed_tx_hex = signed_tx.serialize();
            println!("Signed transaction: 0x{}", hex::encode(signed_tx_hex));
        }
        Err(e) => {
            eprintln!("[xuegao-web3][withdraw_job][sign_transaction err={:?}]", e);
        }
    }
}
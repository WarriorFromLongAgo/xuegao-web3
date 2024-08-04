#[cfg(test)]
mod tests {
    use log::info;
    use serde_json::json;

    use crate::business::eth::model::resp::transaction_resp::Transaction;

    #[test]
    fn test_json_form_value() {
        // 定义一个 JSON 数组字符串
        let json_data = json!([
            {
                "accessList": [],
                "blockHash": "0x957df05e008c7fd81b6c2ea7a8cf57c003bd67c2a5e85e73397afc48f631f262",
                "blockNumber": "0x137f8fb",
                "chainId": "0x1",
                "from": "0x890fb8df0ceaf451e2de4acb83377dc3850728e2",
                "gas": "0x10876",
                "gasPrice": "0x69800e80",
                "hash": "0x225582ea653e06c0d3260855efb43829c8e0ff624b6b23605de6a76a74265134",
                "input": "0x6080604052348015600e575f80fd5b50603e80601a5f395ff3fe60806040525f80fdfea2646970667358221220f5f4d0df2c2a2f5904f9d1c3285c6dcccfcbe769ce2f211e2ba57eede3e9723564736f6c634300081a0033",
                "maxFeePerGas": "0x69800e80",
                "maxPriorityFeePerGas": "0x3b9aca00",
                "nonce": "0x3c",
                "r": "0x5af6a461fbe158fad10c70d0497bba6edf4c1ce10cc938f75a343d0f1c9bab64",
                "s": "0x7c39265e151db21948d87ff0a6e5692050d87c6a2540b2f520198866898ec3ad",
                "to": null,
                "transactionIndex": "0x59",
                "type": "0x2",
                "v": "0x0",
                "value": "0x0",
                "yParity": "0x0"
            }
        ]);


        // 将 JSON 数组字符串转换为 Vec<Transaction> 结构体
        let transactions_result: Result<Vec<Transaction>, serde_json::Error> = serde_json::from_value(json_data);
        match transactions_result {
            Ok(transactions_vec) => {
                // info!("[xuegao-web3][transaction_resp_test][][={:?}]", serde_json::to_string(&transactions_vec));
                eprintln!("[xuegao-web3][transaction_resp_test][][={:?}]", serde_json::to_string(&transactions_vec));
            }
            Err(error) => {
                info!("[xuegao-web3][transaction_resp_test][][error={}]", error);
                eprintln!("[xuegao-web3][transaction_resp_test][][error={:?}]", error);
            }
        }
    }
}
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockStatusEnum {
    Earliest,
    Latest,
    Safe,
    Finalized,
    Pending,
    // 0x10d4f
    Number(String), // 允许传递区块高度
}

impl BlockStatusEnum {
    // 获取区块参数的描述
    pub fn chinese(&self) -> &'static str {
        match self {
            BlockStatusEnum::Earliest => "最早的/创世区块",
            BlockStatusEnum::Latest => "最新提出的区块",
            BlockStatusEnum::Safe => "最新的安全头区块",
            BlockStatusEnum::Finalized => "最新确定的区块",
            BlockStatusEnum::Pending => "待处理状态/交易",
            BlockStatusEnum::Number(_) => "指定高度的区块",
        }
    }

    // 获取区块参数的名称
    pub fn english(&self) -> String {
        match self {
            BlockStatusEnum::Earliest => "earliest".to_string(),
            BlockStatusEnum::Latest => "latest".to_string(),
            BlockStatusEnum::Safe => "safe".to_string(),
            BlockStatusEnum::Finalized => "finalized".to_string(),
            BlockStatusEnum::Pending => "pending".to_string(),
            BlockStatusEnum::Number(String) => String.clone(), // 或者你想要返回的其他值
        }
    }
}
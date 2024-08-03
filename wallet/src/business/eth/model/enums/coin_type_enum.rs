// Coin type	Path component (coin_type')	Symbol	    Coin
// 60	        0x8000003c	                ETH	        Ether
// 61	        0x8000003d	                ETC	        Ether Classic
// 501	        0x800001f5	                SOL	        Solana
// 784	        0x80000310	                SUI	        Sui
// 614	        0x80000266	                OPT	        Optimistic Ethereum
//

/// 定义区块链网络及其链 ID。
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CoinTypeEnum {
    Ether,
    EtherClassic,
    Solana,
    Sui,
    Optimistic,
}

impl CoinTypeEnum {
    /// 获取符号。
    /// 获取 `coin_type`。
    pub fn coin_type(&self) -> u32 {
        match self {
            Self::Ether => 60,
            Self::EtherClassic => 61,
            Self::Solana => 501,
            Self::Sui => 784,
            Self::Optimistic => 614,
        }
    }

    /// 根据 `coin_type` 获取枚举变体。
    pub fn from_coin_type(coin_type: u32) -> Option<Self> {
        match coin_type {
            60 => Some(Self::Ether),
            61 => Some(Self::EtherClassic),
            501 => Some(Self::Solana),
            784 => Some(Self::Sui),
            614 => Some(Self::Optimistic),
            _ => None,
        }
    }

    /// 获取符号。
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Ether => "ETH",
            Self::EtherClassic => "ETC",
            Self::Solana => "SOL",
            Self::Sui => "SUI",
            Self::Optimistic => "OPT",
        }
    }
}

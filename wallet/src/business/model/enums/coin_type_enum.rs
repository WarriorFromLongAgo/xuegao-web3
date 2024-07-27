// Coin type	Path component (coin_type')	Symbol	    Coin
// 60	        0x8000003c	                ETH	        Ether
// 61	        0x8000003d	                ETC	        Ether Classic
// 501	        0x800001f5	                SOL	        Solana
// 784	        0x80000310	                SUI	        Sui
// 614	        0x80000266	                OPT	        Optimistic Ethereum
//

/// 定义区块链网络及其链 ID。
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ChainId {
    Ether,
    Solana,
    Sui,
    OptimisticEthereum,
}

impl ChainId {
    /// 获取符号。
    pub fn coin_type(&self) -> &'static str {
        match self {
            Self::Bitcoin => "BTC",
            Self::Testnet => "Testnet",
            Self::Litecoin => "LTC",
            Self::Dogecoin => "DOGE",
        }
    }

    /// 根据 path_component 获取 CoinType 枚举变体。
    pub fn from_path_component(path_component: u32) -> Option<Self> {
        match path_component {
            0x80000000 => Some(Self::Bitcoin),
            0x80000001 => Some(Self::Testnet),
            0x80000002 => Some(Self::Litecoin),
            0x80000003 => Some(Self::Dogecoin),
            _ => None,
        }
    }


    /// 获取符号。
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Bitcoin => "BTC",
            Self::Testnet => "Testnet",
            Self::Litecoin => "LTC",
            Self::Dogecoin => "DOGE",
        }
    }


}

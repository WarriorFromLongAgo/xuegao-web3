mod block {
    // 默认确认数。在区块链中，一个交易被确认意味着它被添加到区块链的块中，
    // 并且这些块已经被足够多的区块链节点验证和接受。确认数表示在一个交易被视为有效之前，
    // 网络需要在其之后生成的区块数量。
    // 一般来说，确认数越高，确认的安全性就越高，因为攻击者要修改交易历史记录就需要更多的计算能力。
    const DEFAULT_CONFIRMATIONS: u8 = 64;
    // 默认存款间隔。在区块链中，存款间隔可能指定了两个存款之间的最小时间间隔，以确保交易被处理和确认。
    const DEFAULT_DEPOSIT_INTERVAL: u16 = 5000;
    // 默认提现间隔。类似于存款间隔，提款间隔可能指定了两次提款之间的最小时间间隔。
    const DEFAULT_WITHDRAW_INTERVAL: u16 = 500;
    // 默认归集间隔。在某些区块链应用中，可能需要定期收集收入或资产，这个间隔指定了多久收集一次。
    const DEFAULT_COLLECT_INTERVAL: u16 = 500;
    // 默认转冷间隔。在某些区块链中，冷存储用于安全存储加密货币，冷间隔可能是指定冷存储或操作的时间间隔。
    const DEFAULT_COLD_INTERVAL: u16 = 500;
    // 默认块步长。在扫链代码中，处理区块链数据时可能会使用块步长来限制每次处理的块数量，以优化性能或避免超出资源限制。
    const DEFAULT_BLOCKS_STEP: u16 = 500;

    // 空地址
    const ZERO_ADDRESS: &str = "0x00";
}


mod eth {
    const ZK_FAIR_SEPOLIA_CHAIN_ID: u16 = 43851;
    const ZK_FAIR_CHAIN_ID: u16 = 42766;
}

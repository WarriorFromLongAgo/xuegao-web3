SET timezone = 'Asia/Shanghai';
ALTER DATABASE chain_scan SET timezone TO 'Asia/Shanghai';

-- 创建区块表，如果表不存在
CREATE TABLE IF NOT EXISTS blocks
(
    hash        VARCHAR   not null PRIMARY KEY, -- 区块的唯一标识符，使用区块内容生成的哈希值
    parent_hash VARCHAR   NOT NULL,             -- 父区块的哈希值，指向前一个区块
    number      DECIMAL   NOT NULL,             -- 区块的编号，从创世区块开始递增
    timestamp   TIMESTAMP NOT NULL,             -- 区块创建的时间戳，TIMESTAMP 格式
    rlp_bytes   VARCHAR   NOT NULL              -- 区块的原始数据，经过 RLP 编码后的字节字符串
);

-- 为现有表添加 UNIQUE 约束
ALTER TABLE blocks
    ADD CONSTRAINT blocks_number_unique UNIQUE (number);

ALTER TABLE blocks
    ADD CONSTRAINT blocks_parent_hash_unique UNIQUE (parent_hash);

-- 为现有表添加 CHECK 约束
ALTER TABLE blocks
    ADD CONSTRAINT blocks_number_check CHECK (number > 0);

ALTER TABLE blocks
    ADD CONSTRAINT blocks_timestamp_check CHECK (timestamp > '1970-01-01 00:00:00'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS blocks_hash ON blocks (hash);
CREATE INDEX IF NOT EXISTS blocks_number ON blocks (number);
CREATE INDEX IF NOT EXISTS blocks_timestamp ON blocks (timestamp);

-- 添加表注释
COMMENT ON TABLE blocks IS '区块链中的区块数据表';
-- 添加字段注释
COMMENT ON COLUMN blocks.hash IS '区块的唯一标识符，使用区块内容生成的哈希值';
COMMENT ON COLUMN blocks.parent_hash IS '父区块的哈希值，指向前一个区块';
COMMENT ON COLUMN blocks.number IS '区块的编号，从创世区块开始递增';
COMMENT ON COLUMN blocks.timestamp IS '区块创建的时间';
COMMENT ON COLUMN blocks.rlp_bytes IS '区块的原始数据，经过 RLP 编码后的字节字符串';


-- 创建 tokens 表，如果表不存在
CREATE TABLE IF NOT EXISTS tokens
(
    guid           SERIAL PRIMARY KEY,            -- 唯一标识符
    token_address  VARCHAR   NOT NULL,            -- 代币地址
    decimal        SMALLINT  NOT NULL DEFAULT 18, -- 单位（默认值为 18）
    token_name     VARCHAR   NOT NULL,            -- 代币名称
    collect_amount DECIMAL   NOT NULL,            -- 收集的金额
    timestamp      TIMESTAMP NOT NULL             -- 时间
);

-- 为现有表添加 CHECK 约束
ALTER TABLE tokens
    ADD CONSTRAINT tokens_collect_amount_check CHECK (collect_amount > 0);

ALTER TABLE tokens
    ADD CONSTRAINT tokens_timestamp_check CHECK (timestamp > '1970-01-01 00:00:00'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS tokens_timestamp ON tokens (timestamp);
CREATE INDEX IF NOT EXISTS tokens_token_address ON tokens (token_address);
CREATE INDEX IF NOT EXISTS tokens_token_name ON tokens (token_name);

-- 添加表注释
COMMENT ON TABLE tokens IS '代币信息表，存储代币的基本信息和数据';

-- 添加字段注释
COMMENT ON COLUMN tokens.guid IS '代币的唯一标识符';
COMMENT ON COLUMN tokens.token_address IS '代币的地址';
COMMENT ON COLUMN tokens.decimal IS '代币的单位，默认值为 18';
COMMENT ON COLUMN tokens.token_name IS '代币的名称';
COMMENT ON COLUMN tokens.collect_amount IS '收集的金额，必须大于 0';
COMMENT ON COLUMN tokens.timestamp IS '时间';


-- 创建 addresses 表，如果表不存在
CREATE TABLE IF NOT EXISTS addresses
(
    guid         SERIAL PRIMARY KEY, -- 唯一标识符
    user_uid     VARCHAR   NOT NULL, -- 用户的唯一标识符
    address      VARCHAR   NOT NULL, -- 地址
    address_type VARCHAR   NOT NULL, -- 地址类型
    private_key  VARCHAR   NOT NULL, -- 私钥
    public_key   VARCHAR   NOT NULL, -- 公钥
    timestamp    TIMESTAMP NOT NULL  -- 时间
);
-- 为现有表添加 CHECK 约束
ALTER TABLE addresses
    ADD CONSTRAINT addresses_timestamp_check CHECK (timestamp > '1970-01-01 00:00:01'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS addresses_user_uid ON addresses (user_uid);
CREATE INDEX IF NOT EXISTS addresses_address ON addresses (address);
CREATE INDEX IF NOT EXISTS addresses_timestamp ON addresses (timestamp);

-- 添加表注释
COMMENT ON TABLE addresses IS '存储用户地址及其相关信息';

-- 添加字段注释
COMMENT ON COLUMN addresses.guid IS '地址的唯一标识符';
COMMENT ON COLUMN addresses.user_uid IS '用户的唯一标识符';
COMMENT ON COLUMN addresses.address IS '地址';
COMMENT ON COLUMN addresses.address_type IS '地址类型';
COMMENT ON COLUMN addresses.private_key IS '私钥';
COMMENT ON COLUMN addresses.public_key IS '公钥';
COMMENT ON COLUMN addresses.timestamp IS '时间';


-- 创建 balances 表，如果表不存在
CREATE TABLE IF NOT EXISTS balances
(
    guid          SERIAL PRIMARY KEY,           -- 唯一标识符
    address       VARCHAR   NOT NULL,           -- 地址
    token_address VARCHAR   NOT NULL,           -- 代币地址
    balance       DECIMAL   NOT NULL,           -- 余额，必须大于或等于 0
    lock_balance  DECIMAL   NOT NULL,           -- 锁定的余额
    timestamp     TIMESTAMP NOT NULL            -- 时间
);
-- 为现有表添加 CHECK 约束
ALTER TABLE balances
    ADD CONSTRAINT balances_balance_check CHECK (balance >= 0);

ALTER TABLE balances
    ADD CONSTRAINT balances_timestamp_check CHECK (timestamp > '1970-01-01 00:00:01'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS balances_address ON balances (address);
CREATE INDEX IF NOT EXISTS balances_timestamp ON balances (timestamp);

-- 添加表注释
COMMENT ON TABLE balances IS '存储用户的代币余额及其锁定余额';

-- 添加字段注释
COMMENT ON COLUMN balances.guid IS '余额记录的唯一标识符';
COMMENT ON COLUMN balances.address IS '地址';
COMMENT ON COLUMN balances.token_address IS '代币地址';
COMMENT ON COLUMN balances.balance IS '余额，必须大于或等于 0';
COMMENT ON COLUMN balances.lock_balance IS '锁定的余额';
COMMENT ON COLUMN balances.timestamp IS '时间';


-- 创建 transactions 表，如果表不存在
CREATE TABLE IF NOT EXISTS transactions
(
    guid              SERIAL PRIMARY KEY,           -- 唯一标识符
    block_hash        VARCHAR   NOT NULL,           -- 区块哈希值
    block_number      DECIMAL   NOT NULL,           -- 区块编号，必须大于 0
    hash              VARCHAR   NOT NULL,           -- 交易哈希值
    from_address      VARCHAR   NOT NULL,           -- 发起地址
    to_address        VARCHAR   NOT NULL,           -- 目标地址
    token_address     VARCHAR   NOT NULL,           -- 代币地址
    fee               DECIMAL   NOT NULL,           -- 手续费
    amount            DECIMAL   NOT NULL,           -- 交易金额
    status            SMALLINT  NOT NULL DEFAULT 0, -- 交易状态，默认值为 0
    transaction_index DECIMAL   NOT NULL,           -- 交易索引
    tx_type           SMALLINT  NOT NULL DEFAULT 0, -- 交易类型，默认值为 0
    timestamp         TIMESTAMP NOT NULL            -- 时间
);

-- 为现有表添加 CHECK 约束
ALTER TABLE transactions
    ADD CONSTRAINT transactions_block_number_check CHECK (block_number > 0);

ALTER TABLE transactions
    ADD CONSTRAINT transactions_timestamp_check CHECK (timestamp > '1970-01-01 00:00:01'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS transactions_hash ON transactions (hash);
CREATE INDEX IF NOT EXISTS transactions_timestamp ON transactions (timestamp);

-- 添加表注释
COMMENT ON TABLE transactions IS '存储区块链交易的详细信息';

-- 添加字段注释
COMMENT ON COLUMN transactions.guid IS '交易记录的唯一标识符';
COMMENT ON COLUMN transactions.block_hash IS '交易所在区块的哈希值';
COMMENT ON COLUMN transactions.block_number IS '区块编号，必须大于 0';
COMMENT ON COLUMN transactions.hash IS '交易的哈希值';
COMMENT ON COLUMN transactions.from_address IS '发起地址';
COMMENT ON COLUMN transactions.to_address IS '目标地址';
COMMENT ON COLUMN transactions.token_address IS '代币地址';
COMMENT ON COLUMN transactions.fee IS '手续费';
COMMENT ON COLUMN transactions.amount IS '交易金额';
COMMENT ON COLUMN transactions.status IS '交易状态，默认值为 0';
COMMENT ON COLUMN transactions.transaction_index IS '交易索引';
COMMENT ON COLUMN transactions.tx_type IS '交易类型，默认值为 0';
COMMENT ON COLUMN transactions.timestamp IS '时间';


-- 创建 deposits 表，如果表不存在
CREATE TABLE IF NOT EXISTS deposits
(
    guid              SERIAL PRIMARY KEY,           -- 唯一标识符
    block_hash        VARCHAR   NOT NULL,           -- 区块哈希值
    block_number      DECIMAL   NOT NULL,           -- 区块编号，必须大于 0
    hash              VARCHAR   NOT NULL,           -- 交易哈希值
    from_address      VARCHAR   NOT NULL,           -- 发起地址
    to_address        VARCHAR   NOT NULL,           -- 目标地址
    token_address     VARCHAR   NOT NULL,           -- 代币地址
    fee               DECIMAL   NOT NULL,           -- 手续费
    amount            DECIMAL   NOT NULL,           -- 交易金额
    status            SMALLINT  NOT NULL DEFAULT 0, -- 交易状态，默认值为 0
    transaction_index DECIMAL   NOT NULL,           -- 交易索引
    timestamp         TIMESTAMP NOT NULL            -- 时间戳，必须大于 0
);

-- 为现有表添加 CHECK 约束
ALTER TABLE deposits
    ADD CONSTRAINT deposits_block_number_check CHECK (block_number > 0);

ALTER TABLE deposits
    ADD CONSTRAINT deposits_timestamp_check CHECK (timestamp > '1970-01-01 00:00:01'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS deposits_hash ON deposits (hash);
CREATE INDEX IF NOT EXISTS deposits_timestamp ON deposits (timestamp);

-- 添加表注释
COMMENT ON TABLE deposits IS '存储存款交易的详细信息';

-- 添加字段注释
COMMENT ON COLUMN deposits.guid IS '存款记录的唯一标识符';
COMMENT ON COLUMN deposits.block_hash IS '存款所在区块的哈希值';
COMMENT ON COLUMN deposits.block_number IS '区块编号，必须大于 0';
COMMENT ON COLUMN deposits.hash IS '存款的哈希值';
COMMENT ON COLUMN deposits.from_address IS '发起地址';
COMMENT ON COLUMN deposits.to_address IS '目标地址';
COMMENT ON COLUMN deposits.token_address IS '代币地址';
COMMENT ON COLUMN deposits.fee IS '手续费';
COMMENT ON COLUMN deposits.amount IS '存款金额';
COMMENT ON COLUMN deposits.status IS '存款状态，默认值为 0';
COMMENT ON COLUMN deposits.transaction_index IS '交易索引';
COMMENT ON COLUMN deposits.timestamp IS '时间';


-- 创建 withdraws 表，如果表不存在
CREATE TABLE IF NOT EXISTS withdraws
(
    guid              SERIAL PRIMARY KEY,           -- 唯一标识符
    block_hash        VARCHAR   NOT NULL,           -- 区块哈希值
    block_number      DECIMAL   NOT NULL,           -- 区块编号，必须大于 0
    hash              VARCHAR   NOT NULL,           -- 交易哈希值
    from_address      VARCHAR   NOT NULL,           -- 发起地址
    to_address        VARCHAR   NOT NULL,           -- 目标地址
    token_address     VARCHAR   NOT NULL,           -- 代币地址
    fee               DECIMAL   NOT NULL,           -- 手续费
    amount            DECIMAL   NOT NULL,           -- 交易金额
    status            SMALLINT  NOT NULL DEFAULT 0, -- 交易状态，默认值为 0
    transaction_index DECIMAL   NOT NULL,           -- 交易索引
    timestamp         TIMESTAMP NOT NULL,           -- 时间
    tx_sign_hex       VARCHAR   NOT NULL            -- 交易签名的十六进制表示
);

-- 为现有表添加 CHECK 约束
ALTER TABLE withdraws
    ADD CONSTRAINT withdraws_block_number_check CHECK (block_number > 0);

ALTER TABLE withdraws
    ADD CONSTRAINT withdraws_timestamp_check CHECK (timestamp > '1970-01-01 00:00:01'::timestamp);

-- 创建索引以加速查询
CREATE INDEX IF NOT EXISTS withdraws_hash ON withdraws (hash);
CREATE INDEX IF NOT EXISTS withdraws_timestamp ON withdraws (timestamp);

-- 添加表注释
COMMENT ON TABLE withdraws IS '存储提款交易的详细信息';

-- 添加字段注释
COMMENT ON COLUMN withdraws.guid IS '提款记录的唯一标识符';
COMMENT ON COLUMN withdraws.block_hash IS '提款所在区块的哈希值';
COMMENT ON COLUMN withdraws.block_number IS '区块编号，必须大于 0';
COMMENT ON COLUMN withdraws.hash IS '提款的哈希值';
COMMENT ON COLUMN withdraws.from_address IS '发起地址';
COMMENT ON COLUMN withdraws.to_address IS '目标地址';
COMMENT ON COLUMN withdraws.token_address IS '代币地址';
COMMENT ON COLUMN withdraws.fee IS '手续费';
COMMENT ON COLUMN withdraws.amount IS '提款金额';
COMMENT ON COLUMN withdraws.status IS '提款状态，默认值为 0';
COMMENT ON COLUMN withdraws.transaction_index IS '交易索引';
COMMENT ON COLUMN withdraws.timestamp IS '时间';
COMMENT ON COLUMN withdraws.tx_sign_hex IS '交易签名的十六进制表示';





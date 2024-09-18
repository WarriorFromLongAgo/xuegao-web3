package ethtest

import (
	"context"
	"github.com/ethereum/go-ethereum/core/types"
	"log"
	"math/big"

	"github.com/ethereum/go-ethereum/ethclient"
)

func NewGethEthClient(url string) (*ethclient.Client, error) {
	// 连接到本地的 Geth 节点
	client, err := ethclient.Dial(url)
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum client: %v", err)
	}
	return client, nil
}

func NewDefaultGethEthClient() (*ethclient.Client, error) {
	return NewGethEthClient("http://localhost:8545")
}

// GetLatestBlockNumberByClient 获取最新的区块号
func GetLatestBlockNumberByClient(ctx context.Context, client *ethclient.Client) (uint64, error) {
	blockNumber, err := client.BlockNumber(ctx)
	if err != nil {
		log.Fatalf("Failed to get the latest block number: %v", err)
	}
	return blockNumber, nil
}

// GetBlockByBlockNumberByClient 获取最新的区块号
func GetBlockByBlockNumberByClient(ctx context.Context, client *ethclient.Client, number *big.Int) (*types.Block, error) {
	blockNumber, err := client.BlockByNumber(ctx, number)
	if err != nil {
		log.Fatalf("Failed to get the latest block number: %v", err)
	}
	return blockNumber, nil
}

// GetBlockByBlockNumberByClient 获取最新的区块号
func testdashdahdjkad(ctx context.Context, client *ethclient.Client, number *big.Int) (*types.Block, error) {
	//blockNumber, err := client.CodeAt(ctx, number)
	//if err != nil {
	//	log.Fatalf("Failed to get the latest block number: %v", err)
	//}

	//client.SuggestGasPrice()
	//client.SuggestGasTipCap()
	//client.EstimateGas()
	//client.BalanceAt()

	return nil, nil
}

//SuggestGasPrice 建议当前网络条件下合理的 gas 价格，用于提高交易被打包的概率。
//SuggestGasTipCap：建议当前网络条件下合理的 gas 小费，用于 EIP-1559 交易中，提高交易被优先处理的概率。
//EstimateGas：估算执行特定交易所需的 gas 量，用于确保交易能够成功执行。

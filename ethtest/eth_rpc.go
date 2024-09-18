package ethtest

import (
	"context"
	"fmt"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/core/types"
	"log"
	"math/big"
	"strconv"
	"strings"

	"github.com/ethereum/go-ethereum/rpc"
)

const (
	latest   = "latest"
	earliest = "earliest"
	pending  = "pending"
)

const (
	defaultUrl = "http://localhost:8545"
)

func NewGethEthRpc(url string) (*rpc.Client, error) {
	client, err := rpc.Dial(url)
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum rpc: %v", err)
	}
	return client, nil
}

func NewDefaultGethEthRpc() (*rpc.Client, error) {
	return NewGethEthRpc(defaultUrl)
}

func ChainIDByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	var result string
	err := client.CallContext(ctx, &result, "eth_chainId")
	if err != nil {
		log.Fatalf("Failed to ChainIDByRpc: %v", err)
	}
	// 将十六进制字符串转换为 *big.Int
	chainId := new(big.Int)
	// 去掉 "0x" 前缀
	result = strings.TrimPrefix(result, "0x")
	_, ok := chainId.SetString(result, 16)
	if !ok {
		return nil, fmt.Errorf("无法将 chainId 价格转换为大整数: %s", result)
	}
	return chainId, nil
}

// GetLatestBlockNumberByRpc 获取最新的区块号
func GetLatestBlockNumberByRpc(ctx context.Context, client *rpc.Client) (string, error) {
	var blockNumber string
	err := client.CallContext(ctx, &blockNumber, "eth_blockNumber")
	if err != nil {
		return "", err
	}
	return blockNumber, nil
}

// GetLatestBlockNumberReturnIntByRpc 获取最新的区块号
func GetLatestBlockNumberReturnIntByRpc(ctx context.Context, client *rpc.Client) (uint64, error) {
	blockNumberHex, err := GetLatestBlockNumberByRpc(ctx, client)
	if err != nil {
		return 0, err
	}
	blockNumberHex = strings.TrimPrefix(blockNumberHex, "0x")
	decimalValue, err := strconv.ParseInt(blockNumberHex, 16, 64)
	if err != nil {
		log.Fatalf("Failed to convert hex to decimal: %v", err)
	}
	return uint64(decimalValue), err
}

// GetBlockByBlockNumberByRpc 通过区块号从以太坊区块链中检索区块。
// 如果区块号为 nil，则检索最新的已知区块。
// detailFlag 参数指定是否在响应中包含完整的交易详细信息。
//
// 参数:
// - ctx: 请求的上下文，可用于取消请求。
// - client: 用于连接以太坊节点的 RPC 客户端。
// - number: 要检索的区块号。如果为 nil，则检索最新的区块。
// - detailFlag: 如果为 true，则在响应中包含完整的交易详细信息。
//
// 返回值:
// - *types.Block: 检索到的区块。
// - error: 如果请求失败，则返回错误。
func GetBlockByBlockNumberByRpc(ctx context.Context, client *rpc.Client, number *big.Int, detailFlag bool) (*types.Block, error) {
	var block *types.Block
	err := client.CallContext(ctx, &block, "eth_getBlockByNumber", toBlockNumArg(number), detailFlag)
	if err != nil {
		log.Fatalf("Failed to convert hex to decimal: %v", err)
	}
	return block, err
}

// toBlockNumArg 将区块号转换为 JSON-RPC 参数。
// 如果区块号为 nil，则返回 "latest" 以指示最新的区块。
//
// 参数:
// - number: 要转换的区块号。如果为 nil，则返回 "latest"。
//
// 返回值:
// - string: 表示区块号的 JSON-RPC 参数。
func toBlockNumArg(number *big.Int) string {
	if number == nil {
		return "latest"
	}
	return fmt.Sprintf("0x%x", number)
}

// PendingNonceAtByRpc 获取指定账户在待处理状态下的 nonce 值
// 这个 nonce 值表示该账户的下一笔交易应该使用的 nonce 值。
// 通过获取 pending nonce，你可以确保新交易的 nonce 值是正确的，避免重复交易或交易失败，并了解账户的最新交易活动和状态。
//
// 参数:
// - ctx: 请求的上下文，可用于取消请求。
// - client: 用于连接以太坊节点的 RPC 客户端。
// - account: 用户地址
//
// 返回值:
// - uint64: 检索到的区块 nonce 值
// - error: 如果请求失败，则返回错误。
func PendingNonceAtByRpc(ctx context.Context, client *rpc.Client, account common.Address) (uint64, error) {
	var result string
	err := client.CallContext(ctx, &result, "eth_getTransactionCount", account, "pending")
	if err != nil {
		log.Fatalf("Failed to PendingNonceAtByRpc: %v", err)
	}
	// 将十六进制字符串转换为 uint64
	result = strings.TrimPrefix(result, "0x")
	nonce, err := strconv.ParseUint(result, 16, 64)
	if err != nil {
		return 0, fmt.Errorf("无法将 nonce 转换为 uint64: %s", result)
	}
	return nonce, nil
}

func BalanceAtByPendingByRpc(ctx context.Context, client *rpc.Client, account common.Address) (*big.Int, error) {
	return balanceAtByRpc(ctx, client, account, pending)
}

func balanceAtByRpc(ctx context.Context, client *rpc.Client, account common.Address, status string) (*big.Int, error) {
	var result string
	err := client.CallContext(ctx, &result, "eth_getBalance", account, status)
	if err != nil {
		log.Fatalf("Failed to PendingNonceAtByRpc: %v", err)
	}
	// 将十六进制字符串转换为 *big.Int
	nonce := new(big.Int)
	// 去掉 "0x" 前缀
	result = strings.TrimPrefix(result, "0x")
	_, ok := nonce.SetString(result, 16)
	if !ok {
		return nil, fmt.Errorf("无法将 nonce 价格转换为大整数: %s", result)
	}
	return nonce, nil
}

// ConvertBalance 将余额从最小单位转换为人类可读的格式
func ConvertBalance(balance *big.Int, decimals int) *big.Float {
	// 创建一个 10 的 decimals 次方的 big.Int
	decimalsBigInt := new(big.Int).Exp(big.NewInt(10), big.NewInt(int64(decimals)), nil)

	// 将 balance 转换为 big.Float
	balanceFloat := new(big.Float).SetInt(balance)

	// 将 decimalsBigInt 转换为 big.Float
	decimalsFloat := new(big.Float).SetInt(decimalsBigInt)

	// 计算人类可读的余额
	humanReadableBalance := new(big.Float).Quo(balanceFloat, decimalsFloat)

	return humanReadableBalance
}

func GasPriceByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	var result string
	err := client.CallContext(ctx, &result, "eth_gasPrice")
	if err != nil {
		return nil, err
	}
	// 将十六进制字符串转换为 *big.Int
	gasPrice := new(big.Int)
	// 去掉 "0x" 前缀
	result = strings.TrimPrefix(result, "0x")
	_, ok := gasPrice.SetString(result, 16)
	if !ok {
		return nil, fmt.Errorf("无法将 gas 价格转换为大整数: %s", result)
	}
	return gasPrice, nil
}

func SuggestGasPriceByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	return GasPriceByRpc(ctx, client)
}

func MaxPriorityFeePerGasByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	var result string
	err := client.CallContext(ctx, &result, "eth_maxPriorityFeePerGas")
	if err != nil {
		return nil, err
	}
	// 将十六进制字符串转换为 *big.Int
	gasPrice := new(big.Int)
	// 去掉 "0x" 前缀
	result = strings.TrimPrefix(result, "0x")
	_, ok := gasPrice.SetString(result, 16)
	if !ok {
		return nil, fmt.Errorf("无法将 gas 价格转换为大整数: %s", result)
	}
	return gasPrice, nil
}

func SuggestGasTipCapByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	return MaxPriorityFeePerGasByRpc(ctx, client)
}

func NetworkIDByRpc(ctx context.Context, client *rpc.Client) (*big.Int, error) {
	version := new(big.Int)
	var ver string
	err := client.CallContext(ctx, &ver, "net_version")
	if err != nil {
		return nil, err
	}
	// 使用十进制基数
	_, ok := version.SetString(ver, 10)
	if !ok {
		return nil, fmt.Errorf("无法将网络版本转换为大整数: %s", ver)
	}
	return version, nil
}

func TransactionReceiptByTxHash(ctx context.Context, client *rpc.Client, txHash common.Hash) (*types.Receipt, error) {
	var result *types.Receipt
	err := client.CallContext(ctx, &result, "eth_getTransactionReceipt", txHash)
	if err == nil && result == nil {
		return nil, ethereum.NotFound
	}
	if err != nil {
		return nil, err
	}
	return result, err
}

func SendRawTransaction(ctx context.Context, client *rpc.Client, tx *types.Transaction) error {
	data, err := tx.MarshalBinary()
	if err != nil {
		return err
	}
	return client.CallContext(ctx, nil, "eth_sendRawTransaction", hexutil.Encode(data))
}

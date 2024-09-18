package ethtest

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"log"
	"testing"
	"time"
)

func Test_GetLatestBlockNumber(t *testing.T) {
	client, err := NewDefaultGethEthRpc()
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum rpc: %v", err)
	}

	// 设置超时时间为10秒
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	blockNumber, err := GetLatestBlockNumberByRpc(ctx, client)
	if err != nil {
		log.Fatalf("Failed to get the latest block number: %v", err)
	}
	fmt.Printf("Latest block number: %s\n", blockNumber)

	blockNumber2, err := GetLatestBlockNumberReturnIntByRpc(ctx, client)
	if err != nil {
		log.Fatalf("Failed to get the latest block number: %v", err)
	}
	fmt.Printf("Latest block number: %d\n", blockNumber2)
}

func Test_PendingNonceAtByRpc(t *testing.T) {
	client, err := NewDefaultGethEthRpc()
	if err != nil {
		log.Fatalf("Failed to NewDefaultGethEthRpc: %v", err)
	}

	// 设置超时时间为10秒
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	fromAddress := common.HexToAddress(FromAddress)
	pendingNonce, err := PendingNonceAtByRpc(ctx, client, fromAddress)
	if err != nil {
		log.Fatalf("Failed to PendingNonceAtByRpc: %v", err)
	}
	fmt.Printf("pendingNonce: %d\n", pendingNonce)

	balance, err := BalanceAtByPendingByRpc(ctx, client, fromAddress)
	if err != nil {
		log.Fatalf("Failed to BalanceAtByPendingByRpc: %v", err)
	}
	fmt.Printf("balance: %d\n", balance)

	// 转换余额
	humanReadableBalance := ConvertBalance(balance, 18)
	// 打印人类可读的余额
	fmt.Printf("人类可读的余额: %s\n", humanReadableBalance.Text('f', 18))
}

func Test_sendTx(t *testing.T) {
	client, err := NewDefaultGethEthRpc()
	if err != nil {
		log.Fatalf("Failed to connect to the Ethereum rpc: %v", err)
	}
	// 设置超时时间为10秒
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	fromAddress := common.HexToAddress(FromAddress)
	toAddress := common.HexToAddress(ToAddress)

	beforeLatestBlockNumber, _ := GetLatestBlockNumberByRpc(ctx, client)
	fmt.Printf("before latestBlockNumber: %s\n", beforeLatestBlockNumber)
	beforeFromBalance, _ := BalanceAtByPendingByRpc(ctx, client, fromAddress)
	beforeToBalance, _ := BalanceAtByPendingByRpc(ctx, client, toAddress)
	fmt.Printf("before from: %s\n", ConvertBalance(beforeFromBalance, 18).Text('f', 18))
	fmt.Printf("before from: %s\n", ConvertBalance(beforeToBalance, 18).Text('f', 18))

	hash, err := Send1Eth(ctx, client, fromAddress, toAddress)
	if err != nil {
		return
	}
	fmt.Printf("hash: %s\n\n", hash.Hex())

	var receiptByTxHash *types.Receipt
	for {
		receiptByTxHash, err = TransactionReceiptByTxHash(ctx, client, hash)
		if errors.Is(err, ethereum.NotFound) {
			continue
		}
		if err != nil {
			fmt.Println("receiptByTxHash err ", err)
			return
		}
		marshal, _ := json.Marshal(receiptByTxHash)
		fmt.Println("receiptByTxHash ", string(marshal))
		if receiptByTxHash != nil {
			break
		}
		time.Sleep(1 * time.Second) // 暂停一秒
	}
	if receiptByTxHash.Status == 1 {
		fmt.Println("Transaction successful")
	} else {
		fmt.Println("Transaction failed")
	}
	afterLatestBlockNumber, _ := GetLatestBlockNumberByRpc(ctx, client)
	fmt.Printf("before afterLatestBlockNumber: %s\n", afterLatestBlockNumber)
	afterFromBalance, _ := BalanceAtByPendingByRpc(ctx, client, fromAddress)
	afterToBalance, _ := BalanceAtByPendingByRpc(ctx, client, toAddress)
	fmt.Printf("after from: %s\n", ConvertBalance(afterFromBalance, 18).Text('f', 18))
	fmt.Printf("after to: %s\n", ConvertBalance(afterToBalance, 18).Text('f', 18))
}

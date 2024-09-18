package ethtest

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/rpc"
	"log"
	"math/big"
	"strings"
)

const (
	FromAddress = "0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f"
	FromPrvKey  = "0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97"
	ToAddress   = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720"
)

func Send1Eth(ctx context.Context, client *rpc.Client, from common.Address, to common.Address) (common.Hash, error) {
	// 去掉 "0x" 前缀
	privateKeyHex := strings.TrimPrefix(FromPrvKey, "0x")
	fromPrivateKey, err := crypto.HexToECDSA(privateKeyHex)
	if err != nil {
		log.Fatalf("Failed to load private key: %v", err)
	}
	// 从私钥对象中提取公钥
	//publicKey := fromPrivateKey.Public().(*ecdsa.PublicKey)
	// 将公钥转换为字节数组
	//publicKeyBytes := crypto.FromECDSAPub(publicKey)
	// 将公钥字节数组转换为十六进制字符串
	//publicKeyHex := hexutil.Encode(publicKeyBytes)

	chainId, err := ChainIDByRpc(ctx, client)
	if err != nil {
		return common.Hash{}, err
	}
	fmt.Println("chainId:", chainId)
	noncePending, err := PendingNonceAtByRpc(ctx, client, from)
	if err != nil {
		return common.Hash{}, err
	}

	amount := big.NewInt(1e18) // 1 ETH in wei
	gasLimit := uint64(21000)  // gas limit for a standard transaction

	gasPrice, err := SuggestGasPriceByRpc(ctx, client)
	if err != nil {
		return common.Hash{}, err
	}
	gasTipCap, err := SuggestGasTipCapByRpc(ctx, client)
	if err != nil {
		return common.Hash{}, err
	}

	tx := types.NewTx(&types.DynamicFeeTx{
		ChainID:   chainId, // Mainnet chain ID
		Nonce:     noncePending,
		GasTipCap: gasTipCap,
		GasFeeCap: gasPrice,
		Gas:       gasLimit,
		To:        &to,
		Value:     amount,
		Data:      nil,
	})

	//signedTx, err := types.SignTx(tx, types.NewEIP155Signer(chainId), fromPrivateKey)
	signedTx, err := types.SignTx(tx, types.NewLondonSigner(chainId), fromPrivateKey)
	if err != nil {
		log.Fatalf("Failed to sign transaction: %v", err)
	}

	signedTxByte, _ := json.Marshal(signedTx)
	fmt.Println("signedTx ", string(signedTxByte))

	err = SendRawTransaction(ctx, client, signedTx)

	if err != nil {
		log.Fatalf("Failed to send transaction: %v", err)
	}

	fmt.Printf("Transaction sent: %s", signedTx.Hash().Hex())
	return signedTx.Hash(), nil
}

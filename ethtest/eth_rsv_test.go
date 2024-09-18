package ethtest

import (
	"context"
	"crypto/ecdsa"
	"fmt"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/crypto"
	"log"
	"math/big"
	"strings"
	"testing"
)

func Test_rsv(t *testing.T) {
	client, err := NewDefaultGethEthClient()
	if err != nil {
		return
	}
	privateKeyHex := strings.TrimPrefix(FromPrvKey, "0x")
	fromPrivateKey, err := crypto.HexToECDSA(privateKeyHex)
	if err != nil {
		log.Fatal(err)
	}
	publicKey := fromPrivateKey.Public()
	publicKeyECDSA, ok := publicKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatalf("Failed to assert type: publicKey is not of type *ecdsa.PublicKey")
	}

	fromAddress := crypto.PubkeyToAddress(*publicKeyECDSA)
	nonce, err := client.PendingNonceAt(context.Background(), fromAddress)
	if err != nil {
		log.Fatalf("Failed to get nonce: %v", err)
	}

	value := big.NewInt(1000000000000000000) // 1 ether
	gasLimit := uint64(21000)                // in units
	gasTipCap, err := client.SuggestGasTipCap(context.Background())
	if err != nil {
		log.Fatalf("Failed to suggest gas tip cap: %v", err)
	}

	gasFeeCap, err := client.SuggestGasPrice(context.Background())
	if err != nil {
		log.Fatalf("Failed to suggest gas fee cap: %v", err)
	}
	chainID, err := client.NetworkID(context.Background())
	fmt.Println("chainID:", chainID)
	if err != nil {
		log.Fatalf("Failed to get chain ID: %v", err)
	}

	toAddress := common.HexToAddress(ToAddress)
	var data []byte
	tx := types.NewTx(&types.DynamicFeeTx{
		ChainID:   chainID,
		Nonce:     nonce,
		GasTipCap: gasTipCap,
		GasFeeCap: gasFeeCap,
		Gas:       gasLimit,
		To:        &toAddress,
		Value:     value,
		Data:      data,
	})

	signedTx, err := types.SignTx(tx, types.NewLondonSigner(chainID), fromPrivateKey)
	if err != nil {
		log.Fatalf("Failed to sign transaction: %v", err)
	}

	r, s, v := signedTx.RawSignatureValues()
	fmt.Printf("R: %s\nS: %s\nV: %d\n", r.String(), s.String(), v)

	// 解析签名并恢复公钥
	msgHash := signedTx.Hash()
	sig := make([]byte, 65)
	copy(sig[32-len(r.Bytes()):32], r.Bytes())
	copy(sig[64-len(s.Bytes()):64], s.Bytes())
	sig[64] = byte(v.Uint64() - 27) // 处理v值

	pubKey, err := crypto.Ecrecover(msgHash.Bytes(), sig)
	if err != nil {
		log.Fatalf("Failed to recover public key: %v", err)
	}

	fmt.Printf("Recovered Public Key: %x\n", pubKey)

	recoveredPubKey, err := crypto.UnmarshalPubkey(pubKey)
	if err != nil {
		log.Fatalf("Failed to unmarshal public key: %v", err)
	}

	recoveredAddress := crypto.PubkeyToAddress(*recoveredPubKey)
	fmt.Printf("Recovered Address: %s\n", recoveredAddress.Hex())
}

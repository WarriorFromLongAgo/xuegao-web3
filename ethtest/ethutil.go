package ethtest

import (
	"crypto/ecdsa"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/crypto"
	"log"
)

func PrvKeyToAddress(prvKey string) common.Address {
	// 加载私钥
	privateKey, err := crypto.HexToECDSA(prvKey)
	if err != nil {
		log.Fatalf("无法加载私钥: %v", err)
	}
	// 获取公钥
	publicKey := privateKey.Public()
	publicKeyECDSA, ok := publicKey.(*ecdsa.PublicKey)
	if !ok {
		log.Fatalf("无法转换公钥为 ECDSA")
	}
	// 获取发送地址
	fromAddress := crypto.PubkeyToAddress(*publicKeyECDSA)
	return fromAddress
}

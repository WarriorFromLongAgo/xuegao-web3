package cosmostest

import (
	"encoding/hex"
	"fmt"

	"github.com/cosmos/cosmos-sdk/crypto/keys/ed25519"
	"github.com/cosmos/cosmos-sdk/crypto/keys/secp256k1"
	sdk "github.com/cosmos/cosmos-sdk/types"
)

func generateECDSAAddress() {
	// 生成私钥
	privKey := secp256k1.GenPrivKey()

	// 获取公钥
	pubKey := privKey.PubKey()

	// 使用 Cosmos SDK 地址格式化
	address := sdk.AccAddress(pubKey.Address()).String()

	// 打印私钥
	fmt.Println("secp256k1 私钥 (hex):", hex.EncodeToString(privKey.Bytes()))
	// 打印公钥
	fmt.Println("secp256k1 公钥 (hex):", hex.EncodeToString(pubKey.Bytes()))
	fmt.Println("ECDSA (secp256k1) 地址:", address)
}

func generateEdDSAAddress() {
	// 生成私钥和公钥
	privKey := ed25519.GenPrivKey()
	// 获取公钥
	pubKey := privKey.PubKey()

	// 使用公钥生成 Cosmos SDK 地址
	address := sdk.AccAddress(pubKey.Address()).String()
	// 打印私钥
	fmt.Println("Ed25519 私钥 (hex):", hex.EncodeToString(privKey.Bytes()))
	// 打印公钥
	fmt.Println("Ed25519 公钥 (hex):", hex.EncodeToString(pubKey.Bytes()))
	fmt.Println("EdDSA (ed25519) 地址:", address)
}

package ethtest

import (
	"crypto/ecdsa"
	"fmt"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/tyler-smith/go-bip32"
	"github.com/tyler-smith/go-bip39"
)

// GenerateMnemonic
// 返回值:
// - string: 助记词
// - error: 如果没有错误，返回 nil。
func GenerateMnemonic() (string, error) {
	return GenerateMnemonicByBitSize(128)
}

// GenerateMnemonicByBitSize
// 入参：
// - int：bitSize 指定要生成的熵的位数。必须是32的倍数，并且在128到256之间（包括128和256）。
// 返回值:
// - string: 助记词
// - error: 如果没有错误，返回 nil。
func GenerateMnemonicByBitSize(bitSize int) (string, error) {
	entropy, err := bip39.NewEntropy(bitSize) // 128 bits entropy
	if err != nil {
		return "", err
	}
	fmt.Println("Array:", entropy)
	fmt.Printf("Array: %v\n", entropy)

	mnemonic, err := bip39.NewMnemonic(entropy)
	if err != nil {
		return "", err
	}
	fmt.Println("mnemonic ", mnemonic)
	return mnemonic, nil
}

// GeneratePrivateKeyFromMnemonic 从助记词生成私钥
func GeneratePrivateKeyFromMnemonic(mnemonic string) (*ecdsa.PrivateKey, error) {
	seed := bip39.NewSeed(mnemonic, "")
	masterKey, err := bip32.NewMasterKey(seed)
	if err != nil {
		return nil, err
	}
	privateKey, err := crypto.ToECDSA(masterKey.Key)
	if err != nil {
		return nil, err
	}
	return privateKey, nil
}

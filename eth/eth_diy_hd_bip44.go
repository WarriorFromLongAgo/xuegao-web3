package eth

import (
	"crypto/ecdsa"
	"crypto/sha256"
	"fmt"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/tyler-smith/go-bip39"
	"golang.org/x/crypto/pbkdf2"
	"strconv"
	"strings"
)

// GenerateEthereumPrivateKey 从给定的助记词和路径生成以太坊钱包私钥
func GenerateEthereumPrivateKeyByMnemonicAndPath(mnemonic, path string) (*ecdsa.PrivateKey, error) {
	// 生成种子
	seed := bip39.NewSeed(mnemonic, "1234567890")

	// 生成主密钥
	masterKey, err := crypto.ToECDSA(seed[:32])
	if err != nil {
		return nil, err
	}

	// 解析路径
	segments := strings.Split(path, "/")
	if len(segments) != 6 || segments[0] != "m" {
		return nil, fmt.Errorf("invalid path format")
	}

	// 派生路径
	key := masterKey
	for _, segment := range segments[1:] {
		hardened := strings.HasSuffix(segment, "'")
		indexStr := strings.TrimSuffix(segment, "'")
		index, err := strconv.Atoi(indexStr)
		if err != nil {
			return nil, err
		}
		if hardened {
			index += 0x80000000
		}
		key, err = deriveChildKey(key, uint32(index))
		if err != nil {
			return nil, err
		}
	}

	return key, nil
}

// deriveChildKey 使用PBKDF2派生子密钥
func deriveChildKey(parentKey *ecdsa.PrivateKey, index uint32) (*ecdsa.PrivateKey, error) {
	data := append(crypto.FromECDSA(parentKey), byte(index>>24), byte(index>>16), byte(index>>8), byte(index))
	derivedKey := pbkdf2.Key(data, nil, 2048, 32, sha256.New)
	return crypto.ToECDSA(derivedKey)
}

package ethtest

import (
	"crypto/ecdsa"
	"encoding/hex"
	"github.com/ethereum/go-ethereum/common"

	"github.com/ethereum/go-ethereum/crypto"
)

type Address struct {
	PrivateKey string `json:"private_key"`
	PublicKey  string `json:"public_key"`
	Address    string `json:"address"`
}

func CreatePriKeyByKeyPairs() (*ecdsa.PrivateKey, error) {
	prvKey, err := crypto.GenerateKey()
	if err != nil {
		return nil, err
	}
	return prvKey, err
}

func CreateAddressByKeyPairs() (*Address, error) {
	prvKey, err := crypto.GenerateKey()
	if err != nil {
		return nil, err
	}
	address := &Address{
		PrivateKey: hex.EncodeToString(crypto.FromECDSA(prvKey)),
		PublicKey:  hex.EncodeToString(crypto.FromECDSAPub(&prvKey.PublicKey)),
		Address:    crypto.PubkeyToAddress(prvKey.PublicKey).String(),
	}
	return address, nil
}

func CreateAddressByPrvKey(prvKey *ecdsa.PrivateKey) (*Address, error) {
	address := &Address{
		PrivateKey: hex.EncodeToString(crypto.FromECDSA(prvKey)),
		PublicKey:  hex.EncodeToString(crypto.FromECDSAPub(&prvKey.PublicKey)),
		Address:    crypto.PubkeyToAddress(prvKey.PublicKey).String(),
	}
	return address, nil
}

// CreateAddressFromPrivateKey 从 ECDSA 私钥生成地址。
// 参数:
// - prvKey: 指向 ECDSA 私钥的指针。
// 返回值:
// - string: 私钥的十六进制字符串表示。
// - string: 由公钥生成的以太坊地址。
// - error: 如果没有错误，返回 nil。
func CreateAddressFromPrivateKey(prvKey *ecdsa.PrivateKey) (string, string, error) {
	return hex.EncodeToString(prvKey.D.Bytes()), crypto.PubkeyToAddress(prvKey.PublicKey).String(), nil
}

func PublicKeyToAddressV2(publicKey ecdsa.PublicKey) (string, error) {
	return crypto.PubkeyToAddress(publicKey).String(), nil
}

func PublicKeyToAddress(publicKey string) (string, error) {
	publicKeyBytes, err := hex.DecodeString(publicKey)
	if err != nil {
		return "", nil
	}
	addressCommon := common.BytesToAddress(crypto.Keccak256(publicKeyBytes[1:])[12:])
	return addressCommon.String(), err
}

package eth

import (
	"fmt"
	"testing"
)

func Test_CreateAddressByKeyPairs(t *testing.T) {
	address, err := CreateAddressByKeyPairs()
	if err != nil {
		return
	}
	fmt.Println(address.PrivateKey)
	fmt.Println(address.PublicKey)
	fmt.Println(address.Address)
}

func Test_CreateAddressFromPrivateKey(t *testing.T) {
	prvKey, err := CreatePriKeyByKeyPairs()
	if err != nil {
		return
	}
	address, err := CreateAddressByPrvKey(prvKey)
	if err != nil {
		return
	}
	fmt.Println(address.PrivateKey)
	fmt.Println(address.PublicKey)
	fmt.Println(address.Address)

	add, s, err := CreateAddressFromPrivateKey(prvKey)
	if err != nil {
		return
	}
	fmt.Println(add)
	fmt.Println(s)
}

func Test_PublicKeyToAddressV2(t *testing.T) {
	prvKey, err := CreatePriKeyByKeyPairs()
	if err != nil {
		return
	}
	toAddress, err := PublicKeyToAddressV2(prvKey.PublicKey)
	if err != nil {
		return
	}
	fmt.Println(toAddress)
}

func Test_PublicKeyToAddress(t *testing.T) {
	address, err := CreateAddressByKeyPairs()
	if err != nil {
		return
	}
	fmt.Println(address.PrivateKey)
	fmt.Println(address.PublicKey)
	fmt.Println(address.Address)
	toAddress, err := PublicKeyToAddress(address.PublicKey)
	if err != nil {
		return
	}
	fmt.Println(toAddress)
}

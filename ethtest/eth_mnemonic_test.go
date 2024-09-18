package ethtest

import (
	"fmt"
	"testing"
)

func Test_GenerateMnemonic(t *testing.T) {
	mnemonic, err := GenerateMnemonic()
	if err != nil {
		return
	}
	fmt.Println(mnemonic)
}

func Test_GeneratePrivateKeyFromMnemonic(t *testing.T) {
	mnemonic, err := GenerateMnemonic()
	if err != nil {
		return
	}
	prvKey, err := GeneratePrivateKeyFromMnemonic(mnemonic)
	if err != nil {
		return
	}
	fmt.Println(prvKey)
	address, err := CreateAddressByPrvKey(prvKey)
	if err != nil {
		return
	}
	fmt.Println(address.PrivateKey)
	fmt.Println(address.PublicKey)
	fmt.Println(address.Address)
}

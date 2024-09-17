package eth

import (
	"fmt"
	"testing"
)

func Test_GenerateEthereumPrivateKey(t *testing.T) {
	//mnemonic, err := GenerateMnemonic()
	//mnemonic, err := GenerateMnemonic()
	//if err != nil {
	//	return
	//}
	mnemonic := "patch nut pudding season blood wisdom video license weasel daughter rally service"
	//m/44'/10'/0'/0/0
	path := "m/44'/10'/0'/0/0"
	prvKey, err := GenerateEthereumPrivateKeyByMnemonicAndPath(mnemonic, path)
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
}

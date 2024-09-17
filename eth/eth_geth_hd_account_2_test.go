package eth

import (
	"encoding/hex"
	"encoding/json"
	"fmt"
	"testing"
)

func Test_geth_hd_accnount_2(t *testing.T) {
	mnemonic := "patch nut pudding season blood wisdom video license weasel daughter rally service"
	password := "1234567890"
	//addressIndex := "0"
	path := "m/44'/10'/0'/0/0"
	fmt.Println("path:", path)

	//seed := GenerateSeed(mnemonic, password)
	//fmt.Println("seed:", seed)
	//fmt.Printf("seed: %v\n", seed)
	//fmt.Println("Seed (hex):", hex.EncodeToString(seed))

	seed2 := MnemonicToSeed(mnemonic, password)
	fmt.Println("seed2 (hex):", hex.EncodeToString(seed2))
	node, err := FromSeed(seed2, nil)
	if err != nil {
		t.Fatal(err)
	}
	marshal, err := json.Marshal(node)
	if err != nil {
		return
	}
	fmt.Println("node :", string(marshal))

	derivedNode, err := node.DerivePath(path)
	if err != nil {
		return
	}
	fmt.Println("Derived PrivateKey (hex):", derivedNode.PrivateKey)
	fmt.Println("Derived PublicKey (hex):", derivedNode.PublicKey)
	fmt.Println("Derived Mnemonic:", derivedNode.Mnemonic)

	//hd, err := FromSeed(seed2, "")
	//if err != nil {
	//	return
	//}
	//fmt.Println("hd ", hd.PrivateKey)
	//fmt.Println("hd PrivateKey (hex):", hex.EncodeToString(hd.PrivateKey))
	//
	//derivedNode, err := hd.derivePath(path)
	//if err != nil {
	//	return
	//}
	//fmt.Println("Derived PrivateKey (hex):", hex.EncodeToString(derivedNode.PrivateKey))
	//fmt.Println("Derived ChainCode (hex):", hex.EncodeToString(derivedNode.ChainCode))
	//fmt.Println("Derived Mnemonic:", derivedNode.Mnemonic)
	//
	//masterKey, _ := GenerateMasterKey(seed2)
	//derivationPath, _ := accounts.ParseDerivationPath(path)
	//prvKey, _ := DeriveKey(masterKey, derivationPath)
	//fmt.Println("Generated prvKey:", prvKey)
	//address, err := CreateAddressByPrvKey(prvKey)
	//if err != nil {
	//	return
	//}
	//fmt.Println(address.PrivateKey)
	//fmt.Println(address.PublicKey)
	//fmt.Println(address.Address)
}

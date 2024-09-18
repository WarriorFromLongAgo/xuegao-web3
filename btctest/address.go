package btctest

import (
	"github.com/btcsuite/btcd/btcec/v2"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
)

func generateP2PKHAddress() (string, error) {
	privKey, err := btcec.NewPrivateKey()
	if err != nil {
		return "", err
	}
	pubKey := privKey.PubKey()
	address, err := btcutil.NewAddressPubKey(pubKey.SerializeCompressed(), &chaincfg.MainNetParams)
	if err != nil {
		return "", err
	}
	return address.EncodeAddress(), nil
}

func generateP2SHAddress() (string, error) {
	p2pkhAddress, err := generateP2PKHAddress()
	if err != nil {
		return "", err
	}
	address, err := btcutil.DecodeAddress(p2pkhAddress, &chaincfg.MainNetParams)
	if err != nil {
		return "", err
	}
	p2shAddress, err := btcutil.NewAddressScriptHashFromHash(btcutil.Hash160(address.ScriptAddress()), &chaincfg.MainNetParams)
	if err != nil {
		return "", err
	}
	return p2shAddress.EncodeAddress(), nil
}

func generateBech32Address() (string, error) {
	privKey, err := btcec.NewPrivateKey()
	if err != nil {
		return "", err
	}
	pubKey := privKey.PubKey()
	witnessProg := btcutil.Hash160(pubKey.SerializeCompressed())
	address, err := btcutil.NewAddressWitnessPubKeyHash(witnessProg, &chaincfg.MainNetParams)
	if err != nil {
		return "", err
	}
	return address.EncodeAddress(), nil
}

func generateBech32AddressAndPrvKey() (string, *btcec.PrivateKey, error) {
	privKey, err := btcec.NewPrivateKey()
	if err != nil {
		return "", nil, err
	}
	pubKey := privKey.PubKey()
	witnessProg := btcutil.Hash160(pubKey.SerializeCompressed())
	address, err := btcutil.NewAddressWitnessPubKeyHash(witnessProg, &chaincfg.MainNetParams)
	if err != nil {
		return "", nil, err
	}
	return address.EncodeAddress(), privKey, nil
}

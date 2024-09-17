package eth

import (
	"errors"
	"github.com/ethereum/go-ethereum/accounts"
	"github.com/tyler-smith/go-bip39"
)

func createHDWallet() (accounts.Account, error) {
	//derivationPath, err := accounts.ParseDerivationPath("m/44'/10'/0'/0/0")
	//if err != nil {
	//	return accounts.Account{}, err
	//}

	return accounts.Account{}, nil
}

// NewFromMnemonic returns a new wallet from a BIP-39 mnemonic.
func NewFromMnemonic(mnemonic string, passOpt ...string) ([]byte, error) {
	if mnemonic == "" {
		return nil, errors.New("mnemonic is required")
	}

	if !bip39.IsMnemonicValid(mnemonic) {
		return nil, errors.New("mnemonic is invalid")
	}

	seed, err := NewSeedFromMnemonic(mnemonic, passOpt...)
	if err != nil {
		return nil, err
	}

	return seed, nil
}

// NewSeedFromMnemonic returns a BIP-39 seed based on a BIP-39 mnemonic.
func NewSeedFromMnemonic(mnemonic string, passOpt ...string) ([]byte, error) {
	if mnemonic == "" {
		return nil, errors.New("mnemonic is required")
	}

	password := ""
	if len(passOpt) > 0 {
		password = passOpt[0]
	}

	return bip39.NewSeedWithErrorChecking(mnemonic, password)
}

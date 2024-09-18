package ethtest

import (
	"errors"
	"github.com/tyler-smith/go-bip39"
)

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

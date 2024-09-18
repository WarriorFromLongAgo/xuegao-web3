package btctest

import "testing"

func TestGenerateP2PKHAddress(t *testing.T) {
	address, err := generateP2PKHAddress()
	if err != nil {
		t.Fatalf("生成P2PKH地址失败: %v", err)
	}
	if len(address) == 0 {
		t.Fatalf("生成的P2PKH地址为空")
	}
	t.Logf("生成的P2PKH地址: %s", address)
}

func TestGenerateP2SHAddress(t *testing.T) {
	address, err := generateP2SHAddress()
	if err != nil {
		t.Fatalf("生成P2SH地址失败: %v", err)
	}
	if len(address) == 0 {
		t.Fatalf("生成的P2SH地址为空")
	}
	t.Logf("生成的P2SH地址: %s", address)
}

func TestGenerateBech32Address(t *testing.T) {
	address, err := generateBech32Address()
	if err != nil {
		t.Fatalf("生成Bech32地址失败: %v", err)
	}
	if len(address) == 0 {
		t.Fatalf("生成的Bech32地址为空")
	}
	t.Logf("生成的Bech32地址: %s", address)
}

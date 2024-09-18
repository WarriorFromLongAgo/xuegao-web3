package btctest

import (
	"bytes"
	"encoding/hex"
	"fmt"
	"github.com/btcsuite/btcd/btcutil"
	"log"
	"testing"

	"github.com/btcsuite/btcd/chaincfg"
	"github.com/btcsuite/btcd/chaincfg/chainhash"
	"github.com/btcsuite/btcd/txscript"
	"github.com/btcsuite/btcd/wire"
)

func TestSendTx(t *testing.T) {
	// 生成Bech32地址
	bech32Address, privKey, err := generateBech32AddressAndPrvKey()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("生成的Bech32地址:", bech32Address)

	// 发送者地址（假设为生成的Bech32地址）
	senderAddress, err := btcutil.DecodeAddress(bech32Address, &chaincfg.MainNetParams)
	if err != nil {
		log.Fatal(err)
	}

	// 接收者地址（替换为实际接收者地址）
	receiverAddress, err := btcutil.DecodeAddress("receiver-bech32-address", &chaincfg.MainNetParams)
	if err != nil {
		log.Fatal(err)
	}

	// 创建交易
	tx := wire.NewMsgTx(wire.TxVersion)

	// 添加输入（UTXO）
	prevTxHash, err := chainhash.NewHashFromStr("previous-transaction-hash")
	if err != nil {
		log.Fatal(err)
	}
	outPoint := wire.NewOutPoint(prevTxHash, 0) // 假设使用第一个输出
	txIn := wire.NewTxIn(outPoint, nil, nil)
	tx.AddTxIn(txIn)

	// 添加输出
	amount, err := btcutil.NewAmount(0.001) // 发送0.001 BTC
	if err != nil {
		log.Fatal(err)
	}
	pkScript, err := txscript.PayToAddrScript(receiverAddress)
	if err != nil {
		log.Fatal(err)
	}
	txOut := wire.NewTxOut(int64(amount), pkScript)
	tx.AddTxOut(txOut)

	// 签名交易
	sourcePkScript, err := txscript.PayToAddrScript(senderAddress)
	if err != nil {
		log.Fatal(err)
	}
	// 创建一个PrevOutputFetcher
	prevOutputFetcher := txscript.NewMultiPrevOutFetcher(map[wire.OutPoint]*wire.TxOut{
		*outPoint: wire.NewTxOut(int64(amount), sourcePkScript),
	})

	sigHashes := txscript.NewTxSigHashes(tx, prevOutputFetcher)

	sigScript, err := txscript.WitnessSignature(tx, sigHashes, 0, int64(amount), sourcePkScript, txscript.SigHashAll, privKey, true)
	if err != nil {
		log.Fatal(err)
	}
	tx.TxIn[0].Witness = sigScript

	// 序列化交易
	var buf bytes.Buffer
	err = tx.Serialize(&buf)
	if err != nil {
		return
	}
	txHex := hex.EncodeToString(buf.Bytes())

	fmt.Printf("Signed transaction: %s\n", txHex)
}

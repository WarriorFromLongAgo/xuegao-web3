package go_address

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"github.com/btcsuite/btcd/btcec/v2/schnorr"
	"log"
	"strings"
	"testing"

	"github.com/btcsuite/btcd/btcec/v2"
	"github.com/btcsuite/btcd/btcutil"
	"github.com/btcsuite/btcd/chaincfg"
	"github.com/btcsuite/btcd/txscript"
)

func Test_p2pkh___pay_to_public_key_hash(t *testing.T) {
	var addr btcutil.Address
	var err error

	// 创建一个 256 位的随机数作为私钥。
	// 使用椭圆曲线数字签名算法（ECDSA）基于私钥生成对应的公钥。
	privateKey, err := btcec.NewPrivateKey()
	if err != nil {
		t.Fatalf("生成私钥失败: %v", err)
	}

	// 获取公钥
	pubKey := privateKey.PubKey()

	// 创建付款地址
	// 对公钥进行 SHA256 哈希运算。
	// 再对得到的结果进行 RIPEMD160 哈希运算，得到 160 位的公钥哈希。
	pubKeyHash := btcutil.Hash160(pubKey.SerializeCompressed())
	addr, err = btcutil.NewAddressPubKeyHash(pubKeyHash, &chaincfg.MainNetParams)
	if err != nil {
		t.Fatalf("创建地址失败: %v", err)
	}

	address := addr.EncodeAddress()
	t.Logf("成功生成以1开头的比特币地址: %s", address)

	// 检查地址是否以"1"开头
	if strings.HasPrefix(address, "1") {
		t.Logf("成功生成以1开头的比特币地址: %s", address)
		return
	}
}

func Test_p2sh___pay_to_script_hash_p2pkh(t *testing.T) {
	var addr btcutil.Address
	var err error

	// 创建一个 256 位的随机数作为私钥。
	// 使用椭圆曲线数字签名算法（ECDSA）基于私钥生成对应的公钥。
	privateKey, err := btcec.NewPrivateKey()
	if err != nil {
		t.Fatalf("生成私钥失败: %v", err)
	}

	// 获取公钥
	pubKey := privateKey.PubKey()

	// 创建一个 P2PKH 脚本
	pubKeyHash := btcutil.Hash160(pubKey.SerializeCompressed())
	script, err := txscript.NewScriptBuilder().
		// 复制栈顶元素（公钥）
		// 76 - OP_DUP 操作码的十六进制表示。
		AddOp(txscript.OP_DUP).
		// 对栈顶元素（公钥）进行 HASH160 运算（先 SHA256，然后 RIPEMD160）。
		// a9 OP_HASH160 操作码的十六进制表示。
		AddOp(txscript.OP_HASH160).

		// 14 这不是操作码，而是表示接下来有 20 字节（0x14 in hex = 20 in decimal）的数据。
		// c9938440fb4fc5d55e74dd879c10d44c36f55f8e - 这是 20 字节的公钥哈希。在反汇编脚本中直接显示为十六进制。
		// 将公钥哈希（接收方的地址）推入栈中。
		AddData(pubKeyHash).
		// 比较栈顶的两个元素（计算得到的公钥哈希和脚本中的公钥哈希）是否相等。如果不相等，脚本立即失败。
		// 88 - OP_EQUALVERIFY 操作码的十六进制表示。
		AddOp(txscript.OP_EQUALVERIFY).
		// 使用公钥验证交易签名。
		// ac - OP_CHECKSIG 操作码的十六进制表示。
		AddOp(txscript.OP_CHECKSIG).
		Script()
	// 76        - OP_DUP
	// a9        - OP_HASH160
	// 14        - (表示接下来有 20 字节数据，在反汇编中省略)
	// c9938440fb4fc5d55e74dd879c10d44c36f55f8e - (20 字节的公钥哈希)
	// 88        - OP_EQUALVERIFY
	// ac        - OP_CHECKSIG
	if err != nil {
		t.Fatalf("创建脚本失败: %v", err)
	}

	decodeScript(t, script)

	// 创建 P2SH 地址
	// 对脚本进行 SHA256 哈希运算，然后进行 RIPEMD160 哈希运算
	scriptHash := btcutil.Hash160(script)
	addr, err = btcutil.NewAddressScriptHash(scriptHash, &chaincfg.MainNetParams)
	if err != nil {
		t.Fatalf("创建地址失败: %v", err)
	}

	address := addr.EncodeAddress()
	t.Logf("成功生成 P2SH 比特币地址: %s", address)

	// 检查地址是否以"3"开头
	if strings.HasPrefix(address, "3") {
		t.Logf("成功生成以3开头的 P2SH 比特币地址: %s", address)
		return
	} else {
		t.Fatalf("生成的地址不是以3开头的 P2SH 地址: %s", address)
	}
}

func decodeScript(t *testing.T, script []byte) {
	var scriptHex strings.Builder
	scriptHex.WriteString("脚本的原始十六进制表示: ")
	for i, b := range script {
		if i > 0 && i%8 == 0 {
			scriptHex.WriteString(" ")
		}
		scriptHex.WriteString(fmt.Sprintf("%02x", b))
	}
	t.Logf("%s", scriptHex.String())

	// 使用 DisasmString 函数反汇编脚本
	disasm, err := txscript.DisasmString(script)
	if err != nil {
		t.Fatalf("反汇编脚本失败: %v", err)
	}
	t.Logf("反汇编后的脚本: %s", disasm)
}

func Test_p2sh___multisig(t *testing.T) {
	// 创建三个私钥
	privateKey1, _ := btcec.NewPrivateKey()
	privateKey2, _ := btcec.NewPrivateKey()
	privateKey3, _ := btcec.NewPrivateKey()

	// 获取公钥
	pubKey1 := privateKey1.PubKey()
	pubKey2 := privateKey2.PubKey()
	pubKey3 := privateKey3.PubKey()

	// 获取公钥并转换为 AddressPubKey
	compressedPubKey1, _ := btcutil.NewAddressPubKey(pubKey1.SerializeCompressed(), &chaincfg.MainNetParams)
	compressedPubKey2, _ := btcutil.NewAddressPubKey(pubKey2.SerializeCompressed(), &chaincfg.MainNetParams)
	compressedPubKey3, _ := btcutil.NewAddressPubKey(pubKey3.SerializeCompressed(), &chaincfg.MainNetParams)

	// 创建多重签名赎回脚本
	redeemScript, err := txscript.MultiSigScript([]*btcutil.AddressPubKey{compressedPubKey1, compressedPubKey2, compressedPubKey3}, 2)
	if err != nil {
		t.Fatal(err)
	}
	// 反汇编赎回脚本
	redeemScriptDisasm, err := txscript.DisasmString(redeemScript)
	if err != nil {
		t.Fatal(err)
	}

	// 创建 P2SH 地址
	addr, err := btcutil.NewAddressScriptHash(redeemScript, &chaincfg.MainNetParams)
	if err != nil {
		t.Fatal(err)
	}

	// 创建 P2SH 脚本公钥
	scriptPubKey, err := txscript.PayToAddrScript(addr)
	if err != nil {
		t.Fatal(err)
	}

	// 打印结果
	fmt.Printf("赎回脚本: %x\n", redeemScript)
	fmt.Printf("赎回脚本 (反汇编): %s\n", redeemScriptDisasm)
	fmt.Printf("P2SH 地址: %s\n", addr.EncodeAddress())
	fmt.Printf("P2SH 脚本公钥 (hex): %s\n", hex.EncodeToString(scriptPubKey))

	// 反汇编 P2SH 脚本公钥
	disasm, err := txscript.DisasmString(scriptPubKey)
	if err != nil {
		t.Fatal(err)
	}
	fmt.Printf("P2SH 脚本公钥 (反汇编): %s\n", disasm)
}

func Test_p2sh___multisig_2(t *testing.T) {
	// 创建三个私钥
	privateKey1, _ := btcec.NewPrivateKey()
	privateKey2, _ := btcec.NewPrivateKey()
	privateKey3, _ := btcec.NewPrivateKey()

	// 获取公钥
	pubKey1 := privateKey1.PubKey()
	pubKey2 := privateKey2.PubKey()
	pubKey3 := privateKey3.PubKey()

	// 手动构建赎回脚本
	// 2 02fbba2860780251d5af1d8f6ae26de5f11b20aaee38d992cb17df63ec9b3d4435 0287bbb08c2ebc9597f1c28a0f64fc138b7a202136f55aa95ca9d4d4b330aa3d3c 0398712ddf004eb6303666d925559eaa0921ed155fb04d2f44ee4c7787552326b0 3 OP_CHECKMULTISIG
	// 2                                                                   // OP_2: 需要2个签名
	// 032c386ae3d0c0c318d051e6564feb9ba0351700445ec9b74a8c4724bf65b662af  // 公钥1
	// 02fb7085bf5fef2be87fd971cd9f4a9de93b224b24b3d95443f51203496fda7752  // 公钥2
	// 02fb9e66d64197c9d34acbf0f92b480dda8819bbee4ec2752f4841cdb70c684b0c  // 公钥3
	// 3                                                                   // OP_3: 总共3个公钥
	// OP_CHECKMULTISIG                                                    // 执行多重签名检查
	builder := txscript.NewScriptBuilder()
	builder.AddOp(txscript.OP_2) // 需要2个签名
	builder.AddData(pubKey1.SerializeCompressed())
	builder.AddData(pubKey2.SerializeCompressed())
	builder.AddData(pubKey3.SerializeCompressed())
	builder.AddOp(txscript.OP_3) // 总共3个公钥
	builder.AddOp(txscript.OP_CHECKMULTISIG)

	redeemScript, err := builder.Script()
	if err != nil {
		log.Fatal(err)
	}

	// 创建 P2SH 地址
	addr, err := btcutil.NewAddressScriptHash(redeemScript, &chaincfg.MainNetParams)
	if err != nil {
		log.Fatal(err)
	}

	// 手动构建 P2SH 脚本公钥
	p2shBuilder := txscript.NewScriptBuilder()
	p2shBuilder.AddOp(txscript.OP_HASH160)
	p2shBuilder.AddData(addr.ScriptAddress())
	p2shBuilder.AddOp(txscript.OP_EQUAL)

	scriptPubKey, err := p2shBuilder.Script()
	if err != nil {
		log.Fatal(err)
	}

	// 打印结果
	fmt.Printf("赎回脚本 (hex): %s\n", hex.EncodeToString(redeemScript))
	fmt.Printf("赎回脚本 (反汇编):\n%s\n", disasmScript(redeemScript))
	fmt.Printf("P2SH 地址: %s\n", addr.EncodeAddress())
	fmt.Printf("P2SH 脚本公钥 (hex): %s\n", hex.EncodeToString(scriptPubKey))
	fmt.Printf("P2SH 脚本公钥 (反汇编):\n%s\n", disasmScript(scriptPubKey))
}

func disasmScript(script []byte) string {
	disasm, err := txscript.DisasmString(script)
	if err != nil {
		return fmt.Sprintf("反汇编错误: %v", err)
	}
	return disasm
}

func Test_p2wsh_1(t *testing.T) {
	// 生成一个新的私钥
	privateKey, err := btcec.NewPrivateKey()
	if err != nil {
		log.Fatal(err)
	}

	// 获取对应的公钥
	publicKey := privateKey.PubKey()

	// 将公钥转换为压缩格式
	compressedPubKey := publicKey.SerializeCompressed()

	// 创建 P2WPKH 地址
	witnessAddress, err := btcutil.NewAddressWitnessPubKeyHash(btcutil.Hash160(compressedPubKey), &chaincfg.MainNetParams)
	if err != nil {
		log.Fatal(err)
	}

	// 打印 P2WPKH 地址
	fmt.Printf("P2WPKH 地址: %s\n", witnessAddress.EncodeAddress())

	// 打印私钥（仅用于演示，实际使用时请妥善保管）
	fmt.Printf("私钥 (十六进制): %x\n", privateKey.Serialize())
}

func Test_p2wsh_2(t *testing.T) {
	// 生成三个密钥对
	privKey1, _ := btcec.NewPrivateKey()
	privKey2, _ := btcec.NewPrivateKey()
	privKey3, _ := btcec.NewPrivateKey()

	pubKey1 := privKey1.PubKey()
	pubKey2 := privKey2.PubKey()
	pubKey3 := privKey3.PubKey()

	// 构建 2-of-3 多重签名赎回脚本
	builder := txscript.NewScriptBuilder()

	builder.AddOp(txscript.OP_2) // 需要 2 个签名
	builder.AddData(pubKey1.SerializeCompressed())
	builder.AddData(pubKey2.SerializeCompressed())
	builder.AddData(pubKey3.SerializeCompressed())
	builder.AddOp(txscript.OP_3) // 共 3 个公钥
	builder.AddOp(txscript.OP_CHECKMULTISIG)

	redeemScript, _ := builder.Script()

	// 计算赎回脚本的 SHA256 哈希
	witnessScriptHash := sha256.Sum256(redeemScript)

	// 生成 P2WSH 地址
	addr, _ := btcutil.NewAddressWitnessScriptHash(witnessScriptHash[:], &chaincfg.MainNetParams)

	fmt.Printf("多签 P2WSH 地址: %s\n", addr.EncodeAddress())
	fmt.Printf("赎回脚本 (hex): %s\n", hex.EncodeToString(redeemScript))

	// 输出脚本的操作码和数据
	fmt.Println("赎回脚本 (解析):")
	disasm, _ := txscript.DisasmString(redeemScript)
	fmt.Println(disasm)

	// 创建 P2WSH 输出脚本
	p2wshScript, _ := txscript.PayToAddrScript(addr)
	fmt.Printf("P2WSH 输出脚本 (hex): %s\n", hex.EncodeToString(p2wshScript))

	// 输出 P2WSH 脚本的操作码和数据
	fmt.Println("P2WSH 输出脚本 (解析):")
	p2wshDisasm, _ := txscript.DisasmString(p2wshScript)
	fmt.Println(p2wshDisasm)
}

func Test_p2tr_1(t *testing.T) {
	// 1. 生成私钥
	privateKey, err := btcec.NewPrivateKey()
	if err != nil {
		log.Fatal(err)
	}

	// 2. 获取内部公钥
	internalKey := privateKey.PubKey()

	// 3. 创建 Taproot 输出密钥
	taprootKey := txscript.ComputeTaprootKeyNoScript(internalKey)

	// 4. 创建 P2TR 地址
	address, err := btcutil.NewAddressTaproot(schnorr.SerializePubKey(taprootKey), &chaincfg.MainNetParams)
	if err != nil {
		log.Fatal(err)
	}

	// 5. 编码地址
	scriptAddress := address.ScriptAddress()
	encodedAddress := address.EncodeAddress()

	// 打印结果
	fmt.Printf("P2TR 脚本地址: %s\n", hex.EncodeToString(scriptAddress))
	fmt.Printf("P2TR 地址: %s\n", encodedAddress)
	fmt.Printf("私钥 (十六进制): %x\n", privateKey.Serialize())
}

func Test_p2tr_2(t *testing.T) {
	// 生成三个密钥对
	privKey1, _ := btcec.NewPrivateKey()
	privKey2, _ := btcec.NewPrivateKey()
	privKey3, _ := btcec.NewPrivateKey()

	pubKey1 := privKey1.PubKey()
	pubKey2 := privKey2.PubKey()
	pubKey3 := privKey3.PubKey()

	// 创建 2-of-3 多签脚本
	multiSigScript, _ := txscript.NewScriptBuilder().
		AddOp(txscript.OP_2).
		AddData(schnorr.SerializePubKey(pubKey1)).
		AddData(schnorr.SerializePubKey(pubKey2)).
		AddData(schnorr.SerializePubKey(pubKey3)).
		AddOp(txscript.OP_3).
		AddOp(txscript.OP_CHECKMULTISIG).
		Script()

	// 创建 Taproot 脚本树
	tapLeaf := txscript.NewBaseTapLeaf(multiSigScript)
	tapScriptTree := txscript.AssembleTaprootScriptTree(tapLeaf)

	// 创建内部密钥（这里我们使用第一个公钥作为内部密钥）
	internalKey := pubKey1

	// 计算 Taproot 输出密钥
	hash := tapScriptTree.RootNode.TapHash()
	tapRootKey := txscript.ComputeTaprootOutputKey(internalKey, hash.CloneBytes())

	// 创建 P2TR 地址
	addr, _ := btcutil.NewAddressTaproot(schnorr.SerializePubKey(tapRootKey), &chaincfg.MainNetParams)

	// 创建 P2TR 输出脚本
	p2trScript, _ := txscript.PayToAddrScript(addr)

	fmt.Printf("P2TR 多签地址: %s\n", addr.EncodeAddress())
	fmt.Printf("P2TR 输出脚本 (hex): %s\n", hex.EncodeToString(p2trScript))

	// 详细解析 P2TR 输出脚本
	fmt.Println("P2TR 输出脚本 (详细解析):")
	for i, b := range p2trScript {
		if i == 0 {
			fmt.Printf("OP_%d ", b) // 通常是 OP_1 (0x51)
		} else {
			fmt.Printf("%02x", b) // 输出密钥的十六进制表示
		}
	}
	fmt.Println()

	// 解析并显示脚本内容
	disasm, _ := txscript.DisasmString(p2trScript)
	fmt.Println("P2TR 输出脚本 (标准解析):")
	fmt.Println(disasm)

	// 显示多签脚本内容
	multiSigDisasm, _ := txscript.DisasmString(multiSigScript)
	fmt.Println("多签脚本 (解析):")
	fmt.Println(multiSigDisasm)
}

package ethtest

import (
	"crypto/ecdsa"
	"crypto/elliptic"
	"crypto/hmac"
	"crypto/sha256"
	"crypto/sha512"
	"encoding/hex"
	"errors"
	"fmt"
	"github.com/ethereum/go-ethereum/accounts"
	"github.com/ethereum/go-ethereum/crypto"
	"golang.org/x/crypto/pbkdf2"
	"golang.org/x/crypto/ripemd160"
	"golang.org/x/crypto/sha3"
	"hash"
	"log"
	"math/big"
	"strconv"
	"strings"
)

// Normalize normalizes the input string by converting it to NFC form.
func Normalize(input string) string {
	return strings.TrimSpace(input)
}

// Salt generates the salt for PBKDF2.
func Salt(password string) string {
	return "mnemonic" + password
}

// MnemonicToSeed generates a seed from a mnemonic and password using PBKDF2.
func MnemonicToSeed(mnemonic, password string) []byte {
	if mnemonic == "" {
		log.Fatal("Must have mnemonic")
	}

	normalizedMnemonic := Normalize(mnemonic)
	normalizedPassword := Normalize(password)

	mnemonicBuffer := []byte(normalizedMnemonic)
	saltBuffer := []byte(Salt(normalizedPassword))

	seed := pbkdf2.Key(mnemonicBuffer, saltBuffer, 2048, 64, sha512.New)
	return seed
}

const (
	defaultPath  = "m/44'/60'/0'/0/0"
	HardenedBit  = 0x80000000
	MasterSecret = "Bitcoin seed"
)

type Mnemonic struct {
	Phrase string
	Path   string
	Locale string
}

type HDNode struct {
	PrivateKey        string
	PublicKey         string
	Fingerprint       string
	ParentFingerprint string
	Address           string
	Mnemonic          *Mnemonic
	Path              string
	ChainCode         string
	Index             int
	Depth             int
}

func NewHDNode(privateKey, publicKey, parentFingerprint, chainCode string, index, depth int, mnemonicOrPath interface{}) (*HDNode, error) {
	if privateKey == "" && publicKey == "" {
		return nil, errors.New("privateKey or publicKey must be provided")
	}

	node := &HDNode{
		PrivateKey:        privateKey,
		PublicKey:         publicKey,
		ParentFingerprint: parentFingerprint,
		ChainCode:         chainCode,
		Index:             index,
		Depth:             depth,
	}

	if mnemonic, ok := mnemonicOrPath.(Mnemonic); ok {
		node.Mnemonic = &mnemonic
		node.Path = mnemonic.Path
	} else if path, ok := mnemonicOrPath.(string); ok {
		node.Path = path
	}

	// Compute fingerprint and address
	node.Fingerprint = computeFingerprint(node.PublicKey)
	node.Address = computeAddress(node.PublicKey)

	return node, nil
}

func (node *HDNode) ExtendedKey() (string, error) {
	if node.Depth >= 256 {
		return "", errors.New("depth too large")
	}

	// Implement base58check encoding and other necessary steps
	// ...

	return "", nil
}

func (node *HDNode) Neuter() (*HDNode, error) {
	return NewHDNode("", node.PublicKey, node.ParentFingerprint, node.ChainCode, node.Index, node.Depth, node.Path)
}

func (node *HDNode) Derive(index int) (*HDNode, error) {
	if index > 0xffffffff {
		return nil, errors.New("invalid index")
	}

	// Base path
	path := node.Path
	if path != "" {
		path += fmt.Sprintf("/%d", index&^HardenedBit)
	}

	data := make([]byte, 37)

	if index&HardenedBit != 0 {
		if node.PrivateKey == "" {
			return nil, errors.New("cannot derive child of neutered node")
		}

		// Data = 0x00 || ser_256(k_par)
		copy(data[1:], hexToBytes(node.PrivateKey))

		// Hardened path
		if path != "" {
			path += "'"
		}
	} else {
		// Data = ser_p(point(k_par))
		copy(data, hexToBytes(node.PublicKey))
	}

	// Data += ser_32(i)
	for i := 24; i >= 0; i -= 8 {
		data[33+(i>>3)] = byte((index >> (24 - i)) & 0xff)
	}

	I := computeHmacSha512(node.ChainCode, data)
	IL := I[:32]
	IR := I[32:]

	var ki, Ki string

	if node.PrivateKey != "" {
		ki = addPrivateKeys(node.PrivateKey, hex.EncodeToString(IL))
	} else {
		Ki = addPublicKeys(node.PublicKey, hex.EncodeToString(IL))
	}

	var mnemonicOrPath interface{} = path

	if node.Mnemonic != nil {
		mnemonicOrPath = Mnemonic{
			Phrase: node.Mnemonic.Phrase,
			Path:   path,
			Locale: node.Mnemonic.Locale,
		}
	}

	return NewHDNode(ki, Ki, node.Fingerprint, hex.EncodeToString(IR), index, node.Depth+1, mnemonicOrPath)
}

func (node *HDNode) DerivePath(path string) (*HDNode, error) {
	components := splitPath(path)

	if len(components) == 0 || (components[0] == "m" && node.Depth != 0) {
		return nil, errors.New("invalid path - " + path)
	}

	if components[0] == "m" {
		components = components[1:]
	}

	result := node
	for _, component := range components {
		if component[len(component)-1] == '\'' {
			index := parseIndex(component[:len(component)-1])
			if index >= HardenedBit {
				return nil, errors.New("invalid path index - " + component)
			}
			result, _ = result.Derive(HardenedBit + index)
		} else {
			index := parseIndex(component)
			if index >= HardenedBit {
				return nil, errors.New("invalid path index - " + component)
			}
			result, _ = result.Derive(index)
		}
	}

	return result, nil
}

func FromSeed(seed []byte, mnemonic *Mnemonic) (*HDNode, error) {
	if len(seed) < 16 || len(seed) > 64 {
		return nil, errors.New("invalid seed")
	}

	I := computeHmac(sha512.New, []byte("Bitcoin seed"), seed)

	prvKey := hex.EncodeToString(I[:32])
	fmt.Println("FromSeed prvKey :", prvKey)
	signingKey, _ := NewSigningKey(prvKey)
	pubKey := hex.EncodeToString(signingKey.compressedPublicKey)
	fmt.Println("FromSeed signingKey.publicKey :", PublicKeyToString(signingKey.publicKey))
	fmt.Println("FromSeed pubKey :", pubKey)

	fingerprint := hexDataSlice(ripemd160Hash(sha256Hash(signingKey.compressedPublicKey)), 0, 4)

	keccakHash := keccak256(signingKey.compressedPublicKey[1:])
	address := hexDataSlice(keccakHash, len(keccakHash)-20, 20)
	chainCode := hex.EncodeToString(I[32:])

	return &HDNode{
		PrivateKey:        prvKey,
		PublicKey:         pubKey,
		Fingerprint:       fingerprint,
		Address:           address,
		ParentFingerprint: "0x00000000",
		ChainCode:         chainCode,
		Index:             0,
		Depth:             0,
		Mnemonic:          mnemonic,
	}, nil
}

type SigningKey struct {
	curve               elliptic.Curve
	privateKey          *ecdsa.PrivateKey
	publicKey           ecdsa.PublicKey
	compressedPublicKey []byte
	isSigningKey        bool
}

func NewSigningKey(privateKeyHex string) (*SigningKey, error) {
	curve := elliptic.P256() // secp256k1 curve
	privateKeyBytes, err := hex.DecodeString(privateKeyHex)
	if err != nil {
		return nil, err
	}
	if len(privateKeyBytes) != 32 {
		return nil, errors.New("invalid private key length")
	}
	privateKey := new(ecdsa.PrivateKey)
	privateKey.PublicKey.Curve = curve
	privateKey.D = new(big.Int).SetBytes(privateKeyBytes)
	privateKey.PublicKey.X, privateKey.PublicKey.Y = curve.ScalarBaseMult(privateKeyBytes)

	compressedPublicKey := elliptic.MarshalCompressed(curve, privateKey.PublicKey.X, privateKey.PublicKey.Y)

	return &SigningKey{
		curve:               curve,
		privateKey:          privateKey,
		publicKey:           privateKey.PublicKey,
		compressedPublicKey: compressedPublicKey,
		isSigningKey:        true,
	}, nil

	//privateKey, err := ecdsa.GenerateKey(curve, rand.Reader)
	//if err != nil {
	//	return nil, err
	//}
	//return &SigningKey{
	//	curve:               curve,
	//	privateKey:          privateKey,
	//	publicKey:           privateKey.PublicKey,
	//	compressedPublicKey: elliptic.MarshalCompressed(curve, privateKey.PublicKey.X, privateKey.PublicKey.Y),
	//	isSigningKey:        true,
	//}, nil
}

func PublicKeyToString(pub ecdsa.PublicKey) string {
	return fmt.Sprintf("04%s%s",
		hex.EncodeToString(pub.X.Bytes()),
		hex.EncodeToString(pub.Y.Bytes()))
}

func computeHmac(hashFunc func() hash.Hash, key, data []byte) []byte {
	h := hmac.New(hashFunc, key)
	h.Write(data)
	return h.Sum(nil)
}

// 计算 Keccak-256 哈希值
func keccak256(data []byte) []byte {
	hash := sha3.NewLegacyKeccak256()
	hash.Write(data)
	return hash.Sum(nil)
}

// 计算 SHA-256 哈希值
func sha256Hash(data []byte) []byte {
	hash := sha256.Sum256(data)
	return hash[:]
}

// 计算 RIPEMD-160 哈希值
func ripemd160Hash(data []byte) []byte {
	hasher := ripemd160.New()
	hasher.Write(data)
	return hasher.Sum(nil)
}

// 提取前 n 个字节
func hexDataSlice(data []byte, start, length int) string {
	return hex.EncodeToString(data[start : start+length])
}

func arrayify(value interface{}) ([]byte, error) {
	switch v := value.(type) {
	case int:
		if v < 0 {
			return nil, errors.New("invalid arrayify value")
		}
		result := []byte{}
		for v > 0 {
			result = append([]byte{byte(v & 0xff)}, result...)
			v >>= 8
		}
		if len(result) == 0 {
			result = append(result, 0)
		}
		return result, nil
	case string:
		if len(v) >= 2 && v[:2] == "0x" {
			v = v[2:]
		}
		if len(v)%2 != 0 {
			v = "0" + v
		}
		return hex.DecodeString(v)
	default:
		return nil, errors.New("invalid arrayify value")
	}
}

func computeFingerprint(publicKey string) string {
	hash := sha256.Sum256(hexToBytes(publicKey))
	ripemd := ripemd160.New()
	ripemd.Write(hash[:])
	return hex.EncodeToString(ripemd.Sum(nil)[:4])
}

func computeAddress(publicKey string) string {
	// Implement address computation
	// ...

	return ""
}

func computeHmacSha512(key string, data []byte) []byte {
	h := hmac.New(sha512.New, []byte(key))
	h.Write(data)
	return h.Sum(nil)
}

func hexToBytes(hexStr string) []byte {
	bytes, _ := hex.DecodeString(hexStr)
	return bytes
}

func addPrivateKeys(key1, key2 string) string {
	k1 := new(big.Int)
	k1.SetString(key1, 16)
	k2 := new(big.Int)
	k2.SetString(key2, 16)
	N := new(big.Int)
	N.SetString("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16)
	k1.Add(k1, k2).Mod(k1, N)
	return fmt.Sprintf("%064x", k1)
}

func addPublicKeys(pubKey, tweak string) string {
	// Implement public key addition
	// ...

	return ""
}

func splitPath(path string) []string {
	return strings.Split(path, "/")
}

func parseIndex(component string) int {
	index, _ := strconv.Atoi(component)
	return index
}

// GenerateSeed generates a seed from a mnemonic and passphrase using PBKDF2.
func GenerateSeed(mnemonic, passphrase string) []byte {
	return pbkdf2.Key([]byte(mnemonic), []byte("mnemonic"+passphrase), 2048, 64, sha256.New)
}

// GenerateMasterKey generates a master key from a seed.
func GenerateMasterKey(seed []byte) (*ecdsa.PrivateKey, error) {
	hash := sha256.Sum256(seed)
	return crypto.ToECDSA(hash[:])
}

// DeriveKey derives a child key from a parent key and a derivation path.
func DeriveKey(parentKey *ecdsa.PrivateKey, path accounts.DerivationPath) (*ecdsa.PrivateKey, error) {
	key := parentKey
	for _, index := range path {
		key, _ = deriveChildKey_hd(key, index)
	}
	return key, nil
}

// deriveChildKey derives a child key from a parent key and an index.
func deriveChildKey_hd(parentKey *ecdsa.PrivateKey, index uint32) (*ecdsa.PrivateKey, error) {
	// Implement BIP-32 child key derivation logic here
	// This is a simplified example and may not cover all edge cases
	data := append(crypto.FromECDSAPub(&parentKey.PublicKey), byte(index>>24), byte(index>>16), byte(index>>8), byte(index))
	hash := sha256.Sum256(data)
	childKey, err := crypto.ToECDSA(hash[:])
	if err != nil {
		return nil, err
	}
	return childKey, nil
}

// GenerateAddress generates an Ethereum address from a private key.
func GenerateAddress(privateKey *ecdsa.PrivateKey) string {
	publicKey := privateKey.PublicKey
	address := crypto.PubkeyToAddress(publicKey).Hex()
	return address
}

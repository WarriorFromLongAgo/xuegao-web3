import {derivePath, getPublicKey} from "ed25519-hd-key";
import BigNumber from "bignumber.js";
// 引入 tonweb 库
import TonWeb from 'tonweb';
import { mnemonicNew, mnemonicToSeed, keyPairFromSeed } from 'ton-crypto';
import nacl from 'tweetnacl';

const tonweb = new TonWeb();

export async function createTonAddress(seedHex: string, addressIndex: number) {
    const {key} = derivePath("m/44'/607'/1'/" + addressIndex + "'", seedHex);
    const publicKey = getPublicKey(key, false).toString('hex');

    const tonweb = new TonWeb();
    const WalletClass = tonweb.wallet.all['v3R2'];

    const pubKey = new Uint8Array(Buffer.from(publicKey, "hex"))
    const wallet = new WalletClass(tonweb.provider, {
        publicKey: pubKey,
        wc: 0
    });
    const walletAddress = await wallet.getAddress();
    return {
        "privateKey": key.toString('hex') + publicKey,
        "publicKey": publicKey,
        "address": walletAddress.toString(true, true, true, false)
    }
}

// 生成 HD 钱包地址的方法
export async function generateHDWallet() {
    try {
        const mnemonic = await mnemonicNew();
        const seed = await mnemonicToSeed(mnemonic, '', null); // 提供空字符串作为密码

        // 确保种子大小为 32 字节
        if (seed.length !== 32) {
            throw new Error('生成的种子大小不正确');
        }

        const keyPair = keyPairFromSeed(seed);

        const publicKey = keyPair.publicKey;
        const secretKey = keyPair.secretKey;

        const WalletClass = tonweb.wallet.all.v3R2;
        const wallet = new WalletClass(tonweb.provider, {
            publicKey: publicKey,
            wc: 0
        });

        const address = await wallet.getAddress();
        console.log('HD 钱包地址:', address.toString(true, true, true));
        console.log('助记词:', mnemonic.join(' '));
        console.log('公钥:', Buffer.from(publicKey).toString('hex'));
        console.log('私钥:', Buffer.from(secretKey).toString('hex'));
    } catch (error) {
        console.error('生成 HD 钱包地址时出错:', error);
    }
}

// 生成 KeyPair 钱包地址的方法
export async function generateKeyPairWallet() {
    try {
        const keyPair = nacl.sign.keyPair();
        const publicKey = keyPair.publicKey;
        const secretKey = keyPair.secretKey;

        const WalletClass = tonweb.wallet.all.v3R2;
        const wallet = new WalletClass(tonweb.provider, {
            publicKey: publicKey,
            wc: 0
        });

        const address = await wallet.getAddress();
        console.log('KeyPair 钱包地址:', address.toString(true, true, true));
        console.log('公钥:', Buffer.from(publicKey).toString('hex'));
        console.log('私钥:', Buffer.from(secretKey).toString('hex'));
    } catch (error) {
        console.error('生成 KeyPair 钱包地址时出错:', error);
    }
}

export async function SignTransaction(params: {
    from: string;
    to: string;
    memo: string;
    amount: number;
    sequence: number;
    decimal: number;
    privateKey: string;
}) {
    const {from, to, memo, amount, sequence, decimal, privateKey} = params;

    const tonweb = new TonWeb();

    const calcAmount = new BigNumber(amount).times(new BigNumber(10).pow(decimal)).toNumber();

    if (calcAmount % 1 !== 0) throw new Error("amount invalid");
    const keyPair = TonWeb.utils.nacl.sign.keyPair.fromSecretKey(new Uint8Array(Buffer.from(privateKey, 'hex')));
    const WalletClass = tonweb.wallet.all['v4R2'];
    const wallet = new WalletClass(tonweb.provider, {
        publicKey: keyPair.publicKey,
        wc: 0
    });
    let secretKey = keyPair.secretKey
    const walletAddress = await wallet.getAddress();
    console.log(walletAddress.toString(true, true, false, false))
    const fromAddres = walletAddress.toString(true, true, false, false);
    if (from !== fromAddres) throw new Error("from address invalid");
    const toAddress = new TonWeb.utils.Address(to);
    const tx_ret = wallet.methods.transfer({
        secretKey,
        toAddress: toAddress.toString(true, true, false, false),
        amount: calcAmount,
        seqno: sequence,
        payload: memo || "",
        sendMode: 3, // 3 默认为转账, 细节请看调研文档
    })
    const queryData = await tx_ret.getQuery()
    const hash = await queryData.hash()
    const boc = await queryData.toBoc(false);
    return {
        "hash": TonWeb.utils.bytesToBase64(hash),
        "rawtx": TonWeb.utils.bytesToBase64(boc)
    }
}
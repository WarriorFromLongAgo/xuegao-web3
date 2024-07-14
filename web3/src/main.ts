import {derivePath, getPublicKey} from "ed25519-hd-key";
import TonWeb from "tonweb";
import BigNumber from "bignumber.js";

// console.log
// addressInfo=== {
//     privateKey: '1079e489e04925bc3207db41ab0d4341c9f3c7b808078c9a8320e5e2f3471db22dc315b7ba636eec21527be1f2edfac287addb235c1eb71a70300c1208d30f9c',
//     publicKey: '2dc315b7ba636eec21527be1f2edfac287addb235c1eb71a70300c1208d30f9c',
//     address: 'EQA5PEhPzGZgmMOk9Fr2sPEAY03HLueelXFXFWdwKxh0h7Pq'
// }

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



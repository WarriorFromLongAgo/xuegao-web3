import * as ecc from 'tiny-secp256k1';
import {BIP32API, BIP32Factory} from 'bip32';
import * as bitcoin from 'bitcoinjs-lib';

const bitcore: any = require('bitcore-lib');
// 确保 ecc 实例被正确传递给 BIP32Factory
const bip32: BIP32API = BIP32Factory(ecc);

/**
 *
 * receiveOrChange 如果 receiveOrChange 为 '1'，则路径改为找零地址路径
 * */
export function createBtcAddress(seedHex: string, receiveOrChange: '0' | '1', addressIndex: string, network: keyof typeof bitcoin.networks): {
    privateKey: string;
    publicKey: string;
    address: string | undefined
} {
    const root = bip32.fromSeed(Buffer.from(seedHex, 'hex'));
    let path = "m/44'/0'/0'/0/" + addressIndex + '';
    if (receiveOrChange === '1') {
        // 如果 receiveOrChange 为 '1'，则路径改为找零地址路径
        path = "m/44'/0'/0'/1/" + addressIndex + '';
    }
    const child = root.derivePath(path);

    // 生成地址
    const {address} = bitcoin.payments.p2pkh({
        pubkey: child.publicKey,
        network: bitcoin.networks[network]
    });
    return {
        privateKey: child.privateKey ? child.privateKey.toString('hex') : '',
        publicKey: child.publicKey.toString('hex'),
        address
    };
}

// 定义有效的网络名称类型
export type ValidNetwork = 'mainnet' | 'testnet' | 'livenet'; // 根据实际需要添加更多网络名称
/**
 * import address
 * private key
 * network
 * @param params
 */
export function importBtcAddress(params: { privateKey: string; network: ValidNetwork }) {
    const {privateKey, network} = params;
    // 获取网络对象
    const net = bitcore.Networks[network];
    if (!net) {
        throw new Error(`Invalid network: ${network}`);
    }
    // 检查私钥是否有效
    if (!bitcore.PrivateKey.isValid(privateKey, network)) {
        throw new Error('PrivateKey is not valid.');
    }

    // 使用私钥生成私钥对象
    const privateKeyObj = new bitcore.PrivateKey(privateKey, net);

    // 使用私钥对象生成地址
    const address = privateKeyObj.toAddress().toString();

    return address;
}


export function verifyBtcAddress(params: { address: string; network: ValidNetwork }) {
    const {address, network} = params;
    // 获取网络对象
    const net = bitcore.Networks[network];
    if (!net) {
        throw new Error(`Invalid network: ${network}`);
    }

    return bitcore.Address.isValid(address, net, 'script');
}

// 定义 SignObj 接口
export interface SignObj {
    inputs: Array<{
        address: string;
        txid: string;
        amount: number;
        vout: number;
    }>;
    outputs: Array<{
        address: string;
        amount: number;
    }>;
}

/**
 * 暂不支持taproot签名
 * @returns
 * @param params
 */
export function signBtcTransaction(params: { privateKey: string; signObj: SignObj; network: ValidNetwork; }): string {
    const {privateKey, signObj, network} = params;
    // 获取网络对象
    const net = bitcore.Networks[network];

    const inputs = signObj.inputs.map(input => {
        return {
            address: input.address,
            txId: input.txid,
            outputIndex: input.vout,
            script: new bitcore.Script.fromAddress(input.address).toHex(),
            satoshis: input.amount
        }
    });
    const outputs = signObj.outputs.map(output => {
        return {
            address: output.address,
            satoshis: output.amount
        };
    });
    const transaction = new bitcore.Transaction(net).from(inputs).to(outputs);
    transaction.version = 2;
    transaction.sign(privateKey);
    return transaction.toString();
}




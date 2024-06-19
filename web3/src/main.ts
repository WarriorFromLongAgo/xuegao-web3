import * as ecc from 'tiny-secp256k1';
import * as bitcoin from 'bitcoinjs-lib';

import {BIP32API, BIP32Factory} from 'bip32';

const bip32: BIP32API = BIP32Factory(ecc);

const bitcore: any = require('bitcore-lib');

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

export interface CreateAddressParams {
    seedHex: string;
    receiveOrChange: string;
    addressIndex: string;
    network: keyof typeof bitcoin.networks;
    method: string;
}

export function createBtcAddressV2(param: CreateAddressParams): {
    privateKey: string;
    publicKey: string;
    address: string
} {
    const {
        seedHex, receiveOrChange, addressIndex, network, method
    } = param;
    const root = bip32.fromSeed(Buffer.from(seedHex, 'hex'));

    let path = "m/44'/0'/0'/0/" + addressIndex + '';
    if (receiveOrChange === '1') {
        // 如果 receiveOrChange 为 '1'，则路径改为找零地址路径
        path = "m/44'/0'/0'/1/" + addressIndex + '';
    }
    const child = root.derivePath(path);

    let address: string = '';
    switch (method) {
        case 'p2pkh':
            const p2pkhAddress = bitcoin.payments.p2pkh({
                pubkey: child.publicKey,
                network: bitcoin.networks[network]
            });
            if (!p2pkhAddress.address) {
                throw new Error('Failed to generate p2pkh address');
            }
            address = p2pkhAddress.address;
            break;
        case 'p2wpkh':
            const p2wpkhAddress = bitcoin.payments.p2wpkh({
                pubkey: child.publicKey,
                network: bitcoin.networks[network]
            });
            if (!p2wpkhAddress.address) {
                throw new Error('Failed to generate p2wpkh address');
            }
            address = p2wpkhAddress.address;
            break;
        case 'p2sh':
            const p2shAddress = bitcoin.payments.p2sh({
                redeem: bitcoin.payments.p2wpkh({
                    pubkey: child.publicKey,
                    network: bitcoin.networks[network]
                })
            });
            if (!p2shAddress.address) {
                throw new Error('Failed to generate p2sh address');
            }
            address = p2shAddress.address;
            break;
        default:
            console.log('This way can not support');
    }
    if (!child.privateKey) {
        throw new Error('Failed to generate privateKey');
    }
    let privateKey = Buffer.from(child.privateKey).toString('hex');
    let publicKey = Buffer.from(child.publicKey).toString('hex');

    return {
        privateKey: privateKey,
        publicKey: publicKey,
        address
    };
}

export interface MultiSignAddressParams {
    pubkeys: Buffer[]; // 假设公钥是字符串类型
    network: keyof typeof bitcoin.networks;   // 假设网络名称是字符串类型，比如 'bitcoin' 或 'testnet'
    method: 'p2pkh' | 'p2wpkh' | 'p2sh'; // 支持的地址生成方法
    threshold: number; // 阈值，确定需要多少个签名才能花费资金
}

export function createMultiSignAddress(param: MultiSignAddressParams): string {
    const {pubkeys, network, method, threshold} = param;
    switch (method) {
        case 'p2pkh':
            let p2pkhAddress = bitcoin.payments.p2sh({
                redeem: bitcoin.payments.p2ms({
                    m: threshold,
                    network: bitcoin.networks[network],
                    pubkeys
                })
            }).address;
            if (!p2pkhAddress) {
                throw new Error('Failed to generate p2pkh Address');
            }
            return p2pkhAddress;
        case 'p2wpkh':
            let p2wpkhAddress = bitcoin.payments.p2wsh({
                redeem: bitcoin.payments.p2ms({
                    m: threshold,
                    network: bitcoin.networks[network],
                    pubkeys
                })
            }).address;
            if (!p2wpkhAddress) {
                throw new Error('Failed to generate p2wpkh Address');
            }
            return p2wpkhAddress;
        case 'p2sh':
            let p2shAddress = bitcoin.payments.p2sh({
                redeem: bitcoin.payments.p2wsh({
                    redeem: bitcoin.payments.p2ms({
                        m: threshold,
                        network: bitcoin.networks[network],
                        pubkeys
                    })
                })
            }).address;
            if (!p2shAddress) {
                throw new Error('Failed to generate p2sh Address');
            }
            return p2shAddress;
        default:
            console.log('This way can not support');
            return '0x00';
    }
}

export function createSchnorrAddress(params: any): any {
    bitcoin.initEccLib(ecc);
    const {seedHex, receiveOrChange, addressIndex} = params;
    const root = bip32.fromSeed(Buffer.from(seedHex, 'hex'));
    let path = "m/44'/0'/0'/0/" + addressIndex + '';
    if (receiveOrChange === '1') {
        path = "m/44'/0'/0'/1/" + addressIndex + '';
    }
    const childKey = root.derivePath(path);
    const privateKey = childKey.privateKey;
    if (!privateKey) throw new Error('No private key found');

    const publicKey = childKey.publicKey;

    // 计算 taproot 公钥
    const tweak = bitcoin.crypto.taggedHash('TapTweak', publicKey.slice(1, 33));
    const tweakedPublicKey = Buffer.from(publicKey);
    for (let i = 0; i < 32; ++i) {
        tweakedPublicKey[1 + i] ^= tweak[i];
    }

    // 生成 P2TR 地址
    const {address} = bitcoin.payments.p2tr({
        internalPubkey: tweakedPublicKey.slice(1, 33)
    });

    if (!childKey.privateKey) {
        throw new Error('Failed to generate privateKey');
    }
    let newPrivateKey = Buffer.from(childKey.privateKey).toString('hex');
    let newPublicKey = Buffer.from(childKey.publicKey).toString('hex');

    return {
        privateKey: newPrivateKey,
        publicKey: newPublicKey,
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


/**
 * @returns
 * @param params
 */
// export function buildAndSignTx (params: { privateKey: string; signObj: any; network: string; }): string {
//     const { privateKey, signObj, network } = params;
//     const net = bitcore.Networks[network];
//     const inputs = signObj.inputs.map(input => {
//         return {
//             address: input.address,
//             txId: input.txid,
//             outputIndex: input.vout,
//             // eslint-disable-next-line new-cap
//             script: new bitcore.Script.fromAddress(input.address).toHex(),
//             satoshis: input.amount
//         };
//     });
//     const outputs = signObj.outputs.map(output => {
//         return {
//             address: output.address,
//             satoshis: output.amount
//         };
//     });
//     const transaction = new bitcore.Transaction(net).from(inputs).to(outputs);
//     transaction.version = 2;
//     transaction.sign(privateKey);
//     return transaction.toString();
// }
//
// export function buildUnsignTxAndSign (params) {
//     const { keyPair, signObj, network } = params;
//     const psbt = new bitcoin.Psbt({ network });
//     const inputs = signObj.inputs.map(input => {
//         return {
//             address: input.address,
//             txId: input.txid,
//             outputIndex: input.vout,
//             // eslint-disable-next-line new-cap
//             script: new bitcore.Script.fromAddress(input.address).toHex(),
//             satoshis: input.amount
//         };
//     });
//     psbt.addInput(inputs);
//
//     const outputs = signObj.outputs.map(output => {
//         return {
//             address: output.address,
//             satoshis: output.amount
//         };
//     });
//     psbt.addOutput(outputs);
//     psbt.toBase64();
//
//     psbt.signInput(0, keyPair);
//     psbt.finalizeAllInputs();
//
//     const signedTransaction = psbt.extractTransaction().toHex();
//     console.log('signedTransaction==', signedTransaction);
// }




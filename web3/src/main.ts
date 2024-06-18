import * as ecc from 'tiny-secp256k1';
import {BIP32API, BIP32Factory} from 'bip32';
import * as bitcoin from 'bitcoinjs-lib';
import * as bitcore from 'bitcore-lib';
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
export type ValidNetwork = 'livenet' | 'testnet' | 'regtest'; // 根据实际需要添加更多网络名称
/**
 * import address
 * private key
 * network
 * @param params
 */
export function importBtcAddress(params: { privateKey: string; network: ValidNetwork }) {
    const {privateKey, network} = params;
    const net = bitcore.Networks.get(network, []);
    // 检查私钥是否有效
    // if (!bitcore.PrivateKey.isValid(privateKey, net)) {
    //     throw new Error('PrivateKey is not valid.');
    // }
    // 使用私钥生成地址
    const privateKeyObj = new bitcore.PrivateKey(privateKey, net);
    const address = privateKeyObj.toAddress().toString();
    return address;
}


export function verifyBtcAddress (params: { address: string; network: ValidNetwork }) {
    const { address, network } = params;
    const net = bitcore.Networks.get(network, []);
    return bitcore.Address.isValid(address, net);
}





// 定义一个方法来计算两个数字的和
import {ethers} from "ethers";
import {Interface} from "@ethersproject/abi";
import {TransactionRequest} from "@ethersproject/abstract-provider";

/**
 * HD 钱包生成地址
 * */
export function createEthAddress(seedHex: string, addressIndex: string) {
    console.log("createEthAddress seedHex", seedHex)
    let seedHexToBuffer: Buffer = Buffer.from(seedHex, 'hex')

    const hdNode = ethers.utils.HDNode.fromSeed(seedHexToBuffer);
    console.log("createEthAddress hdNode", JSON.stringify(hdNode))

    const path = "m/44'/10'/0'/0/" + addressIndex + '';
    console.log("createEthAddress path", path)
    const {
        privateKey,
        publicKey,
        address
    } = hdNode.derivePath(path);

    console.log("createEthAddress privateKey", privateKey)
    console.log("createEthAddress publicKey", publicKey)
    console.log("createEthAddress address", address)

    return JSON.stringify({
        privateKey,
        publicKey,
        address
    });
}

/**
 * 交易所 钱包生成地址
 * */
export function createEthAddressByCex() {


}

/**
 * import address
 * private key
 * network
 * @param privateKey
 */
export function importEthAddress(privateKey: string) {
    let privateKeyToBuffer: Buffer = Buffer.from(privateKey, 'hex')

    const wallet = new ethers.Wallet(privateKeyToBuffer);

    return JSON.stringify({
        privateKey,
        publicKey: wallet.publicKey,
        address: wallet.address
    });
}


/**
 * address
 * network type
 * @param address
 */
export function verifyEthAddress(address: string) {
    return ethers.utils.isAddress(address);
}

// ETH SDK 支持的 EVM链
const SUPPORT_CHAIN_NETWORK = {
    1: 'Ethereum',
    324: 'ZksyncEra',
    42161: 'Arbitrum',
    42170: 'ArbitrumNova',
    5000: 'Mantle',
    56: 'BscChain',
    128: 'Heco',
    137: 'Polygon',
    10001: 'EthereumPow',
    61: 'EthereumClassic',
    8217: 'klay',
    1101: 'PolygonZk',
    66: 'OkexChain',
    9001: 'Evmos',
    10: 'Optimism',
    59144: 'Linea',
    8453: 'Base'
};


export class EthTransactionSigner {
    // 用于存储以太坊账户的私钥，用于对交易进行签名。
    privateKey: string = '';
    // 交易的序列号，用于确保每笔交易的唯一性，防止重放攻击。
    nonce: number = 0;
    // 发送交易的以太坊账户地址。
    from: string = '';
    // 接收交易的以太坊账户地址。
    to: string = '';
    // 交易的最大 gas 用量，用于限制交易的执行成本。
    gasLimit: number = 0;
    // 每单位 gas 的价格，决定了交易的手续费总额。
    gasPrice: number = 0;
    // 交易的金额，以字符串形式表示，通常是以太币数量。
    amount: string = '';
    // 可选的附加数据，用于发送与合约相关的交易。
    data?: any[];
    // 以太坊网络的链 ID，用于指定交易所在的网络。
    chainId: number = 0;
    // 代币的小数位数，用于计算代币的真实价值。
    decimal: number = 0;
    // 可选的最大 gas 交易费用，用于 EIP-1559 规范中的交易费用设置。
    maxFeePerGas?: number;
    // 可选的最大优先 gas 交易费用，用于 EIP-1559 规范中的交易费用设置。
    maxPriorityFeePerGas?: number;
    // 如果交易是代币转账，则指定代币的合约地址。
    tokenAddress: string = '';
}

/**
 * sign transaction
 * @returns
 * @param params
 */
export async function signEthTransaction(params: EthTransactionSigner): Promise<string> {
    // privateKey remove 0x
    const {
        privateKey,
        nonce,
        from,
        to,
        gasLimit,
        gasPrice,
        amount,
        data,
        chainId,
        decimal,
        maxFeePerGas,
        maxPriorityFeePerGas,
        tokenAddress
    } = params;

    if (!(chainId in SUPPORT_CHAIN_NETWORK)) {
        throw new Error(`chain id ${chainId} is not support.`);
    }
    // 根据私钥创建钱包实例
    const wallet = new ethers.Wallet(Buffer.from(privateKey, 'hex'));
    // 构造交易数据对象
    // ethers.utils.hexlify() 是 ethers.js 库中的一个方法，用于将给定的值转换为十六进制字符串格式。
    const txData: TransactionRequest = {
        nonce: ethers.utils.hexlify(nonce),
        from,
        to,
        gasLimit: ethers.utils.hexlify(gasLimit),
        value: ethers.utils.hexlify(ethers.utils.parseUnits(amount, decimal)),
        chainId
    };
    // 设置最大手续费相关参数（根据以太坊 London 升级）
    if (maxFeePerGas && maxPriorityFeePerGas) {
        txData.maxFeePerGas = ethers.utils.hexlify(maxFeePerGas);
        txData.maxPriorityFeePerGas = ethers.utils.hexlify(maxPriorityFeePerGas);
    } else {
        txData.gasPrice = ethers.utils.hexlify(gasPrice);
    }
    // 如果存在代币地址，则构造代币转账交易
    if (tokenAddress && tokenAddress !== '0x00') {
        const ABI = [
            'function transfer(address to, uint amount)'
        ];
        const abiInterface = new Interface(ABI);
        const idata = abiInterface.encodeFunctionData('transfer', [to, ethers.utils.hexlify(ethers.utils.parseUnits(amount, decimal))]);
        txData.data = idata;
        // 设置代币合约地址为接收地址
        txData.to = tokenAddress;
        // 代币转账时 value 设为 0
        txData.value = 0;
    }
    if (data) {
        txData.data = data;
    }
    return wallet.signTransaction(txData);
}




import { Keypair } from '@solana/web3.js';

export function generateSolanaWallet() {
    const keypair = Keypair.generate();
    const publicKey = keypair.publicKey.toString();
    const privateKey = Buffer.from(keypair.secretKey).toString('hex');

    // 生成地址
    const address = publicKey; // 公钥即为钱包地址

    return { publicKey, privateKey, address };
}

// export function signKdaTransaction(params: any): string {
//
// }



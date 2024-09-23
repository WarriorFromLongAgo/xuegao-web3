// __tests__/index.test.ts
import { generateSolanaAddress } from '../src';

test('generateSolanaAddress should return a valid Solana address', () => {
    const wallet = generateSolanaAddress();
    console.log(" generateSolanaAddress() publicKey", wallet.privateKey.toString());
    console.log(" generateSolanaAddress() publicKey", wallet.publicKey.toString());
    console.log(" generateSolanaAddress() publicKey", wallet.address.toString());
});
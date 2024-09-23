import {generateSolanaWallet} from "./main";


test('create solana address', () => {
    const wallet = generateSolanaWallet();

    // 检查是否生成了公钥、私钥和地址
    expect(wallet).toHaveProperty('publicKey');
    expect(wallet).toHaveProperty('privateKey');
    expect(wallet).toHaveProperty('address');

    // 检查公钥、私钥和地址是否为字符串并且长度符合标准
    expect(typeof wallet.publicKey).toBe('string');
    expect(wallet.publicKey.length).toBeGreaterThan(0);

    expect(typeof wallet.privateKey).toBe('string');
    expect(wallet.privateKey.length).toBeGreaterThan(0);

    expect(wallet.address).toBe(wallet.publicKey); // 地址等于公钥
});





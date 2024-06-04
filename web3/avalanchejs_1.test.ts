import {base58check} from './base58/base58';
import {secp256k1} from './crypto';

describe('Avalanche AVAX', () => {
    it(' create address 1 ', () => {
        const key: string = '24jUJ9vZexUM6expyMcT48LBx27k1m7xpraoV62oSQAHdziao5';
        console.log("key ", key)
        const privKey = base58check.decode(key);
        console.log("privKey ", privKey)
        const pubKey = secp256k1.getPublicKey(privKey);
        console.log("pubKey ", pubKey)

    });
});


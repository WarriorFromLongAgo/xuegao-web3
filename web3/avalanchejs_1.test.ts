import {base58check} from './base58/base58';
import {secp256k1} from './crypto';
import * as address from './utils/address';
import sdk from '@api/ava-cloud';

describe('Avalanche AVAX', () => {
    it(' create address 1 ', () => {
        // const key: string = 0x9771c4c0E413F952cb773052B1C36A20b7b22f1f
        const key: string = '24jUJ9vZexUM6expyMcT48LBx27k1m7xpraoV62oSQAHdziao5';
        console.log("key ", key)
        const privKey = base58check.decode(key);
        // console.log("privKey ", privKey)
        // console.log("privKey (as Array)", Array.from(privKey));
        console.log("privKey (as Hex)", Buffer.from(privKey).toString('hex'));

        const pubKey = secp256k1.getPublicKey(privKey);
        // console.log("pubKey ", pubKey)
        // console.log("pubKey (as Array)", Array.from(pubKey));
        console.log("pubKey (as Hex)", Buffer.from(pubKey).toString('hex'));

        const addrBytes = secp256k1.publicKeyBytesToAddress(pubKey);
        console.log("addrBytes (as Hex)", Buffer.from(addrBytes).toString('hex'));

        const addr = address.format('X', 'avax', addrBytes);
        console.log("addr", addr);
    });

    it('Get the health of the service ', (done) => {
        console.log("dasdsadasdsadsa")
        sdk.healthCheck()
            .then(({data}) => {
                console.log(" asdada ", data);
                done();
            })
            .catch(err => {
                console.error(" dadas ", err);
                done(err);
            });
        console.log("dsadasdadasdasdasda")
    }, 10000)
});


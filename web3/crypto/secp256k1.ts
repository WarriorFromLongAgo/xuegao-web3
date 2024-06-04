import {ripemd160} from '@noble/hashes/ripemd160';
import {sha256} from '@noble/hashes/sha256';
import {
    getPublicKey as secp_getPublicKey,
    signAsync as secp_signAsync,
    Signature as secp_Signature,
    utils as secp_utils,
    verify as secp_verify
} from '@noble/secp256k1';
// const secp256k1  = require('@noble/secp256k1')
import {Address} from 'micro-eth-signer';
import {concatBytes, hexToBuffer} from '../utils/buffer';

export function randomPrivateKey() {
    return secp_utils.randomPrivateKey();
}

export function sign(msg: Uint8Array | string, privKey: Uint8Array) {
    return signHash(sha256(msg), privKey);
}

export async function signHash(hash: Uint8Array, privKey: Uint8Array) {
    const sig = await secp_signAsync(hash, privKey);

    if (sig.recovery !== undefined) {
        return concatBytes(sig.toCompactRawBytes(), new Uint8Array([sig.recovery]));
    } else {
        throw new Error(`Recovery bit is missing.`);
    }
}

export function recoverPublicKey(hash: Uint8Array | string, sig: Uint8Array) {
    const recoveryBit = sig.slice(-1);
    const secpSig = secp_Signature.fromCompact(sig.slice(0, -1)).addRecoveryBit(
        recoveryBit[0],
    );
    const point = secpSig.recoverPublicKey(hash);

    return point.toRawBytes(true);
}

export function getPublicKey(privKey: Uint8Array) {
    return secp_getPublicKey(privKey, true);
}

export function verify(
    sig: Uint8Array,
    hash: Uint8Array | string,
    publicKey: Uint8Array,
) {
    return secp_verify(sig.slice(0, -1), hash, publicKey);
}

export function publicKeyBytesToAddress(publicKey: Uint8Array) {
    return ripemd160(sha256(publicKey));
}

export function publicKeyToEthAddress(key: Uint8Array) {
    return hexToBuffer(Address.fromPublicKey(key));
}

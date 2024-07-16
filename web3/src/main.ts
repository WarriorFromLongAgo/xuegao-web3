import {derivePath, getPublicKey} from "ed25519-hd-key";

export function createAddress(seedHex: string, addressIndex: string) {
    const {key} = derivePath("m/44'/626'/0'/" + addressIndex + "'", seedHex);

    const pubKey = getPublicKey(key, false).toString("hex");

    return {
        privateKey: key.toString("hex") + pubKey,
        publicKey: pubKey,
        address: "k:" + pubKey,
    }
}

export function pubKeyToAddress(pubKey: string): string {
    return "k:" + pubKey
}

// export function signKdaTransaction(params: any): string {
//
// }



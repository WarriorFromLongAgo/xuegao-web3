import createEthAddress, {importEthAddress, verifyEthAddress} from "./main";
import {mnemonicToSeed} from "./bip";

test('hd createAddress', () => {
    const mnemonic = "ability absent absorb again airport alert almost anchor attack auction basket between";
    const params_1 = {
        mnemonic: mnemonic,
        password: "1234567890"
    }
    const seed: Buffer = mnemonicToSeed(params_1)
    // f5d9a3d12c62309945855ad96653ada491dadc14159711196de592202d6fd86f2d2471680670d726689596898fee222993fd96ec2ce47a1578df3fdc4c800cba
    console.log("seed = ", seed.toString("hex"));
    const account = createEthAddress(seed.toString("hex"), "2")
    // {
    //     "privateKey": "0x70772a61218ca415cb69e06d2e42f81a258f681e2e51f155147412bacf941798",
    //     "publicKey": "0x022d9da3aad70c3b53192ae2bc3c131afa6c0d598d038227049123f1ab6abb868b",
    //     "address": "0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C0"
    // }
    console.log("account = ", account)
});


test('HD importEthAddress', () => {
    const account = importEthAddress("70772a61218ca415cb69e06d2e42f81a258f681e2e51f155147412bacf941798")
    // {
    //     "privateKey": "70772a61218ca415cb69e06d2e42f81a258f681e2e51f155147412bacf941798",
    //     "publicKey": "0x042d9da3aad70c3b53192ae2bc3c131afa6c0d598d038227049123f1ab6abb868b91de7c03d0759493f1308d32dcc76942991e99aa6348d2d9ea302a4a94314000",
    //     "address": "0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C0"
    // }
    console.log("account = ", account)
});


test('HD verifyEthAddress', () => {
    // 0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C0 =  true
    console.log("0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C0 = ", verifyEthAddress("0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C0"))
    // 0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C1 =  false
    console.log("0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C1 = ", verifyEthAddress("0x5f487ca29914b5A92Da91aaC16a4d5E35604b7C1"))
});





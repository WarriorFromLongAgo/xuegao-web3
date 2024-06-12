import {mnemonicToSeed} from "./bip";

test('mnemonicToSeed', () => {
    const mnemonic = "ability absent absorb again airport alert almost anchor attack auction basket between";
    const params_1 = {
        mnemonic: mnemonic,
        password: "1234567890"
    }
    const seed = mnemonicToSeed(params_1)
    // f5d9a3d12c62309945855ad96653ada491dadc14159711196de592202d6fd86f2d2471680670d726689596898fee222993fd96ec2ce47a1578df3fdc4c800cba
    console.log(seed.toString("hex"));
});



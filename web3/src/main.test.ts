import {createAddress} from "./main";
import {mnemonicToSeedSync} from "bip39";


test('create kda address', () => {
    const mnemonic = 'raw cherry park music seed credit rather model wrap human demand trade'
    let seed = mnemonicToSeedSync(mnemonic);
    const account = createAddress(seed.toString("hex"), "0")
    console.log(account)
});

// 如何找到 kda 用的是什么方式，生成的地址






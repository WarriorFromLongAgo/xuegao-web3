import bip39 = require('bip39');
import {createTonAddress, SignTransaction} from "./main";


test('create address', async () => {
    const mnemonic = "absurd junior glimpse analyst plug jump account barrel slight swim hidden remove";

    // 这里添加了密码参数
    const seed = bip39.mnemonicToSeedSync(mnemonic, "")
    // 假设这是您定义的函数
    const addressInfo = await createTonAddress(seed.toString(), 0)
    console.log("addressInfo===", addressInfo)
});


test('sign transaction', async () => {
    const param = {
        from: "UQAUAHcUab66DpOV2GaT_QDuSagpMdIn0x6aMmO3_fPVM305",
        to: "EQCQCLTvR0XYTyM0uxh_H8kLAR7u7v98pEKZKpbq8w2peuNY",
        memo: "memo",
        amount: 0.01,
        sequence: 38103804,
        decimal: 10,
        privateKey: "b0e4eb37bc5929491899d2a50f52f0a4613d3a48e56245267fdecff392ead89b7e4fdf79bf78566b85b73787e5739ab4306350d7ad1adc50be9c57fe2102bfcc"
    }
    const sign_message = await SignTransaction(param)
    console.log("sign_message===", sign_message)
})




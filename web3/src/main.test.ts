import {mnemonicToSeed} from "./bip";
import {createBtcAddress, importBtcAddress, ValidNetwork} from "./main";

test('btc createAddress', () => {
    const mnemonic = "lounge face pattern cinnamon shrug average spend rapid field cheese wrist weather";
    const params_1 = {
        mnemonic: mnemonic,
        password: ""
    }
    const seed = mnemonicToSeed(params_1)
    const account = createBtcAddress(seed.toString("hex"), "0", "0", "bitcoin")
    console.log(account)
    // {
    //     privateKey: '67fe93913d415ebb15cf6d86cc913efb0b3d724caa743de442cb795481495429',
    //     publicKey: '02ef67f85c8376cf609a494af8c3a043df98211dec573cf1b0eb17304439cab90d',
    //     address: '1CzkhKbqwmDL4o8StBdttLNesLpDZpddmA'
    // }
});

test('importBtcAddress', () => {
    const params = {
        privateKey: "67fe93913d415ebb15cf6d86cc913efb0b3d724caa743de442cb795481495429",
        network: "testnet" as ValidNetwork
    }
    const account = importBtcAddress(params)
    // 1CzkhKbqwmDL4o8StBdttLNesLpDZpddmA
    console.log(account)
});



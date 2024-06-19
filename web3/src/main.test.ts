import {mnemonicToSeed} from "./bip";
import {createBtcAddress, importBtcAddress, signBtcTransaction, SignObj, ValidNetwork, verifyBtcAddress} from "./main";

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
        // privateKey: "67fe93913d415ebb15cf6d86cc913efb0b3d724caa743de442cb795481495429",
        privateKey: "67fe93913d415ebb15cf6d86cc913efb0b3d724caa743de442cb79548149542911",
        network: "mainnet" as ValidNetwork
    }
    const account = importBtcAddress(params)
    // 1CzkhKbqwmDL4o8StBdttLNesLpDZpddmA
    console.log(account)
});


test('verifyBtcAddress', () => {
    const params = {
        address: "1CzkhKbqwmDL4o8StBdttLNesLpDZpddmA",
        network: "mainnet" as ValidNetwork
    }
    const flag = verifyBtcAddress(params)
    // 1CzkhKbqwmDL4o8StBdttLNesLpDZpddmA
    console.log(flag)
});


test('sign', () => {
    const data: SignObj = {
        inputs: [
            {
                address: "1H1oAqmdfTNECrrHFAJ4AhbTUyPcQjrf72",
                txid: "209706b97a9aed047df158bf57cfbdad94a5e9bd9ac5261034448ec4590bab8f",
                amount: 9000000000000000,
                vout: 0,
            },
        ],
        outputs: [
            {
                amount: 9000000000000000,
                address: "1H1oAqmdfTNECrrHFAJ4AhbTUyPcQjrf72",
            },
        ],
    };
    const rawHex = signBtcTransaction({
        privateKey: "60164bec9512d004af7f71e7ed868c8e9ac2cc6234d8b682037ec80547595f2e",
        signObj: data,
        network: "mainnet"
    });
    console.log(rawHex);
});




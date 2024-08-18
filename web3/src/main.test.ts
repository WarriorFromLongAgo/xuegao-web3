import {CoinBalance, getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { MIST_PER_SUI } from '@mysten/sui/utils';
import { Secp256k1Keypair } from '@mysten/sui/keypairs/secp256k1';

// 获取水龙头
test('get sui faucet', async () => {
    // replace <YOUR_SUI_ADDRESS> with your actual address, which is in the form 0x123...
    const MY_ADDRESS = '0x28bb0eb54568e6ff10298c6b8a43c9d2a88889f0f26a1d2492fe0ab298f2ab19';

// create a new SuiClient object pointing to the network you want to use
    const suiClient = new SuiClient({url: getFullnodeUrl('testnet')});

// Convert MIST to Sui
    const balance = (balance: CoinBalance) => {
        return Number.parseInt(balance.totalBalance) / Number(MIST_PER_SUI);
    };

    // store the JSON representation for the SUI the address owns before using faucet
    const suiBefore = await suiClient.getBalance({
        owner: MY_ADDRESS,
    });

    await requestSuiFromFaucetV1({
        // use getFaucetHost to make sure you're using correct faucet address
        // you can also just use the address (see Sui TypeScript SDK Quick Start for values)
        host: getFaucetHost('testnet'),
        recipient: MY_ADDRESS,
    });

    // store the JSON representation for the SUI the address owns after using faucet
    const suiAfter = await suiClient.getBalance({
        owner: MY_ADDRESS,
    });

// Output result to console.
    console.log(
        `Balance before faucet: ${balance(suiBefore)} SUI. Balance after: ${balance(
            suiAfter,
        )} SUI. Hello, SUI!`,
    );
});

test('get sui faucet', async () => {
    const keypair = new Secp256k1Keypair();
    console.log("keypair", keypair);
    console.log("keypair getSecretKey ", keypair.getSecretKey());
    console.log("keypair toSuiAddress ", keypair.toSuiAddress());
    console.log("keypair getPublicKey ", keypair.getPublicKey().toSuiPublicKey());
});



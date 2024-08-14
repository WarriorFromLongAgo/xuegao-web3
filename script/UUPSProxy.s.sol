// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/proxy/ERC1967/ERC1967Proxy.sol";
import {Script, console} from "forge-std/Script.sol";
import {UUPSProxy} from "../src/UUPSProxy.sol";

contract UUPSProxyScript is Script {
    UUPSProxy public uupsProxy;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        UUPSProxy implementation = new UUPSProxy();

        console.log("UUPSProxy v1 address:", address(implementation));

        ERC1967Proxy proxy = new ERC1967Proxy(address(implementation), "");
        UUPSProxy(address(proxy)).initialize(0xa0Ee7A142d267C1f36714E4a8F75612F20a79720);

        vm.stopBroadcast();

        console.log("UUPS Proxy Address:", address(proxy));

        uint256 value = UUPSProxy(address(proxy)).getValue();
        console.log("run value ", value);
    }
}

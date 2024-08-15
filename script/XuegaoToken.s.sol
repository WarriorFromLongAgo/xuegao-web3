// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import "../src/XuegaoToken.sol";

contract CounterScript is Script {
    XuegaoToken public token;

    function setUp() public {}

    function run() public {
        // 部署XuegaoToken合约
        vm.startBroadcast();
        XuegaoToken xuegaoToken = new XuegaoToken();
        vm.stopBroadcast();

        // 初始化合约
        xuegaoToken.initialize("XuegaoToken", "XGT", msg.sender);
        // 设置总供应量和每个账户铸币限额（根据实际需求调整）
        xuegaoToken.setTotalSupply(1000);
        xuegaoToken.setMaxMintPerAccount(10);

        console.log("XuegaoToken deployed to:", address(xuegaoToken));
    }
}

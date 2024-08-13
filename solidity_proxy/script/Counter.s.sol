// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";
import "../src/CounterProxy.sol";

contract CounterScript is Script {
    Counter public counter;
    CounterProxy public counterProxy;
    address public test_user = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

    function setUp() public {}

    function run() public {
//        vm.startBroadcast();
        console.log("run msg.sender ", msg.sender);
        vm.startPrank(test_user);
        console.log("run msg.sender ", msg.sender);

        // 部署逻辑合约
        counter = new Counter();
        // 部署代理合约，初始指向逻辑合约
        counterProxy = new CounterProxy(address(counter));
        // 打印代理合约地址
        console.log("counter contract deployed at:", address(counter));
        // 打印逻辑合约地址
        console.log("counterProxy contract deployed at:", address(counterProxy));


        vm.stopPrank();
//        vm.stopBroadcast();
    }
}

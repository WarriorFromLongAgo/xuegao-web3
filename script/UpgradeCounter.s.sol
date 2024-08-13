// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";
import "../lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "../lib/openzeppelin-contracts/contracts/proxy/transparent/ProxyAdmin.sol";

contract UpgradeCounterScript is Script {
//    proxyAdmin  0x5FbDB2315678afecb367f032d93F642f64180aa3
    ProxyAdmin public proxyAdmin;
//    proxy  0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0
    TransparentUpgradeableProxy public proxy;

    address public newLogicContractAddress; // 新逻辑合约地址

    function setUp() public {
//        初始化 ProxyAdmin 和代理合约地址
        proxyAdmin = ProxyAdmin(0x5FbDB2315678afecb367f032d93F642f64180aa3); // ProxyAdmin 的地址
        proxy = TransparentUpgradeableProxy(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0); // 代理合约的地址
    }

    function run() public {
        // 开始广播，显示当前地址
        console.log("run msg.sender ", msg.sender);
        vm.startPrank(msg.sender);
        console.log("run msg.sender ", msg.sender);

        // 部署新的逻辑合约
        Counter newCounter = new Counter();
        console.log("run newCounter ", address(newCounter));
        newLogicContractAddress = address(newCounter);
        // 显示新逻辑合约地址
        console.log("New logic contract deployed at ", newLogicContractAddress);

        // 升级代理合约的逻辑合约
        proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(proxy)), newLogicContractAddress, "");

        // 显示升级后的合约地址
        console.log("Upgraded proxy logic contract to ", newLogicContractAddress);

        vm.stopPrank();
    }
}

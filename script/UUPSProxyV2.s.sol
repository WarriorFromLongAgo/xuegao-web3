// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import "../src/UUPSProxyV2.sol";
import {UUPSUpgradeable} from "../lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/UUPSUpgradeable.sol";

contract UUPSProxyScriptV2 is Script {
    UUPSProxyV2 public uupsProxyV2;
    address public proxyAddress = 0x5FC8d32690cc91D4c39d9d3abcBD16989F875707; // 代理合约地址
    address public adminAddress = 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720; // 代理合约的管理员地址

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

//        == Logs ==
//        UUPSProxy v1 address: 0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9
//        UUPS Proxy Address: 0x5FC8d32690cc91D4c39d9d3abcBD16989F875707
//        run value  10000
        UUPSProxyV2 newImplementation = new UUPSProxyV2();
        console.log("UUPSProxy v2 address:", address(newImplementation));

        // 通过代理合约地址创建逻辑合约的实例
        UUPSUpgradeable proxy = UUPSUpgradeable(proxyAddress);
        // 升级逻辑合约
        proxy.upgradeToAndCall(address(newImplementation), "");

        vm.stopBroadcast();
        console.log("UUPS Proxy v2 Address:", address(proxy));


        uint256 value = UUPSProxyV2(address(proxy)).getValue();
        console.log("run value ", value);

        vm.prank(adminAddress);
        UUPSProxyV2(address(proxy)).incrementValue();

        uint256 value2 = UUPSProxyV2(address(proxy)).getValue();
        console.log("run value2 ", value2);
    }
}

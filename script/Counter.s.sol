// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Counter} from "../src/Counter.sol";
import "../lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "../lib/openzeppelin-contracts/contracts/proxy/transparent/ProxyAdmin.sol";

contract CounterScript is Script {
    Counter public counter;
    ProxyAdmin public proxyAdmin;
    TransparentUpgradeableProxy public proxy;

    address public test_user = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    address public proxyAdminAddress;

    function setUp() public {}

    function run() public {
        //        vm.startBroadcast();
        console.log("run msg.sender ", msg.sender);
        vm.startPrank(test_user);
        console.log("run msg.sender ", msg.sender);

        // Deploy ProxyAdmin
        proxyAdmin = new ProxyAdmin(test_user);

        // Deploy LogicContract
        counter = new Counter();

        // Deploy TransparentUpgradeableProxy
        proxy = new TransparentUpgradeableProxy(
            address(counter),
            address(proxyAdmin),
            abi.encodeWithSelector(
                Counter.initialize.selector,
                test_user, // Owner
                999        // Initial number
            )
        );

        proxyAdminAddress = address(proxyAdmin);

        console.log("run counter ", address(counter));
        console.log("run proxy ", address(proxy));
        console.log("run proxyAdmin ", address(proxyAdmin));
        console.log("run proxyAdminAddress ", address(proxyAdminAddress));

        uint256 number = Counter(address(proxy)).getNumber();
        console.log("run number ", number);

        vm.stopPrank();
//        vm.stopBroadcast();
    }
}

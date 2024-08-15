// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

import {Script, console} from "forge-std/Script.sol";
import {XuegaoToken} from "../src/XuegaoToken.sol";

contract XuegaoTokenScript is Script {
    XuegaoToken public xuegaoToken;
    ProxyAdmin public proxy;
    address public testUser = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        uint256 totalSupply = 1000000000 * 1e18;
        xuegaoToken = new XuegaoToken();
        console.log("The Web3 xuegaoToken:", address(xuegaoToken));

        proxy = new ProxyAdmin(testUser);
        console.log("The Web3 proxy:", address(proxy));

        TransparentUpgradeableProxy proxyTheWebThreeTokenn = new TransparentUpgradeableProxy(
            address(xuegaoToken),
            address(proxy),
            abi.encodeWithSelector(XuegaoToken.initialize.selector, totalSupply, testUser)
        );
        console.log("TransparentUpgradeableProxy deployed at:", address(proxyTheWebThreeTokenn));


        vm.stopBroadcast();
    }
}

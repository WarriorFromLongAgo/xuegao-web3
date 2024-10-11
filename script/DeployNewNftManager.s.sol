// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";

import "../src/NewNftManager.sol";

import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin-foundry-upgrades/Upgrades.sol";

contract DeployNewNftManager is Script {

    NewNftManager public nftManagerImpl;

    function setUp() public {}

    function run() public {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployerAddress = vm.addr(deployerPrivateKey);

        vm.startBroadcast(deployerPrivateKey);

        nftManagerImpl = new NewNftManager();

        address proxyNftManager = Upgrades.deployTransparentProxy(
            "NewNftManager.sol:NewNftManager",
            deployerAddress,
            abi.encodeWithSelector(NewNftManager.initialize.selector, deployerAddress)
        );
        console.log("deploy nftManagerImpl:", address(nftManagerImpl));
        console.log("deploy proxyNftManager:", address(proxyNftManager));

        vm.stopBroadcast();
    }
}
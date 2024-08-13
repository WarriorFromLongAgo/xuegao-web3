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

    address public test_user = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;

    function setUp() public {
//        初始化 ProxyAdmin 和代理合约地址
        proxyAdmin = ProxyAdmin(0x5FbDB2315678afecb367f032d93F642f64180aa3); // ProxyAdmin 的地址
        proxy = TransparentUpgradeableProxy(payable(address(0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0)));
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
        // 编码初始化数据 版本1
        bytes memory initializeData = abi.encodeWithSignature(
            "initialize(address,uint256)",
            test_user,  // input_Owner
            2222       // input_number
        );
        proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(proxy)), newLogicContractAddress, initializeData);

        // Ensure the upgrade call is correct
//        proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(proxy)), newLogicContractAddress, bytes(""));

        // 显示升级后的合约地址
        console.log("Upgraded proxy logic contract to ", newLogicContractAddress);

        vm.stopPrank();
    }

    // Helper function to convert bytes to a hex string
    function toHexString(bytes memory data) internal pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(2 + data.length * 2);
        str[0] = "0";
        str[1] = "x";
        for (uint i = 0; i < data.length; i++) {
            str[2 + i * 2] = alphabet[uint(uint8(data[i] >> 4))];
            str[3 + i * 2] = alphabet[uint(uint8(data[i] & 0x0f))];
        }
        return string(str);
    }
}

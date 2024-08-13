// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import "../src/Counter.sol";
import "../src/Proxy.sol";

contract CounterScript_v2 is Script {
    Counter public counter;
    Proxy public proxy_counter;

    address public test_user = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    address public proxyAdminAddress;

    function setUp() public {
//        test_user = 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266;
    }

    function run() public {
        //        vm.startBroadcast();
        console.log("run msg.sender ", msg.sender);
        vm.startPrank(test_user);
        console.log("run msg.sender ", msg.sender);

        counter = new Counter();

        // 编码初始化数据 版本1
        bytes memory initializeData = abi.encodeWithSignature(
            "initialize(address,uint256)",
            test_user,  // input_Owner
            9999       // input_number
        );
        proxy_counter = new Proxy(address(counter), address(test_user), initializeData);

        // 编码初始化数据 版本2
//        proxy_counter = new Proxy(address(counter), address(test_user), "");
//        Counter(address(proxy_counter)).initialize(address(test_user), 8888);

        console.log("run counter ", address(counter));
        console.log("run proxy_counter ", address(proxy_counter));

        uint256 number = Counter(address(proxy_counter)).getNumber();
        console.log("run number ", number);


        vm.stopPrank();
//        vm.stopBroadcast();
    }
}

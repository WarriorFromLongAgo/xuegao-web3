// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {principle_number} from "../src/principle_number.sol";

contract CounterScript is Script {
    principle_number public counter;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        counter = new principle_number();

        vm.stopBroadcast();
    }
}

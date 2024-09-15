// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {principle_number} from "../src/principle_number.sol";

contract CounterTest is Test {
    principle_number public counter;

    function setUp() public {
        counter = new principle_number();
        counter.setNumber(0);
    }

    function test_Increment() public {
        counter.increment();
        assertEq(counter.number1(), 1);
    }

    function testFuzz_SetNumber(uint256 x) public {
        counter.setNumber(x);
        assertEq(counter.number1(), x);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_number {
    uint256 public number1;
    uint128 public number2;
    uint128 public number3;

    function setNumber(uint256 newNumber) public {
        number1 = newNumber;
    }

    function increment() public {
        number1++;
    }
}

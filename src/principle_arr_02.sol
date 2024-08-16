// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_arr_02 {
    uint128 public a = 5;
    uint256[] public myDynamicArray;
    uint64 public b = 1;
    uint256[5] public myStaticArray = [1, 2, 3, 4, 5]; // 存储在storage中，初始化为1-5
    uint64 public c = 1;

    function pushToArray(uint8 _value) public {
        myDynamicArray.push(_value); // 向动态数组中添加元素
    }
}

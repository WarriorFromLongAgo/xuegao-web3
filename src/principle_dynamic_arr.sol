// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_dynamic_arr {
    uint128 public a = 9;
    uint128[] public b;

    constructor(){
        b.push(10);
        b.push(11);
        b.push(12);
        b.push(12);
        b.push(12);
        b.push(12);
        b.push(12);
        b.push(12);
        b.push(12);
    }

    function getEncode(uint k, uint p) public pure returns(bytes memory){
        return abi.encode(k, p);
    }

    function getHash(bytes memory bb) public pure returns(bytes32){
        return keccak256(bb);
    }

    // 数组的slot计算公式，slot = keccak256(p)，其中p为数组状态变量在基本布局中的位置，此时b的位置p为1（状态变量a位置为0）
    // get slot of b[0] 时，variableStatePosition = 1, result = 0xb10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6
    // 对于1维数组只需要计算出第一个元素的slot即可，其他的元素依次排列，直到当前slot填满，再开启下一个slot
    function getSlot(uint128 variableStatePosition) public pure returns(bytes32){
        return keccak256(abi.encode(variableStatePosition));
    }
}

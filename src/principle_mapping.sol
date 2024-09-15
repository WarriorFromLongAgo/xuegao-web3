// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_mapping {
    uint32 public c = 5;
    mapping(uint256 => uint256) public a;
    uint256 public b = 5;

    constructor(){
        a[0] = 1;
        a[1] = 2;
        a[2] = 999999;
        a[3] = 8888;
        a[4] = 666;
    }

    function getEncode(uint k, uint p) public pure returns(bytes memory){
        return abi.encode(k, p);
    }

    function getHash(bytes memory bb) public pure returns(bytes32){
        return keccak256(bb);
    }

    // hashmap的slot计算公式：slot = keccak256(h(k) . p)，其中 . 意味着把前后2个值拼接到一起，类似于abi.encode(h(k), p)
    // get slot of a[0] 时 key = 0, p = 0, result = 0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5
    // get slot of a[1] 时 key = 1, p = 0, result = 0xada5013122d395ba3c54772283fb069b10426056ef8ca54750cb9bb552a59e7d
    function getSlot(uint key, uint p) public pure returns(bytes32){
        return keccak256(abi.encode(key, p));
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_struct {
    struct S {
        uint64 b;
//        uint32 c;
        uint256 d;
    }

    uint128 public a = 5;

//    S public s = S(2, 3, 4);
//    S public s = S(2, 3);
//    S public s = S(2);

    uint64 public d = 1;


    function setNumber() public {
        S memory tempS = S(2, 3);
    }

}

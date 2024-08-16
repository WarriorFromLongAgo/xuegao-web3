// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_struct {
    struct S {
        uint64 b;
        uint32 c;
    }

    uint128 public a = 5;

    S public s = S(2, 3);

    uint64 public d = 1;
}

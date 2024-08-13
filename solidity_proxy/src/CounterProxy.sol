// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract CounterProxy {
    address public implementation;
    address public admin;

    constructor(address _implementation) {
        implementation = _implementation;
        admin = msg.sender;
    }

    modifier onlyAdmin() {
        require(msg.sender == admin, "Not admin");
        _;
    }

    // 更改逻辑合约地址
    function setImplementation(address _implementation) external onlyAdmin {
        implementation = _implementation;
    }

    // 代理调用逻辑合约
    fallback() external payable {
        (bool success, ) = implementation.delegatecall(msg.data);
        require(success, "Delegatecall failed");
    }

    // 添加 receive 函数
    receive() external payable {}
}

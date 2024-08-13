// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "../lib/openzeppelin-contracts/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract Proxy is TransparentUpgradeableProxy {
    receive() external payable {}

    constructor(
        address contract_address,
        address input_owner,
        bytes memory input_data
    ) TransparentUpgradeableProxy(contract_address, input_owner, input_data) {}
}

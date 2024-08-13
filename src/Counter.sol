// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {console} from "forge-std/Script.sol";
import {Initializable} from "../lib/openzeppelin-contracts-upgradeable/contracts/proxy/utils/Initializable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/access/OwnableUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/access/AccessControlUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/utils/PausableUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/utils/ReentrancyGuardUpgradeable.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/utils/SafeERC20.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/IERC20.sol";

contract Counter is Initializable, ReentrancyGuardUpgradeable, OwnableUpgradeable {
    uint256 public number;

    mapping(address => uint256) private balances;

    constructor() {
        _disableInitializers();
    }

    function initialize(address input_Owner, uint256 input_number) public initializer {
        console.log("Counter initialize msg.sender ", msg.sender);
        console.log("Counter initialize input_Owner ", address(input_Owner));
        console.log("Counter initialize input_number ", input_number);

        __Ownable_init(input_Owner);

        __ReentrancyGuard_init();

        number = input_number;
    }

    function setNumber(uint256 input_number) external onlyOwner nonReentrant {
        number = input_number;
    }

    function getNumber() external view returns (uint256) {
        return number;
    }
}

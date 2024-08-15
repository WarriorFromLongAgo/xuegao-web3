// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";
import "../lib/openzeppelin-contracts/contracts/access/Ownable.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Burnable.sol";
import "../lib/openzeppelin-contracts/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ReentrancyGuard} from "../lib/openzeppelin-contracts/contracts/utils/ReentrancyGuard.sol";
import "../lib/openzeppelin-contracts/contracts/utils/Pausable.sol";
import {Initializable} from "../lib/openzeppelin-contracts/contracts/proxy/utils/Initializable.sol";
import "../lib/openzeppelin-contracts/contracts/proxy/Clones.sol";
import {console} from "forge-std/Script.sol";

contract XuegaoToken is Initializable, ERC20, ERC20Burnable, Ownable, ERC20Permit, ReentrancyGuard, Pausable {

    /// @notice 当传递给构造函数的输入中至少有一个为零值时抛出
    error XuegaoToken_ImproperlyInitialized();

    // 代币总供应量 = N
    uint256 public totalSupply;
    // 每个账户的铸币限额 = M
    uint256 public maxMintPerAccount;

    // 记录每个账户已 mint 的数量
    mapping(address => uint256) public mintedAmount;
    // 记录每个账户最后一次 mint 的时间戳
    mapping(address => uint256) public lastMintTimestamp;

    // 事件: 代币克隆成功
    event TokenCloned(address indexed cloneAddress);

    // 构造函数，设置代币名称、符号和初始供应量
    constructor() {
        _disableInitializers();
    }

    function initialize(string memory name, string memory symbol, address input_owner) public initializer {
        if (input_owner == address(0)) revert XuegaoToken_ImproperlyInitialized();

        ERC20(name, symbol);
        Ownable(input_owner);
        ERC20Permit(name);
        Pausable();
    }

    // M 和 N 可以配置。
    function setTotalSupply(uint256 input_totalSupply) public onlyOwner {
        totalSupply = input_totalSupply;
    }
    // M 和 N 可以配置。
    function setMaxMintPerAccount(uint256 input_maxMintPerAccount) public onlyOwner {
        maxMintPerAccount = input_maxMintPerAccount;
    }

    // 克隆代币合约，并初始化克隆合约
    function cloneToken(string memory name, string memory symbol) external returns (address) {
        address clone = Clones.clone(address(this));
        XuegaoToken(clone).initialize(name, symbol, msg.sender);

        emit TokenCloned(clone);

        return clone;
    }

    // 所有人都可以mint
    function all_mint(address to, string memory name, string memory symbol) external onlyOwner nonReentrant {
        require(to != address(0), "XuegaoToken: mint to the zero address");
        // 判断是否超过24小时
        require(currentTimestamp >= lastMintTimestamp[msg.sender], "Cannot mint within 24 hours");
        // 每个账户只能 mint M 份
        require(mintedAmount[msg.sender] + 1 <= maxMintPerAccount, "Exceeds max mint per account");
        mintedAmount[msg.sender] += 1;

        // 是否超过了代币总供应量
        require(totalSupply + 1 <= totalSupply, "Exceeds totalSupply");
        totalSupply -= 1;

        // 铸币时间有限制，不能连续铸币
        uint256 currentTimestamp = block.timestamp;
        console.log("all_mint currentTimestamp = ", currentTimestamp);
        uint256 oneDayInSeconds = 24 * 60 * 60;
        lastMintTimestamp[msg.sender] = currentTimestamp + oneDayInSeconds;

        // clone 合约
        address newToken = cloneToken(name, symbol);
        XuegaoToken(newToken).initialize(name, symbol, to);

        _mint(to, 1);
    }

    // 暂停合约功能，只有所有者可以调用
    function pause() external onlyOwner {
        super._pause();
    }

    // 恢复合约功能，只有所有者可以调用
    function unpause() external onlyOwner {
        super._unpause();
    }

    function _update(address from, address to, uint256 value) internal override {
        // 在这里添加自定义的逻辑，例如检查代币是否被冻结
        require(!paused(), "transfer paused");

        // 调用父类的 _update 函数
        super._update(from, to, value);
    }
}

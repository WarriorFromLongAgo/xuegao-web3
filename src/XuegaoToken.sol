// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "../lib/openzeppelin-contracts-upgradeable/contracts/token/ERC20/ERC20Upgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/access/OwnableUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/token/ERC20/extensions/ERC20BurnableUpgradeable.sol";
import "../lib/openzeppelin-contracts-upgradeable/contracts/token/ERC20/extensions/ERC20PermitUpgradeable.sol";
import {ReentrancyGuardUpgradeable} from "../lib/openzeppelin-contracts-upgradeable/contracts/utils/ReentrancyGuardUpgradeable.sol";

contract XuegaoToken is Initializable, ERC20Upgradeable, ERC20BurnableUpgradeable, OwnableUpgradeable, ERC20PermitUpgradeable, ReentrancyGuardUpgradeable {
    /* ========== STATE VARIABLES ========== */
    string private constant NAME = "Xuegao";
    string private constant SYMBOL = "XG";

    /// @dev The minimum amount of time that must elapse before a mint is allowed
    /// @dev 允许铸币厂之前必须经过的最短时间
    uint256 public constant MIN_MINT_INTERVAL = 365 days;

    /// @dev The denominator of the maximum fractional amount that can be minted
    /// @dev 可以铸造的最大小数金额的分母
    uint256 public constant MINT_CAP_DENOMINATOR = 10_000;

    /// @dev The numerator of the maximum fractional amount that can be minted
    /// @dev 可以铸造的最大小数金额的分子
    uint256 public constant MINT_CAP_MAX_NUMERATOR = 200;

    /// @dev The current numerator of the fractional amount that can be minted
    /// @dev 可以铸造的小数金额的当前分子
    uint256 public mintCapNumerator;

    /// @dev The blockTimeStamp at which mint will be able to be called again
    /// @dev mint 可以再次调用的 blockTimeStamp
    uint256 public nextMint;

    /// @dev Emitted when the mintCapNumerator is set
    /// @param from The address which changed the mintCapNumerator
    /// @param mintCapNumerator The previous mintCapNumerator
    /// @param newMintCapNumerator The new mintCapNumerator
    event MintCapNumeratorChanged(address indexed from, uint256 mintCapNumerator, uint256 newMintCapNumerator);

    /* ========== ERRORS ========== */
    /// @notice Thrown when at least one of the inputs passed into the constructor is a zero value
    /// @notice 当传递给构造函数的输入中至少有一个为零值时抛出
    error XuegaoToken_ImproperlyInitialized();

    /// @notice Thrown when mint is called before the nextMint timestamp has passed
    error XuegaoToken_NextMintTimestampNotElapsed(uint256 currentTimestamp, uint256 nextMintTimestamp);

    /// @notice Thrown when mint is called with a value greater than the maximum allowed
    error XuegaoToken_MintAmountTooLarge(uint256 amount, uint256 maximumAmount);

    /// @notice Thrown when the mintCapNumerator is set to a value greater than the maximum allowed
    error XuegaoToken_MintCapNumeratorTooLarge(uint256 numerator, uint256 maximumNumerator);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    /* ========== INITIALIZER ========== */

    /// @notice Initializes the TheWebThreeToken.sol contract, setting the inital total supply as {initialSupply} and the owner as {_owner}
    /// @dev the mintCapNumerator should not be set as it is initialized as 0
    /// @dev Requirements:
    ///     - all parameters must be non-zero
    /// @param input_initialSupply The initial total supply of the token
    /// @param input_owner The owner of the token
    function initialize(uint256 input_initialSupply, address input_owner) public initializer {
        if (input_initialSupply == 0 || input_owner == address(0)) revert XuegaoToken_ImproperlyInitialized();

        __ReentrancyGuard_init();

//        初始化 ERC20 代币的名称和符号
        __ERC20_init(NAME, SYMBOL);
//        初始化可烧毁代币的功能。
        __ERC20Burnable_init();
//        设置合约所有者
        __Ownable_init(input_owner);
//        初始化 ERC20 许可机制，用于支持 EIP-2612
        __ERC20Permit_init(NAME);

        _mint(input_owner, input_initialSupply);
        nextMint = block.timestamp + MIN_MINT_INTERVAL;
    }

    /// @notice Allows the owner to mint new tokens and increase this token's total supply
    /// @dev Requirements:
    ///     - Only allows minting below an inflation cap at a specified time interval
    ///         - The max mint amount is computed as follows:
    ///             - maxMintAmount = (mintCapNumerator * totalSupply()) / MINT_CAP_DENOMINATOR
    ///              - The specified time interval at which mints can occur is 1 year (365 days)
    ///     - the parameter {amount} must be less than or equal to {maxMintAmount} as computed above
    ///     - the {blockTimestamp} of the block in which this function is called must be greater than or equal to {nextMint}
    /// @param input_account The address to mint tokens to
    /// @param inpuitAmount The amount of tokens to mint
    function xuegao_mint(address input_account, uint256 inpuitAmount) public onlyOwner nonReentrant {
//        分析此次可以 mint 多少
        uint256 maximumMintAmount = (totalSupply() * mintCapNumerator) / MINT_CAP_DENOMINATOR;
        if (inpuitAmount > maximumMintAmount) {
            revert XuegaoToken_MintAmountTooLarge(inpuitAmount, maximumMintAmount);
        }
        if (block.timestamp < nextMint) revert XuegaoToken_NextMintTimestampNotElapsed(block.timestamp, nextMint);

        nextMint = block.timestamp + MIN_MINT_INTERVAL;
        super._mint(input_account, inpuitAmount);
    }

    /// @notice Allows the owner to set the mintCapNumerator
    /// @dev emits a {MintCapNumeratorChanged} event
    /// @dev Requirements:
    ///     - The caller must be the contract owner
    ///     - {_mintCapNumerator} must be less than or equal to {MINT_CAP_MAX_NUMERATOR}
    /// @param input_mintCapNumerator The new numerator of the mint cap
    function xuegao_dealMintCapNumerator(uint256 input_mintCapNumerator) public onlyOwner nonReentrant {
//        设置最小可mint的百分比
        if (input_mintCapNumerator > MINT_CAP_MAX_NUMERATOR) {
            revert XuegaoToken_MintCapNumeratorTooLarge(input_mintCapNumerator, MINT_CAP_MAX_NUMERATOR);
        }
        emit MintCapNumeratorChanged(msg.sender, mintCapNumerator, input_mintCapNumerator);
        mintCapNumerator = input_mintCapNumerator;
    }
}

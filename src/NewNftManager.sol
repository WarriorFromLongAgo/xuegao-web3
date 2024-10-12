// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/Base64.sol";

contract NewNftManager is Initializable, ERC721Upgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    using Strings for uint256;

    uint256 private _nextTokenId;
    mapping(uint256 => uint8) public nftMintType;

    string public basicNftJson;
    string public proNftJson;

    event CreateNFT(
        address indexed creator,
        uint256 tokenId,
        uint8 nftType
    );

    event UpdatedNftJson(uint8 nftType, string newJsonUrl);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _initialOwner) public initializer {
        require(_initialOwner != address(0), "NewNftManager initialize: _initialOwner can't be zero address");
        __ERC721_init("Fishcake Pass NFT", "FNFT");
        __Ownable_init(_initialOwner);
        __ReentrancyGuard_init();

        basicNftJson = "https://www.fishcake.org/image/2.json";
        proNftJson = "https://www.fishcake.org/image/1.json";
    }

    function createNFT(uint8 _type) external nonReentrant returns (uint256) {
        require(_type == 1 || _type == 2, "NewNftManager createNFT: type can only equal 1 or 2");

        uint256 tokenId = _nextTokenId++;
        _safeMint(msg.sender, tokenId);
        nftMintType[tokenId] = _type;

        emit CreateNFT(msg.sender, tokenId, _type);

        return tokenId;
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(_ownerOf(tokenId) != address(0), "ERC721Metadata: URI query for nonexistent token");

        uint8 nftType = nftMintType[tokenId];
        return nftType == 1 ? basicNftJson : proNftJson;
    }

    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return super.supportsInterface(interfaceId);
    }

    function updateNftJson(uint8 _type, string memory _newJsonUrl) external onlyOwner {
        require(_type == 1 || _type == 2, "Invalid NFT type");
        if (_type == 1) {
            basicNftJson = _newJsonUrl;
        } else {
            proNftJson = _newJsonUrl;
        }
        emit UpdatedNftJson(_type, _newJsonUrl);
    }
}
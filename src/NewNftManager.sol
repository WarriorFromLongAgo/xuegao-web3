// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721URIStorageUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "@openzeppelin/contracts-upgradeable/utils/ReentrancyGuardUpgradeable.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract NewNftManager is Initializable, ERC721Upgradeable, ERC721URIStorageUpgradeable, OwnableUpgradeable, ReentrancyGuardUpgradeable {
    using Strings for uint256;

    uint256 private _nextTokenId;
    mapping(uint256 => uint8) public nftMintType;

    string public constant BASIC_NFT_URL = "https://www.fishcake.io/_next/image?url=%2Ficons%2Ffishcake%2Fnft-basic-create.png&w=256&q=75";
    string public constant PRO_NFT_URL = "https://www.fishcake.io/_next/image?url=%2Ficons%2Ffishcake%2Fnft-pro-create.png&w=256&q=75";

    event CreateNFT(
        address indexed creator,
        uint256 tokenId,
        uint8 nftType
    );

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _initialOwner) public initializer {
        require(_initialOwner != address(0), "NewNftManager initialize: _initialOwner can't be zero address");
        __ERC721_init("Fishcake Pass NFT", "FNFT");
        __ERC721URIStorage_init();
        __Ownable_init(_initialOwner);
        __ReentrancyGuard_init();
    }

    function createNFT(uint8 _type) external nonReentrant returns (uint256) {
        require(_type == 1 || _type == 2, "NewNftManager createNFT: type can only equal 1 or 2");

        uint256 tokenId = _nextTokenId++;
        _safeMint(msg.sender, tokenId);
        nftMintType[tokenId] = _type;
        string memory uri = _type == 1 ? BASIC_NFT_URL : PRO_NFT_URL;
        _setTokenURI(tokenId, uri);

        emit CreateNFT(msg.sender, tokenId, _type);

        return tokenId;
    }

    function tokenURI(uint256 tokenId) public view override(ERC721Upgradeable, ERC721URIStorageUpgradeable) returns (string memory) {
        return super.tokenURI(tokenId);
    }

    function supportsInterface(bytes4 interfaceId) public view override(ERC721Upgradeable, ERC721URIStorageUpgradeable) returns (bool) {
        return super.supportsInterface(interfaceId);
    }

}
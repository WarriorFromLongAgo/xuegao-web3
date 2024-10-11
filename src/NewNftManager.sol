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
    mapping(uint256 => string) public nftImageUrls;

    event CreateNFT(
        address indexed creator,
        uint256 tokenId,
        uint8 nftType
    );

    event UpdatedNftImage(uint8 nftType, string newImageUrl);

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address _initialOwner) public initializer {
        require(_initialOwner != address(0), "NewNftManager initialize: _initialOwner can't be zero address");
        __ERC721_init("Fishcake Pass NFT", "FNFT");
        __Ownable_init(_initialOwner);
        __ReentrancyGuard_init();
    }

    function createNFT(uint8 _type, string memory _imageUrl) external nonReentrant returns (uint256) {
        require(_type == 1 || _type == 2, "NewNftManager createNFT: type can only equal 1 or 2");

        uint256 tokenId = _nextTokenId++;
        _safeMint(msg.sender, tokenId);
        nftMintType[tokenId] = _type;
        nftImageUrls[tokenId] = _imageUrl;

        emit CreateNFT(msg.sender, tokenId, _type);

        return tokenId;
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        require(_ownerOf(tokenId) != address(0), "ERC721Metadata: URI query for nonexistent token");

        uint8 nftType = nftMintType[tokenId];
        string memory image = nftImageUrls[tokenId];
        string memory name = nftType == 1 ? "Fishcake Basic Pass" : "Fishcake Pro Pass";
        string memory description = nftType == 1 ? "A Fishcake Basic Pass" : "A Fishcake Pro Pass";

        string memory json = Base64.encode(
            bytes(
                string(
                    abi.encodePacked(
                        '{"name": "', name, '", ',
                        '"description": "', description, '", ',
                        '"image": "', image, '"}'
                    )
                )
            )
        );

        return string(abi.encodePacked("data:application/json;base64,", json));
    }

    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        return super.supportsInterface(interfaceId);
    }

}
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract principle_byte_string {
    string public shortString = "WTFewrewrewrwerewrwrwreewtertert3t34sgfdgf53454dsgfsdgfwe43523423frdsfwsdr42342dfshjdfgjhewr3738rsjhdfh7832yr372yrjhsdgfjdshf73y8ryjhdsfs";
    bytes public longBytes = hex"365f5f375f5f365f73bebebebedadadaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabebebebebebebebebebebebebebebebe5af43d5f5f3e5f3d91602a57fd5bf3";

    function getHash(bytes memory bb) public pure returns(bytes32){
        return keccak256(bb);
    }
}

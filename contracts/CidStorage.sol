// SPDX-License-Identifier: GPL-3.0
pragma solidity >=0.5;

/**
 * @title CID Storage
 * @dev Store & retrieve a CID
 */
contract CidStorage {
    string cid;

    /**
     * @dev Store CID
     * @param new_cid CID to store
     */
    function store(string memory new_cid) public {
        cid = new_cid;
    }

    /**
     * @dev Return CID 
     * @return value of 'cid'
     */
    function retrieve() public view returns (string memory){
        return cid;
    }
}

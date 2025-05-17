// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract Pinger {
    uint256 public number;

    event Ping(address indexed caller, uint256 newNumber);
    event Poke(address indexed caller, uint256 oldNumber, uint256 newNumber);
    event StressRelieved(address indexed caller, uint256 newNumber);

    function ping() external {
        number++; 
        emit Ping(msg.sender, number);
    }

    function poke(uint256 how_much_to_annoy) external {
        require(how_much_to_annoy > 0, "0 pokes not fun");
        uint256 old_number = number;
        number += how_much_to_annoy;
        emit Poke(msg.sender, old_number, number);
    }

    function relieveStress() external {
        number = 0;
        emit StressRelieved(msg.sender, number);
    }

    function getAnnoyance() public view returns (uint256) {
        return number;
    }

    // Yay solidity overflow checks
}
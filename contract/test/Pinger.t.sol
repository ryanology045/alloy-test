// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console} from "forge-std/Test.sol";
import {Pinger} from "../src/Pinger.sol";

contract PingerTest is Test {
    Pinger public pinger;

    function initialize() public {
        pinger = new Pinger();
        pinger.relieveStress();
    }

    function ping() public {
        pinger.ping();
        assertEq(pinger.number(), 1);
    }

    function fuzzPoke(uint256 x) public {
        pinger.poke(x);
        assertEq(pinger.number(), x);
    }

    // other tests too lazy to write
}

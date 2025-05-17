// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "../lib/forge-std/src/Script.sol";
import "../lib/forge-std/src/console.sol";
import "../src/Pinger.sol";

contract DeployPinger is Script {
    function run() external returns (Pinger pinger) {
        vm.startBroadcast();
        Pinger pinger = new Pinger();

        console.log("Pinger contract address:", address(pinger));

        vm.stopBroadcast();
    }
}




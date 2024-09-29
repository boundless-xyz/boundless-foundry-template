// Copyright (c) 2024 RISC Zero, Inc.
//
// All rights reserved.

pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {EvenNumber} from "../src/EvenNumber.sol";

contract Deploy is Script {
    function run() external {
        // load ENV variables first
        uint256 key = vm.envUint("WALLET_PRIVATE_KEY");
        address verifierAddress = vm.envAddress("SET_VERIFIER_ADDRESS");
        vm.startBroadcast(key);

        IRiscZeroVerifier verifier = IRiscZeroVerifier(verifierAddress);
        EvenNumber evenNumber = new EvenNumber(verifier);
        address evenNumberAddress = address(evenNumber);
        console2.log("Deployed EvenNumber to", evenNumberAddress);

        vm.stopBroadcast();
    }
}

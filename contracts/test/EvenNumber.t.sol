// Copyright (c) 2024 RISC Zero, Inc.
//
// All rights reserved.

pragma solidity ^0.8.20;

import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {Receipt as RiscZeroReceipt} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0/test/RiscZeroMockVerifier.sol";
import {EvenNumber} from "../src/EvenNumber.sol";
import {ImageID} from "../src/ImageID.sol";

contract EvenNumberTest is RiscZeroCheats, Test {
    EvenNumber public evenNumber;
    RiscZeroMockVerifier public verifier;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(0);
        evenNumber = new EvenNumber(verifier);
        assertEq(evenNumber.get(), 0);
    }

    function test_SetEven() public {
        uint256 number = 12345678;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_EVEN_ID, sha256(abi.encode(number)));

        evenNumber.set(number, receipt.seal);
        assertEq(evenNumber.get(), number);
    }

    function test_SetZero() public {
        uint256 number = 0;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_EVEN_ID, sha256(abi.encode(number)));

        evenNumber.set(number, receipt.seal);
        assertEq(evenNumber.get(), number);
    }
}

// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pragma solidity ^0.8.20;

import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {Receipt as RiscZeroReceipt} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0/test/RiscZeroMockVerifier.sol";
import {VerificationFailed} from "risc0/IRiscZeroVerifier.sol";
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

    // Try using a proof for the evenness of 4 to set 1 on the contract.
    function test_RejectInvalidProof() public {
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.IS_EVEN_ID, sha256(abi.encode(4)));

        vm.expectRevert(VerificationFailed.selector);
        evenNumber.set(1, receipt.seal);
    }
}

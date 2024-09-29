// Copyright (c) 2024 RISC Zero, Inc.
//
// All rights reserved.

pragma solidity ^0.8.20;

interface IEvenNumber {
    function set(uint256 x, bytes calldata seal) external;
    function get() external view returns (uint256);
}

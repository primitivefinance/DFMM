// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp, ConstantSum } from "./SetUp.sol";

contract ConstantSumConstructorTest is ConstantSumSetUp {
    function test_ConstantSum_constructor() public {
        ConstantSum constantSum = new ConstantSum(address(dfmm));
        assertEq(constantSum.dfmm(), address(dfmm));
    }
}

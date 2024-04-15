// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSolver } from "src/ConstantSum/ConstantSumSolver.sol";
import { ConstantSumSetUp } from "./SetUp.sol";

contract ConstantSumValidateSwapTest is ConstantSumSetUp {
    function test_ConstantSum_simulateSwap_RevertsInvalidSwapX()
        public
        defaultPool
    {
        uint256 amountIn = 1.1 ether;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.prepareSwap(POOL_ID, 0, 1, amountIn);
    }

    function test_ConstantSum_simulateSwap_RevertsInvalidSwapY()
        public
        defaultPool
    {
        uint256 amountIn = 2.1 ether;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.prepareSwap(POOL_ID, 1, 0, amountIn);
    }
}

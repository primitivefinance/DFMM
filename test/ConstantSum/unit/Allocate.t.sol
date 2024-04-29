// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp } from "./SetUp.sol";
import {
    computeDeltaLiquidityRoundDown,
    ConstantSumParams
} from "src/ConstantSum/ConstantSumMath.sol";

contract ConstantSumAllocateTest is ConstantSumSetUp {
    function test_ConstantSum_allocate_Works() public defaultPool {
        uint256 deltaX = 0.1 ether;
        uint256 deltaY = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = deltaY;

        ConstantSumParams memory params =
            abi.decode(constantSum.getPoolParams(POOL_ID), (ConstantSumParams));

        uint256 deltaL =
            computeDeltaLiquidityRoundDown(deltaX, deltaY, params.price);
        dfmm.allocate(POOL_ID, abi.encode(deltaX, deltaY, deltaL));
    }
}

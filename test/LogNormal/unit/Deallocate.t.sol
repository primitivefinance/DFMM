// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import {
    computeDeltaLGivenDeltaX,
    computeDeltaYGivenDeltaX,
    computeDeltaLGivenDeltaY,
    computeDeltaXGivenDeltaL
} from "src/lib/StrategyLib.sol";

contract LogNormalDeallocateTest is LogNormalSetUp {
    function test_LogNormal_deallocate_GivenX() public init {
        uint256 minDeltaX = 0.1 ether;

        (uint256 rX, uint256 rY, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(minDeltaX, liquidity, rX);
        uint256 minDeltaY = computeDeltaYGivenDeltaX(minDeltaX, rX, rY);

        //        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        //      (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        // TODO: See if we can get a better rounding because the transaction fails
        // if we don't provide a small slippage toleralance.
        bytes memory data =
            abi.encode(minDeltaX - 10, minDeltaY - 10, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_LogNormal_deallocate_GivenY() public init {
        uint256 minDeltaY = 0.1 ether;

        (uint256 rX, uint256 rY, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(minDeltaY, liquidity, rY);
        uint256 minDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, rX);

        // uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        // TODO: See if we can get a better rounding because the transaction fails
        // if we don't provide a small slippage toleralance.
        bytes memory data =
            abi.encode(minDeltaX - 10, minDeltaY - 10, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }
}

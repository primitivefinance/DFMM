// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { computeDeltaGivenDeltaLRoundUp } from "src/LogNormal/LogNormalMath.sol";
import {
    computeDeltaLGivenDeltaY,
    computeDeltaXGivenDeltaL
} from "src/lib/StrategyLib.sol";

contract LogNormalAllocateTest is LogNormalSetUp {
    function test_LogNormal_allocate_GivenX() public init {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity = 0.1 ether;
        uint256 maxDeltaX = computeDeltaGivenDeltaLRoundUp(
            reserves[0], deltaLiquidity, totalLiquidity
        );
        uint256 maxDeltaY = computeDeltaGivenDeltaLRoundUp(
            reserves[1], deltaLiquidity, totalLiquidity
        );

        // uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        uint256[] memory deltas = dfmm.allocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_LogNormal_allocate_GivenY() public init {
        uint256 maxDeltaY = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(maxDeltaY, liquidity, reserves[1]);
        uint256 maxDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserves[0]);

        // uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { computeDeltaGivenDeltaLRoundUp } from
    "src/SYCoveredCall/SYCoveredCallMath.sol";
import {
    computeDeltaLGivenDeltaY,
    computeDeltaLGivenDeltaX,
    computeDeltaXGivenDeltaL,
    computeDeltaYGivenDeltaL
} from "src/lib/StrategyLib.sol";

contract SYCoveredCallAllocateTest is SYCoveredCallSetUp {
    function test_SYCoveredCall_allocate_GivenL() public init {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity = 0.1 ether;
        uint256 maxDeltaX = computeDeltaGivenDeltaLRoundUp(
            reserves[0], deltaLiquidity, totalLiquidity
        );
        uint256 maxDeltaY = computeDeltaGivenDeltaLRoundUp(
            reserves[1], deltaLiquidity, totalLiquidity
        );

        (, uint256 preTotalLiquidity) = solver.getReservesAndLiquidity(POOL_ID);
        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        console2.log(preTotalLiquidity);
        console2.log(preLiquidityBalance);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (, uint256 postTotalLiquidity) = solver.getReservesAndLiquidity(POOL_ID);
        uint256 postLiquidityBalance = liquidityOf(address(this), POOL_ID);
        console2.log(postTotalLiquidity);
        console2.log(postLiquidityBalance);

        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        uint256 deltaLiquidityBalance =
            postLiquidityBalance - preLiquidityBalance;

        assertEq(deltaTotalLiquidity, deltaLiquidityBalance);
    }

    function test_SYCoveredCall_allocate_GivenX() public init {
        uint256 deltaX = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(deltaX, liquidity, reserves[0]);
        uint256 deltaYMax =
            computeDeltaYGivenDeltaL(deltaLiquidity, liquidity, reserves[1]);
        // uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(deltaX, deltaYMax, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }

    // if we assert positive invariant, not less than epsilon, can someone sandwich a tx whereby the put the invariant to some extremely large number in front of an allocate?

    function test_SYCoveredCall_allocate_GivenY() public init {
        uint256 maxDeltaY = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(maxDeltaY, liquidity, reserves[1]);
        uint256 maxDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserves[0]);
        console2.log(maxDeltaX);

        // uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        /*
        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_SYCoveredCall_allocate_x_maintains_price() public init {
        uint256 startPrice = solver.internalPrice(POOL_ID);
        uint256 deltaX = 0.77 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(deltaX, liquidity, reserves[0]);
        uint256 deltaYMax =
            computeDeltaYGivenDeltaL(deltaLiquidity, liquidity, reserves[1]);

        bytes memory data = abi.encode(deltaX, deltaYMax, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        uint256 endPrice = solver.internalPrice(POOL_ID);

        assertEq(startPrice, endPrice);
    }

    function test_SYCoveredCall_allocate_y_maintains_price() public init {
        uint256 maxDeltaY = 0.77 ether;
        uint256 startPrice = solver.internalPrice(POOL_ID);

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(maxDeltaY, liquidity, reserves[1]);
        uint256 maxDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserves[0]);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);
        uint256 endPrice = solver.internalPrice(POOL_ID);

        assertEq(startPrice, endPrice);
    }
}

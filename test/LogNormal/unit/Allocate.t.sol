// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { computeDeltaGivenDeltaLRoundUp } from "src/LogNormal/LogNormalMath.sol";
import {
    computeDeltaLGivenDeltaY,
    computeDeltaLGivenDeltaX,
    computeDeltaXGivenDeltaL,
    computeDeltaYGivenDeltaL
} from "src/lib/StrategyLib.sol";

contract LogNormalAllocateTest is LogNormalSetUp {
    function test_LogNormal_allocate_GivenL() public init {
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

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
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

    function test_LogNormal_single_sided_allocate() public init {
        uint256 deltaX = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        console2.log("X", reserves[0]);
        console2.log("Y", reserves[1]);
        console2.log("L", liquidity);

        uint256 estimatedL = solver.getNextLiquidity(
            POOL_ID, reserves[0] + deltaX, reserves[1], liquidity
        );
        uint256 deltaLiquidity = estimatedL - liquidity;

        bool swapXForY = true;

        (bool valid, uint256 amountOut,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXForY, deltaX);

        console2.log("Amount out: ", amountOut);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = 0;

        console2.log("X", reserves[0] + deltas[0]);
        console2.log("Y", reserves[1] + deltas[1]);
        console2.log("L", liquidity + deltaLiquidity);

        console2.log(
            "Pool reserves before allocation: ", reserves[0], reserves[1]
        );
        console2.log("Liquidity before allocation: ", liquidity);

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postLiquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        console2.log(
            "Pool reserves after allocation: ", postReserves[0], postReserves[1]
        );
        console2.log("Liquidity after allocation: ", postLiquidity);

        console2.log(
            "Deltas",
            postReserves[0] - reserves[0],
            postReserves[1] - reserves[1],
            postLiquidity - liquidity
        );
    }

    function logPoolState() public view {
        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        console2.log("X", reserves[0]);
        console2.log("Y", reserves[1]);
        console2.log("L", liquidity);
    }

    function test_LogNormal_single_sided_allocate_deallocate() public init {
        uint256 deltaX = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 estimatedL = solver.getNextLiquidity(
            POOL_ID, reserves[0] + deltaX, reserves[1], liquidity
        );
        uint256 deltaLiquidity = estimatedL - liquidity;

        bool swapXForY = true;

        (bool valid, uint256 amountOut,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXForY, deltaX);

        console2.log("Estimated delta y given delta x: ", amountOut);

        uint256 estimatedLiquidityPayment = computeDeltaLXIn(
            deltaX,
            reserves[0],
            reserves[1],
            liquidity,
            solver.getPoolParams(POOL_ID)
        );
        console2.log("Estimated liquidity payment: ", estimatedLiquidityPayment);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = 0;

        logPoolState();

        console2.log("Allocate x");

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postLiquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        logPoolState();

        deltas[0] = 0;
        deltas[1] = amountOut * 1 ether / (1 ether + TEST_SWAP_FEE); // try to get the amount out excluding the swap fee

        estimatedL = solver.getNextLiquidity(
            POOL_ID, postReserves[0], postReserves[1] - deltas[1], postLiquidity
        );
        deltaLiquidity = postLiquidity - estimatedL;

        console2.log("Deallocate y");

        data = abi.encode(deltas, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (postReserves, postLiquidity) = solver.getReservesAndLiquidity(POOL_ID);

        logPoolState();
    }

    function test_LogNormal_allocate_GivenX() public init {
        uint256 deltaX = 0.1 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(deltaX, liquidity, reserves[0]);
        uint256 deltaYMax =
            computeDeltaYGivenDeltaL(deltaLiquidity, liquidity, reserves[1]);
        // uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = deltaYMax;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
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

    function test_LogNormal_allocate_GivenY() public init {
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

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
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

    function test_LogNormal_allocate_x_maintains_price() public init {
        uint256 startPrice = solver.internalPrice(POOL_ID);
        uint256 deltaX = 0.77 ether;

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(deltaX, liquidity, reserves[0]);
        uint256 deltaYMax =
            computeDeltaYGivenDeltaL(deltaLiquidity, liquidity, reserves[1]);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = deltaYMax;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        uint256 endPrice = solver.internalPrice(POOL_ID);

        assertEq(startPrice, endPrice);
    }

    function test_LogNormal_allocate_y_maintains_price() public init {
        uint256 maxDeltaY = 0.77 ether;
        uint256 startPrice = solver.internalPrice(POOL_ID);

        (uint256[] memory reserves, uint256 liquidity) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(maxDeltaY, liquidity, reserves[1]);
        uint256 maxDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserves[0]);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);
        uint256 endPrice = solver.internalPrice(POOL_ID);

        assertEq(startPrice, endPrice);
    }
}

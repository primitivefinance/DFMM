// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";
import { LPToken } from "src/LPToken.sol";

contract G3MAllocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_allocate() public init {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](2);
        deltas[0] = 1 ether;
        deltas[1] = 1 ether;
        bytes memory allocateData = solver.prepareAllocation(POOL_ID, deltas);
        (, uint256 deltaLiquidity) =
            abi.decode(allocateData, (uint256[], uint256));

        (deltas) = dfmm.allocate(POOL_ID, allocateData);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);
    }

    function test_G3M_allocate_GivenX_large_delta() public init {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256 maxDeltaX = 10_000 ether;

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;

        bytes memory allocateData =
            solver.prepareAllocationProportional(POOL_ID, deltas);
        (uint256[] memory usedDeltas, uint256 deltaLiquidity) =
            abi.decode(allocateData, (uint256[], uint256));

        (deltas) = dfmm.allocate(POOL_ID, allocateData);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + usedDeltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + usedDeltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);
    }

    function test_G3M_allocate_MultipleTimes() public init {
        uint256[] memory deltas = new uint256[](2);
        deltas[0] = 1 ether;
        deltas[1] = 1 ether;
        bytes memory allocateData = solver.prepareAllocation(POOL_ID, deltas);

        deltas[0] = deltas[0].mulDivUp(101, 100);
        deltas[1] = deltas[1].mulDivUp(101, 100);
        bytes memory allocateData2 = solver.prepareAllocation(POOL_ID, deltas);
        (, uint256 deltaLiquidity) =
            abi.decode(allocateData2, (uint256[], uint256));

        dfmm.allocate(POOL_ID, allocateData);
        dfmm.allocate(POOL_ID, allocateData2);
    }

    /// todo: need to replace this with proper min liquidity minted checks
    function test_G3M_allocate_RevertsIfMoreThanMaxDeltaX() public init {
        skip();
        // todo: adjust to check for liquidity slippage
    }

    function test_G3M_allocate_GivenY() public init {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256 maxDeltaY = 0.1 ether;
        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[1] = maxDeltaY;

        bytes memory allocateData =
            solver.prepareAllocationProportional(POOL_ID, deltas);

        (uint256[] memory usedDeltas, uint256 deltaLiquidity) =
            abi.decode(allocateData, (uint256[], uint256));

        (deltas) = dfmm.allocate(POOL_ID, allocateData);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + usedDeltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + usedDeltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);
    }

    function test_G3M_allocate_ReceiveAppropriateLpTokens() public init_100 {
        (uint256[] memory reserves, uint256 initialL) = getReservesAndLiquidity(POOL_ID);
        Pool memory pool = dfmm.pools(POOL_ID);
        LPToken liquidityToken = LPToken(pool.liquidityToken);

        uint256 startBalance = liquidityToken.balanceOf(address(this));

        uint[] memory deltas = new uint[](reserves.length);
        deltas[1] = 100 ether;
        
        bytes memory allocateData = solver.prepareAllocationProportional(POOL_ID, deltas);

        dfmm.allocate(POOL_ID, allocateData);

        (, uint256 nextL) = getReservesAndLiquidity(POOL_ID);
        uint256 endBalance = liquidityToken.balanceOf(address(this));

        // Add 1_000 wei to account for liquidity that was burnt on init
        assertEq(startBalance + 1000, initialL);
        assertEq(endBalance + 1000, nextL);
    }
}

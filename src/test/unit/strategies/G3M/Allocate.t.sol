// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MAllocateTest is G3MSetUp {
    function test_G3M_allocate_GivenX() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY) = dfmm.allocate(POOL_ID, data);

        (
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedLiquidity
        ) = dfmm.getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserveX, reserveX + deltaX);
        assertEq(adjustedReserveY, reserveY + deltaY);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_G3M_allocate_GivenY() public init {
        uint256 maxDeltaY = 0.1 ether;

        (uint256 maxDeltaX, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaY(POOL_ID, maxDeltaY);
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY) = dfmm.allocate(POOL_ID, data);

        (
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedLiquidity
        ) = dfmm.getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserveX, reserveX + deltaX);
        assertEq(adjustedReserveY, reserveY + deltaY);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }
}

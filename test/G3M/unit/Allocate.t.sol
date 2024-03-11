// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";

contract G3MAllocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_allocate_GivenX() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);
        (uint256[] memory reserves, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256[] memory deltas) = dfmm.allocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_G3M_allocate_MultipleTimes() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);

        bytes memory data = abi.encode(
            maxDeltaX.mulDivUp(101, 100),
            maxDeltaY.mulDivUp(101, 100),
            deltaLiquidity
        );
        dfmm.allocate(POOL_ID, data);
        dfmm.allocate(POOL_ID, data);
    }

    function test_G3M_allocate_RevertsIfMoreThanMaxDeltaX() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);

        bytes memory data = abi.encode(maxDeltaX - 1, maxDeltaY, deltaLiquidity);
        vm.expectRevert();
        dfmm.allocate(POOL_ID, data);
    }

    function test_G3M_allocate_RevertsIfMoreThanMaxDeltaY() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY - 1, deltaLiquidity);
        vm.expectRevert();
        dfmm.allocate(POOL_ID, data);
    }

    function test_G3M_allocate_GivenY() public init {
        uint256 maxDeltaY = 0.1 ether;

        (uint256 maxDeltaX, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaY(POOL_ID, maxDeltaY);
        (uint256[] memory reserves, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256[] memory deltas) = dfmm.allocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }
}

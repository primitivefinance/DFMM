// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";
import { LPToken } from "src/LPToken.sol";

contract G3MAllocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_allocate_GivenX() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        (deltas) = dfmm.allocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_G3M_allocate_GivenX_large_delta() public init {
        uint256 maxDeltaX = 10_000 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        (deltas) = dfmm.allocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        /*
        assertEq(
            preLiquidityBalance + deltaLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_G3M_allocate_MultipleTimes() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);

        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX.mulDivUp(101, 100);
        deltas[1] = maxDeltaY.mulDivUp(101, 100);

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);
        dfmm.allocate(POOL_ID, data);
    }

    /// todo: need to replace this with proper min liquidity minted checks
    function test_G3M_allocate_RevertsIfMoreThanMaxDeltaX() public init {
        skip();
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);

        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX - 1;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        vm.expectRevert();
        dfmm.allocate(POOL_ID, data);
    }

    /// todo: need to replace this with proper min liquidity minted checks
    function test_G3M_allocate_RevertsIfMoreThanMaxDeltaY() public init {
        skip();
        uint256 maxDeltaX = 0.1 ether;

        (uint256 maxDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, maxDeltaX);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY - 1;

        bytes memory data = abi.encode(deltas, deltaLiquidity);
        vm.expectRevert();
        dfmm.allocate(POOL_ID, data);
    }

    function test_G3M_allocate_GivenY() public init {
        uint256 maxDeltaY = 0.1 ether;

        (uint256 maxDeltaX, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaY(POOL_ID, maxDeltaY);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);
        console2.log("liquidity", liquidity);

        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        console2.log(preLiquidityBalance);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = maxDeltaX;
        deltas[1] = maxDeltaY;

        bytes memory data = abi.encode(deltas, deltaLiquidity);

        console2.log(maxDeltaX);
        console2.log(maxDeltaY);
        console2.log(deltaLiquidity);

        (deltas) = dfmm.allocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] + deltas[0]);
        assertEq(adjustedReserves[1], reserves[1] + deltas[1]);
        assertEq(adjustedLiquidity, liquidity + deltaLiquidity);

        assertEq(
            preLiquidityBalance + deltaLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
    }

    function test_G3M_allocate_ReceiveAppropriateLpTokens() public init_100 {
        (, uint256 initialL) = getReservesAndLiquidity(POOL_ID);
        Pool memory pool = dfmm.pools(POOL_ID);
        LPToken liquidityToken = LPToken(pool.liquidityToken);

        uint256 startBalance = liquidityToken.balanceOf(address(this));

        uint256 dyMax = 100 ether;
        (uint256 dxMax, uint256 dL) = solver.allocateGivenDeltaY(POOL_ID, dyMax);

        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = dxMax;
        deltas[1] = dyMax;

        bytes memory data = abi.encode(deltas, dL);

        dfmm.allocate(POOL_ID, data);

        (, uint256 nextL) = getReservesAndLiquidity(POOL_ID);
        uint256 endBalance = liquidityToken.balanceOf(address(this));

        // Add 1_000 wei to account for liquidity that was burnt on init
        assertEq(startBalance + 1000, initialL);
        assertEq(endBalance + 1000, nextL);
    }
}

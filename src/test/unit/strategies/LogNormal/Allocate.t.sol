// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalAllocateTest is LogNormalSetUp {
    function test_LogNormal_allocate_GivenX() public init {
        uint256 maxDeltaX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(maxDeltaX, liquidity, reserveX);
        uint256 maxDeltaY =
            computeDeltaYGivenDeltaX(maxDeltaX, reserveX, reserveY);

        console.log("maxDeltaX:", maxDeltaX);
        console.log("maxDeltaY:", maxDeltaY);
        console.log("deltaLiquidity:", deltaLiquidity);

        // uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY) = dfmm.allocate(POOL_ID, data);

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
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 deltaLiquidity) =
            solver.allocateGivenY(POOL_ID, amountX);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(reserveX, reserveY, deltaLiquidity);
        dfmm.allocate(POOL_ID, data);

        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = postTotalLiquidity - preTotalLiquidity;
        assertEq(
            preLiquidityBalance + deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MAllocateTest is G3MSetUp {
    function test_G3M_allocate_GivenX() public init {
        uint256 amountX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        console.log("reserveX:", reserveX);
        console.log("reserveY:", reserveY);
        console.log("liquidity:", liquidity);

        GeometricMeanParams memory params = solver.getPoolParams(POOL_ID);

        uint256 S = computePrice(reserveX, reserveY, params);
        console.log("S:", S);

        uint256 deltaLiquidity = computeLGivenX(amountX, S, params);
        console.log("deltaLiquidity:", deltaLiquidity);

        uint256 deltaY = computeY(amountX, S, params);
        console.log("deltaY:", deltaY);

        // uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        // (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(amountX * 2, deltaY * 2, deltaLiquidity);
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

    function test_G3M_allocate_GivenY() public init {
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

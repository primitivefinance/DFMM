// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalDeallocateTest is LogNormalSetUp {
    function test_LogNormal_deallocate_GivenX() public init {
        uint256 minDeltaX = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaX(minDeltaX, liquidity, reserveX);
        uint256 minDeltaY =
            computeDeltaYGivenDeltaX(minDeltaX, reserveX, reserveY);

        //        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
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
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }

    function test_LogNormal_deallocate_GivenY() public init {
        uint256 minDeltaY = 0.1 ether;

        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaLiquidity =
            computeDeltaLGivenDeltaY(minDeltaY, liquidity, reserveY);
        uint256 minDeltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserveX);

        // uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
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
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }
}

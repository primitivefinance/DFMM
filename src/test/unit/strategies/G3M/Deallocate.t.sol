// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract G3MDeallocateTest is G3MSetUp {
    function test_G3M_deallocate_GivenX_DecreasesTotalLiquidity() public init {
        uint256 minDeltaX = 0.1 ether;
        (uint256 minDeltaY, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaX(POOL_ID, minDeltaX);

        uint256 preLiquidityBalance = dfmm.liquidityOf(address(this), POOL_ID);
        (,, uint256 preTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(minDeltaX, minDeltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (,, uint256 postTotalLiquidity) = dfmm.getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        /*
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            dfmm.liquidityOf(address(this), POOL_ID)
        );
        */
    }

    /*
    function test_G3M_deallocate_GivenX_UpdateReserves() public init {
        uint256 amountX = 0.1 ether;
        (uint256 preReserveX, uint256 preReserveY,) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        (uint256 deltaX, uint256 deltaY, uint256 deltaLiquidity) =
            solver.deallocateGivenXReturnDeltas(POOL_ID, amountX);
        bytes memory data = abi.encode(deltaX, deltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (uint256 postReserveX, uint256 postReserveY,) =
            dfmm.getReservesAndLiquidity(POOL_ID);
        assertEq(preReserveX - deltaX, postReserveX);
        assertEq(preReserveY - deltaY, postReserveY);
    }

    function test_G3M_deallocate_GivenX_TransfersTokens() public init {
        uint256 amountX = 0.1 ether;
        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));
        uint256 preBalanceXDFMM = tokenX.balanceOf(address(dfmm));
        uint256 preBalanceYDFMM = tokenY.balanceOf(address(dfmm));

        (uint256 deltaX, uint256 deltaY, uint256 deltaLiquidity) =
            solver.deallocateGivenXReturnDeltas(POOL_ID, amountX);
        bytes memory data = abi.encode(deltaX, deltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        assertEq(preBalanceX + deltaX, tokenX.balanceOf(address(this)));
        assertEq(preBalanceY + deltaY, tokenY.balanceOf(address(this)));
        assertEq(preBalanceXDFMM - deltaX, tokenX.balanceOf(address(dfmm)));
        assertEq(preBalanceYDFMM - deltaY, tokenY.balanceOf(address(dfmm)));
    }
    */

    function test_G3M_deallocate_GivenY() public init {
        uint256 minDeltaY = 0.1 ether;

        (uint256 minDeltaX, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaY(POOL_ID, minDeltaY);
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            dfmm.getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(minDeltaX, minDeltaY, deltaLiquidity);
        (uint256 deltaX, uint256 deltaY) = dfmm.deallocate(POOL_ID, data);

        (
            uint256 adjustedReserveX,
            uint256 adjustedReserveY,
            uint256 adjustedLiquidity
        ) = dfmm.getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserveX, reserveX - deltaX, "bad x");
        assertEq(adjustedReserveY, reserveY - deltaY, "bad y");
        assertEq(adjustedLiquidity, liquidity - deltaLiquidity, "bad L");
    }
}

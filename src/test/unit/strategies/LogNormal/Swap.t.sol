// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract LogNormalSwapTest is LogNormalSetUp {
    function test_LogNormal_swap_SwapsXforY() public init {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));

        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        uint256 amountIn = 0.1 ether;
        bool swapXForY = true;

        (bool valid, uint256 amountOut,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXForY, amountIn);
        assertEq(valid, true);

        console.log("amountOut:", amountOut);

        (uint256 deltaX, uint256 deltaY) = dfmm.swap(POOL_ID, payload);
        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + deltaX);
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - deltaY);
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - deltaX);
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY + deltaY);
    }

    function test_LogNormal_swap_SwapsYforX() public init {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));

        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        uint256 amountIn = 0.1 ether;
        bool swapXForY = false;

        (bool valid,,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXForY, amountIn);
        assertEq(valid, true);
        (uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, payload);

        /*
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );

        assertEq(tokenY.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
        */
    }
}

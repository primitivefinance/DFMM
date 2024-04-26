// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";

contract G3MSwapTest is G3MSetUp {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    function test_G3M_swap_SwapsXforY() public init {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        uint256 amountIn = 0.1 ether;

        (, uint256 amountOut, bytes memory payload) =
            solver.prepareSwap(POOL_ID, 0, 1, amountIn);
        (,, uint256 deltaX, uint256 deltaY) =
            dfmm.swap(POOL_ID, address(this), payload, "");
        assertEq(amountIn, deltaX);
        assertEq(amountOut, deltaY);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + deltaX);
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - deltaY);
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - deltaX);
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY + deltaY);
    }

    function test_G3M_swap_SwapsYforX() public init {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        uint256 amountIn = 0.1 ether;

        (, uint256 amountOut, bytes memory payload) =
            solver.prepareSwap(POOL_ID, 1, 0, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, address(this), payload, "");

        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceX - outputAmount
        );
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY + inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceX + outputAmount
        );
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY - inputAmount);
    }
}

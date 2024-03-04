// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";

contract G3MSwapTest is G3MSetUp {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    function test_G3M_swap_SwapsXforY() public init {
        uint256 amountIn = 0.1 ether;
        bool swapXForY = true;

        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        (bool valid, uint256 amountOut,, bytes memory payload) =
            solver.simulateSwap(POOL_ID, swapXForY, amountIn);
        assertEq(valid, true);
        (uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, payload);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );

        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenY.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }

    function test_G3M_swap_SwapsYforX() public init {
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

        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );

        assertEq(tokenY.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }
}

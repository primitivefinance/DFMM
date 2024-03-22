// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp } from "./SetUp.sol";

contract ConstantSumSwapTest is ConstantSumSetUp {
    function test_ConstantSum_swap_SwapsXNoFee() public zeroFeePool {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = true;
        uint256 amountIn = 0.1 ether;

        (,, bytes memory swapData) =
            solver.simulateSwap(POOL_ID, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, address(this), swapData);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenY.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }

    function test_ConstantSum_swap_SwapsX() public defaultPool {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = true;
        uint256 amountIn = 0.1 ether;
        (,, bytes memory swapData) =
            solver.simulateSwap(POOL_ID, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, address(this), swapData);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenY.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }

    function test_ConstantSum_swap_SwapsY() public defaultPool {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = false;
        uint256 amountIn = 0.1 ether;
        (,, bytes memory swapData) =
            solver.simulateSwap(POOL_ID, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, address(this), swapData);

        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceX - outputAmount
        );
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY + inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceX + outputAmount
        );
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY - inputAmount);
    }

    function test_ConstantSum_swap_SwapsYNoFee() public zeroFeePool {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = false;
        uint256 amountIn = 0.1 ether;
        (,, bytes memory swapData) =
            solver.simulateSwap(POOL_ID, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(POOL_ID, address(this), swapData);

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

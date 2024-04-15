// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "forge-std/console2.sol";

contract G3MDeallocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_deallocate_GivenX_DecreasesTotalLiquidity() public init {
        uint256 minDeltaX = 0.1 ether;
        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, minDeltaX, 0);
        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);

        (, uint256 preTotalLiquidity) = getReservesAndLiquidity(POOL_ID);

        dfmm.deallocate(POOL_ID, deallocateData);

        (, uint256 postTotalLiquidity) = getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
    }

    function test_G3M_deallocate_GivenX_UpdateReserves() public init {
        uint256 minDeltaX = 0.1 ether;
        (uint256[] memory preReserves,) =
            solver.getReservesAndLiquidity(POOL_ID);

        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, minDeltaX, 0);
        dfmm.deallocate(POOL_ID, deallocateData);

        (, uint256 deltaY, uint256 deltaL) =
            abi.decode(deallocateData, (uint256, uint256, uint256));

        (uint256[] memory postReserves,) =
            solver.getReservesAndLiquidity(POOL_ID);
        assertEq(preReserves[0] - minDeltaX, postReserves[0]);
        assertEq(preReserves[1] - deltaY, postReserves[1]);
    }

    function test_G3M_deallocate_GivenX_TransfersTokens() public init {
        uint256 minDeltaX = 0.1 ether;
        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));
        uint256 preBalanceXDFMM = tokenX.balanceOf(address(dfmm));
        uint256 preBalanceYDFMM = tokenY.balanceOf(address(dfmm));

        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, minDeltaX, 0);
        (, uint256 deltaY,) =
            abi.decode(deallocateData, (uint256, uint256, uint256));
        dfmm.deallocate(POOL_ID, deallocateData);

        assertEq(preBalanceX + minDeltaX, tokenX.balanceOf(address(this)));
        assertEq(preBalanceY + deltaY, tokenY.balanceOf(address(this)));
        assertEq(preBalanceXDFMM - minDeltaX, tokenX.balanceOf(address(dfmm)));
        assertEq(preBalanceYDFMM - deltaY, tokenY.balanceOf(address(dfmm)));
    }

    function test_G3M_deallocate_GivenY() public init {
        uint256 minDeltaY = 0.1 ether;

        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, minDeltaY, 0);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);
        (,, uint256 deltaLiquidity) =
            abi.decode(deallocateData, (uint256, uint256, uint256));

        (uint256[] memory deltas) = dfmm.deallocate(POOL_ID, deallocateData);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] - deltas[0], "bad x");
        assertEq(adjustedReserves[1], reserves[1] - deltas[1], "bad y");
        assertEq(adjustedLiquidity, liquidity - deltaLiquidity, "bad L");
    }
}

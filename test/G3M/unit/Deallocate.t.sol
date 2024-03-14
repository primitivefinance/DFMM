// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";
import "forge-std/console2.sol";

contract G3MDeallocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_deallocate_GivenX_DecreasesTotalLiquidity() public init {
        uint256 minDeltaX = 0.1 ether;
        (uint256 deltaY, uint256 deltaLiquidity) =
            solver.deallocateGivenDeltaX(POOL_ID, minDeltaX);
        console2.log(deltaY);
        console2.log(deltaLiquidity);
        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);

        (, uint256 preTotalLiquidity) = getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(minDeltaX, deltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (, uint256 postTotalLiquidity) = getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
    }

    function test_G3M_deallocate_GivenX_UpdateReserves() public init {
        uint256 minDeltaX = 0.1 ether;
        (uint256 preReserveX, uint256 preReserveY,) =
            solver.getReservesAndLiquidity(POOL_ID);

        (uint256 deltaY, uint256 deltaLiquidity) =
            solver.deallocateGivenDeltaX(POOL_ID, minDeltaX);
        bytes memory data = abi.encode(minDeltaX, deltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        (uint256 postReserveX, uint256 postReserveY,) =
            solver.getReservesAndLiquidity(POOL_ID);
        assertEq(preReserveX - minDeltaX, postReserveX);
        assertEq(preReserveY - deltaY, postReserveY);
    }

    function test_G3M_deallocate_GivenX_TransfersTokens() public init {
        uint256 minDeltaX = 0.1 ether;
        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));
        uint256 preBalanceXDFMM = tokenX.balanceOf(address(dfmm));
        uint256 preBalanceYDFMM = tokenY.balanceOf(address(dfmm));

        (uint256 deltaY, uint256 deltaLiquidity) =
            solver.deallocateGivenDeltaX(POOL_ID, minDeltaX);
        bytes memory data = abi.encode(minDeltaX, deltaY, deltaLiquidity);
        dfmm.deallocate(POOL_ID, data);

        assertEq(preBalanceX + minDeltaX, tokenX.balanceOf(address(this)));
        assertEq(preBalanceY + deltaY, tokenY.balanceOf(address(this)));
        assertEq(preBalanceXDFMM - minDeltaX, tokenX.balanceOf(address(dfmm)));
        assertEq(preBalanceYDFMM - deltaY, tokenY.balanceOf(address(dfmm)));
    }

    function test_G3M_deallocate_GivenY() public init {
        uint256 minDeltaY = 0.1 ether;

        (uint256 minDeltaX, uint256 deltaLiquidity) =
            solver.allocateGivenDeltaY(POOL_ID, minDeltaY);
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(POOL_ID);

        bytes memory data = abi.encode(minDeltaX, minDeltaY, deltaLiquidity);
        (uint256[] memory deltas) = dfmm.deallocate(POOL_ID, data);

        (uint256[] memory adjustedReserves, uint256 adjustedLiquidity) =
            getReservesAndLiquidity(POOL_ID);

        assertEq(adjustedReserves[0], reserves[0] - deltas[0], "bad x");
        assertEq(adjustedReserves[1], reserves[1] - deltas[1], "bad y");
        assertEq(adjustedLiquidity, liquidity - deltaLiquidity, "bad L");
    }
}

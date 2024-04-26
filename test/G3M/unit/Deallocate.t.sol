// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "solmate/utils/FixedPointMathLib.sol";

contract G3MDeallocateTest is G3MSetUp {
    using FixedPointMathLib for uint256;

    function test_G3M_deallocate_DecreasesTotalLiquidity() public init {
        (, uint256 preTotalLiquidity) = getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, preLiquidityBalance / 2);
        dfmm.deallocate(POOL_ID, deallocateData);

        (, uint256 postTotalLiquidity) = getReservesAndLiquidity(POOL_ID);
        uint256 deltaTotalLiquidity = preTotalLiquidity - postTotalLiquidity;
        assertEq(
            preLiquidityBalance - deltaTotalLiquidity,
            liquidityOf(address(this), POOL_ID)
        );
    }

    function test_G3M_deallocate_UpdateReserves() public init {
        (uint256[] memory preReserves,) =
            solver.getReservesAndLiquidity(POOL_ID);

        uint256 preLiquidityBalance = liquidityOf(address(this), POOL_ID);
        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, preLiquidityBalance / 2);
        uint256[] memory deltas = dfmm.deallocate(POOL_ID, deallocateData);

        (uint256[] memory postReserves,) =
            solver.getReservesAndLiquidity(POOL_ID);
        assertEq(preReserves[0] - deltas[0], postReserves[0]);
        assertEq(preReserves[1] - deltas[1], postReserves[1]);
    }

    function test_G3M_deallocate_TransfersTokens() public init {
        uint256 preBalanceX = tokenX.balanceOf(address(this));
        uint256 preBalanceY = tokenY.balanceOf(address(this));
        uint256 preBalanceXDFMM = tokenX.balanceOf(address(dfmm));
        uint256 preBalanceYDFMM = tokenY.balanceOf(address(dfmm));

        // TODO: Use an actual amount of liquidity here
        bytes memory deallocateData =
            solver.prepareDeallocation(POOL_ID, 0.5 ether);
        (uint256[] memory usedDeltas) = dfmm.deallocate(POOL_ID, deallocateData);

        assertEq(preBalanceX + usedDeltas[0], tokenX.balanceOf(address(this)));
        assertEq(preBalanceY + usedDeltas[1], tokenY.balanceOf(address(this)));
        assertEq(
            preBalanceXDFMM - usedDeltas[0], tokenX.balanceOf(address(dfmm))
        );
        assertEq(
            preBalanceYDFMM - usedDeltas[1], tokenY.balanceOf(address(dfmm))
        );
    }
}

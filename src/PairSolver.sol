// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IStrategy, Pool } from "src/interfaces/IStrategy.sol";
import "src/lib/StrategyLib.sol";

/**
 * @title Pair strategy base contract for DFMM.
 * @notice This abstract contract defines the basic behavior of
 * a two-token strategy for DFMM. It is meant to be inherited by
 * a concrete strategy implementation.
 * @author Primitive
 */
abstract contract PairSolver {
    function prepareAllocationDeltasGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaX(deltaX, rX, rY, liquidity);
    }

    function prepareAllocationDeltasGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaY(deltaY, rX, rY, liquidity);
    }

    function prepareAllocationDeltasGivenDeltaL(
        uint256 poolId,
        uint256 deltaL
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaL(deltaL, rX, rY, liquidity);
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        virtual
        returns (uint256, uint256, uint256);
}

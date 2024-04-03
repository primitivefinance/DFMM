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
    /**
     * @notice Prepares the allocation deltas given a change in reserve X.
     * @dev This function calculates the changes in reserves and liquidity based on the change in reserve X.
     * @param poolId The ID of the pool.
     * @param deltaX The change in reserve X.
     * @return The encoded allocation deltas.
     */
    function prepareAllocationDeltasGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaX(deltaX, rX, rY, liquidity);
    }

    /**
     * @notice Prepares the allocation deltas given a change in reserve Y.
     * @dev This function calculates the changes in reserves and liquidity based on the change in reserve Y.
     * @param poolId The ID of the pool.
     * @param deltaY The change in reserve Y.
     * @return The encoded allocation deltas.
     */
    function prepareAllocationDeltasGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaY(deltaY, rX, rY, liquidity);
    }

    /**
     * @notice Prepares the allocation deltas given a change in liquidity.
     * @dev This function calculates the changes in reserves based on the change in liquidity.
     * @param poolId The ID of the pool.
     * @param deltaL The change in liquidity.
     * @return The encoded allocation deltas.
     */
    function prepareAllocationDeltasGivenDeltaL(
        uint256 poolId,
        uint256 deltaL
    ) public view returns (bytes memory) {
        (uint256 rX, uint256 rY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        return encodeAllocationDeltasGivenDeltaL(deltaL, rX, rY, liquidity);
    }

    /**
     * @notice Retrieves the reserves and liquidity for a given pool.
     * @dev This function is virtual and should be overridden by the concrete implementation.
     * @param poolId The ID of the pool.
     * @return The reserve of token X, the reserve of token Y, and the total liquidity.
     */
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        virtual
        returns (uint256, uint256, uint256);
}

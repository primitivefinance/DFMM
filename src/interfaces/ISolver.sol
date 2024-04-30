// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IStrategy } from "src/interfaces/IStrategy.sol";

/**
 * @notice Thrown when the provided token index is not available in the
 * current pool.
 */
error InvalidTokenIndex();

/**
 * @dev Thrown when an array of deltas doesn't have the expected length.
 */
error InvalidDeltasLength();

/**
 * @title Solver Interface
 * @author Primitive
 * @notice This interface contains generic functions that DFMM solvers must
 * implement in order to simplify offchain interactions.
 * @dev Note that in addition to the functions defined in this interface,
 * solvers should also implement the two following functions:
 * - `prepareInit`: This function should accept any parameters required to
 * initialize a pool and return the encoded data to execute the initialization.
 * - `getPoolParams`: This function should accept a `poolId` parameter and
 * return the decoded pool parameters as defined by the strategy contract.
 */
interface ISolver {
    /**
     * @notice Prepares the data to allocate liquidity into a pool.
     * @param poolId Id of the target DFMM pool.
     * @param deltas Array of token deltas to allocate, please note that:
     * - The order of the deltas must match the order of the pool tokens,
     * - The length of the deltas array must match the pool reserves length,
     * - The deltas must be expressed in WAD.
     * @return data Encoded data to execute the allocation.
     */
    function prepareAllocation(
        uint256 poolId,
        uint256[] memory deltas
    ) external view returns (bytes memory data);

    /**
     * @notice Prepares the token deltas and liquidity amounts expecting a proportion of tokens
     * that aligns with the pool's proportions.
     */
    function prepareAllocationProportional(
        uint256 poolId,
        uint256[] memory deltas
    ) external view returns (bytes memory data);

    /**
     * @notice Prepares the data to deallocate liquidity from a pool.
     * @param poolId Id of the target DFMM pool.
     * @param liquidityDelta Amount of liquidity to remove from the pool.
     * @return data Encoded data to execute the deallocation.
     */
    function prepareDeallocation(
        uint256 poolId,
        uint256 liquidityDelta
    ) external view returns (bytes memory data);

    /**
     * @notice Checks if a swap is valid, returns the expected output amount
     * along with the associated encoded data to execute the swap.
     * @param poolId Id of the target pool.
     * @param tokenInIndex Index of the input token.
     * @param tokenOutIndex Index of the output token.
     * @param amountIn Amount to swap in.
     * @return valid True if the swap is valid.
     * @return amountOut The expected output amount.
     * @return data Encoded data to execute the swap.
     */
    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    )
        external
        view
        returns (bool valid, uint256 amountOut, bytes memory data);

    /**
     * @notice Estimated price of tokenIn in terms of tokenOut.
     * @param poolId Id of the target pool.
     * @param tokenInIndex Token index of the input token.
     * @param tokenOutIndex Token index of the output token.
     * @return Estimated price expressed in WAD.
     */
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) external view returns (uint256);

    /**
     * @notice Returns the reserves and total liquidity of a pool.
     * @param poolId Id of the target pool.
     * @return reserves Array of token reserves expressed in WAD.
     * @return totalLiquidity Total liquidity of the pool.
     */
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256[] memory reserves, uint256 totalLiquidity);

    /**
     * @notice Returns the address of the associated strategy contract.
     */
    function strategy() external view returns (IStrategy);

    // TODO: This function might get removed because `swapFee` might be
    // merged into default DFMM pool parameters.
    // function getSwapFee(uint256 poolId) external view returns (uint256);
}

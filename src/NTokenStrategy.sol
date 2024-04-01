// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IStrategy, Pool } from "src/interfaces/IStrategy.sol";
import "forge-std/console2.sol";

/**
 * @dev Thrown when the length of the deltas array is not the
 * same as the length of the reserves array.
 */
error InvalidTokenDeltas();

/**
 * @title N-token strategy base contract for DFMM.
 * @notice This abstract contract defines the basic behavior of
 * a n-token strategy for DFMM. It is meant to be inherited by
 * a concrete strategy implementation.
 * @author Primitive
 */
abstract contract NTokenStrategy is IStrategy {
    /// @inheritdoc IStrategy
    address public immutable dfmm;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    /// @dev Restricts the caller to the DFMM contract.
    modifier onlyDFMM() {
        if (msg.sender != address(dfmm)) revert NotDFMM();
        _;
    }

    /// @inheritdoc IStrategy
    function validateAllocate(
        address,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory tokenDeltas,
            uint256 deltaLiquidity
        )
    {
        // We use `deltaL` as a temporary variable because
        // we cannot assign to `deltaLiquidity` directly.
        (uint256[] memory maxTokenDeltas, uint256 deltaL) =
            abi.decode(data, (uint256[], uint256));
        if (maxTokenDeltas.length != pool.reserves.length) {
            revert InvalidTokenDeltas();
        }
        deltaLiquidity = deltaL;

        (uint256[] memory deltas, uint256[] memory nextReserves) =
        _computeAllocateDeltasAndReservesGivenDeltaL(
            deltaLiquidity, maxTokenDeltas, pool
        );
        tokenDeltas = deltas;

        invariant = tradingFunction(
            nextReserves,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    /// @inheritdoc IStrategy
    function validateDeallocate(
        address,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory tokenDeltas,
            uint256 deltaLiquidity
        )
    {
        (uint256[] memory minTokenDeltas, uint256 deltaL) =
            abi.decode(data, (uint256[], uint256));
        if (minTokenDeltas.length != pool.reserves.length) {
            revert InvalidTokenDeltas();
        }
        deltaLiquidity = deltaL;

        (uint256[] memory deltas, uint256[] memory nextReserves) =
        _computeDeallocateDeltasAndReservesGivenDeltaL(
            deltaLiquidity, minTokenDeltas, pool
        );
        tokenDeltas = deltas;

        invariant = tradingFunction(
            nextReserves,
            pool.totalLiquidity - deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    /// @inheritdoc IStrategy
    function validateSwap(
        address,
        uint256 poolId,
        Pool memory pool,
        bytes memory data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256 tokenInIndex,
            uint256 tokenOutIndex,
            uint256 amountIn,
            uint256 amountOut,
            uint256 deltaLiquidity
        )
    {
        bytes memory params = getPoolParams(poolId);

        (tokenInIndex, tokenOutIndex, amountIn, amountOut) =
            abi.decode(data, (uint256, uint256, uint256, uint256));

        deltaLiquidity = _computeSwapDeltaLiquidity(
            pool, params, tokenInIndex, tokenOutIndex, amountIn, amountOut
        );

        console2.log("deltaLiquidity: %d", deltaLiquidity);

        pool.reserves[tokenInIndex] += amountIn;
        pool.reserves[tokenOutIndex] -= amountOut;

        invariant = tradingFunction(
            pool.reserves, pool.totalLiquidity + deltaLiquidity, params
        );

        valid = invariant >= 0;
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId)
        public
        view
        virtual
        returns (bytes memory);

    /// @inheritdoc IStrategy
    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public view virtual returns (int256);

    /**
     * @notice Computes the token deltas and the next reserves for
     * an allocation.
     * @param deltaLiquidity Amount of liquidity to allocate.
     * @param maxDeltas Maximum token deltas to spend (in WAD).
     * @param pool Structure containing the pool.
     * @return deltas Required token deltas to allocate (in WAD).
     * @return nextReserves Reserves after the allocation.
     */
    function _computeAllocateDeltasAndReservesGivenDeltaL(
        uint256 deltaLiquidity,
        uint256[] memory maxDeltas,
        Pool memory pool
    )
        internal
        view
        virtual
        returns (uint256[] memory deltas, uint256[] memory nextReserves);

    /**
     * @notice Computes the token deltas and the next reserves for
     * a deallocation.
     * @param deltaLiquidity Amount of liquidity to deallocate.
     * @param minDeltas Minimum token deltas to receive (in WAD).
     * @param pool Structure containing the pool.
     * @return deltas Token deltas being deallocated (in WAD).
     * @return nextReserves Reserves after the deallocation.
     */
    function _computeDeallocateDeltasAndReservesGivenDeltaL(
        uint256 deltaLiquidity,
        uint256[] memory minDeltas,
        Pool memory pool
    )
        internal
        view
        virtual
        returns (uint256[] memory deltas, uint256[] memory nextReserves);

    /**
     * @dev Computes the deltaLiquidity for a swap operation.
     */
    function _computeSwapDeltaLiquidity(
        Pool memory pool,
        bytes memory params,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn,
        uint256 amountOut
    ) internal view virtual returns (uint256);
}

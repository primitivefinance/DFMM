// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IStrategy, Pool } from "src/interfaces/IStrategy.sol";

/**
 * @title Pair strategy base contract for DFMM.
 * @notice This abstract contract defines the basic behavior of
 * a two-token strategy for DFMM. It is meant to be inherited by
 * a concrete strategy implementation.
 * @author Primitive
 */
abstract contract PairStrategy is IStrategy {
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
        Pool memory pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        // We use `deltaL` as a temporary variable because
        // we cannot assign to `deltaLiquidity` directly.
        (uint256 maxDeltaX, uint256 maxDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));
        deltaLiquidity = deltaL;

        deltas = _computeAllocateDeltasGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );

        if (deltas[0] > maxDeltaX) {
            revert DeltaError(maxDeltaX, deltas[0]);
        }

        if (deltas[1] > maxDeltaY) {
            revert DeltaError(maxDeltaY, deltas[1]);
        }

        pool.reserves[0] += deltas[0];
        pool.reserves[1] += deltas[1];

        invariant = tradingFunction(
            pool.reserves,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    /// @inheritdoc IStrategy
    function validateDeallocate(
        address,
        uint256 poolId,
        Pool memory pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        (uint256 minDeltaX, uint256 minDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity = deltaL;
        deltas = _computeDeallocateDeltasGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );

        if (minDeltaX > deltas[0]) {
            revert DeltaError(minDeltaX, deltas[0]);
        }

        if (minDeltaY > deltas[1]) {
            revert DeltaError(minDeltaY, deltas[1]);
        }

        pool.reserves[0] -= deltas[0];
        pool.reserves[1] -= deltas[1];

        invariant = tradingFunction(
            pool.reserves,
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
            uint256 deltaLiquidity,
            bytes memory params
        )
    {
        params = getPoolParams(poolId);

        (tokenInIndex, tokenOutIndex, amountIn, amountOut) =
            abi.decode(data, (uint256, uint256, uint256, uint256));

        deltaLiquidity = _computeSwapDeltaLiquidity(
            pool, params, tokenInIndex, tokenOutIndex, amountIn, amountOut
        );

        pool.reserves[tokenInIndex] += amountIn;
        pool.reserves[tokenOutIndex] -= amountOut;

        invariant = tradingFunction(
            pool.reserves, pool.totalLiquidity + deltaLiquidity, params
        );

        valid = invariant >= 0;
    }

    function postSwapHook(
        address,
        uint256,
        Pool memory,
        bytes calldata
    ) external onlyDFMM { }

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
     * @dev Computes the deltas to allocate given a liquidity delta.
     * This function is meant to be implemented by the strategy
     * inheriting from this contract.
     * @param deltaLiquidity Amount of liquidity to allocate.
     * @param pool Structure containing the pool.
     * @param data Additional data for the strategy.
     * @return deltas Amount of tokens to allocate (expressed in WAD).
     */
    function _computeAllocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory data
    ) internal view virtual returns (uint256[] memory);

    /**
     * @dev Computes the deltas to deallocate given a liquidity.
     * delta. This function is meant to be implemented by the
     * strategy inheriting from this contract.
     * @param deltaLiquidity Amount of liquidity to deallocate.
     * @param pool Structure containing the pool.
     * @param data Additional data for the strategy.
     * @return deltas Amount of tokens to deallocate (expressed in WAD).
     */
    function _computeDeallocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory data
    ) internal view virtual returns (uint256[] memory);

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

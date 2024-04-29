// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IStrategy, Pool } from "src/interfaces/IStrategy.sol";

/**
 * @dev Thrown when the length of the deltas array is not the
 * same as the length of the reserves array.
 */
error InvalidTokenDeltas();

/**
 * @title Strategy base contract for DFMM.
 * @notice This abstract contract defines the basic implementations of
 * validating allocate, deallocate, and swap operations.
 * It is meant to be inherited by a concrete strategy implementation.
 * @author Primitive
 */
abstract contract Strategy is IStrategy {
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
        uint256 len = pool.reserves.length;
        (tokenDeltas, deltaLiquidity) = abi.decode(data, (uint256[], uint256));
        if (tokenDeltas.length != len) {
            revert InvalidTokenDeltas();
        }

        uint256[] memory nextReserves = new uint256[](len);
        for (uint256 i = 0; i < len; i++) {
            nextReserves[i] = pool.reserves[i] + tokenDeltas[i];
        }

        // todo: work on this
        /* uint256 expectedDeltaL =
            _computeDeltaLGivenDeltas(tokenDeltas, pool, getPoolParams(poolId));

        if (expectedDeltaL < deltaLiquidity) {
            revert DeltaError(expectedDeltaL, deltaLiquidity);
        }

        deltaLiquidity = expectedDeltaL; */

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
        uint256 len = pool.reserves.length;
        (tokenDeltas, deltaLiquidity) = abi.decode(data, (uint256[], uint256));
        if (tokenDeltas.length != pool.reserves.length) {
            revert InvalidTokenDeltas();
        }

        uint256[] memory nextReserves = new uint256[](len);
        for (uint256 i = 0; i < len; i++) {
            nextReserves[i] = pool.reserves[i] - tokenDeltas[i];
        }

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
    ) external virtual onlyDFMM { }

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

    /// todo: We most likely need functions to compute liquidity on allocates given the token deltas,
    /// and compute deltas given the liquidity on deallocates.
    /// Then we compare these values against the max/min deltas provided by the user to validate the operation.
    /// Additionally, we might want to use the computed values if they are beneficial to the user while also passing the invariant.
    /// Where I am hesitant about this is that not all computations might be clean, like in the log normal case where we usually use solvers
    /// that are doing root finding to find these values to submit for the users.
    /// If we do make these, they'd look something like `computeLiquidityGivenDeltas` and `computeDeltasGivenLiquidity`.
    /// Which would be implemented by the strategy contracts and used in the `validateAllocate` and `validateDeallocate` functions
    /// to compute the actual values to compare against the user provided values.

    /**
     * @dev Computes the deltas to allocate given a liquidity delta.
     * This function is meant to be implemented by the strategy
     * inheriting from this contract.
     * @param tokenDeltas Amount of tokens to allocate.
     * @param pool Structure containing the pool.
     * @param params Strategy parameters
     * @return deltaLiquidity Amount of liquidity allocated computed from the token deltas.
     */
    /* function _computeDeltaLGivenDeltas(
        uint256[] memory tokenDeltas,
        Pool memory pool,
        bytes memory params
    ) internal view virtual returns (uint256); */

    /**
     * @dev Computes the amounts of tokens deallocated given
     * `deltaLiquidity`. This function is meant to be implemented by the
     * strategy inheriting from this contract.
     * @param deltaLiquidity Amount of liquidity to deallocate.
     * @param pool Structure containing the pool.
     * @param data Additional data for the strategy.
     * @return deltas Amount of tokens to deallocate (expressed in WAD).
     */
    /* function _computeDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory data
    ) internal view virtual returns (uint256[] memory); */
}

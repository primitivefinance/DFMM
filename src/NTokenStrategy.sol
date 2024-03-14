/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IStrategy, Pool } from "src/interfaces/IStrategy.sol";

/**
 * @title Strategy base contract for DFMM.
 * @author Primitive
 */
abstract contract NTokenStrategy is IStrategy {
    /// @inheritdoc IStrategy
    address public immutable dfmm;

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
        (uint256[] memory maxTokenDeltas, uint256 deltaL) =
            abi.decode(data, (uint256[], uint256));

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
        (tokenInIndex, tokenOutIndex, amountIn, amountOut, deltaLiquidity) =
            abi.decode(data, (uint256, uint256, uint256, uint256, uint256));

        pool.reserves[tokenInIndex] += amountIn;
        pool.reserves[tokenOutIndex] -= amountOut;

        invariant = tradingFunction(
            pool.reserves,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        virtual
        returns (bytes memory);

    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public view virtual returns (int256);

    function _computeAllocateDeltasAndReservesGivenDeltaL(
        uint256 deltaLiquidity,
        uint256[] memory maxDeltas,
        Pool memory pool
    )
        internal
        view
        virtual
        returns (uint256[] memory deltas, uint256[] memory nextReserves);

    function _computeDeallocateDeltasAndReservesGivenDeltaL(
        uint256 deltaLiquidity,
        uint256[] memory minDeltas,
        Pool memory pool
    )
        internal
        view
        virtual
        returns (uint256[] memory deltas, uint256[] memory nextReserves);
}

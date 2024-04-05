// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import {
    FixedPointMathLib,
    computeTradingFunction,
    computeSwapDeltaLiquidity,
    computeDeltaLiquidity
} from "./ConstantSumMath.sol";
import {
    decodePriceUpdate,
    decodeFeeUpdate,
    decodeControllerUpdate
} from "./ConstantSumUtils.sol";
import { PairStrategy, IStrategy, Pool } from "src/PairStrategy.sol";
import { EPSILON } from "src/lib/StrategyLib.sol";

struct InternalParams {
    uint256 price;
    uint256 swapFee;
    address controller;
}

struct ConstantSumParams {
    uint256 price;
    uint256 swapFee;
    address controller;
}

enum UpdateCode {
    Invalid,
    SwapFee,
    Price,
    Controller
}

contract ConstantSum is PairStrategy {
    using FixedPointMathLib for uint256;

    /// @notice Thrown when the expected liquidity is not met.
    error InvalidDeltaLiquidity();

    /// @inheritdoc IStrategy
    string public constant name = "ConstantSum";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) PairStrategy(dfmm_) { }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;

        (reserves, params) = abi.decode(data, (uint256[], ConstantSumParams));
        totalLiquidity =
            computeDeltaLiquidity(reserves[0], reserves[1], params.price);

        if (pool.reserves.length != 2 || reserves.length != 2) {
            revert InvalidReservesLength();
        }

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;
        internalParams[poolId].controller = params.controller;

        // Get the trading function and check this is valid
        invariant =
            tradingFunction(reserves, totalLiquidity, abi.encode(params));

        valid = invariant >= 0 && invariant <= EPSILON;

        return (valid, invariant, reserves, totalLiquidity);
    }

    function validateAllocate(
        address,
        uint256 poolId,
        Pool memory pool,
        bytes calldata data
    )
        external
        view
        override
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        (uint256 deltaX, uint256 deltaY, uint256 minDeltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity =
            computeDeltaLiquidity(deltaX, deltaY, internalParams[poolId].price);
        if (deltaLiquidity < minDeltaL) revert InvalidDeltaLiquidity();

        deltas = new uint256[](2);
        deltas[0] = deltaX;
        deltas[1] = deltaY;

        pool.reserves[0] += deltaX;
        pool.reserves[1] += deltaY;

        invariant = tradingFunction(
            pool.reserves,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    function validateDeallocate(
        address,
        uint256 poolId,
        Pool memory pool,
        bytes calldata data
    )
        external
        view
        override
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        (uint256 deltaX, uint256 deltaY, uint256 maxDeltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity =
            computeDeltaLiquidity(deltaX, deltaY, internalParams[poolId].price);
        if (deltaLiquidity > maxDeltaL) revert InvalidDeltaLiquidity();

        deltas = new uint256[](2);
        deltas[0] = deltaX;
        deltas[1] = deltaY;

        pool.reserves[0] -= deltaX;
        pool.reserves[1] -= deltaY;

        invariant = tradingFunction(
            pool.reserves,
            pool.totalLiquidity - deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = invariant >= 0;
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        Pool calldata,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        UpdateCode updateCode = abi.decode(data, (UpdateCode));

        if (updateCode == UpdateCode.Price) {
            (internalParams[poolId].price) = decodePriceUpdate(data);
        } else if (updateCode == UpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == UpdateCode.Controller) {
            internalParams[poolId].controller = decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId)
        public
        view
        override
        returns (bytes memory)
    {
        ConstantSumParams memory params;

        params.price = internalParams[poolId].price;
        params.swapFee = internalParams[poolId].swapFee;
        params.controller = internalParams[poolId].controller;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return computeTradingFunction(
            reserves,
            totalLiquidity,
            abi.decode(params, (ConstantSumParams)).price
        );
    }

    /// @inheritdoc PairStrategy
    function _computeAllocateDeltasGivenDeltaL(
        uint256,
        Pool memory,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        return new uint256[](2);
    }

    /// @inheritdoc PairStrategy
    function _computeDeallocateDeltasGivenDeltaL(
        uint256,
        Pool memory,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        return new uint256[](2);
    }

    /// @inheritdoc PairStrategy
    function _computeSwapDeltaLiquidity(
        Pool memory,
        bytes memory params,
        uint256 tokenInIndex,
        uint256,
        uint256 amountIn,
        uint256
    ) internal pure override returns (uint256) {
        return computeSwapDeltaLiquidity(
            amountIn, abi.decode(params, (ConstantSumParams)), tokenInIndex == 0
        );
    }
}

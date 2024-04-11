// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { Pool } from "src/interfaces/IDFMM.sol";
import { PairStrategy, IStrategy } from "src/PairStrategy.sol";
import { DynamicParamLib, DynamicParam } from "src/lib/DynamicParamLib.sol";
import {
    computeTradingFunction,
    computeDeltaGivenDeltaLRoundUp,
    computeDeltaGivenDeltaLRoundDown,
    computeDeltaLXIn,
    computeDeltaLYIn
} from "src/LogNormal/LogNormalMath.sol";
import {
    decodeFeeUpdate,
    decodeMeanUpdate,
    decodeWidthUpdate,
    decodeControllerUpdate
} from "src/LogNormal/LogNormalUtils.sol";
import { EPSILON } from "src/lib/StrategyLib.sol";

enum UpdateCode {
    Invalid,
    SwapFee,
    Width,
    Mean,
    Controller
}

struct InternalParams {
    DynamicParam mean;
    DynamicParam width;
    uint256 swapFee;
    address controller;
}

/// @dev Parameterization of the Log Normal curve.
struct LogNormalParams {
    uint256 mean;
    uint256 width;
    uint256 swapFee;
    address controller;
}

/// @dev Thrown when the mean parameter is not within the allowed bounds.
error InvalidMean();

/// @dev Thrown when the width parameter is not within the allowed bounds.
error InvalidWidth();

uint256 constant MIN_WIDTH = 1;
uint256 constant MAX_WIDTH = uint256(type(int256).max);
uint256 constant MIN_MEAN = 1;
uint256 constant MAX_MEAN = uint256(type(int256).max);

/**
 * @title LogNormal Strategy for DFMM.
 * @author Primitive
 */
contract LogNormal is PairStrategy {
    using DynamicParamLib for DynamicParam;

    /// @inheritdoc IStrategy
    string public constant override name = "LogNormal";

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
        LogNormalParams memory params;
        (reserves, totalLiquidity, params) =
            abi.decode(data, (uint256[], uint256, LogNormalParams));

        if (params.mean < MIN_MEAN || params.mean > MAX_MEAN) {
            revert InvalidMean();
        }

        if (params.width < MIN_WIDTH || params.width > MAX_WIDTH) {
            revert InvalidWidth();
        }

        if (pool.reserves.length != 2 || reserves.length != 2) {
            revert InvalidReservesLength();
        }

        internalParams[poolId].mean.lastComputedValue = params.mean;
        internalParams[poolId].width.lastComputedValue = params.width;
        internalParams[poolId].swapFee = params.swapFee;
        internalParams[poolId].controller = params.controller;

        invariant =
            tradingFunction(reserves, totalLiquidity, getPoolParams(poolId));
        valid = invariant >= 0 && invariant <= EPSILON;
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

        if (updateCode == UpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == UpdateCode.Width) {
            (uint256 targetWidth, uint256 targetTimestamp) =
                decodeWidthUpdate(data);
            if (targetWidth < MIN_WIDTH || targetWidth > MAX_WIDTH) {
                revert InvalidWidth();
            }
            internalParams[poolId].width.set(targetWidth, targetTimestamp);
        } else if (updateCode == UpdateCode.Mean) {
            (uint256 targetMean, uint256 targetTimestamp) =
                decodeMeanUpdate(data);
            if (targetMean < MIN_MEAN || targetMean > MAX_MEAN) {
                revert InvalidMean();
            }
            internalParams[poolId].mean.set(targetMean, targetTimestamp);
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
        LogNormalParams memory params;

        params.width = internalParams[poolId].width.actualized();
        params.mean = internalParams[poolId].mean.actualized();
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return computeTradingFunction(
            reserves[0],
            reserves[1],
            totalLiquidity,
            abi.decode(params, (LogNormalParams))
        );
    }

    /// @inheritdoc PairStrategy
    function _computeAllocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        uint256[] memory deltas = new uint256[](2);

        deltas[0] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );

        return deltas;
    }

    /// @inheritdoc PairStrategy
    function _computeDeallocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        uint256[] memory deltas = new uint256[](2);

        deltas[0] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );
        return deltas;
    }

    function _computeSwapDeltaLiquidity(
        Pool memory pool,
        bytes memory params,
        uint256 tokenInIndex,
        uint256,
        uint256 amountIn,
        uint256
    ) internal pure override returns (uint256) {
        LogNormalParams memory poolParams =
            abi.decode(params, (LogNormalParams));

        if (tokenInIndex == 0) {
            return computeDeltaLXIn(
                amountIn,
                pool.reserves[0],
                pool.reserves[1],
                pool.totalLiquidity,
                poolParams
            );
        }

        return computeDeltaLYIn(
            amountIn,
            pool.reserves[0],
            pool.reserves[1],
            pool.totalLiquidity,
            poolParams
        );
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { PairStrategy, IStrategy } from "src/PairStrategy.sol";
import { DynamicParam, DynamicParamLib } from "src/lib/DynamicParamLib.sol";
import { Pool } from "src/interfaces/IDFMM.sol";
import {
    computeTradingFunction,
    computeDeltaGivenDeltaLRoundUp,
    computeDeltaGivenDeltaLRoundDown
} from "./G3MMath.sol";
import { ONE } from "src/lib/StrategyLib.sol";

/**
 * @dev Parameterization of the GeometricMean curve.
 * @param wX Weight of token X in WAD.
 * @param wY Weight of token Y in WAD.
 * @param swapFee Swap fee in WAD.
 * @param controller Address of the controller.
 */
struct GeometricMeanParams {
    uint256 wX;
    uint256 wY;
    uint256 swapFee;
    address controller;
}

enum UpdateCode {
    Invalid,
    SwapFee,
    WeightX,
    Controller
}

/**
 * @notice Geometric Mean Market Maker.
 */
contract GeometricMean is PairStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam wX;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy
    string public constant override name = "GeometricMean";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) PairStrategy(dfmm_) { }

    /// @dev Thrown if the weight of X is greater than 1 (in WAD).
    error InvalidWeightX();

    struct InitState {
        bool valid;
        int256 invariant;
        uint256 reserveX;
        uint256 reserveY;
        address controller;
        uint256 swapFee;
        uint256 wX;
        uint256[] reserves;
        uint256 totalLiquidity;
    }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        Pool calldata,
        bytes calldata data
    ) external onlyDFMM returns (bool, int256, uint256[] memory, uint256) {
        InitState memory state;

        state.reserves = new uint256[](2);

        (
            state.reserves[0],
            state.reserves[1],
            state.totalLiquidity,
            state.wX,
            state.swapFee,
            state.controller
        ) = abi.decode(
            data, (uint256, uint256, uint256, uint256, uint256, address)
        );

        if (state.wX >= ONE) {
            revert InvalidWeightX();
        }

        internalParams[poolId].wX.lastComputedValue = state.wX;
        internalParams[poolId].swapFee = state.swapFee;
        internalParams[poolId].controller = state.controller;

        state.invariant = computeTradingFunction(
            state.reserves[0],
            state.reserves[1],
            state.totalLiquidity,
            abi.decode(getPoolParams(poolId), (GeometricMeanParams))
        );

        state.valid = state.invariant >= 0;

        return
            (state.valid, state.invariant, state.reserves, state.totalLiquidity);
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
            (, internalParams[poolId].swapFee) =
                abi.decode(data, (UpdateCode, uint256));
        } else if (updateCode == UpdateCode.WeightX) {
            (, uint256 targetWeightX, uint256 targetTimestamp) =
                abi.decode(data, (UpdateCode, uint256, uint256));
            internalParams[poolId].wX.set(targetWeightX, targetTimestamp);
        } else if (updateCode == UpdateCode.Controller) {
            (, internalParams[poolId].controller) =
                abi.decode(data, (UpdateCode, address));
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
        GeometricMeanParams memory params;

        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
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
            reserves[0],
            reserves[1],
            totalLiquidity,
            abi.decode(params, (GeometricMeanParams))
        );
    }

    /// @inheritdoc PairStrategy
    function _computeAllocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory deltas) {
        deltas = new uint256[](2);
        deltas[0] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );
    }

    /// @inheritdoc PairStrategy
    function _computeDeallocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory deltas) {
        deltas = new uint256[](2);
        deltas[0] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );
    }
}

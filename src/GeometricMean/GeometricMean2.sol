// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { Strategy2, IStrategy2 } from "src/Strategy2.sol";
import { DynamicParam, DynamicParamLib } from "src/lib/DynamicParamLib.sol";
import { IDFMM2 } from "src/interfaces/IDFMM2.sol";
import { GeometricMeanLib, FixedPointMathLib } from "./GeometricMeanLib.sol";
import { ONE } from "src/lib/StrategyLib.sol";

import { GeometricMeanParams } from "./GeometricMean.sol";

/**
 * @notice Geometric Mean Market Maker.
 */
contract GeometricMean2 is Strategy2 {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam wX;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy2
    string public constant override name = "GeometricMean";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) Strategy2(dfmm_) { }

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

    /// @inheritdoc IStrategy2
    function init(
        address,
        uint256 poolId,
        IDFMM2.Pool calldata,
        bytes calldata data
    ) external onlyDFMM returns (bool, int256, uint256[] memory, uint256) {
        InitState memory state;

        (
            state.reserveX,
            state.reserveY,
            state.totalLiquidity,
            state.wX,
            state.swapFee,
            state.controller
        ) = abi.decode(
            data, (uint256, uint256, uint256, uint256, uint256, address)
        );

        state.reserves[0] = state.reserveX;
        state.reserves[1] = state.reserveY;

        if (state.wX >= ONE) {
            revert InvalidWeightX();
        }

        internalParams[poolId].wX.lastComputedValue = state.wX;
        internalParams[poolId].swapFee = state.swapFee;
        internalParams[poolId].controller = state.controller;

        state.invariant = GeometricMeanLib.tradingFunction(
            state.reserves[0],
            state.reserves[1],
            state.totalLiquidity,
            abi.decode(getPoolParams(poolId), (GeometricMeanParams))
        );

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        state.valid = -(EPSILON) < state.invariant && state.invariant < EPSILON;

        return
            (state.valid, state.invariant, state.reserves, state.totalLiquidity);
    }

    /// @inheritdoc IStrategy2
    function update(
        address sender,
        uint256 poolId,
        IDFMM2.Pool calldata,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        GeometricMeanLib.GeometricMeanUpdateCode updateCode =
            abi.decode(data, (GeometricMeanLib.GeometricMeanUpdateCode));

        if (updateCode == GeometricMeanLib.GeometricMeanUpdateCode.SwapFee) {
            internalParams[poolId].swapFee =
                GeometricMeanLib.decodeFeeUpdate(data);
        } else if (
            updateCode == GeometricMeanLib.GeometricMeanUpdateCode.WeightX
        ) {
            (uint256 targetWeightX, uint256 targetTimestamp) =
                GeometricMeanLib.decodeWeightXUpdate(data);
            internalParams[poolId].wX.set(targetWeightX, targetTimestamp);
        } else if (
            updateCode == GeometricMeanLib.GeometricMeanUpdateCode.Controller
        ) {
            internalParams[poolId].controller =
                GeometricMeanLib.decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy2
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

    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return GeometricMeanLib.tradingFunction(
            reserves[0],
            reserves[1],
            totalLiquidity,
            abi.decode(params, (GeometricMeanParams))
        );
    }

    function _computeDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM2.Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory deltas) {
        deltas[0] = pool.reserves[0].mulWadDown(
            deltaLiquidity.divWadDown(pool.totalLiquidity)
        );

        deltas[1] = pool.reserves[1].mulWadDown(
            deltaLiquidity.divWadDown(pool.totalLiquidity)
        );
    }
}

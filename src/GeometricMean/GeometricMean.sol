// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { Strategy, IStrategy } from "src/Strategy.sol";
import { DynamicParam, DynamicParamLib } from "src/lib/DynamicParamLib.sol";
import { IDFMM } from "src/interfaces/IDFMM.sol";
import { GeometricMeanLib, FixedPointMathLib } from "./GeometricMeanLib.sol";
import { ONE } from "src/lib/StrategyLib.sol";

/// @dev Parameterization of the GeometricMean curve.
struct GeometricMeanParams {
    uint256 wX;
    uint256 wY;
    uint256 swapFee;
    address controller;
}

/**
 * @notice Geometric Mean Market Maker.
 */
contract GeometricMean is Strategy {
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
    constructor(address dfmm_) Strategy(dfmm_) { }

    error InvalidWeightX();

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserveX, reserveY, totalLiquidity,,,) =
            _decodeInit(poolId, data);
    }

    function _decodeInit(
        uint256 poolId,
        bytes calldata data
    )
        private
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity,
            uint256 wX,
            uint256 swapFee,
            address controller
        )
    {
        (reserveX, reserveY, totalLiquidity, wX, swapFee, controller) = abi
            .decode(data, (uint256, uint256, uint256, uint256, uint256, address));

        if (wX >= ONE) {
            revert InvalidWeightX();
        }

        internalParams[poolId].wX.lastComputedValue = wX;
        internalParams[poolId].swapFee = swapFee;
        internalParams[poolId].controller = controller;

        invariant = GeometricMeanLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (GeometricMeanParams))
        );

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        IDFMM.Pool calldata pool,
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

    function tradingFunction(
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return GeometricMeanLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(params, (GeometricMeanParams))
        );
    }

    function _computeDeltaXGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory
    ) internal pure override returns (uint256) {
        return pool.reserveX.mulWadDown(
            deltaLiquidity.divWadDown(pool.totalLiquidity)
        );
    }

    function _computeDeltaYGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory
    ) internal pure override returns (uint256) {
        return pool.reserveY.mulWadDown(
            deltaLiquidity.divWadDown(pool.totalLiquidity)
        );
    }
}

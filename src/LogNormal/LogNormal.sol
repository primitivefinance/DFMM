// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IDFMM } from "src/interfaces/IDFMM.sol";
import { Strategy, IStrategy } from "src/Strategy.sol";
import { DynamicParamLib, DynamicParam } from "src/lib/DynamicParamLib.sol";
import "./LogNormalLib.sol";

/**
 * @title LogNormal Strategy for DFMM.
 * @author Primitive
 */
contract LogNormal is Strategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

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

    /// @inheritdoc IStrategy
    string public constant override name = "LogNormal";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) Strategy(dfmm_) { }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        IDFMM.Pool calldata,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserveX, reserveY, totalLiquidity,) =
            _decodeInit(poolId, data);
    }

    /// @dev Decodes, stores and validates pool initialization parameters.
    /// Note that this function was purely made to avoid the stack too deep
    /// error in the `init()` function.
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
            LogNormalParams memory params
        )
    {
        (reserveX, reserveY, totalLiquidity, params) =
            abi.decode(data, (uint256, uint256, uint256, LogNormalParams));

        internalParams[poolId].mean.lastComputedValue = params.mean;
        internalParams[poolId].width.lastComputedValue = params.width;
        internalParams[poolId].swapFee = params.swapFee;
        internalParams[poolId].controller = params.controller;

        invariant = LogNormalLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (LogNormalParams))
        );
        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        IDFMM.Pool calldata,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        LogNormalLib.LogNormalUpdateCode updateCode =
            abi.decode(data, (LogNormalLib.LogNormalUpdateCode));

        if (updateCode == LogNormalLib.LogNormalUpdateCode.SwapFee) {
            internalParams[poolId].swapFee = LogNormalLib.decodeFeeUpdate(data);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Width) {
            (uint256 targetWidth, uint256 targetTimestamp) =
                LogNormalLib.decodeWidthUpdate(data);
            internalParams[poolId].width.set(targetWidth, targetTimestamp);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Mean) {
            (uint256 targetMean, uint256 targetTimestamp) =
                LogNormalLib.decodeMeanUpdate(data);
            internalParams[poolId].mean.set(targetMean, targetTimestamp);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Controller) {
            internalParams[poolId].controller =
                LogNormalLib.decodeControllerUpdate(data);
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
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return LogNormalLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(params, (LogNormalParams))
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

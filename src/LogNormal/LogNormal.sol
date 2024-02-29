// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "src/lib/DynamicParamLib.sol";
import "src/lib/StrategyLib.sol";
import "./LogNormalExtendedLib.sol";
import "./LogNormalLib.sol";

/// @notice Log Normal has three variable parameters:
/// K - strike price
/// sigma - volatility
/// tau - time to expiry
///
/// Swaps are validated by the trading function:
/// Gaussian.ppf(x / L) + Gaussian.ppf(y / KL) = -sigma * sqrt(tau)
contract LogNormal is IStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam sigma;
        DynamicParam tau;
        DynamicParam strike;
        uint256 swapFee;
        address controller;
    }

    /// @dev Parameterization of the Log Normal curve.
    struct LogNormalParams {
        uint256 strike;
        uint256 sigma;
        uint256 tau;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy
    address public dfmm;

    /// @inheritdoc IStrategy
    string public constant name = "LogNormal";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    modifier onlyDFMM() {
        if (msg.sender != dfmm) revert NotDFMM();
        _;
    }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
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

        internalParams[poolId].sigma.lastComputedValue = params.sigma;
        internalParams[poolId].tau.lastComputedValue = params.tau;
        internalParams[poolId].strike.lastComputedValue = params.strike;
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

    error DeltaError(uint256 expected, uint256 actual);

    /// @inheritdoc IStrategy
    function validateAllocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        public
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 maxDeltaX, uint256 maxDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity = deltaL;
        deltaX = computeDeltaXGivenDeltaL(
            deltaLiquidity, pool.totalLiquidity, pool.reserveX
        );
        deltaY = computeDeltaYGivenDeltaX(deltaX, pool.reserveX, pool.reserveY);

        if (deltaX > maxDeltaX) revert DeltaError(maxDeltaX, deltaX);
        if (deltaY > maxDeltaY) revert DeltaError(maxDeltaY, deltaY);

        uint256 poolId = poolId;

        invariant = LogNormalLib.tradingFunction(
            pool.reserveX + deltaX,
            pool.reserveY + deltaY,
            pool.totalLiquidity + deltaLiquidity,
            abi.decode(getPoolParams(poolId), (LogNormalParams))
        );

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function validateDeallocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        public
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 minDeltaX, uint256 minDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity = deltaL;
        deltaX = computeDeltaXGivenDeltaL(
            deltaLiquidity, pool.totalLiquidity, pool.reserveX
        );
        deltaY = computeDeltaYGivenDeltaX(deltaX, pool.reserveX, pool.reserveY);

        if (minDeltaX > deltaX) revert DeltaError(minDeltaX, deltaX);
        if (minDeltaY > deltaY) revert DeltaError(minDeltaY, deltaY);

        uint256 poolId = poolId;

        invariant = LogNormalLib.tradingFunction(
            pool.reserveX - deltaX,
            pool.reserveY - deltaY,
            pool.totalLiquidity - deltaLiquidity,
            abi.decode(getPoolParams(poolId), (LogNormalParams))
        );

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function validateSwap(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes memory data
    )
        public
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity,
            bool isSwapXForY
        )
    {
        LogNormalParams memory params =
            abi.decode(getPoolParams(poolId), (LogNormalParams));

        (deltaX, deltaY, isSwapXForY) =
            abi.decode(data, (uint256, uint256, bool));

        if (isSwapXForY) {
            uint256 fees = deltaX.mulWadUp(params.swapFee);
            deltaLiquidity =
                fees.mulWadUp(pool.totalLiquidity).divWadUp(pool.reserveX);
            invariant = LogNormalLib.tradingFunction(
                pool.reserveX + deltaX,
                pool.reserveY - deltaY,
                pool.totalLiquidity + deltaLiquidity,
                params
            );
        } else {
            uint256 fees = deltaY.mulWadUp(params.swapFee);
            deltaLiquidity =
                fees.mulWadUp(pool.totalLiquidity).divWadUp(pool.reserveY);
            invariant = LogNormalLib.tradingFunction(
                pool.reserveX - deltaX,
                pool.reserveY + deltaY,
                pool.totalLiquidity + deltaLiquidity,
                params
            );
        }

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
        LogNormalLib.LogNormalUpdateCode updateCode =
            abi.decode(data, (LogNormalLib.LogNormalUpdateCode));

        if (updateCode == LogNormalLib.LogNormalUpdateCode.SwapFee) {
            internalParams[poolId].swapFee = LogNormalLib.decodeFeeUpdate(data);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Sigma) {
            (uint256 targetSigma, uint256 targetTimestamp) =
                LogNormalLib.decodeSigmaUpdate(data);
            internalParams[poolId].sigma.set(targetSigma, targetTimestamp);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Tau) {
            (uint256 targetTau, uint256 targetTimestamp) =
                LogNormalLib.decodeTauUpdate(data);
            internalParams[poolId].tau.set(targetTau, targetTimestamp);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Strike) {
            (uint256 targetStrike, uint256 targetTimestamp) =
                LogNormalLib.decodeStrikeUpdate(data);
            internalParams[poolId].strike.set(targetStrike, targetTimestamp);
        } else if (updateCode == LogNormalLib.LogNormalUpdateCode.Controller) {
            internalParams[poolId].controller =
                LogNormalLib.decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        LogNormalParams memory params;

        params.sigma = internalParams[poolId].sigma.actualized();
        params.strike = internalParams[poolId].strike.actualized();
        params.tau = internalParams[poolId].tau.actualized();
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) public view returns (int256) {
        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));
        return LogNormalLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (LogNormalParams))
        );
    }
}

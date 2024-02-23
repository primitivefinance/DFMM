// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./GeometricMeanLib.sol";
import "src/interfaces/IStrategy.sol";
import "src/lib/DynamicParamLib.sol";
import "./G3MExtendedLib.sol";

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
contract GeometricMean is IStrategy {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    struct InternalParams {
        DynamicParam wX;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy
    address public immutable dfmm;

    /// @inheritdoc IStrategy
    string public constant name = "GeometricMean";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    // TODO: Move these errors into an interface
    error InvalidWeightX();

    /// @dev Restricts the caller to the DFMM contract.
    modifier onlyDFMM() {
        if (msg.sender != address(dfmm)) revert NotDFMM();
        _;
    }

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

    error DeltaError(uint256 expected, uint256 actual);

    /// @inheritdoc IStrategy
    function validateAllocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 maxDeltaX, uint256 maxDeltaY, uint256 deltaLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        GeometricMeanParams memory params =
            abi.decode(getPoolParams(poolId), (GeometricMeanParams));

        uint256 adjustedReserveX = computeXGivenL(
            pool.totalLiquidity + deltaLiquidity,
            computePrice(pool.reserveX, pool.reserveY, params),
            params
        );

        uint256 adjustedReserveY = computeYGivenL(
            pool.totalLiquidity + deltaLiquidity,
            computePrice(pool.reserveX, pool.reserveY, params),
            params
        );

        deltaX = adjustedReserveX - pool.reserveX;
        deltaY = adjustedReserveY - pool.reserveY;

        if (deltaX > maxDeltaX) {
            revert DeltaError(maxDeltaX, deltaX);
        }

        if (deltaY > maxDeltaY) {
            revert DeltaError(maxDeltaY, deltaY);
        }

        uint256 adjustedLiquidity = pool.totalLiquidity + deltaLiquidity;

        invariant = GeometricMeanLib.tradingFunction(
            adjustedReserveX,
            adjustedReserveY,
            adjustedLiquidity,
            abi.decode(getPoolParams(poolId), (GeometricMeanParams))
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
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 minDeltaX, uint256 minDeltaY, uint256 deltaLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        GeometricMeanParams memory params =
            abi.decode(getPoolParams(poolId), (GeometricMeanParams));

        uint256 adjustedReserveX = computeXGivenL(
            pool.totalLiquidity - deltaLiquidity,
            computePrice(pool.reserveX, pool.reserveY, params),
            params
        );

        uint256 adjustedReserveY = computeYGivenL(
            pool.totalLiquidity - deltaLiquidity,
            computePrice(pool.reserveX, pool.reserveY, params),
            params
        );

        deltaX = pool.reserveX - adjustedReserveX;
        deltaY = pool.reserveY - adjustedReserveY;

        if (minDeltaX > deltaX) {
            revert DeltaError(minDeltaX, deltaX);
        }

        if (minDeltaY > deltaY) {
            revert DeltaError(minDeltaY, deltaY);
        }

        uint256 adjustedLiquidity = pool.totalLiquidity - deltaLiquidity;

        invariant = GeometricMeanLib.tradingFunction(
            adjustedReserveX,
            adjustedReserveY,
            adjustedLiquidity,
            abi.decode(getPoolParams(poolId), (GeometricMeanParams))
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
        external
        view
        returns (
            bool valid,
            int256 invariant,
            int256 liquidityDelta,
            uint256 nextRx,
            uint256 nextRy,
            uint256 nextL
        )
    {
        GeometricMeanParams memory params =
            abi.decode(getPoolParams(poolId), (GeometricMeanParams));

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 amountIn;
        uint256 fees;
        uint256 minLiquidityDelta;

        if (nextRx > pool.reserveX) {
            amountIn = nextRx - pool.reserveX;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta +=
                fees.mulWadUp(pool.totalLiquidity).divWadUp(pool.reserveX);
        } else if (nextRy > pool.reserveY) {
            amountIn = nextRy - pool.reserveY;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta +=
                fees.mulWadUp(pool.totalLiquidity).divWadUp(pool.reserveY);
        } else {
            revert("invalid swap: inputs x and y have the same sign!");
        }

        uint256 poolId = poolId;

        liquidityDelta = int256(nextL)
            - int256(
                GeometricMeanLib.computeNextLiquidity(
                    pool.reserveX,
                    pool.reserveY,
                    abi.decode(getPoolParams(poolId), (GeometricMeanParams))
                )
            );

        invariant =
            GeometricMeanLib.tradingFunction(nextRx, nextRy, nextL, params);
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
    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        GeometricMeanParams memory params;

        params.wX = internalParams[poolId].wX.actualized();
        params.wY = ONE - params.wX;
        params.swapFee = internalParams[poolId].swapFee;
        params.controller = internalParams[poolId].controller;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        return GeometricMeanLib.tradingFunction(
            rx, ry, L, abi.decode(getPoolParams(poolId), (GeometricMeanParams))
        );
    }
}

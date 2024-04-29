// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { Pool, IDFMM } from "src/interfaces/IDFMM.sol";
import {
    encodeFeeUpdate,
    encodeMeanUpdate,
    encodeWidthUpdate,
    encodeControllerUpdate,
    computeInitialPoolData
} from "src/LogNormal/LogNormalUtils.sol";
import { LogNormalParams } from "src/LogNormal/LogNormal.sol";
import {
    computeTradingFunction,
    computeNextLiquidity,
    computeXGivenL,
    computeNextRx,
    computeYGivenL,
    computeNextRy,
    computePriceGivenX,
    computePriceGivenY,
    computeDeltaLXIn,
    computeDeltaLYIn
} from "src/LogNormal/LogNormalMath.sol";
import {
    ISolver,
    InvalidTokenIndex,
    InvalidDeltasLength
} from "src/interfaces/ISolver.sol";
import {
    computeDeltaLGivenDeltaX,
    computeDeltaLGivenDeltaY,
    computeDeltaYGivenDeltaL,
    computeDeltaXGivenDeltaL
} from "src/lib/StrategyLib.sol";

/**
 * @title Solver contract for the LogNormal strategy.
 * @author Primitive
 */
contract LogNormalSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    /// @inheritdoc ISolver
    IStrategy public strategy;

    /**
     * @param strategy_ Address of the LogNormal strategy.
     */
    constructor(IStrategy strategy_) {
        strategy = strategy_;
    }

    /**
     * @notice Prepares the data to initialize a new LogNormal pool.
     * @param reserveX Initial reserve of token X, expressed in WAD.
     * @param S Strike price of the pool, expressed in WAD.
     * @param poolParams Parameters as defined in the LogNormal strategy.
     */
    function prepareInit(
        uint256 reserveX,
        uint256 S,
        LogNormalParams calldata poolParams
    ) external pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, poolParams);
    }

    /// @inheritdoc ISolver
    function prepareAllocation(
        uint256 poolId,
        uint256[] calldata deltas
    ) external view override returns (bytes memory) {
        if (deltas.length != 2) revert InvalidDeltasLength();

        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);

        uint256 deltaLGivenDeltaX =
            computeDeltaLGivenDeltaX(deltas[0], totalLiquidity, reserves[0]);
        uint256 deltaYGivenDeltaX = computeDeltaYGivenDeltaL(
            deltaLGivenDeltaX, totalLiquidity, reserves[1]
        );

        uint256 deltaLGivenDeltaY =
            computeDeltaLGivenDeltaY(deltas[1], totalLiquidity, reserves[1]);
        uint256 deltaXGivenDeltaL = computeDeltaXGivenDeltaL(
            deltaLGivenDeltaY, totalLiquidity, reserves[0]
        );

        if (deltaLGivenDeltaX < deltaLGivenDeltaY) {
            return abi.encode(deltas[0], deltaYGivenDeltaX, deltaLGivenDeltaX);
        } else {
            return abi.encode(deltaXGivenDeltaL, deltas[1], deltaLGivenDeltaY);
        }
    }

    /// @inheritdoc ISolver
    function prepareDeallocation(
        uint256 poolId,
        uint256 deltaLiquidity
    ) external view override returns (bytes memory) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);

        uint256 deltaX =
            computeDeltaXGivenDeltaL(deltaLiquidity, liquidity, reserves[0]);
        uint256 deltaY =
            computeDeltaYGivenDeltaL(deltaLiquidity, liquidity, reserves[1]);

        return abi.encode(deltaX, deltaY, deltaLiquidity);
    }

    struct SimulateSwapState {
        uint256 amountOut;
        uint256 deltaLiquidity;
        uint256 fees;
    }

    /// @inheritdoc ISolver
    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) revert InvalidTokenIndex();

        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        LogNormalParams memory poolParams = getPoolParams(poolId);

        SimulateSwapState memory state;

        {
            uint256 startComputedL = getNextLiquidity(
                poolId, preReserves[0], preReserves[1], preTotalLiquidity
            );

            if (tokenInIndex == 0) {
                state.deltaLiquidity = computeDeltaLXIn(
                    amountIn,
                    preReserves[0],
                    preReserves[1],
                    preTotalLiquidity,
                    poolParams
                );

                endReserves.rx = preReserves[0] + amountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice = computePriceGivenX(
                    endReserves.rx, endReserves.L, poolParams
                );

                endReserves.ry = getNextReserveY(
                    poolId, endReserves.rx, endReserves.L, approxPrice
                );

                require(
                    endReserves.ry < preReserves[1],
                    "invalid swap: y reserve increased!"
                );
                state.amountOut = preReserves[1] - endReserves.ry;
            } else {
                state.deltaLiquidity = computeDeltaLYIn(
                    amountIn,
                    preReserves[0],
                    preReserves[1],
                    preTotalLiquidity,
                    poolParams
                );

                endReserves.ry = preReserves[1] + amountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice = computePriceGivenY(
                    endReserves.ry, endReserves.L, poolParams
                );

                endReserves.rx = getNextReserveX(
                    poolId, endReserves.ry, endReserves.L, approxPrice
                );

                require(
                    endReserves.rx < preReserves[0],
                    "invalid swap: x reserve increased!"
                );
                state.amountOut = preReserves[0] - endReserves.rx;
            }
        }

        Pool memory pool;
        pool.reserves = new uint256[](2);
        pool.reserves[0] = endReserves.rx;
        pool.reserves[1] = endReserves.ry;
        pool.totalLiquidity = preTotalLiquidity;

        bytes memory swapData;

        if (tokenInIndex == 0) {
            swapData = abi.encode(0, 1, amountIn, state.amountOut);
        } else {
            swapData = abi.encode(1, 0, amountIn, state.amountOut);
        }

        uint256 poolId = poolId;
        (bool valid,,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (
            valid,
            state.amountOut,
            swapData
        );
    }

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) public view returns (uint256 price) {
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) revert InvalidTokenIndex();

        if (tokenInIndex == 0) {
            (uint256[] memory reserves, uint256 L) =
                getReservesAndLiquidity(poolId);
            price = computePriceGivenX(reserves[0], L, getPoolParams(poolId));
        } else {
            // TODO: Implement this.
        }
    }

    /// @inheritdoc ISolver
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        override
        returns (uint256[] memory reserves, uint256)
    {
        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
    }

    /**
     * @notice Returns the pool parameters of pool `poolId`.
     * @param poolId Id of the target pool.
     * @return Pool parameters as defined in the LogNormal strategy.
     */
    function getPoolParams(uint256 poolId)
        public
        view
        returns (LogNormalParams memory)
    {
        return abi.decode(strategy.getPoolParams(poolId), (LogNormalParams));
    }

    /**
     * @notice Prepares the data for updating the swap fee.
     * @param newSwapFee New swap fee to set.
     * @return Encoded data for updating the swap fee.
     */
    function prepareSwapFeeUpdate(uint256 newSwapFee)
        public
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(newSwapFee);
    }

    /**
     * @notice Prepares the data for updating the mean.
     * @param targetMean Final value of the mean parameter.
     * @param targetTimestamp Timestamp of the end of the update.
     * @return Encoded data for updating the mean.
     */
    function prepareMeanUpdate(
        uint256 targetMean,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeMeanUpdate(targetMean, targetTimestamp);
    }

    /**
     * @notice Prepares the data for updating the width.
     * @param targetWidth Final value of the width parameter.
     * @param targetTimestamp Timestamp of the end of the update.
     * @return Encoded data for updating the width.
     */
    function prepareWidthUpdate(
        uint256 targetWidth,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeWidthUpdate(targetWidth, targetTimestamp);
    }

    /**
     * @notice Prepares the data for updating the controller address.
     * @param newController Address of the new controller.
     * @return Encoded data for updating the controller.
     */
    function prepareControllerUpdate(address newController)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(newController);
    }

    function getNextLiquidity(
        uint256 poolId,
        uint256 rx,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        LogNormalParams memory poolParams = getPoolParams(poolId);

        int256 invariant = computeTradingFunction(rx, ry, L, poolParams);
        return computeNextLiquidity(rx, ry, invariant, L, poolParams);
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        LogNormalParams memory poolParams = getPoolParams(poolId);
        uint256 approximatedRx = computeXGivenL(L, S, poolParams);
        int256 invariant =
            computeTradingFunction(approximatedRx, ry, L, poolParams);
        return computeNextRx(ry, L, invariant, approximatedRx, poolParams);
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        LogNormalParams memory poolParams = getPoolParams(poolId);
        uint256 approximatedRy = computeYGivenL(L, S, poolParams);
        int256 invariant =
            computeTradingFunction(rx, approximatedRy, L, poolParams);
        return computeNextRy(rx, L, invariant, approximatedRy, poolParams);
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { Pool, IDFMM } from "src/interfaces/IDFMM.sol";
import {
    computeAllocationGivenX,
    computeAllocationGivenY
} from "src/lib/StrategyLib.sol";
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

contract LogNormalSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    uint256 public constant BISECTION_EPSILON = 0;
    uint256 public constant MAX_BISECTION_ITERS = 120;

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (LogNormalParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (LogNormalParams)
        );
    }

    function prepareFeeUpdate(uint256 swapFee)
        external
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(swapFee);
    }

    function prepareMeanUpdate(
        uint256 targetMean,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeMeanUpdate(targetMean, targetTimestamp);
    }

    function prepareWidthUpdate(
        uint256 targetWidth,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return encodeWidthUpdate(targetWidth, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        external
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(controller);
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        LogNormalParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256 deltaY, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 adjustedReserveX, uint256 adjustedLiquidity) =
            computeAllocationGivenX(true, deltaX, reserves[0], liquidity);
        uint256 approximatedPrice =
            getPriceGivenXL(poolId, adjustedReserveX, adjustedLiquidity);
        uint256 adjustedReserveY = getNextReserveY(
            poolId, adjustedReserveX, adjustedLiquidity, approximatedPrice
        );
        deltaY = adjustedReserveY - reserves[1];
        deltaLiquidity = adjustedLiquidity - liquidity;
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256 deltaX, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 adjustedReserveY, uint256 adjustedLiquidity) =
            computeAllocationGivenY(true, deltaY, reserves[1], liquidity);
        uint256 approximatedPrice =
            getPriceGivenYL(poolId, adjustedReserveY, adjustedLiquidity);
        uint256 adjustedReserveX = getNextReserveX(
            poolId, adjustedReserveY, adjustedLiquidity, approximatedPrice
        );
        deltaX = adjustedReserveX - reserves[0];
        deltaLiquidity = adjustedLiquidity - liquidity;
    }

    function allocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(true, amountX, reserves[0], L);
        uint256 approximatedPrice = getPriceGivenXL(poolId, nextRx, nextL);
        uint256 nextRy =
            getNextReserveY(poolId, nextRx, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function allocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(true, amountY, reserves[1], L);
        uint256 approximatedPrice = getPriceGivenYL(poolId, nextRy, nextL);
        uint256 nextRx =
            getNextReserveX(poolId, nextRy, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, reserves[0], L);
        uint256 approximatedPrice = getPriceGivenXL(poolId, nextRx, nextL);
        uint256 nextRy =
            getNextReserveY(poolId, nextRx, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, reserves[1], L);
        uint256 approximatedPrice = getPriceGivenYL(poolId, nextRy, nextL);
        uint256 nextRx =
            getNextReserveX(poolId, nextRy, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
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

    struct SimulateSwapState {
        uint256 amountOut;
        uint256 deltaLiquidity;
        uint256 fees;
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        LogNormalParams memory poolParams = getPoolParams(poolId);

        SimulateSwapState memory state;

        {
            uint256 startComputedL = getNextLiquidity(
                poolId, preReserves[0], preReserves[1], preTotalLiquidity
            );

            if (swapXIn) {
                state.deltaLiquidity = computeDeltaLXIn(
                    amountIn,
                    preReserves[0],
                    preReserves[1],
                    preTotalLiquidity,
                    poolParams
                );

                endReserves.rx = preReserves[0] + amountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice =
                    getPriceGivenXL(poolId, endReserves.rx, endReserves.L);

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
                uint256 approxPrice =
                    getPriceGivenYL(poolId, endReserves.ry, endReserves.L);

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
        pool.reserves = preReserves;
        pool.totalLiquidity = preTotalLiquidity;

        bytes memory swapData;

        if (swapXIn) {
            swapData = abi.encode(
                0, 1, amountIn, state.amountOut, state.deltaLiquidity
            );
        } else {
            swapData = abi.encode(
                1, 0, amountIn, state.amountOut, state.deltaLiquidity
            );
        }

        uint256 poolId = poolId;
        (bool valid,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (
            valid,
            state.amountOut,
            computePriceGivenX(endReserves.rx, endReserves.L, poolParams),
            swapData
        );
    }

    function getPriceGivenYL(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256 price) {
        price = computePriceGivenY(ry, L, getPoolParams(poolId));
    }

    function getPriceGivenXL(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256 price) {
        price = computePriceGivenX(rx, L, getPoolParams(poolId));
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        price = computePriceGivenX(reserves[0], L, getPoolParams(poolId));
    }
}

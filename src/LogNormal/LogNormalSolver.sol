// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "../lib/BisectionLib.sol";
import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
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
  computeNextLiquidity,
  computeXGivenL,
  computeNextRx,
  computeYGivenL,
  computeNextRy,
  computePriceGivenX,
  computePriceGivenY
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

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (LogNormalParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (LogNormalParams)
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
        return
            IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
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
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = rx;
        reserves[1] = ry;

        int256 invariant = IStrategy(strategy).tradingFunction(
            reserves, L, IStrategy(strategy).getPoolParams(poolId)
        );
        return
            computeNextLiquidity(rx, ry, invariant, L, fetchPoolParams(poolId));
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        uint256[] memory reserves = new uint256[](2);
        reserves[1] = ry;
        uint256 approximatedRx = computeXGivenL(L, S, fetchPoolParams(poolId));
        reserves[0] = approximatedRx;
        int256 invariant = IStrategy(strategy).tradingFunction(
            reserves, L, IStrategy(strategy).getPoolParams(poolId)
        );
        return computeNextRx(
            ry, L, invariant, approximatedRx, fetchPoolParams(poolId)
        );
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = rx;
        uint256 approximatedRy = computeYGivenL(L, S, fetchPoolParams(poolId));
        reserves[1] = approximatedRy;
        int256 invariant = IStrategy(strategy).tradingFunction(
            reserves, L, IStrategy(strategy).getPoolParams(poolId)
        );
        return computeNextRy(
            rx, L, invariant, approximatedRy, fetchPoolParams(poolId)
        );
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
        Reserves memory startReserves;
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        LogNormalParams memory poolParams = fetchPoolParams(poolId);

        SimulateSwapState memory state;

        {
            uint256 startComputedL = getNextLiquidity(
                poolId, preReserves[0], preReserves[1], preTotalLiquidity
            );

            if (swapXIn) {
                state.deltaLiquidity = amountIn.mulWadUp(poolParams.swapFee);

                endReserves.rx = preReserves[0] + amountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice =
                    getPriceGivenXL(poolId, endReserves.rx, endReserves.L);

                endReserves.ry = getNextReserveY(
                    poolId, endReserves.rx, endReserves.L, approxPrice
                );
                endReserves.ry += 1;

                require(
                    endReserves.ry < preReserves[1],
                    "invalid swap: y reserve increased!"
                );
                state.amountOut = preReserves[1] - endReserves.ry;
            } else {
                state.deltaLiquidity = amountIn.mulWadUp(poolParams.swapFee)
                    .divWadUp(poolParams.mean);

                endReserves.ry = preReserves[1] + amountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice =
                    getPriceGivenYL(poolId, endReserves.ry, endReserves.L);

                endReserves.rx = getNextReserveX(
                    poolId, endReserves.ry, endReserves.L, approxPrice
                );
                endReserves.rx += 1;

                require(
                    endReserves.rx < preReserves[0],
                    "invalid swap: x reserve increased!"
                );
                state.amountOut = preReserves[0] - endReserves.rx;
            }
        }

        IDFMM.Pool memory pool;
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
            computePriceGivenX(
                endReserves.rx, endReserves.L, poolParams
                ),
            swapData
        );
    }

    function getPriceGivenYL(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256 price) {
        price = computePriceGivenY(ry, L, fetchPoolParams(poolId));
    }

    function getPriceGivenXL(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256 price) {
        price = computePriceGivenX(rx, L, fetchPoolParams(poolId));
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        price = computePriceGivenX(
            reserves[0], L, fetchPoolParams(poolId)
        );
    }

}

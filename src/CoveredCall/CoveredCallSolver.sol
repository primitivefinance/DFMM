// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { Pool, IDFMM } from "src/interfaces/IDFMM.sol";
import { SignedWadMathLib } from "src/lib/SignedWadMath.sol";
import {
    computeAllocationGivenX,
    computeAllocationGivenY
} from "src/lib/StrategyLib.sol";
import {
    encodeFeeUpdate,
    encodeControllerUpdate,
    computeInitialPoolData,
    computeInitialPoolDataGivenY
} from "src/CoveredCall/CoveredCallUtils.sol";
import { CoveredCallParams } from "src/CoveredCall/CoveredCall.sol";
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
    computeDeltaLYIn,
    computeAllocationGivenDeltaX,
    computeAllocationGivenDeltaY,
    computeDeallocationGivenDeltaX,
    computeDeallocationGivenDeltaY,
    YEAR,
    ONE
} from "src/CoveredCall/CoveredCallMath.sol";
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

contract CoveredCallSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using SignedWadMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    uint256 public constant BISECTION_EPSILON = 0;
    uint256 public constant MAX_BISECTION_ITERS = 120;

    IStrategy public strategy;

    constructor(IStrategy _strategy) {
        strategy = _strategy;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (CoveredCallParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (CoveredCallParams)
        );
    }

    function getPoolParamsCustomTimestamp(
        uint256 poolId,
        uint256 timestamp
    ) public view returns (CoveredCallParams memory) {
        CoveredCallParams memory params = getPoolParams(poolId);
        params.lastTimestamp = timestamp;
        return params;
    }

    function prepareFeeUpdate(uint256 swapFee)
        external
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(swapFee);
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

    function prepareInit(
        uint256 reserveX,
        uint256 S,
        CoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, params);
    }

    function prepareInitGivenX(
        uint256 reserveX,
        uint256 S,
        CoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, params);
    }

    function prepareInitGivenY(
        uint256 reserveY,
        uint256 S,
        CoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolDataGivenY(reserveY, S, params);
    }

    function prepareAllocation(
        uint256 poolId,
        uint256[] calldata deltas
    ) public view returns (bytes memory) {
        if (deltas.length != 2) revert InvalidDeltasLength();

        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);

        CoveredCallParams memory params = getPoolParams(poolId);
        int256 lastInvariant = computeTradingFunction(
            reserves[0], reserves[1], totalLiquidity, params
        );

        uint256 nextReserveX = reserves[0] + deltas[0];
        uint256 nextReserveY = reserves[1] + deltas[1];
        uint256 nextLiquidity = computeNextLiquidity(
            nextReserveX, nextReserveY, lastInvariant, totalLiquidity, params
        );
        uint256 deltaLiquidity = nextLiquidity - totalLiquidity;

        return abi.encode(deltas, deltaLiquidity);
    }

    function prepareAllocationProportional(
        uint256 poolId,
        uint256[] calldata deltas
    ) public view returns (bytes memory) {
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

        uint256[] memory allocateDeltas = deltas;
        if (deltaLGivenDeltaX < deltaLGivenDeltaY) {
            allocateDeltas[1] = deltaYGivenDeltaX;
            return abi.encode(allocateDeltas, deltaLGivenDeltaX);
        } else {
            allocateDeltas[0] = deltaXGivenDeltaL;
            return abi.encode(allocateDeltas, deltaLGivenDeltaY);
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

        uint256[] memory deltas = new uint256[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = deltaY;

        return abi.encode(deltas, deltaLiquidity);
    }

    struct SimulateSwapState {
        uint256 amountOut;
        uint256 deltaLiquidity;
        uint256 fees;
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        CoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);

        SimulateSwapState memory state;

        uint256 startComputedL = getNextLiquidity(
            poolId, preReserves[0], preReserves[1], preTotalLiquidity
        );
        {
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
                uint256 approxPrice = getEstimatedPrice(poolId, 0, 1);

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
                uint256 approxPrice = getEstimatedPrice(poolId, 1, 0);

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

        if (tokenInIndex == 0) {
            swapData =
                abi.encode(0, 1, amountIn, state.amountOut, startComputedL);
        } else {
            swapData =
                abi.encode(1, 0, amountIn, state.amountOut, startComputedL);
        }

        uint256 poolId = poolId;
        (bool valid,,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (valid, state.amountOut, swapData);
    }

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) public view returns (uint256 price) {
        CoveredCallParams memory params =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) revert InvalidTokenIndex();

        if (tokenInIndex == 0) {
            price = computePriceGivenX(reserves[0], L, params);
        } else {
            price = computePriceGivenY(reserves[1], L, params);
        }
    }

    function getInvariant(uint256 poolId) public view returns (int256) {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        return computeTradingFunction(
            reserves[0], reserves[1], L, getPoolParams(poolId)
        );
    }

    function getNextLiquidity(
        uint256 poolId,
        uint256 rx,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        CoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);

        int256 invariant = computeTradingFunction(rx, ry, L, poolParams);
        return computeNextLiquidity(rx, ry, invariant, L, poolParams);
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        CoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
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
        CoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
        uint256 approximatedRy = computeYGivenL(L, S, poolParams);
        int256 invariant =
            computeTradingFunction(rx, approximatedRy, L, poolParams);
        return computeNextRy(rx, L, invariant, approximatedRy, poolParams);
    }
}

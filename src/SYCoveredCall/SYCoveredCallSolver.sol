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
} from "src/SYCoveredCall/SYCoveredCallUtils.sol";
import { SYCoveredCallParams } from "src/SYCoveredCall/SYCoveredCall.sol";
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
    computeKGivenLastPrice,
    YEAR,
    ONE
} from "src/SYCoveredCall/SYCoveredCallMath.sol";
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
import "forge-std/console2.sol";

contract SYCoveredCallSolver is ISolver {
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
        returns (SYCoveredCallParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (SYCoveredCallParams)
        );
    }

    function getPoolParamsCustomTimestamp(
        uint256 poolId,
        uint256 timestamp
    ) public view returns (SYCoveredCallParams memory) {
        SYCoveredCallParams memory params = getPoolParams(poolId);
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
        SYCoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, params);
    }

    function prepareInitGivenX(
        uint256 reserveX,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, params);
    }

    function prepareInitGivenY(
        uint256 reserveY,
        uint256 S,
        SYCoveredCallParams memory params
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

    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        SYCoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);

        SimulateSwapState memory state;

        poolParams.mean = computeKGivenLastPrice(
            preReserves[0], preTotalLiquidity, poolParams
        );

        int256 prevInvariant = computeTradingFunction(
            preReserves[0], preReserves[1], preTotalLiquidity, poolParams
        );

        console2.log("prevInvariant", prevInvariant);

        uint256 poolId = poolId;
        uint256 swapAmountIn = amountIn;
        {
            if (tokenInIndex == 0) {
                state.deltaLiquidity = computeDeltaLXIn(
                    swapAmountIn,
                    preReserves[0],
                    preReserves[1],
                    preTotalLiquidity,
                    poolParams
                );

                endReserves.rx = preReserves[0] + swapAmountIn;
                endReserves.L = preTotalLiquidity + state.deltaLiquidity;
                uint256 approxPrice = getIntermediatePrice(
                    endReserves.rx, endReserves.L, 0, 1, poolParams
                );

                endReserves.ry = getNextReserveY(
                    endReserves.rx,
                    endReserves.L,
                    approxPrice,
                    prevInvariant,
                    poolParams
                );

                require(
                    endReserves.ry < preReserves[1],
                    "invalid swap: y reserve increased!"
                );
                state.amountOut = preReserves[1] - endReserves.ry;
            } else {
                state.deltaLiquidity = computeDeltaLYIn(
                    swapAmountIn,
                    preReserves[0],
                    preReserves[1],
                    preTotalLiquidity,
                    poolParams
                );

                endReserves.ry = preReserves[1] + swapAmountIn;
                endReserves.L = preTotalLiquidity + state.deltaLiquidity;
                uint256 approxPrice = getIntermediatePrice(
                    endReserves.ry, endReserves.L, 1, 0, poolParams
                );

                endReserves.rx = getNextReserveX(
                    endReserves.ry,
                    endReserves.L,
                    approxPrice,
                    prevInvariant,
                    poolParams
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
            swapData = abi.encode(0, 1, swapAmountIn, state.amountOut);
        } else {
            swapData = abi.encode(1, 0, swapAmountIn, state.amountOut);
        }

        (bool valid,,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (valid, state.amountOut, swapData);
    }

    function getIntermediatePrice(
        uint256 independentReserve,
        uint256 L,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        SYCoveredCallParams memory params
    ) public view returns (uint256 price) {
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) revert InvalidTokenIndex();

        if (tokenInIndex == 0) {
            price = computePriceGivenX(independentReserve, L, params);
        } else {
            price = computePriceGivenY(independentReserve, L, params);
        }
    }

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) public view returns (uint256 price) {
        SYCoveredCallParams memory params =
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

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        (uint256[] memory reserves, uint256 L) = getReservesAndLiquidity(poolId);
        price = computePriceGivenX(reserves[0], L, getPoolParams(poolId));
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
        SYCoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);

        int256 invariant = computeTradingFunction(rx, ry, L, poolParams);
        return computeNextLiquidity(rx, ry, invariant, L, poolParams);
    }

    function getNextReserveX(
        uint256 ry,
        uint256 L,
        uint256 S,
        int256 prevInvariant,
        SYCoveredCallParams memory params
    ) public view returns (uint256) {
        uint256 approximatedRx = computeXGivenL(L, S, params);
        int256 invariant = computeTradingFunction(approximatedRx, ry, L, params);
        console2.log("intermediate invariant", invariant);
        return computeNextRx(
            ry, L, invariant, prevInvariant, approximatedRx, params
        );
    }

    function getNextReserveY(
        uint256 rx,
        uint256 L,
        uint256 S,
        int256 prevInvariant,
        SYCoveredCallParams memory params
    ) public view returns (uint256) {
        uint256 approximatedRy = computeYGivenL(L, S, params);
        int256 invariant = computeTradingFunction(rx, approximatedRy, L, params);
        console2.log("intermediate invariant", invariant);
        return computeNextRy(
            rx, L, invariant, prevInvariant, approximatedRy, params
        );
    }
}

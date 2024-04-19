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
import "forge-std/console2.sol";

contract SYCoveredCallSolver {
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

    address public strategy;

    constructor(address _strategy) {
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

    function getInitialPoolDataGivenX(
        uint256 rX,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rX, S, params);
    }

    function getInitialPoolDataGivenY(
        uint256 rY,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolDataGivenY(rY, S, params);
    }

    function prepareInitialPoolDataGivenY(
        uint256 rY,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolDataGivenY(rY, S, params);
    }

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256 deltaY, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (deltaY, deltaLiquidity) = computeAllocationGivenDeltaX(
            deltaX, reserves[0], reserves[1], liquidity
        );
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256 deltaX, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (deltaX, deltaLiquidity) = computeAllocationGivenDeltaY(
            deltaY, reserves[0], reserves[1], liquidity
        );
    }

    function deallocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256 deltaY, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (deltaY, deltaLiquidity) = computeDeallocationGivenDeltaX(
            deltaX, reserves[0], reserves[1], liquidity
        );
    }

    function deallocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256 deltaX, uint256 deltaLiquidity) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (deltaX, deltaLiquidity) = computeDeallocationGivenDeltaY(
            deltaY, reserves[0], reserves[1], liquidity
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
        uint256 poolId,
        uint256 ry,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        SYCoveredCallParams memory poolParams =
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
        SYCoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
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
        uint256 amountIn,
        uint256 timestamp
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        SYCoveredCallParams memory poolParams =
            getPoolParamsCustomTimestamp(poolId, timestamp);

        SimulateSwapState memory state;

        uint256 startComputedL = getNextLiquidity(
            poolId, preReserves[0], preReserves[1], preTotalLiquidity
        );
        poolParams.mean = computeKGivenLastPrice(
            preReserves[0], preTotalLiquidity, poolParams
        );
        {
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
        SYCoveredCallParams memory params =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
        price = computePriceGivenY(ry, L, params);
    }

    function getPriceGivenXL(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256 price) {
        SYCoveredCallParams memory params =
            getPoolParamsCustomTimestamp(poolId, block.timestamp);
        price = computePriceGivenX(rx, L, params);
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
}

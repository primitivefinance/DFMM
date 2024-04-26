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
        uint256 rx,
        uint256 ry,
        uint256 L,
        SYCoveredCallParams memory params
    ) public pure returns (uint256) {
        int256 invariant = computeTradingFunction(rx, ry, L, params);
        return computeNextLiquidity(rx, ry, invariant, L, params);
    }

    function getNextReserveX(
        uint256 ry,
        uint256 L,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (uint256) {
        uint256 approximatedRx = computeXGivenL(L, S, params);
        int256 invariant =
            computeTradingFunction(approximatedRx, ry, L, params);
        return computeNextRx(ry, L, invariant, approximatedRx, params);
    }

    function getNextReserveY(
        uint256 rx,
        uint256 L,
        uint256 S,
        SYCoveredCallParams memory params
    ) public pure returns (uint256) {
        uint256 approximatedRy = computeYGivenL(L, S, params);
        int256 invariant =
            computeTradingFunction(rx, approximatedRy, L, params);
        return computeNextRy(rx, L, invariant, approximatedRy, params);
    }

    struct SimulateSwapState {
        uint256 amountOut;
        uint256 deltaLiquidity;
        uint256 fees;
        uint256 timestamp;
    }

    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn,
        uint256 timestamp
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory endReserves;
        (uint256[] memory preReserves, uint256 preTotalLiquidity) =
            getReservesAndLiquidity(poolId);
        SYCoveredCallParams memory params =
            getPoolParamsCustomTimestamp(poolId, timestamp);

        SimulateSwapState memory state;
        state.timestamp = timestamp;
        params.lastTimestamp = timestamp;
        console2.log("initial K", params.mean);

        params.mean = computeKGivenLastPrice(
            preReserves[0], preTotalLiquidity, params 
        );
        console2.log("updated K", params.mean);

        console2.log("preL", preTotalLiquidity);
        uint256 startComputedL = getNextLiquidity(
            preReserves[0], preReserves[1], preTotalLiquidity, params 
        );
        console2.log("computedL", startComputedL);

        uint256 poolId = poolId;
        uint256 swapAmountIn = amountIn;
        bool swapXToY = swapXIn;
        {
            if (swapXToY) {
                state.deltaLiquidity = computeDeltaLXIn(
                    swapAmountIn,
                    preReserves[0],
                    preReserves[1],
                    startComputedL,
                    params 
                );

                endReserves.rx = preReserves[0] + swapAmountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice =
                    getPriceGivenXL(endReserves.rx, endReserves.L, params);

                endReserves.ry = getNextReserveY(
                    endReserves.rx, endReserves.L, approxPrice, params 
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
                    startComputedL,
                    params 
                );

                endReserves.ry = preReserves[1] + swapAmountIn;
                endReserves.L = startComputedL + state.deltaLiquidity;
                uint256 approxPrice =
                    getPriceGivenYL(endReserves.ry, endReserves.L, params);

                endReserves.rx = getNextReserveX(
                    endReserves.ry, endReserves.L, approxPrice, params 
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

        if (swapXToY) {
            swapData = abi.encode(
                0,
                1,
                swapAmountIn,
                state.amountOut,
                startComputedL,
                state.timestamp
            );
        } else {
            swapData = abi.encode(
                1,
                0,
                swapAmountIn,
                state.amountOut,
                startComputedL,
                state.timestamp
            );
        }

        (bool valid,,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (
            valid,
            state.amountOut,
            computePriceGivenX(endReserves.rx, endReserves.L, params),
            swapData
        );
    }

    function getPriceGivenYL(
        uint256 ry,
        uint256 L,
        SYCoveredCallParams memory params
    ) public pure returns (uint256 price) {
        price = computePriceGivenY(ry, L, params);
    }

    function getPriceGivenXL(
        uint256 rx,
        uint256 L,
        SYCoveredCallParams memory params
    ) public pure returns (uint256 price) {
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

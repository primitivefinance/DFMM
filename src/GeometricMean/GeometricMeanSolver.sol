// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy2 } from "src/interfaces/IStrategy2.sol";
import { IDFMM2 } from "src/interfaces/IDFMM2.sol";
import { computeAllocationGivenX } from "src/lib/StrategyLib.sol";
import { GeometricMeanParams } from "./GeometricMean2.sol";
import {
    encodeFeeUpdate,
    encodeWeightXUpdate,
    encodeControllerUpdate,
    computeInitialPoolData
} from "./G3MUtils.sol";
import {
    computeL,
    computePrice,
    computeLGivenX,
    computeY,
    computeX,
    computeLGivenY,
    computeNextLiquidity,
    computeNextRx,
    computeNextRy,
    computeTradingFunction
} from "./G3MMath.sol";

contract GeometricMeanSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    IStrategy2 public strategy;

    constructor(address strategy_) {
        strategy = IStrategy2(strategy_);
    }

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return encodeFeeUpdate(swapFee);
    }

    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return encodeWeightXUpdate(targetWeightX, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(controller);
    }

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory)
    {
        return abi.decode(
            IStrategy2(strategy).getPoolParams(poolId), (GeometricMeanParams)
        );
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            IDFMM2(IStrategy2(strategy).dfmm()).getReservesAndLiquidity(poolId);
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        GeometricMeanParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function allocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(true, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (nextRx, nextRy, nextL);
    }

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256, uint256) {
        (uint256 reserveX, uint256 reserveY,) = getReservesAndLiquidity(poolId);
        GeometricMeanParams memory params = getPoolParams(poolId);
        uint256 S = computePrice(reserveX, reserveY, params);
        uint256 deltaLiquidity = computeLGivenX(deltaX, S, params);
        uint256 deltaY = computeY(deltaX, S, params);
        return (deltaY, deltaLiquidity);
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256, uint256) {
        (uint256 reserveX, uint256 reserveY,) = getReservesAndLiquidity(poolId);
        GeometricMeanParams memory params = getPoolParams(poolId);
        uint256 S = computePrice(reserveX, reserveY, params);
        uint256 deltaLiquidity = computeLGivenY(deltaY, S, params);
        uint256 deltaX = computeX(deltaY, S, params);
        return (deltaX, deltaLiquidity);
    }

    function allocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(true, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenXReturnDeltas(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (rx - nextRx, ry - nextRy, L - nextL);
    }

    function deallocateGivenYReturnDeltas(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (rx - nextRx, ry - nextRy, L - nextL);
    }

    function deallocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (nextRx, nextRy, nextL);
    }

    function getNextLiquidity(
        uint256 poolId,
        uint256 rx,
        uint256 ry
    ) public view returns (uint256) {
        return computeNextLiquidity(rx, ry, fetchPoolParams(poolId));
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRx(ry, L, fetchPoolParams(poolId));
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRy(rx, L, fetchPoolParams(poolId));
    }

    struct SimulateSwapState {
        uint256 amountIn;
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
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        IDFMM2.Pool memory pool =
            IDFMM2(IStrategy2(strategy).dfmm()).getPool(poolId);

        SimulateSwapState memory state;

        state.fees = amountIn.mulWadUp(params.swapFee);

        if (swapXIn) {
            state.deltaLiquidity = pool.totalLiquidity.divWadUp(
                pool.reserves[0]
            ).mulWadUp(state.fees).mulWadUp(params.wX);
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 d = uint256(
                int256((pool.reserves[0] + amountIn)).powWad(int256(params.wX))
            );
            uint256 a = uint256(
                int256(n.divWadUp(d)).powWad(
                    int256(FixedPointMathLib.WAD.divWadUp(params.wY))
                )
            );
            state.amountOut = pool.reserves[1] - a;
        } else {
            state.deltaLiquidity = pool.totalLiquidity.divWadUp(
                pool.reserves[1]
            ).mulWadUp(state.fees).mulWadUp(params.wY);
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 d = uint256(
                int256((pool.reserves[1] + amountIn)).powWad(int256(params.wY))
            );
            uint256 a = uint256(
                int256(n.divWadUp(d)).powWad(
                    int256(FixedPointMathLib.WAD.divWadUp(params.wX))
                )
            );
            state.amountOut = pool.reserves[1] - a;
        }

        bytes memory swapData;

        if (swapXIn) {
            swapData = abi.encode(
                amountIn, state.amountOut, state.deltaLiquidity, true
            );
        } else {
            swapData = abi.encode(
                state.amountOut, amountIn, state.deltaLiquidity, false
            );
        }

        (bool valid,,,,,,) = IStrategy2(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (
            valid,
            state.amountOut,
            computePrice(pool.reserves[0], pool.reserves[1], params),
            swapData
        );
    }

    /*
    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalLower(S, rx, ry, L, vUpper, params);
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalRaise(S, rx, ry, L, vUpper, params);
    }

    function calculateDiffLower(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffLower(S, rx, ry, L, v, params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffRaise(S, rx, ry, L, v, params);
    }
    */

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry,) = getReservesAndLiquidity(poolId);
        price = computePrice(rx, ry, params);
    }

    function checkSwapConstant(
        uint256 poolId,
        bytes calldata data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        return computeTradingFunction(rx, ry, L, params);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory params)
    {
        return abi.decode(
            IStrategy2(strategy).getPoolParams(poolId), (GeometricMeanParams)
        );
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { IDFMM, Pool } from "src/interfaces/IDFMM.sol";
import { GeometricMeanParams } from "./GeometricMean.sol";
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
    computeTradingFunction,
    computeAllocationGivenDeltaX,
    computeAllocationGivenDeltaY,
    computeDeallocationGivenDeltaX,
    computeDeallocationGivenDeltaY
} from "./G3MMath.sol";

contract GeometricMeanSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    IStrategy public strategy;

    constructor(address strategy_) {
        strategy = IStrategy(strategy_);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory params)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (GeometricMeanParams)
        );
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves[0], pool.reserves[1], pool.totalLiquidity);
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

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        GeometricMeanParams memory params = getPoolParams(poolId);
        (uint256 rx, uint256 ry,) = getReservesAndLiquidity(poolId);
        price = computePrice(rx, ry, params);
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        GeometricMeanParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256, uint256) {
        (uint256 rX, uint256 rY, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 deltaY, uint256 deltaLiquidity) =
            computeAllocationGivenDeltaX(deltaX, rX, rY, totalLiquidity);
        return (deltaY, deltaLiquidity);
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256, uint256) {
        (uint256 rX, uint256 rY, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 deltaX, uint256 deltaLiquidity) =
            computeAllocationGivenDeltaY(deltaY, rX, rY, totalLiquidity);
        return (deltaX, deltaLiquidity);
    }

    function deallocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256, uint256) {
        (uint256 rX, uint256 rY, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 deltaY, uint256 deltaLiquidity) =
            computeDeallocationGivenDeltaX(deltaX, rX, rY, totalLiquidity);
        return (deltaY, deltaLiquidity);
    }

    function deallocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256, uint256) {
        (uint256 rX, uint256 rY, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 deltaX, uint256 deltaLiquidity) =
            computeDeallocationGivenDeltaY(deltaY, rX, rY, totalLiquidity);
        return (deltaX, deltaLiquidity);
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRx(ry, L, getPoolParams(poolId));
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRy(rx, L, getPoolParams(poolId));
    }

    struct SimulateSwapState {
        uint256 amountIn;
        uint256 amountOut;
        uint256 inReserve;
        uint256 outReserve;
        uint256 inWeight;
        uint256 outWeight;
        uint256 deltaLiquidity;
        uint256 fees;
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        GeometricMeanParams memory params = getPoolParams(poolId);
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);

        SimulateSwapState memory state;

        state.inReserve = pool.reserves[tokenInIndex];
        state.outReserve = pool.reserves[tokenOutIndex];
        if (tokenInIndex == 0) {
            state.inWeight = params.wX;
            state.outWeight = params.wY;
        } else {
            state.inWeight = params.wY;
            state.outWeight = params.wX;
        }

        state.fees = amountIn.mulWadUp(params.swapFee);
        state.deltaLiquidity = pool.totalLiquidity.divWadUp(state.inReserve)
            .mulWadUp(state.fees).mulWadUp(state.inWeight);
        {
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 d = uint256(
                int256((state.inReserve + amountIn)).powWad(
                    int256(state.inWeight)
                )
            );
            uint256 a = uint256(
                int256(n.divWadUp(d)).powWad(
                    int256(FixedPointMathLib.WAD.divWadUp(state.outWeight))
                )
            );

            state.amountOut = state.outReserve - a;
        }

        bytes memory swapData = abi.encode(
            tokenInIndex,
            tokenOutIndex,
            amountIn,
            state.amountOut,
            state.deltaLiquidity
        );

        (bool valid,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (valid, state.amountOut, swapData);
    }

    function checkSwapConstant(
        uint256 poolId,
        bytes calldata data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        GeometricMeanParams memory params = getPoolParams(poolId);
        return computeTradingFunction(rx, ry, L, params);
    }
}

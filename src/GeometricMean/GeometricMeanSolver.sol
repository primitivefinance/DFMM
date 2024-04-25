// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { IDFMM, Pool } from "src/interfaces/IDFMM.sol";
import { GeometricMeanParams } from "./GeometricMean.sol";
import {
    encodeFeeUpdate,
    encodeWeightXUpdate,
    encodeControllerUpdate
} from "./G3MUtils.sol";
import {
    computeInitialPoolData,
    computeNextRx,
    computeNextRy,
    computeTradingFunction,
    computeAllocationGivenDeltaX,
    computeAllocationGivenDeltaY,
    computeDeallocationGivenDeltaX,
    computeDeallocationGivenDeltaY,
    computePrice,
    computeSwapDeltaLiquidity,
    computeDeltaGivenDeltaLRoundUp,
    ONE
} from "./G3MMath.sol";
import { ISolver } from "src/interfaces/ISolver.sol";

error InvalidTokenIndex();
error InvalidDeltasLength();

contract GeometricMeanSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @inheritdoc ISolver
    IStrategy public strategy;

    /// @param strategy_ Address of the ConstantSum strategy contract.
    constructor(IStrategy strategy_) {
        strategy = strategy_;
    }

    function prepareInit(
        uint256 reserveX,
        uint256 S,
        GeometricMeanParams memory poolParams
    ) external pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, S, poolParams);
    }

    /// @inheritdoc ISolver
    function prepareAllocation(
        uint256 poolId,
        uint256[] memory deltas
    ) external view returns (bytes memory) {
        if (deltas.length != 2) revert InvalidDeltasLength();

        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);

        (uint256 deltaYGivenX, uint256 deltaLGivenX) =
        computeAllocationGivenDeltaX(
            deltas[0], reserves[0], reserves[1], totalLiquidity
        );

        (uint256 deltaXGivenY, uint256 deltaLGivenY) =
        computeAllocationGivenDeltaY(
            deltas[1], reserves[0], reserves[1], totalLiquidity
        );

        if (deltaLGivenX < deltaLGivenY) {
            return abi.encode(deltas[0], deltaYGivenX, deltaLGivenX);
        } else {
            return abi.encode(deltaXGivenY, deltas[1], deltaLGivenY);
        }
    }

    /// @inheritdoc ISolver
    function prepareDeallocation(
        uint256 poolId,
        uint256 deltaLiquidity
    ) external view returns (bytes memory) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);

        uint256 deltaX = computeDeltaGivenDeltaLRoundUp(
            reserves[0], deltaLiquidity, liquidity
        );

        uint256 deltaY = computeDeltaGivenDeltaLRoundUp(
            reserves[1], deltaLiquidity, liquidity
        );

        return abi.encode(deltaX, deltaY, deltaLiquidity);
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

    /// @inheritdoc ISolver
    function prepareSwap(
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

        state.deltaLiquidity = computeSwapDeltaLiquidity(
            amountIn,
            state.inReserve,
            pool.totalLiquidity,
            state.inWeight,
            params.swapFee
        );

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

        bytes memory swapData =
            abi.encode(tokenInIndex, tokenOutIndex, amountIn, state.amountOut);

        (bool valid,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (valid, state.amountOut, swapData);
    }

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) public view returns (uint256) {
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) {
            revert InvalidTokenIndex();
        }

        GeometricMeanParams memory params = getPoolParams(poolId);
        (uint256[] memory reserves,) = getReservesAndLiquidity(poolId);

        if (tokenInIndex == 0) {
            return computePrice(
                reserves[tokenInIndex], reserves[tokenOutIndex], params
            );
        } else {
            return ONE.divWadUp(
                computePrice(
                    reserves[tokenOutIndex], reserves[tokenInIndex], params
                )
            );
        }
    }

    /// @inheritdoc ISolver
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        override
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory params)
    {
        return abi.decode(strategy.getPoolParams(poolId), (GeometricMeanParams));
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

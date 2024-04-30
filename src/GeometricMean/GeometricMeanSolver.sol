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
    computeAllocationGivenDeltaX,
    computeAllocationGivenDeltaY,
    computePrice,
    computeSwapDeltaLiquidity,
    computeDeltaGivenDeltaLRoundUp,
    computeL,
    ONE
} from "./G3MMath.sol";
import {
    ISolver,
    InvalidTokenIndex,
    InvalidDeltasLength
} from "src/interfaces/ISolver.sol";

/**
 * @title Solver for the GeometricMean strategy.
 * @author Primitive
 */
contract GeometricMeanSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @inheritdoc ISolver
    IStrategy public strategy;

    /// @param strategy_ Address of the ConstantSum strategy contract.
    constructor(IStrategy strategy_) {
        strategy = strategy_;
    }

    /**
     * @notice Prepares the data to initialize a new GeometricMean pool.
     * @param reserveX Initial reserve of token X.
     * @param S Price of the pool, in WAD units.
     * @param poolParams Parameters as defined by the GeometricMean strategy.
     */
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

        GeometricMeanParams memory params = getPoolParams(poolId);

        uint256 deltaLiquidity =
            computeL(reserves[0] + deltas[0], reserves[1] + deltas[1], params) - totalLiquidity;

        return abi.encode(deltas, deltaLiquidity);
    }

    /// @inheritdoc ISolver
    function prepareAllocationProportional(
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
            deltas[1] = deltaYGivenX;
            return abi.encode(deltas, deltaLGivenX);
        } else {
            deltas[0] = deltaXGivenY;
            return abi.encode(deltas, deltaLGivenY);
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

        uint[] memory deltas = new uint[](reserves.length);
        deltas[0] = deltaX;
        deltas[1] = deltaY;

        return abi.encode(deltas, deltaLiquidity);
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
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) {
            revert InvalidTokenIndex();
        }

        GeometricMeanParams memory params = getPoolParams(poolId);
        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);

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

    /**
     * @notice Returns the parameters of the pool `poolId`.
     * @param poolId Id of the target pool.
     * @return Parameters as defined by the GeometricMean strategy.
     */
    function getPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory)
    {
        return abi.decode(strategy.getPoolParams(poolId), (GeometricMeanParams));
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
     * @notice Prepares the data for updating the weight of token X.
     * @param targetWeightX Final weight of token X.
     * @param targetTimestamp Timestamp of the end of the weight update.
     */
    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return encodeWeightXUpdate(targetWeightX, targetTimestamp);
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
}

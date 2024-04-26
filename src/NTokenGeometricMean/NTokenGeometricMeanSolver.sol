// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";
import { IDFMM, Pool } from "src/interfaces/IDFMM.sol";
import { NTokenGeometricMeanParams } from
    "src/NTokenGeometricMean/NTokenGeometricMean.sol";
import {
    encodeFeeUpdate,
    encodeWeightsUpdate,
    encodeControllerUpdate,
    computeInitialPoolData
} from "src/NTokenGeometricMean/NTokenGeometricMeanUtils.sol";
import {
    computeAllocationDeltasGivenDeltaT,
    computeDeallocationDeltasGivenDeltaT,
    computeNextLiquidity,
    computeSwapDeltaLiquidity,
    computeDeltaGivenDeltaLRoundDown
} from "src/NTokenGeometricMean/NTokenGeometricMeanMath.sol";
import {
    ISolver,
    InvalidTokenIndex,
    InvalidDeltasLength
} from "src/interfaces/ISolver.sol";

/**
 * @title Solver for the NTokenGeometricMean strategy.
 * @author Primitive
 */
contract NTokenGeometricMeanSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @inheritdoc ISolver
    IStrategy public strategy;

    /// @param strategy_ Address of the NTokenGeometricMean strategy contract.
    constructor(IStrategy strategy_) {
        strategy = strategy_;
    }

    /**
     * @notice Prepares the data to initialize a new NTokenGeometricMean pool.
     * @param numeraireAmount Amount of the numeraire token to deposit.
     * @param prices Prices of the tokens in the pool in WAD units.
     * @param params Parameters as defined by the NTokenGeometricMean strategy.
     */
    function prepareInit(
        uint256 numeraireAmount,
        uint256[] memory prices,
        NTokenGeometricMeanParams memory params
    ) external pure returns (bytes memory) {
        return computeInitialPoolData(numeraireAmount, prices, params);
    }

    /// @inheritdoc ISolver
    function prepareAllocation(
        uint256 poolId,
        uint256[] calldata deltas
    ) public view returns (bytes memory) {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        uint256 length = deltas.length;
        if (deltas.length != reserves.length) revert InvalidDeltasLength();

        uint256 minDeltaLiquidity =
            totalLiquidity.mulDivDown(deltas[0], reserves[0]);
        uint256 minDeltaLiquidityIndex = 0;

        for (uint256 i = 0; i < length; i++) {
            uint256 dLiquidity =
                totalLiquidity.mulDivDown(deltas[i], reserves[i]);

            if (dLiquidity < minDeltaLiquidity) {
                minDeltaLiquidity = dLiquidity;
                minDeltaLiquidityIndex = i;
            }
        }

        (uint256[] memory deltaTokens, uint256 deltaLiquidity) =
        computeAllocationDeltasGivenDeltaT(
            deltas[minDeltaLiquidityIndex],
            minDeltaLiquidityIndex,
            reserves,
            totalLiquidity
        );

        return abi.encode(deltaTokens, deltaLiquidity);
    }

    /// @inheritdoc ISolver
    function prepareDeallocation(
        uint256 poolId,
        uint256 deltaLiquidity
    ) public view returns (bytes memory) {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        uint256 length = reserves.length;

        uint256[] memory deltas = new uint256[](length);

        for (uint256 i = 0; i < length; i++) {
            deltas[i] = computeDeltaGivenDeltaLRoundDown(
                reserves[i], deltaLiquidity, totalLiquidity
            );
        }

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
        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);

        uint256 maxIndex = pool.reserves.length - 1;
        if (
            tokenInIndex > maxIndex || tokenOutIndex > maxIndex
                || tokenInIndex == tokenOutIndex
        ) {
            revert InvalidTokenIndex();
        }

        NTokenGeometricMeanParams memory params = getPoolParams(poolId);
        SimulateSwapState memory state;

        state.inReserve = pool.reserves[tokenInIndex];
        state.outReserve = pool.reserves[tokenOutIndex];
        state.inWeight = params.weights[tokenInIndex];
        state.outWeight = params.weights[tokenOutIndex];

        state.deltaLiquidity = computeSwapDeltaLiquidity(
            amountIn,
            state.inReserve,
            pool.totalLiquidity,
            state.inWeight,
            params.swapFee
        );

        {
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 accumulator = FixedPointMathLib.WAD;
            for (uint256 i = 0; i < pool.reserves.length; i++) {
                if (i != tokenOutIndex && i != tokenInIndex) {
                    uint256 di = uint256(
                        int256(pool.reserves[i]).powWad(
                            int256(params.weights[i])
                        )
                    );
                    accumulator = accumulator.mulWadUp(di);
                }
            }
            uint256 d = uint256(
                int256((state.inReserve + amountIn)).powWad(
                    int256(state.inWeight)
                )
            );
            uint256 a = uint256(
                int256(n.divWadUp(d.mulWadUp(accumulator))).powWad(
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

    /**
     * @notice Returns the parameters of the pool `poolId`.
     * @param poolId Id of the target pool.
     * @return params Pool parameters as defined by the NTokenGeometricMean strategy.
     */
    function getPoolParams(uint256 poolId)
        public
        view
        returns (NTokenGeometricMeanParams memory)
    {
        return abi.decode(
            strategy.getPoolParams(poolId), (NTokenGeometricMeanParams)
        );
    }

    /// @inheritdoc ISolver
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
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
     * @notice Prepares the data for updating the weights.
     * @param targetWeights New weights to set.
     * @param targetTimestamp Timestamp at which the update will end.
     * @return Encoded data for updating the weights.
     */
    function prepareWeightsUpdate(
        uint256[] calldata targetWeights,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return encodeWeightsUpdate(targetWeights, targetTimestamp);
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

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) external view returns (uint256) {
        (uint256[] memory reserves,) = getReservesAndLiquidity(poolId);
        NTokenGeometricMeanParams memory params = getPoolParams(poolId);

        // TODO: Use a predefined function ideally defined in the related
        // Math library instead of the following implementation.
        uint256 a = params.weights[tokenInIndex].divWadDown(
            params.weights[tokenOutIndex]
        );
        uint256 b = reserves[tokenOutIndex].divWadDown(reserves[tokenInIndex]);
        return a.mulWadDown(b);
    }
}

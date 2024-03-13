// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "src/interfaces/IStrategy.sol";
import { IDFMM } from "src/interfaces/IDFMM.sol";
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
    computeNextLiquidity
} from "src/NTokenGeometricMean/NTokenGeometricMeanMath.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";

contract NTokenGeometricMeanSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256[] reserves;
        uint256 L;
    }

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (NTokenGeometricMeanParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (NTokenGeometricMeanParams)
        );
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256[] memory, uint256)
    {
        return IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
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

    function simulateSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        NTokenGeometricMeanParams memory params = getPoolParams(poolId);
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).getPool(poolId);

        SimulateSwapState memory state;

        state.inReserve = pool.reserves[tokenInIndex];
        state.outReserve = pool.reserves[tokenOutIndex];
        state.inWeight = params.weights[tokenInIndex];
        state.outWeight = params.weights[tokenOutIndex];

        state.fees = amountIn.mulWadUp(params.swapFee);
        state.deltaLiquidity = pool.totalLiquidity.divWadUp(state.inReserve)
            .mulWadUp(state.fees).mulWadUp(state.inWeight);
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

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return encodeFeeUpdate(swapFee);
    }

    function prepareWeightsUpdate(
        uint256[] calldata targetWeights,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return encodeWeightsUpdate(targetWeights, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(controller);
    }

    function computePriceOfToken(
        uint256 rT,
        uint256 rNumeraire,
        uint256 wT,
        uint256 wNumeraire
    ) public pure returns (uint256 price) {
        uint256 a = wT.divWadDown(wNumeraire);
        uint256 b = rNumeraire.divWadDown(rT);
        price = a.mulWadDown(b);
    }

    function getInitialPoolData(
        uint256 numeraireAmount,
        uint256[] memory prices,
        NTokenGeometricMeanParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(numeraireAmount, prices, params);
    }

    function getAllocationDeltasGivenDeltaT(
        uint256 poolId,
        uint256 indexT,
        uint256 deltaT
    ) public view returns (uint256[] memory, uint256) {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        return computeAllocationDeltasGivenDeltaT(
            deltaT, indexT, reserves, totalLiquidity
        );
    }

    function getDeallocationDeltasGivenDeltaT(
        uint256 poolId,
        uint256 indexT,
        uint256 deltaT
    ) public view returns (uint256[] memory, uint256) {
        (uint256[] memory reserves, uint256 totalLiquidity) =
            getReservesAndLiquidity(poolId);
        return computeDeallocationDeltasGivenDeltaT(
            deltaT, indexT, reserves, totalLiquidity
        );
    }

    function getNextLiquidity(uint256 poolId) public view returns (uint256) {
        (uint256[] memory reserves,) = getReservesAndLiquidity(poolId);
        return computeNextLiquidity(reserves, getPoolParams(poolId));
    }
}

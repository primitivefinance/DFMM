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
    computeSwapDeltaLiquidity
} from "./G3MMath.sol";
import { ISolver } from "src/interfaces/ISolver.sol";
import {
    encodeAllocationDeltasGivenDeltaX,
    encodeAllocationDeltasGivenDeltaY
} from "src/lib/StrategyLib.sol";

error InvalidTokenIndex();

contract GeometricMeanSolver is ISolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    IStrategy public strategy;

    constructor(address strategy_) {
        strategy = IStrategy(strategy_);
    }

    function prepareInit(
        bytes calldata params,
        bytes calldata poolParams
    ) external pure returns (bytes memory) {
        (uint256 reserveX, uint256 S) = abi.decode(params, (uint256, uint256));

        return computeInitialPoolData(
            reserveX, S, abi.decode(poolParams, (GeometricMeanParams))
        );
    }

    function prepareAllocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 amount
    ) external view returns (bytes memory) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);

        if (tokenIndex == 0) {
            return encodeAllocationDeltasGivenDeltaX(
                amount, reserves[0], reserves[1], liquidity
            );
        } else if (tokenIndex == 1) {
            return encodeAllocationDeltasGivenDeltaY(
                amount, reserves[0], reserves[1], liquidity
            );
        } else {
            revert InvalidTokenIndex();
        }
    }

    function prepareDeallocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 amount
    ) external view returns (bytes memory) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);

        if (tokenIndex == 0) {
            return encodeAllocationDeltasGivenDeltaX(
                amount, reserves[0], reserves[1], liquidity
            );
        } else if (tokenIndex == 1) {
            return encodeAllocationDeltasGivenDeltaY(
                amount, reserves[0], reserves[1], liquidity
            );
        } else {
            revert InvalidTokenIndex();
        }
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

    /// @dev Computes the internal price using this strategie's slot parameters.
    function getPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) public view returns (uint256 price) {
        GeometricMeanParams memory params = getPoolParams(poolId);
        (uint256[] memory reserves,) = getReservesAndLiquidity(poolId);
        price = computePrice(
            reserves[tokenInIndex], reserves[tokenOutIndex], params
        );
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        override
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
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

    // TODO: Let's see if we can remove the following functions

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        GeometricMeanParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
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

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { Pool, IStrategy } from "src/interfaces/IStrategy.sol";
import { IDFMM } from "src/interfaces/IDFMM.sol";
import { ConstantSumParams } from "./ConstantSum.sol";
import {
    encodePriceUpdate,
    encodeFeeUpdate,
    encodeControllerUpdate
} from "./ConstantSumUtils.sol";
import {
    computeAllocationGivenX,
    computeAllocationGivenY
} from "src/lib/StrategyLib.sol";
import {
    ONE,
    computeInitialPoolData,
    FixedPointMathLib,
    computeSwapDeltaLiquidity
} from "./ConstantSumMath.sol";

contract ConstantSumSolver {
    error NotEnoughLiquidity();

    using FixedPointMathLib for uint256;

    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    address public strategy;

    constructor(address strategy_) {
        strategy = strategy_;
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 ry,
        ConstantSumParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, ry, params);
    }

    struct SimulateSwapState {
        uint256 amountOut;
        uint256 deltaLiquidity;
    }

    /// @notice used by kit
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (ConstantSumParams)
        );

        SimulateSwapState memory state;

        if (swapXIn) {
            computeSwapDeltaLiquidity(amountIn, poolParams, true);
            state.amountOut = amountIn.mulWadDown(poolParams.price).mulWadDown(
                ONE - poolParams.swapFee
            );

            if (pool.reserves[1] < state.amountOut) {
                revert NotEnoughLiquidity();
            }
        } else {
            computeSwapDeltaLiquidity(amountIn, poolParams, false);
            state.amountOut = (ONE - poolParams.swapFee).mulWadDown(amountIn)
                .divWadDown(poolParams.price);

            if (pool.reserves[0] < state.amountOut) {
                revert NotEnoughLiquidity();
            }
        }

        bytes memory swapData;

        if (swapXIn) {
            swapData = abi.encode(0, 1, amountIn, state.amountOut);
        } else {
            swapData = abi.encode(1, 0, amountIn, state.amountOut);
        }

        (bool valid,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (valid, state.amountOut, swapData);
    }

    /// @notice used by kit
    function preparePriceUpdate(uint256 newPrice)
        public
        pure
        returns (bytes memory)
    {
        return encodePriceUpdate(newPrice);
    }

    /// @notice used by kit
    function prepareSwapFeeUpdate(uint256 newSwapFee)
        public
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(newSwapFee);
    }

    /// @notice used by kit
    function prepareControllerUpdate(address newController)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(newController);
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (ConstantSumParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (ConstantSumParams)
        );
    }

    function getPrice(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256 price) {
        ConstantSumParams memory poolParams = getPoolParams(poolId);
        return poolParams.price;
    }

    function prepareAllocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (bytes memory) {
        (uint256[] memory reserves, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 adjustedReserveX, uint256 adjustedLiquidity) =
            computeAllocationGivenX(true, deltaX, reserves[0], liquidity);
        uint256 approximatedPrice =
            getPrice(poolId, adjustedReserveX, adjustedLiquidity);
        // todo
        uint256 adjustedReserveY = getNextReserveY(
            poolId, adjustedReserveX, adjustedLiquidity, approximatedPrice
        );
        uint256 deltaY = adjustedReserveY - reserves[1];
        uint256 deltaLiquidity = adjustedLiquidity - liquidity;
        return abi.encode(deltaY, deltaLiquidity);
    }
}

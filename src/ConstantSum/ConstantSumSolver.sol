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

    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (ConstantSumParams)
        );

        uint256 amountOut;

        if (swapXIn) {
            amountOut = amountIn.mulWadDown(poolParams.price).mulWadDown(
                ONE - poolParams.swapFee
            );

            if (pool.reserves[1] < amountOut) {
                revert NotEnoughLiquidity();
            }
        } else {
            amountOut = (ONE - poolParams.swapFee).mulWadDown(amountIn)
                .divWadDown(poolParams.price);

            if (pool.reserves[0] < amountOut) {
                revert NotEnoughLiquidity();
            }
        }

        bytes memory swapData;

        if (swapXIn) {
            swapData = abi.encode(0, 1, amountIn, amountOut);
        } else {
            swapData = abi.encode(1, 0, amountIn, amountOut);
        }

        (bool valid,,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (valid, amountOut, swapData);
    }

    function preparePriceUpdate(uint256 newPrice)
        public
        pure
        returns (bytes memory)
    {
        return encodePriceUpdate(newPrice);
    }

    function prepareSwapFeeUpdate(uint256 newSwapFee)
        public
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(newSwapFee);
    }

    function prepareControllerUpdate(address newController)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(newController);
    }
}

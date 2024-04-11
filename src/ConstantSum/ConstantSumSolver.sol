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
import { PairSolver } from "src/PairSolver.sol";

contract ConstantSumSolver is PairSolver {
    error NotEnoughLiquidity();

    using FixedPointMathLib for uint256;

    /// @dev Reserves struct to hold reserve amounts and liquidity
    struct Reserves {
        uint256 reserveX;
        /// @dev Reserve amount of token X
        uint256 reserveY;
        /// @dev Reserve amount of token Y
        uint256 liquidity;
    }
    /// @dev Total liquidity

    /// @dev Address of the strategy contract
    address public strategy;

    /// @notice Constructor to set the strategy address
    /// @param strategy_ Address of the strategy contract
    constructor(address strategy_) {
        strategy = strategy_;
    }

    /// @notice Computes the initial pool data for a Constant Sum pool
    /// @param reserveX The reserve amount of token X
    /// @param reserveY The reserve amount of token Y
    /// @param params The Constant Sum pool parameters
    /// @return The initial pool data encoded as bytes
    function getInitialPoolData(
        uint256 reserveX,
        uint256 reserveY,
        ConstantSumParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, reserveY, params);
    }

    /// @notice Simulates a swap in a Constant Sum pool
    /// @dev Used by the kit to simulate a swap and check if it's valid
    /// @param poolId The id of the pool to simulate the swap in
    /// @param swapXIn Whether the swap is X in for Y out (true) or Y in for X out (false)
    /// @param amountIn The amount of tokens to swap in
    /// @return valid Whether the simulated swap is valid
    /// @return amountOut The amount of tokens that would be received in the swap
    /// @return swapData The encoded swap data that can be used to perform the actual swap
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

        (bool valid,,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (valid, amountOut, swapData);
    }

    /// @notice Prepares the data for updating the price
    /// @dev Used by the kit to update the price
    /// @param newPrice The new price to set
    /// @return The encoded data for updating the price
    function preparePriceUpdate(uint256 newPrice)
        public
        pure
        returns (bytes memory)
    {
        return encodePriceUpdate(newPrice);
    }

    /// @notice Prepares the data for updating the swap fee
    /// @dev Used by the kit to update the swap fee
    /// @param newSwapFee The new swap fee to set
    /// @return The encoded data for updating the swap fee
    function prepareSwapFeeUpdate(uint256 newSwapFee)
        public
        pure
        returns (bytes memory)
    {
        return encodeFeeUpdate(newSwapFee);
    }

    /// @notice Prepares the data for updating the controller address
    /// @dev Used by the kit to update the controller
    /// @param newController The address of the new controller
    /// @return The encoded data for updating the controller
    function prepareControllerUpdate(address newController)
        public
        pure
        returns (bytes memory)
    {
        return encodeControllerUpdate(newController);
    }

    /// @notice Gets the reserves and liquidity for a given pool
    /// @param poolId The id of the pool
    /// @return The reserve of token X, the reserve of token Y, and the total liquidity
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        override
        returns (uint256, uint256, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves[0], pool.reserves[1], pool.totalLiquidity);
    }

    /// @dev gets the pool params
    /// @param poolId The pool id
    /// @return params The pool params
    function getPoolParams(uint256 poolId)
        public
        view
        returns (ConstantSumParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (ConstantSumParams)
        );
    }

    /// @dev Computes the change in allocation given a change in X reserve
    /// @param poolId The pool id
    /// @param deltaX The change in X reserve
    /// @return encodedAllocationDelta The encoded change in allocation
    function prepareAllocationDeltaGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (bytes memory) {
        ConstantSumParams memory params = getPoolParams(poolId);
        uint256 deltaL = deltaX.mulWadDown(params.price);
        return abi.encode(deltaX, 0, deltaL);
    }

    /// @dev Computes the change in allocation given a change in Y reserve
    /// @param deltaY The change in Y reserve
    /// @return encodedAllocationDelta The encoded change in allocation
    function prepareAllocationDeltaGivenDeltaY(uint256 deltaY)
        public
        pure
        returns (bytes memory)
    {
        return abi.encode(0, deltaY, deltaY);
    }
}

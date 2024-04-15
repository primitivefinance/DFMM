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
import { ISolver, InvalidTokenIndex } from "src/interfaces/ISolver.sol";

contract ConstantSumSolver is ISolver {
    error NotEnoughLiquidity();

    using FixedPointMathLib for uint256;

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

    function prepareInit(
        bytes calldata params,
        bytes calldata poolParams
    ) public pure returns (bytes memory) {
        (uint256 reserveX, uint256 reserveY) =
            abi.decode(params, (uint256, uint256));
        return getInitialPoolData(
            reserveX, reserveY, abi.decode(poolParams, (ConstantSumParams))
        );
    }

    function prepareAllocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 delta
    ) external view returns (bytes memory) {
        ConstantSumParams memory params = getPoolParams(poolId);

        if (tokenIndex == 0) {
            uint256 deltaL = delta.mulWadDown(params.price);
            return abi.encode(delta, 0, deltaL);
        } else if (tokenIndex == 1) {
            return abi.encode(0, delta, delta);
        } else {
            revert InvalidTokenIndex();
        }
    }

    function prepareDeallocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 delta
    ) external view returns (bytes memory) { }

    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (ConstantSumParams)
        );

        uint256 amountOut;

        if (tokenInIndex == 0) {
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

        if (tokenInIndex == 0) {
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
        returns (uint256[] memory, uint256)
    {
        Pool memory pool = IDFMM(IStrategy(strategy).dfmm()).pools(poolId);
        return (pool.reserves, pool.totalLiquidity);
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

    function getPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) external view override returns (uint256) {
        // TODO: Does it make sense to return a price here since it's a parameter.
    }
}

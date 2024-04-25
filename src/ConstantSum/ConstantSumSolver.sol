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
    computeSwapDeltaLiquidity,
    computeDeltaLiquidityRoundDown
} from "./ConstantSumMath.sol";
import { ISolver, InvalidTokenIndex } from "src/interfaces/ISolver.sol";

error NotEnoughLiquidity();
error InvalidDeltasLength();

contract ConstantSumSolver is ISolver {
    using FixedPointMathLib for uint256;

    /// @inheritdoc ISolver
    IStrategy public strategy;

    /// @param strategy_ Address of the ConstantSum strategy contract.
    constructor(IStrategy strategy_) {
        strategy = strategy_;
    }

    /**
     * @notice Prepares the data to initialize a new Constant Sum pool.
     * @param reserveX Initial reserve of token X.
     * @param reserveY Initial reserve of token Y.
     * @param poolParams Parameters as defined by the ConstantSum strategy.
     */
    function prepareInit(
        uint256 reserveX,
        uint256 reserveY,
        ConstantSumParams calldata poolParams
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(reserveX, reserveY, poolParams);
    }

    /// @inheritdoc ISolver
    function prepareAllocation(
        uint256 poolId,
        uint256[] memory deltas
    ) external view returns (bytes memory) {
        if (deltas.length != 2) revert InvalidDeltasLength();

        ConstantSumParams memory params = getPoolParams(poolId);

        uint256 deltaLGivenDeltaX =
            computeDeltaLiquidityRoundDown(deltas[0], 0, params.price);

        uint256 deltaLGivenDeltaY =
            computeDeltaLiquidityRoundDown(0, deltas[1], params.price);

        if (deltaLGivenDeltaX < deltaLGivenDeltaY) {
            return abi.encode(deltas[0], 0, deltaLGivenDeltaX);
        } else {
            return abi.encode(0, deltas[1], deltaLGivenDeltaY);
        }
    }

    /// @inheritdoc ISolver
    function prepareDeallocation(
        uint256 poolId,
        uint256 deltaLiquidity
    ) external view returns (bytes memory) {
        // TODO: Implement this.
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

        Pool memory pool = IDFMM(strategy.dfmm()).pools(poolId);
        ConstantSumParams memory poolParams =
            abi.decode(strategy.getPoolParams(poolId), (ConstantSumParams));

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

        (bool valid,,,,,,) =
            strategy.validateSwap(address(this), poolId, pool, swapData);
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

    /// @dev gets the pool params
    /// @param poolId The pool id
    /// @return params The pool params
    function getPoolParams(uint256 poolId)
        public
        view
        returns (ConstantSumParams memory)
    {
        return abi.decode(strategy.getPoolParams(poolId), (ConstantSumParams));
    }

    /// @inheritdoc ISolver
    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) external view override returns (uint256) {
        if (
            tokenInIndex > 1 || tokenOutIndex > 1
                || tokenInIndex == tokenOutIndex
        ) {
            revert InvalidTokenIndex();
        }

        if (tokenInIndex == 0) {
            return getPoolParams(poolId).price;
        } else {
            return ONE.divWadDown(getPoolParams(poolId).price);
        }
    }
}

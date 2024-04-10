// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import {
    FixedPointMathLib,
    computeTradingFunction,
    computeSwapDeltaLiquidity,
    computeDeltaLiquidity
} from "./ConstantSumMath.sol";
import {
    decodePriceUpdate,
    decodeFeeUpdate,
    decodeControllerUpdate
} from "./ConstantSumUtils.sol";
import { Strategy, IStrategy, Pool } from "src/Strategy.sol";
import { EPSILON } from "src/lib/StrategyLib.sol";

struct InternalParams {
    uint256 price;
    uint256 swapFee;
    address controller;
}

struct ConstantSumParams {
    uint256 price;
    uint256 swapFee;
    address controller;
}

enum UpdateCode {
    Invalid,
    SwapFee,
    Price,
    Controller
}

contract ConstantSum is Strategy {
    using FixedPointMathLib for uint256;

    /// @notice Thrown when the expected liquidity is not met.
    error InvalidDeltaLiquidity();

    /// @inheritdoc IStrategy
    string public constant name = "ConstantSum";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) Strategy(dfmm_) { }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;

        (reserves, params) = abi.decode(data, (uint256[], ConstantSumParams));
        totalLiquidity =
            computeDeltaLiquidity(reserves[0], reserves[1], params.price);

        if (pool.reserves.length != 2 || reserves.length != 2) {
            revert InvalidReservesLength();
        }

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;
        internalParams[poolId].controller = params.controller;

        // Get the trading function and check this is valid
        invariant =
            tradingFunction(reserves, totalLiquidity, abi.encode(params));

        valid = invariant >= 0 && invariant <= EPSILON;

        return (valid, invariant, reserves, totalLiquidity);
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        Pool calldata,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        UpdateCode updateCode = abi.decode(data, (UpdateCode));

        if (updateCode == UpdateCode.Price) {
            (internalParams[poolId].price) = decodePriceUpdate(data);
        } else if (updateCode == UpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == UpdateCode.Controller) {
            internalParams[poolId].controller = decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId)
        public
        view
        override
        returns (bytes memory)
    {
        ConstantSumParams memory params;

        params.price = internalParams[poolId].price;
        params.swapFee = internalParams[poolId].swapFee;
        params.controller = internalParams[poolId].controller;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return computeTradingFunction(
            reserves,
            totalLiquidity,
            abi.decode(params, (ConstantSumParams)).price
        );
    }

    /// @inheritdoc Strategy
    function _computeSwapDeltaLiquidity(
        Pool memory,
        bytes memory params,
        uint256 tokenInIndex,
        uint256,
        uint256 amountIn,
        uint256
    ) internal pure override returns (uint256) {
        return computeSwapDeltaLiquidity(
            amountIn, abi.decode(params, (ConstantSumParams)), tokenInIndex == 0
        );
    }
}

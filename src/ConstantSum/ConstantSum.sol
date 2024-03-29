// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import {
    FixedPointMathLib, computeTradingFunction
} from "./ConstantSumMath.sol";
import {
    decodePriceUpdate,
    decodeFeeUpdate,
    decodeControllerUpdate
} from "./ConstantSumUtils.sol";
import { PairStrategy, IStrategy, Pool } from "src/PairStrategy.sol";

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

contract ConstantSum is PairStrategy {
    using FixedPointMathLib for uint256;

    /// @inheritdoc IStrategy
    string public constant name = "ConstantSum";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) PairStrategy(dfmm_) { }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        Pool calldata,
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
        (reserves, totalLiquidity, params) =
            abi.decode(data, (uint256[], uint256, ConstantSumParams));

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;

        // Get the trading function and check this is valid
        invariant =
            tradingFunction(reserves, totalLiquidity, abi.encode(params));

        valid = invariant >= 0;

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

    /// @inheritdoc PairStrategy
    function _computeAllocateDeltasGivenDeltaL(
        uint256,
        Pool memory,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        return new uint256[](0);
    }

    /// @inheritdoc PairStrategy
    function _computeDeallocateDeltasGivenDeltaL(
        uint256,
        Pool memory,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        return new uint256[](0);
    }
}

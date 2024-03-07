// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./ConstantSumLib.sol";
import "src/interfaces/IDFMM.sol";
import { Strategy, IStrategy } from "src/Strategy.sol";

contract ConstantSum is Strategy {
    using FixedPointMathLib for uint256;

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

    /// @inheritdoc IStrategy
    string public constant name = "ConstantSum";

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address dfmm_) Strategy(dfmm_) { }

    function init(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;
        (reserveX, reserveY, totalLiquidity, params) =
            abi.decode(data, (uint256, uint256, uint256, ConstantSumParams));

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;

        // Get the trading function and check this is valid
        invariant = ConstantSumLib.tradingFunction(
            reserveX, reserveY, totalLiquidity, params.price
        );

        valid = -EPSILON < invariant && invariant < EPSILON;

        return (valid, invariant, reserveX, reserveY, totalLiquidity);
    }

    function update(
        address sender,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        ConstantSumLib.ConstantSumUpdateCode updateCode =
            abi.decode(data, (ConstantSumLib.ConstantSumUpdateCode));

        if (updateCode == ConstantSumLib.ConstantSumUpdateCode.Price) {
            internalParams[poolId].price =
                ConstantSumLib.decodePriceUpdate(data);
        } else if (updateCode == ConstantSumLib.ConstantSumUpdateCode.SwapFee) {
            internalParams[poolId].swapFee =
                ConstantSumLib.decodeFeeUpdate(data);
        } else if (
            updateCode == ConstantSumLib.ConstantSumUpdateCode.Controller
        ) {
            internalParams[poolId].controller =
                ConstantSumLib.decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

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

    function tradingFunction(
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        return ConstantSumLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(params, (ConstantSumParams)).price
        );
    }

    function _computeDeltaXGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory data
    ) internal view virtual override returns (uint256) {
        // TODO: Implement this.
        return deltaLiquidity;
    }

    function _computeDeltaYGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory data
    ) internal view virtual override returns (uint256) {
        // TODO: Implement this.
        return deltaLiquidity;
    }
}

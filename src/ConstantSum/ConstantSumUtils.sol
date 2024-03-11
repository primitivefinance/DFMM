/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {
    ConstantSumParams,
    UpdateCode
} from "src/ConstantSum/ConstantSum.sol";
import {
    computeY,
    computeL,
    computeTradingFunction,
    computeNextLiquidity
} from "./ConstantSumMath.sol";

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function encodePriceUpdate(
    uint256 targetPrice,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.Price, targetPrice, targetTimestamp);
}

function encodeControllerUpdate(address controller)
    pure
    returns (bytes memory data)
{
    return abi.encode(UpdateCode.Controller, controller);
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (UpdateCode, uint256));
    return swapFee;
}

function decodePriceUpdate(bytes memory data)
    pure
    returns (uint256 targetPrice, uint256 targetTimestamp)
{
    (, targetPrice, targetTimestamp) =
        abi.decode(data, (UpdateCode, uint256, uint256));
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (address controller)
{
    (, controller) = abi.decode(data, (UpdateCode, address));
}

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    GeometricMeanParams memory params
) pure returns (bytes memory) {
    uint256 rY = computeY(amountX, initialPrice, params);
    uint256 L = computeL(amountX, rY, params);

    int256 invariant =
        computeTradingFunction({ rX: amountX, rY: rY, L: L, params: params });

    L = computeNextLiquidity(amountX, rY, params);

    return
        abi.encode(amountX, rY, L, params.wX, params.swapFee, params.controller);
}

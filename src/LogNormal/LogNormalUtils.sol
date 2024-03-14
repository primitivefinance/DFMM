// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { LogNormalParams, UpdateCode } from "src/LogNormal/LogNormal.sol";
import {
    computeLGivenX,
    computeYGivenL,
    computeTradingFunction,
    computeNextLiquidity
} from "./LogNormalMath.sol";

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function encodeWidthUpdate(
    uint256 targetWidth,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.Width, targetWidth, targetTimestamp);
}

function encodeMeanUpdate(
    uint256 targetMean,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.Width, targetMean, targetTimestamp);
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

function decodeWidthUpdate(bytes memory data)
    pure
    returns (uint256 targetWidth, uint256 targetTimestamp)
{
    (, targetWidth, targetTimestamp) =
        abi.decode(data, (UpdateCode, uint256, uint256));
}

function decodeMeanUpdate(bytes memory data)
    pure
    returns (uint256 targetMean, uint256 targetTimestamp)
{
    (, targetMean, targetTimestamp) =
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
    LogNormalParams memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 ry = computeYGivenL(L, initialPrice, params);
    int256 invariant = computeTradingFunction(amountX, ry, L, params);
    L = computeNextLiquidity(amountX, ry, invariant, L, params);
    uint256[] memory reserves = new uint256[](2);
    reserves[0] = amountX;
    reserves[1] = ry;
    return abi.encode(reserves, L, params);
}
/// @dev Casts a positived signed integer to an unsigned integer, reverting if `x` is negative.

function toUint(int256 x) pure returns (uint256) {
    unchecked {
        require(x >= 0, "toUint: negative");
        return uint256(x);
    }
}

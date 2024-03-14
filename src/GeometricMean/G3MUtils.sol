// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { UpdateCode } from "src/GeometricMean/GeometricMean.sol";

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function encodeWeightXUpdate(
    uint256 targetWeightX,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.WeightX, targetWeightX, targetTimestamp);
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

function decodeWeightXUpdate(bytes memory data)
    pure
    returns (uint256 targetWeightX, uint256 targetTimestamp)
{
    (, targetWeightX, targetTimestamp) =
        abi.decode(data, (UpdateCode, uint256, uint256));
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (address controller)
{
    (, controller) = abi.decode(data, (UpdateCode, address));
}

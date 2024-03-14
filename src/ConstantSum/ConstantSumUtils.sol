/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { UpdateCode } from "src/ConstantSum/ConstantSum.sol";

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function encodePriceUpdate(uint256 price) pure returns (bytes memory) {
    return abi.encode(UpdateCode.Price, price);
}

function encodeControllerUpdate(address controller)
    pure
    returns (bytes memory)
{
    return abi.encode(UpdateCode.Controller, controller);
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256 swapFee) {
    (, swapFee) = abi.decode(data, (UpdateCode, uint256));
    return swapFee;
}

function decodePriceUpdate(bytes memory data)
    pure
    returns (uint256 targetPrice)
{
    (, targetPrice) = abi.decode(data, (UpdateCode, uint256));
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (address controller)
{
    (, controller) = abi.decode(data, (UpdateCode, address));
}

/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {
    NTokenGeometricMeanParams,
    UpdateCode
} from "src/NTokenGeometricMean/NTokenGeometricMean.sol";
import {
    computeReserveFromNumeraire,
    computeL,
    computeTradingFunction,
    computeNextLiquidity
} from "./NTokenGeometricMeanMath.sol";

function encodeFeeUpdate(uint256 swapFee) pure returns (bytes memory) {
    return abi.encode(UpdateCode.SwapFee, uint256(swapFee));
}

function decodeFeeUpdate(bytes memory data) pure returns (uint256) {
    (, uint256 swapFee) = abi.decode(data, (UpdateCode, uint256));
    return swapFee;
}

function encodeWeightsUpdate(
    uint256[] calldata targetWeights,
    uint256 targetTimestamp
) pure returns (bytes memory data) {
    return abi.encode(UpdateCode.Weights, targetWeights, targetTimestamp);
}

function decodeWeightsUpdate(bytes memory data)
    pure
    returns (uint256[] memory targetWeights, uint256 targetTimestamp)
{
    (, targetWeights, targetTimestamp) =
        abi.decode(data, (UpdateCode, uint256[], uint256));
}

function encodeControllerUpdate(address controller)
    pure
    returns (bytes memory data)
{
    return abi.encode(UpdateCode.Controller, controller);
}

function decodeControllerUpdate(bytes memory data)
    pure
    returns (address controller)
{
    (, controller) = abi.decode(data, (UpdateCode, address));
}

function computeInitialPoolData(
    uint256 numeraireAmount,
    uint256[] memory prices,
    NTokenGeometricMeanParams memory params
) pure returns (bytes memory) {
    uint256[] memory reserves = new uint256[](prices.length);
    uint256 numerairePrice = prices[prices.length - 1];
    uint256 numeraireWeight = params.weights[params.weights.length - 1];
    for (uint256 i = 0; i < prices.length - 1; i++) {
        // compute the amount of reserves for token T given numeraireAmount and weights wT and wNumeraire
        uint256 amountT = computeReserveFromNumeraire(
            numeraireAmount, numerairePrice, params.weights[i], numeraireWeight
        );
        reserves[i] = amountT;
    }
    reserves[prices.length - 1] = numeraireAmount;

    uint256 L = computeNextLiquidity(reserves, params);

    return abi.encode(
        reserves, L, params.weights, params.swapFee, params.controller
    );
}

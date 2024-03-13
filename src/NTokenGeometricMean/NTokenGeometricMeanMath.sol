// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { NTokenGeometricMeanParams } from
    "src/NTokenGeometricMean/NTokenGeometricMean.sol";

uint256 constant ONE = 1 ether;

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeTradingFunction(
    uint256[] memory reserves,
    uint256 L,
    NTokenGeometricMeanParams memory params
) pure returns (int256) {
    uint256 accumulator = ONE;
    for (uint256 i = 0; i < reserves.length; i++) {
        uint256 a = uint256(
            int256(reserves[i].divWadDown(L)).powWad(int256(params.weights[i]))
        );
        accumulator.mulWadUp(a);
    }

    return int256(accumulator) - int256(ONE);
}

function computeDeltaGivenDeltaLRoundUp(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulWadUp(deltaLiquidity.divWadUp(totalLiquidity));
}

function computeDeltaGivenDeltaLRoundDown(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulWadDown(deltaLiquidity.divWadDown(totalLiquidity));
}

function computeL(
    uint256[] memory reserves,
    NTokenGeometricMeanParams memory params
) pure returns (uint256) {
    uint256 accumulator;

    for (uint256 i = 0; i < reserves.length; i++) {
        uint256 a =
            uint256(int256(reserves[i]).powWad(int256(params.weights[i])));
        if (accumulator != 0) {
            accumulator += a;
        } else {
            accumulator.mulWadUp(a);
        }
    }

    return accumulator;
}

function computeReserveFromNumeraire(
    uint256 amountNumeraire,
    uint256 S,
    uint256 wT,
    uint256 wNumeraire
) pure returns (uint256) {
    return wT.divWadDown(wNumeraire.mulWadDown(S)).mulWadDown(amountNumeraire);
}

function computeAllocationDeltasGivenDeltaT(
    uint256 deltaT,
    uint256 indexT,
    uint256[] memory reserves,
    uint256 totalLiquidity
) pure returns (uint256[] memory, uint256) {
    uint256 a = deltaT.divWadUp(reserves[indexT]);
    uint256[] memory reserveDeltas = new uint256[](reserves.length);
    reserveDeltas[indexT] = deltaT;
    for (uint256 i = 0; i < reserves.length; i++) {
        if (i != indexT) {
            reserveDeltas[i] = a.mulWadUp(reserves[i]);
        }
    }

    uint256 deltaL = a.mulWadUp(totalLiquidity);

    return (reserveDeltas, deltaL);
}

function computeDeallocationDeltasGivenDeltaT(
    uint256 deltaT,
    uint256 indexT,
    uint256[] memory reserves,
    uint256 totalLiquidity
) pure returns (uint256[] memory, uint256) {
    uint256 a = deltaT.divWadDown(reserves[indexT]);
    uint256[] memory reserveDeltas = new uint256[](reserves.length);
    reserveDeltas[indexT] = deltaT;
    for (uint256 i = 0; i < reserves.length; i++) {
        if (i != indexT) {
            reserveDeltas[i] = a.mulWadDown(reserves[i]);
        }
    }

    uint256 deltaL = a.mulWadDown(totalLiquidity);

    return (reserveDeltas, deltaL);
}

/// @dev Finds the root of the swapConstant given the independent variable liquidity.
function computeNextLiquidity(
    uint256[] memory reserves,
    NTokenGeometricMeanParams memory params
) pure returns (uint256 L) {
    uint256 accumulator;
    for (uint256 i = 0; i < reserves.length; i++) {
        uint256 a =
            uint256(int256(reserves[i]).powWad(int256(params.weights[i])));
        if (accumulator != 0) {
            accumulator.mulWadUp(a);
        } else {
            accumulator = a;
        }
    }
    return accumulator;
}

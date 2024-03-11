// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./NTokenGeometricMean.sol";
import "../lib/BisectionLib.sol";
import "../lib/SignedWadMath.sol";
import "solmate/tokens/ERC20.sol";
import "forge-std/console2.sol";
import "src/lib/StrategyLib.sol";
import "./NTokenGeometricMeanLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

function computeAllocateDeltasGivenDeltaT(
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

function computeReserveFromNumeraire(
    uint256 amountNumeraire,
    uint256 S,
    uint256 wT,
    uint256 wNumeraire
) pure returns (uint256) {
    return wT.divWadDown(wNumeraire.mulWadDown(S)).mulWadDown(amountNumeraire);
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

function computeInitialPoolDataFromPrices(
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
    reserves[prices.length - 1] = ONE;

    uint256 L = computeNextLiquidity(reserves, params);

    return abi.encode(
        reserves, L, params.weights, params.swapFee, params.controller
    );
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
            numeraireAmount,
            numerairePrice,
            params.weights[i],
            numeraireWeight
        );
        reserves[i] = amountT;
    }
    reserves[prices.length - 1] = ONE;

    uint256 L = computeNextLiquidity(reserves, params);

    return abi.encode(
        reserves, L, params.weights, params.swapFee, params.controller
    );
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

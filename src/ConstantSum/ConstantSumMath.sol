// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";
import { ONE } from "src/lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeTradingFunction(
    uint256[] memory reserves,
    uint256 totalLiquidity,
    uint256 price
) pure returns (int256) {
    return int256(reserves[0].divWadUp(totalLiquidity))
        + int256(reserves[1].divWadUp(totalLiquidity.mulWadUp(price))) - int256(ONE);
}

function computeInitialPoolData(
    uint256 rx,
    uint256 ry,
    ConstantSumParams memory params
) pure returns (bytes memory) {
    // The pool can be initialized with any non-negative amount of rx, and ry.
    // so we have to allow a user to pass an amount of both even if one is zero.
    uint256 L = rx + ry.divWadUp(params.price);
    uint256[] memory reserves = new uint256[](2);
    reserves[0] = rx;
    reserves[1] = ry;
    return abi.encode(reserves, L, params);
}

function computeDeallocateGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadDown(rX);
    if (rY > 0) {
        deltaY = a.mulWadDown(rY);
    }
    deltaL = a.mulWadDown(totalLiquidity);
}

function computeDeallocateGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadDown(rY);
    if (rX > 0) {
        deltaX = a.mulWadDown(rX);
    }
    deltaL = a.mulWadDown(totalLiquidity);
}

function computeAllocateGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadUp(rX);
    if (rY > 0) {
        deltaY = a.mulWadUp(rY);
    }
    deltaL = a.mulWadUp(totalLiquidity);
}

function computeAllocateGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadUp(rY);
    if (rX > 0) {
        deltaX = a.mulWadUp(rX);
    }
    deltaL = a.mulWadUp(totalLiquidity);
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

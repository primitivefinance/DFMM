// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";

/// @dev Taking the square root of a WAD value returns a value with units of 1E9.
/// Multiplying the result by SQRT_WAD will normalize it back to WAD units.
uint256 constant SQRT_WAD = 1e9;
uint256 constant TWO = 2e18;
uint256 constant HALF = 0.5e18;
uint256 constant ONE = 1e18;
uint256 constant INFINITY_IS_NOT_REAL = type(uint256).max;
uint256 constant ZERO = 0;
int256 constant I_ONE = int256(ONE);
int256 constant I_TWO = int256(TWO);
int256 constant I_HALF = int256(HALF);

/// @dev the swap constant should never fall outside of range [-EPSILON, EPSILON]
int256 constant EPSILON = 20;

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeAllocationGivenX(
    bool add,
    uint256 amountX,
    uint256 rx,
    uint256 L
) pure returns (uint256 nextRx, uint256 nextL) {
    uint256 liquidityPerRx = L.divWadUp(rx);
    uint256 deltaL = amountX.mulWadUp(liquidityPerRx);
    nextRx = add ? rx + amountX : rx - amountX;
    nextL = add ? L + deltaL : L - deltaL;
}

function computeAllocationGivenY(
    bool add,
    uint256 amountY,
    uint256 ry,
    uint256 L
) pure returns (uint256 nextRy, uint256 nextL) {
    uint256 liquidityPerRy = L.divWadUp(ry);
    uint256 deltaL = amountY.mulWadUp(liquidityPerRy);
    nextRy = add ? ry + amountY : ry - amountY;
    nextL = add ? L + deltaL : L - deltaL;
}

function computeDeltaLGivenDeltaX(
    uint256 deltaX,
    uint256 liquidity,
    uint256 reserveX
) pure returns (uint256 deltaL) {
    return liquidity.mulWadDown(deltaX.divWadDown(reserveX));
}

function computeDeltaLGivenDeltaY(
    uint256 deltaY,
    uint256 liquidity,
    uint256 reserveY
) pure returns (uint256 deltaL) {
    return liquidity.mulWadDown(deltaY.divWadDown(reserveY));
}

function computeDeltaYGivenDeltaX(
    uint256 deltaX,
    uint256 reserveX,
    uint256 reserveY
) pure returns (uint256 deltaY) {
    return reserveY.mulWadDown(deltaX.divWadUp(reserveX));
}

function computeDeltaXGivenDeltaL(
    uint256 deltaL,
    uint256 liquidity,
    uint256 reserveX
) pure returns (uint256 deltaX) {
    return reserveX.mulWadUp(deltaL.divWadUp(liquidity));
}

function computeDeltaYGivenDeltaL(
    uint256 deltaL,
    uint256 liquidity,
    uint256 reserveY
) pure returns (uint256 deltaX) {
    return reserveY.mulWadUp(deltaL.divWadUp(liquidity));
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { GeometricMeanParams } from "src/GeometricMean/GeometricMean2.sol";

uint256 constant ONE = 1 ether;

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeTradingFunction(
    uint256 rX,
    uint256 rY,
    uint256 L,
    GeometricMeanParams memory params
) pure returns (int256) {
    uint256 a = uint256(int256(rX.divWadDown(L)).powWad(int256(params.wX)));
    uint256 b = uint256(int256(rY.divWadDown(L)).powWad(int256(params.wY)));

    return int256(a.mulWadUp(b)) - int256(1 ether);
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

function computeLGivenX(
    uint256 x,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    int256 a = int256(params.wY.divWadUp(params.wX).mulWadUp(S));
    int256 b = a.powWad(int256(params.wY));
    return x.mulWadUp(uint256(b));
}

function computeLGivenY(
    uint256 y,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return y.mulWadUp(params.wX).divWadUp(params.wY.mulWadUp(S));
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wX.mulWadUp(L).divWadUp(params.wY.mulWadUp(S));
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wY.mulWadUp(L).divWadUp(params.wX.mulWadUp(S));
}

function computeY(
    uint256 x,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wY.divWadDown(params.wX).mulWadDown(S).mulWadDown(x);
}

function computeX(
    uint256 y,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wX.divWadDown(params.wY.mulWadDown(S)).mulWadDown(y);
}

function computeL(
    uint256 x,
    uint256 y,
    GeometricMeanParams memory params
) pure returns (uint256) {
    uint256 a = uint256(int256(x).powWad(int256(params.wX)));
    uint256 b = uint256(int256(y).powWad(int256(params.wY)));

    return a.mulWadUp(b);
}

/// @dev Finds the root of the swapConstant given the independent variable rX.
function computeNextRy(
    uint256 rX,
    uint256 liquidity,
    GeometricMeanParams memory params
) pure returns (uint256 rY) {
    rY = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rX).powWad(int256(params.wX))))
        ).powWad(int256(ONE.divWadUp(params.wY)))
    );
}

/// @dev Finds the root of the swapConstant given the independent variable rY.
function computeNextRx(
    uint256 rY,
    uint256 liquidity,
    GeometricMeanParams memory params
) pure returns (uint256 rX) {
    rX = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rY).powWad(int256(params.wY))))
        ).powWad(int256(ONE.divWadUp(params.wX)))
    );
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rX,
    uint256 rY,
    GeometricMeanParams memory params
) pure returns (uint256 price) {
    uint256 n = rY.divWadDown(params.wY);
    uint256 d = rX.divWadDown(params.wX);
    price = n.divWadDown(d);
}

/// @dev Finds the root of the swapConstant given the independent variable liquidity.
function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    GeometricMeanParams memory params
) pure returns (uint256 L) {
    return uint256(int256(rX).powWad(int256(params.wX))).mulWadUp(
        uint256(int256(rY).powWad(int256(params.wY)))
    );
}

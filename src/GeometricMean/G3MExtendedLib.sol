// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./GeometricMean.sol";
import "../lib/BisectionLib.sol";
import "../lib/SignedWadMath.sol";
import "solmate/tokens/ERC20.sol";
import "forge-std/console2.sol";

// import { wadMul, wadDiv } from "../../lib/SignedWadMath.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

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

function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    GeometricMeanParams memory params
) pure returns (bytes memory) {
    uint256 rY = computeY(amountX, initialPrice, params);
    uint256 L = computeL(amountX, rY, params);

    int256 invariant = GeometricMeanLib.tradingFunction({
        rX: amountX,
        rY: rY,
        L: L,
        params: params
    });

    L = computeNextLiquidity(amountX, rY, invariant, L, params);

    return
        abi.encode(amountX, rY, L, params.wX, params.swapFee, params.controller);
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
/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.

function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rX, uint256 rY,, GeometricMeanParams memory params) =
        abi.decode(data, (uint256, uint256, int256, GeometricMeanParams));
    return GeometricMeanLib.tradingFunction({
        rX: rX,
        rY: rY,
        L: L,
        params: params
    });
}

function findRootLower(bytes memory data, uint256 v) pure returns (int256) {
    (
        uint256 S,
        uint256 rX,
        uint256 rY,
        uint256 L,
        GeometricMeanParams memory params
    ) = abi.decode(
        data, (uint256, uint256, uint256, uint256, GeometricMeanParams)
    );
    return diffLower({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

function findRootRaise(bytes memory data, uint256 v) pure returns (int256) {
    (
        uint256 S,
        uint256 rX,
        uint256 rY,
        uint256 L,
        GeometricMeanParams memory params
    ) = abi.decode(
        data, (uint256, uint256, uint256, uint256, GeometricMeanParams)
    );
    return diffRaise({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

struct DiffLowerStruct {
    uint256 wX;
    uint256 rX;
    uint256 rY;
    uint256 L;
    uint256 v;
    uint256 yOverXPowWx;
    uint256 yOverXPowWy;
    uint256 gamma;
}

function computeDiffLowerNumerator(DiffLowerStruct memory params)
    pure
    returns (uint256)
{
    uint256 first = params.L.mulWadDown(params.wX).mulWadDown(params.rX)
        .mulWadDown(params.yOverXPowWx);
    uint256 second = (params.v - params.v.mulWadDown(params.wX) + params.rX)
        .mulWadDown(params.rY).mulWadDown(ONE - params.gamma);
    uint256 third =
        uint256(int256(params.v + params.rX).powWad(-int256(params.wX)));
    uint256 fourth = params.L
        + params.v.mulWadDown(params.yOverXPowWy).mulWadDown(ONE - params.gamma);
    return (first - second).mulWadDown(
        uint256(
            int256(third.mulWadDown(fourth)).powWad(
                int256(ONE.divWadDown(ONE - params.wX))
            )
        )
    );
}

function computeDiffLowerDenominator(DiffLowerStruct memory params)
    pure
    returns (uint256)
{
    uint256 dFirst = ONE - params.wX;
    uint256 dSecond = params.v + params.rX;
    uint256 dThird =
        params.L.mulWadDown(params.rX).mulWadDown(uint256(params.yOverXPowWx));
    uint256 dFourth =
        params.v.mulWadDown(params.rY).mulWadDown(ONE - params.gamma);
    return dFirst.mulWadDown(dSecond).mulWadDown(dThird + dFourth);
}

function computeDiffLowerResult(
    uint256 wX,
    uint256 wY,
    uint256 rX,
    uint256 rY,
    uint256 S,
    uint256 L,
    uint256 v,
    uint256 gamma
) pure returns (int256) {
    int256 yOverX = int256(rY.divWadDown(rX));

    DiffLowerStruct memory params = DiffLowerStruct({
        wX: wX,
        rX: rX,
        rY: rY,
        L: L,
        v: v,
        yOverXPowWx: uint256(yOverX.powWad(int256(wX))),
        yOverXPowWy: uint256(yOverX.powWad(int256(wY))),
        gamma: gamma
    });

    uint256 numerator = computeDiffLowerNumerator(params);
    uint256 denominator = computeDiffLowerDenominator(params);

    return -int256(S) + int256(numerator.divWadDown(denominator));
}

// todo(matt): refactor this to only use int256
function diffLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    GeometricMeanParams memory params
) pure returns (int256) {
    uint256 gamma = ONE - params.swapFee;
    return computeDiffLowerResult(params.wX, params.wY, rX, rY, S, L, v, gamma);
}

function computeDiffRaiseNumerator(DiffRaiseStruct memory params)
    pure
    returns (int256)
{
    int256 first =
        int256(params.wX).wadMul(int256(params.v) + int256(params.rY));
    int256 third = (params.vPlusYPow.wadMul(params.lMinusVTimesXOverYPowWx))
        .powWad(I_ONE.wadDiv(int256(params.wX)));
    int256 fourth = int256(params.L).wadMul(-I_ONE + int256(params.wX));
    int256 fifth = params.xOverYPowWx.wadMul(
        int256(params.v).wadMul(int256(params.wX)) + int256(params.rY)
    ).wadMul(-I_ONE + params.gamma);
    return first.wadMul(params.lMinusVTimesXOverYPowWx)
        + int256(params.S).wadMul(third).wadMul(fourth - fifth);
}

struct DiffRaiseStruct {
    uint256 wX;
    uint256 v;
    uint256 rY;
    int256 lMinusVTimesXOverYPowWx;
    int256 vPlusYPow;
    uint256 L;
    int256 xOverYPowWx;
    int256 gamma;
    uint256 S;
    int256 vTimesXOverYPowWx;
}

function getDiffRaiseStruct(
    uint256 wX,
    uint256 rX,
    uint256 v,
    uint256 rY,
    uint256 L,
    uint256 S,
    uint256 swapFee
) pure returns (DiffRaiseStruct memory) {
    int256 vPlusYPow = (int256(v) + int256(rY)).powWad(-I_ONE + int256(wX));
    int256 xOverYPowWx = (int256(rX).wadDiv(int256(rY))).powWad(int256(wX));
    int256 vTimesXOverYPowWx = int256(v).wadMul(xOverYPowWx);
    int256 gamma = I_ONE - int256(swapFee);
    int256 lMinusVTimesXOverYPowWx =
        int256(L) - vTimesXOverYPowWx.wadMul(-I_ONE + gamma);

    return DiffRaiseStruct({
        wX: wX,
        v: v,
        rY: rY,
        lMinusVTimesXOverYPowWx: lMinusVTimesXOverYPowWx,
        vPlusYPow: vPlusYPow,
        L: L,
        xOverYPowWx: xOverYPowWx,
        gamma: gamma,
        S: S,
        vTimesXOverYPowWx: vTimesXOverYPowWx
    });
}

function diffRaise(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    GeometricMeanParams memory params
) pure returns (int256) {
    DiffRaiseStruct memory diffRaiseParams =
        getDiffRaiseStruct(params.wX, rX, v, rY, L, S, params.swapFee);

    int256 numerator = computeDiffRaiseNumerator(diffRaiseParams);

    int256 denominator;
    {
        int256 first = int256(params.wX).wadMul(int256(v) + int256(rY));
        int256 second = -int256(L)
            + diffRaiseParams.vTimesXOverYPowWx.wadMul(
                -I_ONE + diffRaiseParams.gamma
            );
        denominator = first.wadMul(second);
    }

    return numerator.wadDiv(denominator);
}

function computeOptimalLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 vUpper,
    GeometricMeanParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = 1000;
    int256 lowerBoundOutput = diffLower(S, rX, rY, L, lower, params);
    if (lowerBoundOutput < 0) {
        return 0;
    }
    v = bisection(
        abi.encode(S, rX, rY, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootLower
    );
}

function computeOptimalRaise(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 vUpper,
    GeometricMeanParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = 1000;
    int256 lowerBoundOutput = diffRaise(S, rX, rY, L, lower, params);
    if (lowerBoundOutput < 0) {
        return 0;
    }
    v = bisection(
        abi.encode(S, rX, rY, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootRaise
    );
}

function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    int256 invariant,
    uint256 approximatedL,
    GeometricMeanParams memory params
) pure returns (uint256 L) {
    uint256 upper = approximatedL;
    uint256 lower = approximatedL;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant = GeometricMeanLib.tradingFunction({
                rX: rX,
                rY: rY,
                L: lower,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant = GeometricMeanLib.tradingFunction({
                rX: rX,
                rY: rY,
                L: upper,
                params: params
            });
        }
    }
    L = bisection(
        abi.encode(rX, rY, computedInvariant, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootLiquidity
    );
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "../interfaces/IDFMM.sol";
import "../lib/BisectionLib.sol";
import "../lib/SignedWadMath.sol";
import "./LogNormalLib.sol";
import "./LogNormal.sol";
import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

uint256 constant MAX_ITER = 64;

/// @dev Computes reserves L given rx, S.
/// @param rx The reserve of x.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return L The reserve L computed as L(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
function computeLGivenX(
    uint256 rx,
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 L) {
    int256 d1 = computeD1({ S: S, params: params });
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = LogNormalLib.toUint(cdf);

    L = rx.divWadUp(ONE - unsignedCdf);
}

/// @dev Computes reserves y given L(x, S).
/// @return ry The reserve y computed as y(x, s) = K * L_x(x, S) * cdf[d2(S, K, sigma, tau)]
function computeYGivenL(
    uint256 L,
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 ry) {
    int256 d2 = computeD2(S, params);
    int256 cdf = Gaussian.cdf(d2);
    uint256 unsignedCdf = LogNormalLib.toUint(cdf);

    // TODO: Double check this formula
    ry = L.mulWadUp(unsignedCdf);
}

/// @dev Computes reserves x given L(y, S).
/// @return rx The reserve x computed as x(y, s) = L_y(y, S) * (WAD - cdf[d1(S, K, sigma, tau)])
function computeXGivenL(
    uint256 L,
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 rx) {
    int256 d1 = computeD1(S, params);
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = LogNormalLib.toUint(cdf);
    rx = L.mulWadUp(ONE - unsignedCdf);
}

/**
 * @dev Computes the d1 parameter for the Black-Scholes formula.
 *
 * $$d_1(S;\mu,\sigma) = \frac{\ln\frac{S}{\mu}+\frac{1}{2}\sigma^2 }{\sigma}$$
 *
 * @param S The price of X in Y, in WAD units.
 * @param params LogNormParameters of the Log Normal distribution.
 */
function computeD1(
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (int256 d1) {
    int256 lnSDivK = computeLnSDivK(S, params.mean);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaSquared(params.width);
    d1 = (lnSDivK + int256(halfSigmaPowTwoTau)).wadDiv(int256(params.width));
}

/// @dev Computes the d2 parameter for the Black-Scholes formula.
/// $$d_2(S;\mu,\sigma) = \frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2 }{\sigma}$$
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (int256 d2) {
    int256 lnSDivK = computeLnSDivK(S, params.mean);
    uint256 halfSigmaPowTwo = computeHalfSigmaSquared(params.width);
    d2 = (lnSDivK - int256(halfSigmaPowTwo)).wadDiv(int256(params.width));
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, LogNormal.LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormal.LogNormalParams));
    return LogNormalLib.tradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, LogNormal.LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormal.LogNormalParams));
    return LogNormalLib.tradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry,, LogNormal.LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormal.LogNormalParams));
    return LogNormalLib.tradingFunction(rx, ry, L, params);
}

/// @dev Computes the trading function given an amountX and an initialPrice.
function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    LogNormal.LogNormalParams memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 ry = computeYGivenL(L, initialPrice, params);
    int256 invariant = LogNormalLib.tradingFunction(amountX, ry, L, params);
    L = computeNextLiquidity(amountX, ry, invariant, L, params);
    return abi.encode(amountX, ry, L, params);
}

function computeNextLiquidity(
    uint256 rx,
    uint256 ry,
    int256 invariant,
    uint256 approximatedL,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 L) {
    uint256 upper = approximatedL;
    uint256 lower = approximatedL;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(rx, ry, lower, params);
        }
    } else {
        while (computedInvariant > 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(rx, ry, upper, params);
        }
    }
    L = bisection(
        abi.encode(rx, ry, computedInvariant, params),
        lower,
        upper,
        uint256(EPSILON),
        MAX_ITER,
        findRootLiquidity
    );
}

function computeNextRx(
    uint256 ry,
    uint256 L,
    int256 invariant,
    uint256 approximatedRx,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 rx) {
    uint256 upper = approximatedRx;
    uint256 lower = approximatedRx;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(upper, ry, L, params);
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(lower, ry, L, params);
        }
    }
    rx = bisection(
        abi.encode(ry, L, computedInvariant, params),
        lower,
        upper,
        uint256(EPSILON),
        MAX_ITER,
        findRootX
    );
}

function computeNextRy(
    uint256 rx,
    uint256 L,
    int256 invariant,
    uint256 approximatedRy,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 ry) {
    uint256 upper = approximatedRy;
    uint256 lower = approximatedRy;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(rx, upper, L, params);
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant =
                LogNormalLib.tradingFunction(rx, lower, L, params);
        }
    }
    ry = bisection(
        abi.encode(rx, L, computedInvariant, params),
        lower,
        upper,
        uint256(EPSILON),
        MAX_ITER,
        findRootY
    );
}

function findRootLower(bytes memory data, uint256 v) pure returns (int256) {
    (uint256 S, uint256 rX, uint256 L, LogNormal.LogNormalParams memory params)
    = abi.decode(data, (uint256, uint256, uint256, LogNormal.LogNormalParams));
    return diffLower({
        S: int256(S),
        rX: int256(rX),
        L: int256(L),
        v: int256(v),
        params: params
    });
}

function findRootRaise(bytes memory data, uint256 v) pure returns (int256) {
    (uint256 S, uint256 rY, uint256 L, LogNormal.LogNormalParams memory params)
    = abi.decode(data, (uint256, uint256, uint256, LogNormal.LogNormalParams));
    return diffRaise({
        S: int256(S),
        rY: int256(rY),
        L: int256(L),
        v: int256(v),
        params: params
    });
}

struct DiffLowerStruct {
    int256 ierfcResult;
    int256 width;
    int256 mean;
    int256 gamma;
    int256 rX;
    int256 L;
    int256 v;
    int256 S;
}

function createDiffLowerStruct(
    int256 S,
    int256 rx,
    int256 L,
    int256 gamma,
    int256 v,
    LogNormal.LogNormalParams memory params
) pure returns (DiffLowerStruct memory) {
    int256 a = I_TWO.wadMul(v + rx);
    int256 b = L + v - v.wadMul(gamma);
    int256 ierfcRes = Gaussian.ierfc(a.wadDiv(b));

    DiffLowerStruct memory ints = DiffLowerStruct({
        ierfcResult: ierfcRes,
        width: int256(params.width),
        mean: int256(params.mean),
        gamma: gamma,
        rX: rx,
        L: L,
        v: v,
        S: S
    });

    return ints;
}

function computeLowerA(DiffLowerStruct memory params) pure returns (int256) {
    /*
    int256 firstExp =
        -(params.sigma.wadMul(params.sigma).wadMul(params.tau).wadDiv(I_TWO));
    int256 secondExp = params.sqrtTwo.wadMul(params.sigma).wadMul(
        params.sqrtTau
    ).wadMul(params.ierfcResult);

    int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
    int256 second =
        params.strike.wadMul(params.L + params.rX.wadMul(-I_ONE + params.gamma));

    int256 firstNum = first.wadMul(second);
    int256 firstDen = params.L + params.v - params.v.wadMul(params.gamma);
    return firstNum.wadDiv(firstDen);
    */
}

function computeLowerB(DiffLowerStruct memory params) pure returns (int256) {
    /*
    int256 a = I_HALF.wadMul(params.strike).wadMul(-I_ONE + params.gamma);
    int256 b = params.sigma.wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
    return a.wadMul(Gaussian.erfc(b - params.ierfcResult));
    */
}

function diffLower(
    int256 S,
    int256 rX,
    int256 L,
    int256 v,
    LogNormal.LogNormalParams memory params
) pure returns (int256) {
    int256 gamma = I_ONE - int256(params.swapFee);
    DiffLowerStruct memory ints =
        createDiffLowerStruct(S, rX, L, gamma, v, params);
    int256 a = computeLowerA(ints);
    int256 b = computeLowerB(ints);

    return -ints.S + a + b;
}

struct DiffRaiseStruct {
    int256 ierfcResult;
    int256 strike;
    int256 sigma;
    int256 tau;
    int256 gamma;
    int256 rY;
    int256 L;
    int256 v;
    int256 S;
    int256 sqrtTwo;
    int256 sqrtTau;
}

function createDiffRaiseStruct(
    int256 S,
    int256 ry,
    int256 L,
    int256 gamma,
    int256 v,
    LogNormal.LogNormalParams memory params
) pure returns (DiffRaiseStruct memory) {
    /*
    int256 a = I_TWO.wadMul(v + ry);
    int256 b = int256(params.strike).wadMul(L) + v - v.wadMul(gamma);
    int256 ierfcRes = Gaussian.ierfc(a.wadDiv(b));

    int256 sqrtTwo = int256(FixedPointMathLib.sqrt(TWO) * 1e9);
    int256 sqrtTau = int256(FixedPointMathLib.sqrt(params.tau) * 1e9);

    DiffRaiseStruct memory ints = DiffRaiseStruct({
        ierfcResult: ierfcRes,
        strike: int256(params.strike),
        sigma: int256(params.sigma),
        tau: int256(params.tau),
        gamma: gamma,
        rY: ry,
        L: L,
        S: S,
        v: v,
        sqrtTwo: sqrtTwo,
        sqrtTau: sqrtTau
    });

    return ints;
    */
}

function computeRaiseA(DiffRaiseStruct memory params) pure returns (int256) {
    int256 firstExp =
        -(params.sigma.wadMul(params.sigma).wadMul(params.tau).wadDiv(I_TWO));
    int256 secondExp = params.sqrtTwo.wadMul(params.sigma).wadMul(
        params.sqrtTau
    ).wadMul(params.ierfcResult);
    int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
    int256 second = params.S.wadMul(
        params.strike.wadMul(params.L) + params.rY.wadMul(-I_ONE + params.gamma)
    );

    int256 num = first.wadMul(second);
    int256 den = params.strike.wadMul(
        params.strike.wadMul(params.L) + params.v
            - params.v.wadMul(params.gamma)
    );
    return num.wadDiv(den);
}

function computeRaiseB(DiffRaiseStruct memory params) pure returns (int256) {
    int256 first = params.S.wadMul(-I_ONE + params.gamma);
    int256 erfcFirst =
        params.sigma.wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
    int256 num = first.wadMul(Gaussian.erfc(erfcFirst - params.ierfcResult));
    int256 den = I_TWO.wadMul(params.strike);
    return num.wadDiv(den);
}

function diffRaise(
    int256 S,
    int256 rY,
    int256 L,
    int256 v,
    LogNormal.LogNormalParams memory params
) pure returns (int256) {
    int256 gamma = I_ONE - int256(params.swapFee);
    DiffRaiseStruct memory ints =
        createDiffRaiseStruct(S, rY, L, gamma, v, params);
    int256 a = computeRaiseA(ints);
    int256 b = computeRaiseB(ints);

    return -I_ONE + a + b;
}

function computeOptimalLower(
    int256 S,
    int256 rX,
    int256 L,
    uint256 vUpper,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = 1000;
    int256 lowerBoundOutput = diffLower(S, rX, L, int256(lower), params);
    if (lowerBoundOutput < 0) {
        return 0;
    }
    v = bisection(
        abi.encode(S, rX, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootLower
    );
}

function computeOptimalRaise(
    int256 S,
    int256 rY,
    int256 L,
    uint256 vUpper,
    LogNormal.LogNormalParams memory params
) pure returns (uint256 v) {
    uint256 upper = vUpper;
    uint256 lower = 1000;
    int256 lowerBoundOutput = diffRaise(S, rY, L, int256(lower), params);
    if (lowerBoundOutput < 0) {
        return 0;
    }
    v = bisection(
        abi.encode(S, rY, L, params),
        lower,
        upper,
        uint256(1),
        256,
        findRootRaise
    );
}

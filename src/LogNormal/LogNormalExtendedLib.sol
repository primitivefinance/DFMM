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

    ry = params.strike.mulWadUp(L).mulWadUp(unsignedCdf);
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

/// @dev Computes the d1 parameter for the Black-Scholes formula.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d1 = (ln(S/K) + tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD1(
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (int256 d1) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaTauSquared(sigma, tau);

    d1 = (lnSDivK + int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @dev Computes the d2 parameter for the Black-Scholes formula.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 S,
    LogNormal.LogNormalParams memory params
) pure returns (int256 d2) {
    (uint256 K, uint256 sigma, uint256 tau) =
        (params.strike, params.sigma, params.tau);
    uint256 sigmaSqrtTau = computeSigmaSqrtTau(sigma, tau);
    int256 lnSDivK = computeLnSDivK(S, K);
    uint256 halfSigmaPowTwoTau = computeHalfSigmaTauSquared(sigma, tau);

    d2 = (lnSDivK - int256(halfSigmaPowTwoTau)) * 1e18 / int256(sigmaSqrtTau);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, LogNormal.LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormal.LogNormalParams));
    return
        LogNormalLib.tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, LogNormal.LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormal.LogNormalParams));
    return
        LogNormalLib.tradingFunction({ rx: rx, ry: ry, L: L, params: params });
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
    return
        LogNormalLib.tradingFunction({ rx: rx, ry: ry, L: L, params: params });
}

/// @dev Computes the trading function given an amountX and an initialPrice.
function computeInitialPoolData(
    uint256 amountX,
    uint256 initialPrice,
    LogNormal.LogNormalParams memory params
) pure returns (bytes memory) {
    uint256 L = computeLGivenX(amountX, initialPrice, params);
    uint256 ry = computeYGivenL(L, initialPrice, params);
    int256 invariant = LogNormalLib.tradingFunction({
        rx: amountX,
        ry: ry,
        L: L,
        params: params
    });
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
            computedInvariant = LogNormalLib.tradingFunction({
                rx: rx,
                ry: ry,
                L: lower,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant = LogNormalLib.tradingFunction({
                rx: rx,
                ry: ry,
                L: upper,
                params: params
            });
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
            computedInvariant = LogNormalLib.tradingFunction({
                rx: upper,
                ry: ry,
                L: L,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant = LogNormalLib.tradingFunction({
                rx: lower,
                ry: ry,
                L: L,
                params: params
            });
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
            computedInvariant = LogNormalLib.tradingFunction({
                rx: rx,
                ry: upper,
                L: L,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant = LogNormalLib.tradingFunction({
                rx: rx,
                ry: lower,
                L: L,
                params: params
            });
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
    (
        uint256 S,
        uint256 rX,
        uint256 rY,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) = abi.decode(
        data, (uint256, uint256, uint256, uint256, LogNormal.LogNormalParams)
    );
    return diffLower({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

function findRootRaise(bytes memory data, uint256 v) pure returns (int256) {
    (
        uint256 S,
        uint256 rX,
        uint256 rY,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) = abi.decode(
        data, (uint256, uint256, uint256, uint256, LogNormal.LogNormalParams)
    );
    return diffRaise({ S: S, rX: rX, rY: rY, L: L, v: v, params: params });
}

struct DiffLowerStruct {
    int256 ierfcNum;
    int256 ierfcDen;
    int256 ierfcRes;
    uint256 sigma;
    uint256 tau;
    int256 sqrtTau;
    uint256 rX;
    uint256 strike;
    uint256 v;
    int256 sqrtTwo;
    int256 gamma;
    uint256 L;
}

function computeDiffLowerFirstFrac(DiffLowerStruct memory params)
    pure
    returns (int256)
{
    int256 firstExp = -(
        int256(params.sigma).powWad(I_TWO).wadMul(int256(params.tau)).wadDiv(
            I_TWO
        )
    );
    int256 secondExp = params.sqrtTwo.wadMul(int256(params.sigma)).wadMul(
        params.sqrtTau
    ).wadMul(params.ierfcRes);

    int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
    int256 second = int256(params.strike).wadMul(int256(params.rX)).wadMul(
        int256(params.gamma)
    );

    int256 firstNum = first.wadMul(second);
    int256 firstDen = int256(params.v) + int256(params.rX)
        - int256(params.v).wadMul(int256(params.gamma));
    return firstNum.wadDiv(firstDen);
}

function computeDiffLowerSecondFrac(DiffLowerStruct memory params)
    pure
    returns (int256)
{
    int256 first = int256(params.strike).wadMul(int256(params.L)).wadMul(
        -I_ONE + params.gamma
    );
    int256 erfcFirst =
        int256(params.sigma).wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
    int256 erfcSecond = params.ierfcRes;
    int256 num = first.wadMul(Gaussian.erfc(erfcFirst - erfcSecond));
    int256 den = I_TWO.wadMul(int256(params.rX));
    return num.wadDiv(den);
}

function diffLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    LogNormal.LogNormalParams memory params
) pure returns (int256) {
    /*
    (int256 strike, int256 sigma, int256 tau, int256 swapFee) = (
        int256(params.strike),
        int256(params.sigma),
        int256(params.tau),
        int256(params.swapFee)
    );
    */
    int256 sqrtTwo = int256(FixedPointMathLib.sqrt(TWO) * 1e9);
    int256 sqrtTau = int256(FixedPointMathLib.sqrt(params.tau) * 1e9);
    /*
    int256 iS = int256(S);
    int256 iX = int256(rX);
    int256 iL = int256(L);
    int256 iV = int256(v);
    int256 gamma = I_ONE - swapFee;
    */

    int256 ierfcNum = I_TWO.wadMul(int256(rX)).wadMul(int256(v) + int256(rX));
    int256 ierfcDen = int256(L).wadMul(
        int256(v) + (int256(rX))
            - int256(v).wadMul(I_ONE - int256(params.swapFee))
    );
    int256 ierfcRes = Gaussian.ierfc(ierfcNum.wadDiv(ierfcDen));

    DiffLowerStruct memory parameters = DiffLowerStruct({
        ierfcNum: ierfcNum,
        ierfcDen: ierfcDen,
        ierfcRes: ierfcRes,
        sigma: params.sigma,
        tau: params.tau,
        sqrtTau: sqrtTau,
        rX: rX,
        strike: params.strike,
        L: L,
        v: v,
        sqrtTwo: sqrtTwo,
        gamma: I_ONE - int256(params.swapFee)
    });

    int256 firstFrac = computeDiffLowerFirstFrac(parameters);
    int256 secondFrac = computeDiffLowerSecondFrac(parameters);
    return computeDiffLowerResult(S, firstFrac, secondFrac);
}

function computeDiffLowerResult(
    uint256 S,
    int256 firstFrac,
    int256 secondFrac
) pure returns (int256) {
    return -int256(S) + firstFrac + secondFrac;
}

struct DiffRaiseStruct {
    int256 ierfcNum;
    uint256 sigma;
    uint256 tau;
    int256 gamma;
    int256 sqrtTwo;
    int256 sqrtTau;
    int256 ierfcRes;
    uint256 rY;
    uint256 S;
    uint256 v;
    uint256 strike;
    uint256 L;
}

function computeDiffRaiseFirstFrac(DiffRaiseStruct memory params)
    pure
    returns (int256)
{
    int256 firstExp = -(
        int256(params.sigma).powWad(I_TWO).wadMul(int256(params.tau)).wadDiv(
            I_TWO
        )
    );
    int256 secondExp = params.sqrtTwo.wadMul(int256(params.sigma)).wadMul(
        params.sqrtTau
    ).wadMul(params.ierfcRes);
    int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
    int256 second =
        int256(params.S).wadMul(int256(params.rY)).wadMul(params.gamma);
    int256 firstNum = first.wadMul(second);
    int256 firstDen = int256(params.strike).wadMul(
        int256(params.v) + int256(params.rY)
            - int256(params.v).wadMul(params.gamma)
    );
    return firstNum.wadDiv(firstDen);
}

function computeDiffRaiseSecondFrac(DiffRaiseStruct memory params)
    pure
    returns (int256)
{
    int256 first =
        int256(params.L).wadMul(int256(params.S)).wadMul(-I_ONE + params.gamma);
    int256 erfcFirst =
        int256(params.sigma).wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
    int256 erfcSecond = params.ierfcRes;
    int256 num = first.wadMul(Gaussian.erfc(erfcFirst - erfcSecond));
    int256 den = I_TWO.wadMul(int256(params.rY));
    return num.wadDiv(den);
}

function diffRaise(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 v,
    LogNormal.LogNormalParams memory params
) pure returns (int256) {
    int256 sqrtTwo = int256(FixedPointMathLib.sqrt(TWO) * 1e9);
    int256 sqrtTau = int256(FixedPointMathLib.sqrt(params.tau) * 1e9);
    int256 gamma = I_ONE - int256(params.swapFee);
    int256 ierfcNum = I_TWO.wadMul(int256(rY)).wadMul(int256(v) + int256(rY));
    int256 ierfcDen = -int256(params.strike).wadMul(int256(L)).wadMul(int256(v) + int256(rY))
        + int256(params.strike).wadMul(int256(L)).wadMul(int256(v)).wadMul(gamma);
    int256 ierfcRes = Gaussian.ierfc(-ierfcNum.wadDiv(ierfcDen));

    DiffRaiseStruct memory parameters = DiffRaiseStruct({
        ierfcNum: ierfcNum,
        sigma: params.sigma,
        tau: params.tau,
        gamma: gamma,
        sqrtTwo: sqrtTwo,
        sqrtTau: sqrtTau,
        ierfcRes: ierfcRes,
        S: S,
        rY: rY,
        v: v,
        strike: params.strike,
        L: L
    });

    int256 firstFrac = computeDiffRaiseFirstFrac(parameters);
    int256 secondFrac = computeDiffRaiseSecondFrac(parameters);

    return -I_ONE + firstFrac + secondFrac;
}

function computeOptimalLower(
    uint256 S,
    uint256 rX,
    uint256 rY,
    uint256 L,
    uint256 vUpper,
    LogNormal.LogNormalParams memory params
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
    LogNormal.LogNormalParams memory params
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

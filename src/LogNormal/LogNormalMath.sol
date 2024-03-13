// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { SignedWadMathLib } from "src/lib/SignedWadMath.sol";
import { ONE, TWO, HALF } from "src/lib/StrategyLib.sol";
import { LogNormalParams } from "src/LogNormal/LogNormal.sol";
import { Gaussian } from "solstat/Gaussian.sol";
import { toUint } from "src/LogNormal/LogNormalUtils.sol";
import { bisection } from "src/lib/BisectionLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

uint256 constant MAX_ITER = 128;

function computeTradingFunction(
    uint256 rX,
    uint256 rY,
    uint256 L,
    LogNormalParams memory params
) pure returns (int256) {
    int256 a = Gaussian.ppf(int256(rX.divWadDown(L)));
    int256 b = Gaussian.ppf(int256(rY.divWadDown(L.mulWadDown(params.mean))));
    return a + b + int256(params.width);
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

function computeLnSDivK(uint256 S, uint256 K) pure returns (int256 lnSDivK) {
    lnSDivK = int256(S.divWadUp(K)).lnWad();
}

/**
 * @dev Computes the half of the square of sigma.
 *
 * $$\frac{1}{2}\sigma^2$$
 *
 */
function computeHalfSigmaSquared(uint256 sigma) pure returns (uint256) {
    return HALF.mulWadDown(sigma.mulWadUp(sigma));
}

/// @dev Computes reserves L given rx, S.
/// @param rx The reserve of x.
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return L The reserve L computed as L(x, s) = K * L_x(x, S) * Gaussian.cdf[d2(S, K, sigma, tau)]
function computeLGivenX(
    uint256 rx,
    uint256 S,
    LogNormalParams memory params
) pure returns (uint256 L) {
    int256 d1 = computeD1({ S: S, params: params });
    uint256 cdf = toUint(Gaussian.cdf(d1));

    L = rx.divWadUp(ONE - cdf);
}

/// @dev Computes reserves y given L(x, S).
/// @return ry The reserve y computed as y(x, s) = K * L_x(x, S) * cdf[d2(S, K, sigma, tau)]
function computeYGivenL(
    uint256 L,
    uint256 S,
    LogNormalParams memory params
) pure returns (uint256 ry) {
    int256 d2 = computeD2({ S: S, params: params });
    uint256 cdf = toUint(Gaussian.cdf(d2));

    ry = params.mean.mulWadUp(L).mulWadUp(cdf);
}

/// @dev Computes reserves x given L(y, S).
/// @return rx The reserve x computed as x(y, s) = L_y(y, S) * (WAD - cdf[d1(S, K, sigma, tau)])
function computeXGivenL(
    uint256 L,
    uint256 S,
    LogNormalParams memory params
) pure returns (uint256 rx) {
    int256 d1 = computeD1(S, params);
    int256 cdf = Gaussian.cdf(d1);
    uint256 unsignedCdf = toUint(cdf);
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
    LogNormalParams memory params
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
    LogNormalParams memory params
) pure returns (int256 d2) {
    int256 lnSDivK = computeLnSDivK(S, params.mean);
    uint256 halfSigmaPowTwo = computeHalfSigmaSquared(params.width);
    d2 = (lnSDivK - int256(halfSigmaPowTwo)).wadDiv(int256(params.width));
}

/**
 * @dev Computes the price using the reserve of token X.
 *
 * $$P_X(x, L; \mu, \sigma) = \mu \exp (\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
 *
 */
function computePriceGivenX(
    uint256 rX,
    uint256 L,
    LogNormalParams memory params
) pure returns (uint256) {
    uint256 a = computeHalfSigmaSquared(params.width);
    // $$\Phi^{-1} (1 - \frac{x}{L})$$
    int256 b = Gaussian.ppf(int256(ONE - rX.divWadDown(L)));

    // $$\exp(\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
    int256 exp = (b.wadMul(int256(params.width)) - int256(a)).expWad();

    // $$\mu \exp (\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
    return params.mean.mulWadUp(uint256(exp));
}

function computePriceGivenY(
    uint256 rY,
    uint256 L,
    LogNormalParams memory params
) pure returns (uint256) {
    uint256 a = computeHalfSigmaSquared(params.width);

    // $$\Phi^{-1} (\frac{y}{\mu L})$$
    int256 b = Gaussian.ppf(int256(rY.divWadDown(params.mean.mulWadDown(L))));

    // $$\exp (\Phi^{-1} (\frac{y}{\mu L}) \sigma  + \frac{1}{2} \sigma^2  )$$
    int256 exp = (b.wadMul(int256(params.width)) + int256(a)).expWad();

    // $$\mu \exp (\Phi^{-1} (\frac{y}{\mu L}) \sigma  + \frac{1}{2} \sigma^2  )$$
    return params.mean.mulWadUp(uint256(exp));
}

function computeDeltaLXIn(
    uint256 amountIn,
    uint256 rx,
    uint256 ry,
    uint256 L,
    LogNormalParams memory params
) pure returns (uint256 deltaL) {
    uint256 fees = params.swapFee.mulWadUp(amountIn);
    uint256 px = computePriceGivenX(rx, L, params);
    deltaL = px.mulWadUp(L).mulWadUp(fees).divWadDown(px.mulWadDown(rx) + ry);
}

function computeDeltaLYIn(
    uint256 amountIn,
    uint256 rx,
    uint256 ry,
    uint256 L,
    LogNormalParams memory params
) pure returns (uint256 deltaL) {
    uint256 fees = params.swapFee.mulWadUp(amountIn);
    uint256 px = computePriceGivenX(rx, L, params);
    deltaL = L.mulWadUp(fees).divWadDown(px.mulWadDown(rx) + ry);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L,, LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormalParams));
    return computeTradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L,, LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormalParams));
    return computeTradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry,, LogNormalParams memory params) =
        abi.decode(data, (uint256, uint256, int256, LogNormalParams));
    return computeTradingFunction(rx, ry, L, params);
}

function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    int256 invariant,
    uint256 approximatedL,
    LogNormalParams memory params
) pure returns (uint256 L) {
    uint256 upper = approximatedL;
    uint256 lower = approximatedL;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            lower = lower.mulDivDown(999, 1000);
            uint256 min = rX > rY.divWadDown(params.mean)
                ? rX + 1000
                : rY.divWadDown(params.mean) + 1000;
            lower = lower < rX ? min : lower;
            computedInvariant = computeTradingFunction({
                rX: rX,
                rY: rY,
                L: lower,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant = computeTradingFunction({
                rX: rX,
                rY: rY,
                L: upper,
                params: params
            });
        }
    }
    (uint256 rootInput,, uint256 lowerInput) = bisection(
        abi.encode(rX, rY, computedInvariant, params),
        lower,
        upper,
        1,
        MAX_ITER,
        findRootLiquidity
    );

    if (
        computeTradingFunction({ rX: rX, rY: rY, L: rootInput, params: params })
            == 0
    ) {
        L = rootInput;
    } else {
        L = lowerInput;
    }
}

function computeNextRx(
    uint256 rY,
    uint256 L,
    int256 invariant,
    uint256 approximatedRx,
    LogNormalParams memory params
) pure returns (uint256 rX) {
    uint256 upper = approximatedRx;
    uint256 lower = approximatedRx;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            upper = upper.mulDivUp(1001, 1000);
            upper = upper > L ? L : upper;
            computedInvariant = computeTradingFunction({
                rX: upper,
                rY: rY,
                L: L,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            lower = lower > L ? L : lower;
            computedInvariant = computeTradingFunction({
                rX: lower,
                rY: rY,
                L: L,
                params: params
            });
        }
    }
    (uint256 rootInput, uint256 upperInput,) = bisection(
        abi.encode(rY, L, computedInvariant, params),
        lower,
        upper,
        0,
        MAX_ITER,
        findRootX
    );
    // `upperInput` should be positive, so if root is < 0 return upperInput instead
    if (
        computeTradingFunction({ rX: rootInput, rY: rY, L: L, params: params })
            == 0
    ) {
        rX = rootInput;
    } else {
        rX = upperInput;
    }
}

function computeNextRy(
    uint256 rX,
    uint256 L,
    int256 invariant,
    uint256 approximatedRy,
    LogNormalParams memory params
) pure returns (uint256 rY) {
    uint256 upper = approximatedRy;
    uint256 lower = approximatedRy;
    int256 computedInvariant = invariant;
    if (computedInvariant < 0) {
        while (computedInvariant < 0) {
            upper = upper.mulDivUp(1001, 1000);
            computedInvariant = computeTradingFunction({
                rX: rX,
                rY: upper,
                L: L,
                params: params
            });
        }
    } else {
        while (computedInvariant > 0) {
            lower = lower.mulDivDown(999, 1000);
            computedInvariant = computeTradingFunction({
                rX: rX,
                rY: lower,
                L: L,
                params: params
            });
        }
    }
    (uint256 rootInput, uint256 upperInput,) = bisection(
        abi.encode(rX, L, computedInvariant, params),
        lower,
        upper,
        0,
        MAX_ITER,
        findRootY
    );
    // `upperInput` should be positive, so if root is < 0 return upperInput instead
    if (
        computeTradingFunction({ rX: rX, rY: rootInput, L: L, params: params })
            == 0
    ) {
        rY = rootInput;
    } else {
        rY = upperInput;
    }
}

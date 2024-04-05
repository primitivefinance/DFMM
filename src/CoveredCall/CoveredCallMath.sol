// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { SignedWadMathLib } from "src/lib/SignedWadMath.sol";
import { ONE, HALF } from "src/lib/StrategyLib.sol";
import { CoveredCallParams } from "src/CoveredCall/CoveredCall.sol";
import { Gaussian } from "solstat/Gaussian.sol";
import { toUint } from "src/CoveredCall/CoveredCallUtils.sol";
import { bisection } from "src/lib/BisectionLib.sol";
import "forge-std/console2.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

uint256 constant MAX_ITER = 128;
uint256 constant YEAR = 31_536_000;

function computeTradingFunction(
    uint256 rX,
    uint256 rY,
    uint256 L,
    CoveredCallParams memory params
) pure returns (int256) {
    int256 a = Gaussian.ppf(int256(rX.divWadUp(L)));
    int256 b = Gaussian.ppf(int256(rY.divWadUp(L.mulWadUp(params.mean))));
    uint256 tau = getTau(params);
    int256 c = int256(computeSigmaSqrtTau(params.width, tau));
    return a + b + c;
}

function getTau(CoveredCallParams memory params) pure returns (uint256) {
    if (params.timestamp >= params.maturity) {
        return 0;
    } else {
        return ONE * (params.maturity - params.timestamp) / YEAR;
    }
}

function computeDeltaGivenDeltaLRoundUp(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulDivUp(deltaLiquidity, totalLiquidity);
}

function computeDeltaGivenDeltaLRoundDown(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulDivDown(deltaLiquidity, totalLiquidity);
}

function computeLnSDivMean(
    uint256 S,
    uint256 mean
) pure returns (int256 lnSDivMean) {
    lnSDivMean = int256(S.divWadUp(mean)).lnWad();
}

/**
 * @dev Computes the half of the square of sigma.
 *
 * $$\frac{1}{2}\sigma^2$$
 *
 */
function computeHalfSigmaSquaredTau(
    uint256 sigma,
    uint256 tau
) pure returns (uint256) {
    uint256 innerTerm = sigma.mulWadDown(sigma).mulWadDown(tau);
    return HALF.mulWadDown(innerTerm);
}

function computeSigmaSqrtTau(
    uint256 sigma,
    uint256 tau
) pure returns (uint256 sigmaSqrtTau) {
    uint256 sqrtTau = FixedPointMathLib.sqrt(tau) * 10 ** 9;
    sigmaSqrtTau = sigma.mulWadUp(sqrtTau);
}

/**
 * @dev Computes reserves L given rx, S.
 *
 * $$L_0 = \frac{x}{1-\Phi(d_1(S;\mu,\sigma))}$$
 *
 * @param rx The reserve of x.
 * @param S The price of X in Y, in WAD units.
 * @param params LogNormParameters of the Log Normal distribution.
 * @return L The liquidity given rx, S
 */
function computeLGivenX(
    uint256 rx,
    uint256 S,
    CoveredCallParams memory params
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
    CoveredCallParams memory params
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
    CoveredCallParams memory params
) pure returns (uint256 rx) {
    int256 d1 = computeD1({ S: S, params: params });
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
    CoveredCallParams memory params
) pure returns (int256 d1) {
    int256 lnSDivMean = computeLnSDivMean(S, params.mean);
    uint256 tau = getTau(params);
    uint256 halfSigmaSquaredTau = computeHalfSigmaSquaredTau(params.width, tau);
    d1 = (lnSDivMean + int256(halfSigmaSquaredTau)).wadDiv(int256(params.width));
}

/// @dev Computes the d2 parameter for the Black-Scholes formula.
/// $$d_2(S;\mu,\sigma) = \frac{\ln\frac{S}{K}-\frac{1}{2}\sigma^2 }{\sigma}$$
/// @param S The price of X in Y, in WAD units.
/// @param params LogNormParameters of the Log Normal distribution.
/// @return d2 = d1 - sigma * sqrt(tau), alternatively d2 = (ln(S/K) - tau * sigma^2 / 2) / (sigma * sqrt(tau))
function computeD2(
    uint256 S,
    CoveredCallParams memory params
) pure returns (int256 d2) {
    int256 lnSDivMean = computeLnSDivMean(S, params.mean);
    uint256 tau = getTau(params);
    uint256 halfSigmaSquaredTau = computeHalfSigmaSquaredTau(params.width, tau);
    d2 = (lnSDivMean - int256(halfSigmaSquaredTau)).wadDiv(int256(params.width));
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
    CoveredCallParams memory params
) pure returns (uint256) {
    uint256 tau = getTau(params);
    uint256 a = computeHalfSigmaSquaredTau(params.width, tau);
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
    CoveredCallParams memory params
) pure returns (uint256) {
    uint256 tau = getTau(params);
    uint256 a = computeHalfSigmaSquaredTau(params.width, tau);

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
    CoveredCallParams memory params
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
    CoveredCallParams memory params
) pure returns (uint256 deltaL) {
    uint256 fees = params.swapFee.mulWadUp(amountIn);
    uint256 px = computePriceGivenX(rx, L, params);
    deltaL = L.mulWadUp(fees).divWadDown(px.mulWadDown(rx) + ry);
}

function computeAllocationGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadUp(rX);
    deltaY = a.mulWadUp(rY);
    deltaL = a.mulWadUp(totalLiquidity);
}

function computeAllocationGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadUp(rY);
    deltaX = a.mulWadUp(rX);
    deltaL = a.mulWadUp(totalLiquidity);
}

function computeDeallocationGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadDown(rX);
    deltaY = a.mulWadDown(rY);
    deltaL = a.mulWadDown(totalLiquidity);
}

function computeDeallocationGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadDown(rY);
    deltaX = a.mulWadDown(rX);
    deltaL = a.mulWadDown(totalLiquidity);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveYWad.
function findRootY(bytes memory data, uint256 ry) pure returns (int256) {
    (uint256 rx, uint256 L, CoveredCallParams memory params) =
        abi.decode(data, (uint256, uint256, CoveredCallParams));
    return computeTradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the reserveXWad.
function findRootX(bytes memory data, uint256 rx) pure returns (int256) {
    (uint256 ry, uint256 L, CoveredCallParams memory params) =
        abi.decode(data, (uint256, uint256, CoveredCallParams));
    return computeTradingFunction(rx, ry, L, params);
}

/// @dev This is a pure anonymous function defined at the file level, which allows
/// it to be passed as an argument to another function. BisectionLib.sol takes this
/// function as an argument to find the root of the trading function given the liquidity.
function findRootLiquidity(
    bytes memory data,
    uint256 L
) pure returns (int256) {
    (uint256 rx, uint256 ry, CoveredCallParams memory params) =
        abi.decode(data, (uint256, uint256, CoveredCallParams));
    return computeTradingFunction(rx, ry, L, params);
}

function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    int256 invariant,
    uint256 approximatedL,
    CoveredCallParams memory params
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
        abi.encode(rX, rY, params), lower, upper, 1, MAX_ITER, findRootLiquidity
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
    CoveredCallParams memory params
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
        abi.encode(rY, L, params), lower, upper, 0, MAX_ITER, findRootX
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
    CoveredCallParams memory params
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
        abi.encode(rX, L, params), lower, upper, 0, MAX_ITER, findRootY
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

function sqrt(uint256 x) pure returns (uint256 r) {
    assembly ("memory-safe") {
        // r = floor(log2(x))
        r := shl(7, gt(x, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF))
        let xx := shr(r, x)

        let rr := shl(6, gt(x, 0xFFFFFFFFFFFFFFFF))
        xx := shr(rr, xx)
        r := or(r, rr)

        rr := shl(5, gt(xx, 0xFFFFFFFF))
        xx := shr(rr, xx)
        r := or(r, rr)

        rr := shl(4, gt(xx, 0xFFFF))
        xx := shr(rr, xx)
        r := or(r, rr)

        rr := shl(3, gt(xx, 0xFF))
        xx := shr(rr, xx)
        r := or(r, rr)

        rr := shl(2, gt(xx, 0x0F))
        xx := shr(rr, xx)
        r := or(r, rr)

        rr := shl(1, gt(xx, 0x03))
        xx := shr(rr, xx)
        r := or(r, rr)

        r := shl(shr(1, r), 1)

        // Newton's method
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))
        r := shr(1, add(r, div(x, r)))

        // r = min(r, x/r)
        r := sub(r, gt(r, div(x, r)))
    }
}

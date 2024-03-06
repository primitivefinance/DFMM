// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "solstat/Gaussian.sol";
import "src/lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeLnSDivK(uint256 S, uint256 K) pure returns (int256 lnSDivK) {
    lnSDivK = int256(S.divWadUp(K)).lnWad();
}

function computeSigmaSqrtTau(
    uint256 sigma,
    uint256 tau
) pure returns (uint256 sigmaSqrtTau) {
    uint256 sqrtTau = FixedPointMathLib.sqrt(tau) * 10 ** 9;
    sigmaSqrtTau = sigma.mulWadDown(sqrtTau);
}

function computeHalfSigmaTauSquared(
    uint256 sigma,
    uint256 tau
) pure returns (uint256 halfSigmaPower2Tau) {
    uint256 innerTerm =
        uint256(int256(sigma).powWad(int256(TWO))).mulWadDown(tau);

    halfSigmaPower2Tau = HALF.mulWadDown(innerTerm);
}

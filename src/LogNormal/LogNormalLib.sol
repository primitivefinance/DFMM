// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./LogNormalMath.sol";
import "./LogNormal.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

library LogNormalLib {
    enum LogNormalUpdateCode {
        Invalid,
        SwapFee,
        Width,
        Mean,
        Controller
    }

    function encodeFeeUpdate(uint256 swapFee)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encode(LogNormalUpdateCode.SwapFee, uint256(swapFee));
    }

    function decodeFeeUpdate(bytes memory data)
        internal
        pure
        returns (uint256)
    {
        (, uint256 swapFee) = abi.decode(data, (LogNormalUpdateCode, uint256));
        return swapFee;
    }

    function encodeMeanUpdate(
        uint256 targetMean,
        uint256 targetTimestamp
    ) internal pure returns (bytes memory) {
        return abi.encode(LogNormalUpdateCode.Mean, targetMean, targetTimestamp);
    }

    function decodeMeanUpdate(bytes memory data)
        internal
        pure
        returns (uint256 targetMean, uint256 targetTimestamp)
    {
        (, targetMean, targetTimestamp) =
            abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
    }

    function encodeWidthUpdate(
        uint256 targetWidth,
        uint256 targetTimestamp
    ) internal pure returns (bytes memory) {
        return
            abi.encode(LogNormalUpdateCode.Width, targetWidth, targetTimestamp);
    }

    function decodeWidthUpdate(bytes memory data)
        internal
        pure
        returns (uint256 targetWidth, uint256 targetTimestamp)
    {
        (, targetWidth, targetTimestamp) =
            abi.decode(data, (LogNormalUpdateCode, uint256, uint256));
    }

    function encodeControllerUpdate(address controller)
        internal
        pure
        returns (bytes memory data)
    {
        return abi.encode(LogNormalUpdateCode.Controller, controller);
    }

    function decodeControllerUpdate(bytes memory data)
        internal
        pure
        returns (address controller)
    {
        (, controller) = abi.decode(data, (LogNormalUpdateCode, address));
    }

    function tradingFunction(
        uint256 rX,
        uint256 rY,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (int256) {
        int256 a = Gaussian.ppf(int256(rX.divWadDown(L)));
        int256 b =
            Gaussian.ppf(int256(rY.divWadDown(L.mulWadDown(params.mean))));
        return a + b + int256(params.width);
    }

    function computeHalfSigmaSquared(uint256 sigma)
        internal
        pure
        returns (uint256)
    {
        int256 sigmaSquaredWad = int256(sigma).powWad(int256(TWO));
        return HALF.mulWadDown(uint256(sigmaSquaredWad));
    }

    /// @dev Computes the approximated spot price given current reserves and liquidity.
    function computePriceGivenX(
        uint256 rx,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256 price) {
        uint256 sigmaSqrtTau = computeSigmaSqrtTau(params.sigma, params.tau);
        uint256 halfSigmaSquared =
            computeHalfSigmaTauSquared(params.sigma, params.tau);
        uint256 halfSigmaSquaredTau = halfSigmaSquared.mulWadDown(params.tau);

        // Gaussian.ppf has a range of [-inf, inf], so we need to make sure the input is in [0, 1].
        int256 reserveXDivLiquidity = int256(rx.divWadDown(L));
        // As x -> 1, price -> 0.
        if (reserveXDivLiquidity >= int256(ONE)) {
            return 0;
        }
        // As x -> 0, price -> infinity.
        if (reserveXDivLiquidity <= int256(ZERO)) {
            // todo: can returning an infinity price be worse than returning zero or reverting?
            return INFINITY_IS_NOT_REAL;
        }
        // The output can be negative so we have to be careful not to lose that information by casting.
        int256 inverse_cdf_result =
            Gaussian.ppf(int256(ONE) - reserveXDivLiquidity);
        int256 exponent = inverse_cdf_result * int256(sigmaSqrtTau)
            / int256(ONE) - int256(halfSigmaSquaredTau);

        // This result cannot be negative!
        int256 exp_result = exponent.expWad();
        uint256 exp_result_uint = toUint(exp_result);
        price = params.strike.mulWadUp(exp_result_uint);
    }

    function computePriceGivenY(
        uint256 ry,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256 price) {
        uint256 sigmaSqrtTau = computeSigmaSqrtTau(params.sigma, params.tau);
        uint256 halfSigmaSquared =
            computeHalfSigmaTauSquared(params.sigma, params.tau);
        uint256 halfSigmaSquaredTau = halfSigmaSquared.mulWadDown(params.tau);

        // Gaussian.ppf has a range of [-inf, inf], so we need to make sure the input is in [0, 1].
        int256 yOverKL = int256(ry.divWadDown(params.strike.mulWadDown(L)));
        // As x -> 1, price -> 0.
        if (yOverKL >= int256(ONE)) {
            return 0;
        }
        // As x -> 0, price -> infinity.
        if (yOverKL <= int256(ZERO)) {
            // todo: can returning an infinity price be worse than returning zero or reverting?
            return INFINITY_IS_NOT_REAL;
        }
        // The output can be negative so we have to be careful not to lose that information by casting.
        int256 inverse_cdf_result = Gaussian.ppf(yOverKL);
        int256 exponent = inverse_cdf_result * int256(sigmaSqrtTau)
            / int256(ONE) + int256(halfSigmaSquaredTau);

        // This result cannot be negative!
        int256 exp_result = exponent.expWad();
        uint256 exp_result_uint = toUint(exp_result);
        price = params.strike.mulWadUp(exp_result_uint);
    }

    /// @dev Casts a positived signed integer to an unsigned integer, reverting if `x` is negative.
    function toUint(int256 x) internal pure returns (uint256) {
        unchecked {
            require(x >= 0, "toUint: negative");
            return uint256(x);
        }
    }
}

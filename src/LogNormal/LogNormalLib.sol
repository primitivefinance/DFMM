// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./LogNormalMath.sol";
import "./LogNormal.sol";
import { SignedWadMathLib } from "src/lib/SignedWadMath.sol";
import { Gaussian } from "solstat/Gaussian.sol";
import { ONE, EPSILON } from "src/lib/StrategyLib.sol";

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;
using SignedWadMathLib for int256;

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

    /**
     * @dev Computes the price using the reserve of token X.
     *
     * $$P_X(x, L; \mu, \sigma) = \mu \exp (\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
     *
     */
    function computePriceGivenX(
        uint256 rX,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256) {
        // $$\frac{1}{2} \sigma^2$$
        uint256 a = HALF.mulWadDown(
            uint256(int256(params.width).powWad(int256(2 ether)))
        );
        // $$\Phi^{-1} (1 - \frac{x}{L})$$
        int256 b = Gaussian.ppf(int256(ONE - rX.divWadDown(L)));

        // $$\exp(\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
        int256 exp = (b.wadMul(int256(params.width)) - int256(a)).expWad();

        // $$\mu \exp (\Phi^{-1}  (1 - \frac{x}{L} ) \sigma  - \frac{1}{2} \sigma^2  )$$
        return params.mean.mulWadUp(a.mulWadUp(uint256(exp)));
    }

    function computePriceGivenY(
        uint256 rY,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256) {
        // $$\frac{1}{2} \sigma^2$$
        uint256 a = HALF.mulWadDown(
            uint256(int256(params.width).powWad(int256(2 ether)))
        );

        // $$\Phi^{-1} (\frac{y}{\mu L})$$
        int256 b =
            Gaussian.ppf(int256(rY.divWadDown(params.mean.mulWadDown(L))));

        // $$\exp (\Phi^{-1} (\frac{y}{\mu L}) \sigma  + \frac{1}{2} \sigma^2  )$$
        int256 exp = (b.wadMul(int256(params.width)) + int256(a)).expWad();

        // $$\mu \exp (\Phi^{-1} (\frac{y}{\mu L}) \sigma  + \frac{1}{2} \sigma^2  )$$
        return params.mean.mulWadUp(a.mulWadUp(uint256(exp)));
    }

    /// @dev Casts a positived signed integer to an unsigned integer, reverting if `x` is negative.
    function toUint(int256 x) internal pure returns (uint256) {
        unchecked {
            require(x >= 0, "toUint: negative");
            return uint256(x);
        }
    }
}

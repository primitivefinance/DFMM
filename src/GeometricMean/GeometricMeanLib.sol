// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./GeometricMean.sol";
import "../lib/StrategyLib.sol";
import "solmate/utils/FixedPointMathLib.sol";

library GeometricMeanLib {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    enum GeometricMeanUpdateCode {
        Invalid,
        SwapFee,
        WeightX,
        Controller
    }

    function encodeFeeUpdate(uint256 swapFee)
        internal
        pure
        returns (bytes memory)
    {
        return abi.encode(GeometricMeanUpdateCode.SwapFee, uint256(swapFee));
    }

    function decodeFeeUpdate(bytes memory data)
        internal
        pure
        returns (uint256)
    {
        (, uint256 swapFee) =
            abi.decode(data, (GeometricMeanUpdateCode, uint256));
        return swapFee;
    }

    function encodeWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) internal pure returns (bytes memory data) {
        return abi.encode(
            GeometricMeanUpdateCode.WeightX, targetWeightX, targetTimestamp
        );
    }

    function decodeWeightXUpdate(bytes memory data)
        internal
        pure
        returns (uint256 targetWeightX, uint256 targetTimestamp)
    {
        (, targetWeightX, targetTimestamp) =
            abi.decode(data, (GeometricMeanUpdateCode, uint256, uint256));
    }

    function encodeControllerUpdate(address controller)
        internal
        pure
        returns (bytes memory data)
    {
        return abi.encode(GeometricMeanUpdateCode.Controller, controller);
    }

    function decodeControllerUpdate(bytes memory data)
        internal
        pure
        returns (address controller)
    {
        (, controller) = abi.decode(data, (GeometricMeanUpdateCode, address));
    }

    function tradingFunction(
        uint256 rX,
        uint256 rY,
        uint256 L,
        GeometricMean.GeometricMeanParams memory params
    ) internal pure returns (int256) {
        uint256 a = uint256(int256(rX.divWadDown(L)).powWad(int256(params.wX)));
        uint256 b = uint256(int256(rY.divWadDown(L)).powWad(int256(params.wY)));

        return int256(a.mulWadUp(b)) - int256(1 ether);
    }

    /// @dev Finds the root of the swapConstant given the independent variable liquidity.
    function computeNextLiquidity(
        uint256 rX,
        uint256 rY,
        GeometricMean.GeometricMeanParams memory params
    ) internal pure returns (uint256 L) {
        return uint256(int256(rX).powWad(int256(params.wX))).mulWadUp(
            uint256(int256(rY).powWad(int256(params.wY)))
        );
    }
}

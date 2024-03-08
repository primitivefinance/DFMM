// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./NTokenGeometricMean.sol";
import "../lib/StrategyLib.sol";
import "solmate/utils/FixedPointMathLib.sol";

library NTokenGeometricMeanLib {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    enum GeometricMeanUpdateCode {
        Invalid,
        SwapFee,
        Weights,
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

    function encodeWeightsUpdate(
        uint256[] calldata targetWeights,
        uint256 targetTimestamp
    ) internal pure returns (bytes memory data) {
        return abi.encode(
            GeometricMeanUpdateCode.Weights, targetWeights, targetTimestamp
        );
    }

    function decodeWeightsUpdate(bytes memory data)
        internal
        pure
        returns (uint256[] memory targetWeights, uint256 targetTimestamp)
    {
        (, targetWeights, targetTimestamp) =
            abi.decode(data, (GeometricMeanUpdateCode, uint256[], uint256));
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
        uint256[] memory reserves,
        uint256 L,
        NTokenGeometricMeanParams memory params
    ) internal pure returns (int256) {
        uint256 accumulator = ONE;
        for (uint256 i = 0; i < reserves.length; i++) {
            uint256 a = uint256(
                int256(reserves[i].divWadDown(L)).powWad(
                    int256(params.weights[i])
                )
            );
            accumulator.mulWadUp(a);
        }

        return int256(accumulator) - int256(ONE);
    }

    /// @dev Finds the root of the swapConstant given the independent variable liquidity.
    function computeNextLiquidity(
        uint256[] memory reserves,
        NTokenGeometricMeanParams memory params
    ) internal pure returns (uint256 L) {
        uint256 accumulator;
        for (uint256 i = 0; i < reserves.length; i++) {
            uint256 a =
                uint256(int256(reserves[i]).powWad(int256(params.weights[i])));
            if (accumulator != 0) {
                accumulator.mulWadUp(a);
            } else {
                accumulator = a;
            }
        }
        return accumulator;
        /*
        return uint256(int256(rX).powWad(int256(params.wX))).mulWadUp(
            uint256(int256(rY).powWad(int256(params.wY)))
        );
        */
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/GeometricMean/G3MUtils.sol";

contract GeometricMeanLibTest is Test {
    function testFuzz_GeometricMeanLib_encodeFeeUpdate(uint256 swapFee)
        public
    {
        bytes memory data = encodeFeeUpdate(swapFee);
        assertEq(swapFee, decodeFeeUpdate(data));
    }

    function testFuzz_GeometricMeanLib_encodeWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public {
        bytes memory data = encodeWeightXUpdate(targetWeightX, targetTimestamp);

        (uint256 decodedTargetWeightX, uint256 decodedTargetTimestamp) =
            decodeWeightXUpdate(data);
        assertEq(targetWeightX, decodedTargetWeightX);
        assertEq(targetTimestamp, decodedTargetTimestamp);
    }

    function testFuzz_GeometricMeanLib_encodeControllerUpdate(
        address controller
    ) public {
        bytes memory data = encodeControllerUpdate(controller);
        assertEq(controller, decodeControllerUpdate(data));
    }
}

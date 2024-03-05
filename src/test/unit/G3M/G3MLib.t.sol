// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/GeometricMean/GeometricMeanLib.sol";

contract GeometricMeanLibTest is Test {
    function testFuzz_GeometricMeanLib_encodeFeeUpdate(uint256 swapFee)
        public
    {
        bytes memory data = GeometricMeanLib.encodeFeeUpdate(swapFee);
        assertEq(swapFee, GeometricMeanLib.decodeFeeUpdate(data));
    }

    function testFuzz_GeometricMeanLib_encodeWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public {
        bytes memory data =
            GeometricMeanLib.encodeWeightXUpdate(targetWeightX, targetTimestamp);

        (uint256 decodedTargetWeightX, uint256 decodedTargetTimestamp) =
            GeometricMeanLib.decodeWeightXUpdate(data);
        assertEq(targetWeightX, decodedTargetWeightX);
        assertEq(targetTimestamp, decodedTargetTimestamp);
    }

    function testFuzz_GeometricMeanLib_encodeControllerUpdate(
        address controller
    ) public {
        bytes memory data = GeometricMeanLib.encodeControllerUpdate(controller);
        assertEq(controller, GeometricMeanLib.decodeControllerUpdate(data));
    }

    function test_GeometricMeanLib_tradingFunction() public {
        // TODO: Add a differential test here
    }

    function test_GeometricMeanLib_computeLiquidity() public {
        // TODO: Add a differential test here
    }
}

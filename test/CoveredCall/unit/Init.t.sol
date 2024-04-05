// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "forge-std/Test.sol";

contract CoveredCallInitTest is CoveredCallSetUp {
    function test_CoveredCall_init_ReturnsPriceOfOne() public init_quarterly {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenYL = solver.getPriceGivenYL(POOL_ID, reserves[1], L);
        uint256 priceGivenXL = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
        console2.log("priceGivenYL", priceGivenYL);
        console2.log("priceGivenXL", priceGivenXL);

        assertApproxEqAbs(priceGivenXL, ONE, 10);
        assertApproxEqAbs(priceGivenYL, ONE, 10);
    }
}

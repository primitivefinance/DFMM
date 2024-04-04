// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "forge-std/Test.sol";

contract CoveredCallInitTest is CoveredCallSetUp {
    function test_CoveredCall_init_RevertsIfInvalidTokens() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenX);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: defaultInitialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidDuplicateTokens.selector);
        dfmm.init(initParams);
    }

    function test_CoveredCall_init_ReturnsPriceOfOne() public init {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenYL = solver.getPriceGivenYL(POOL_ID, reserves[1], L);
        uint256 priceGivenXL = solver.getPriceGivenXL(POOL_ID, reserves[0], L);

        assertApproxEqAbs(priceGivenXL, ONE, 10);
        assertApproxEqAbs(priceGivenYL, ONE, 10);
    }
}

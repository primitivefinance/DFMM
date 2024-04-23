// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "forge-std/Test.sol";

contract LogNormalInitTest is LogNormalSetUp {
    function test_LogNormal_init_StoresPoolParameters() public init {
        /*
        (
            address strategy,
            address[] memory tokens,
            uint256[] memory reserves,
            uint256 totalLiquidity,
        ) = dfmm.pools(POOL_ID);

        assertEq(strategy, address(logNormal));
        /* assertEq(tokenX, address(tokenX));
        assertEq(tokenY, address(tokenY));
        assertEq(reserveX, defaultReserveX);
        assertEq(reserveY, reserveY); */
        // assertEq(totalLiquidity, totalLiquidity);
    }

    function test_LogNormal_init_RevertsIfInvalidTokens() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenX);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: defaultInitialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidDuplicateTokens.selector);
        dfmm.init(initParams);
    }

    function test_LogNormal_init_ReturnsPriceOfOne() public init {
        (uint256 rX, uint256 rY, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenYL = solver.getPriceGivenYL(POOL_ID, rY, L);
        uint256 priceGivenXL = solver.getPriceGivenXL(POOL_ID, rX, L);

        assertApproxEqAbs(priceGivenXL, ONE, 10);
        assertApproxEqAbs(priceGivenYL, ONE, 10);
    }
}

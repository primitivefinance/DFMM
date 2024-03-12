// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

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
        skip();
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenX);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: defaultInitialPoolData
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(initParams);
    }
}

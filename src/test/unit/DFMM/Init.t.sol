// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IDFMM } from "src/interfaces/IDFMM.sol";
import { DFMMSetUp } from "./SetUp.sol";

contract DFMMInit is DFMMSetUp {
    bool valid = true;
    int256 initialInvariant = 1 ether;
    uint256 initialReserveX = 1 ether;
    uint256 initialReserveY = 1 ether;
    uint256 initialLiquidity = 1 ether;
    bytes defaultData = abi.encode(
        valid,
        initialInvariant,
        initialReserveX,
        initialReserveY,
        initialLiquidity
    );

    function test_DFMM_init_StoresStrategyInitialReservesAndLiquidity()
        public
    {
        bytes memory foo;
        (uint256 poolId,,,) = dfmm.init(getDefaultPoolParams(foo));
        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            dfmm.getReservesAndLiquidity(poolId);
        assertEq(initialLiquidity, totalLiquidity);
        assertEq(initialReserveX, reserveX);
        assertEq(initialReserveY, reserveY);
    }

    function test_DFMM_init_ReturnsStrategyInitialReserves() public {
        (, uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            dfmm.init(getDefaultPoolParams(defaultData));
        // A bit of the liquidity is burnt
        assertEq(initialLiquidity - 1000, totalLiquidity);
        assertEq(initialReserveX, reserveX);
        assertEq(initialReserveY, reserveY);
    }

    function test_DFMM_init_IncrementsPoolId() public {
        (uint256 poolId,,,) = dfmm.init(getDefaultPoolParams(defaultData));
        assertEq(poolId, 0);
        (poolId,,,) = dfmm.init(getDefaultPoolParams(defaultData));
        assertEq(poolId, 1);
    }

    function test_DFMM_init_TransfersInitialReserves() public {
        uint256 dfmmPreBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 dfmmPreBalanceY = tokenY.balanceOf(address(dfmm));

        uint256 tokenXPreBalance = tokenX.balanceOf(address(this));
        uint256 tokenYPreBalance = tokenY.balanceOf(address(this));

        (uint256 poolId,,,) = dfmm.init(getDefaultPoolParams(defaultData));
        (uint256 reserveX, uint256 reserveY,) =
            dfmm.getReservesAndLiquidity(poolId);

        assertEq(tokenX.balanceOf(address(dfmm)), dfmmPreBalanceX + reserveX);
        assertEq(tokenY.balanceOf(address(dfmm)), dfmmPreBalanceY + reserveY);

        assertEq(tokenX.balanceOf(address(this)), tokenXPreBalance - reserveX);
        assertEq(tokenY.balanceOf(address(this)), tokenYPreBalance - reserveY);
    }

    function test_dfmm_init_EmitsInitEvent() public {
        vm.expectEmit(true, true, true, true, address(dfmm));
        emit IDFMM.Init(
            address(this),
            address(strategy),
            address(0xDD4c722d1614128933d6DC7EFA50A6913e804E12),
            address(tokenX),
            address(tokenY),
            0,
            initialReserveX,
            initialReserveY,
            initialLiquidity
        );

        dfmm.init(getDefaultPoolParams(defaultData));
    }

    function test_DFMM_init_DeploysLPTokenClone() public initPool {
        (,,,,,, address liquidityToken) = dfmm.pools(POOL_ID);
        assertTrue(liquidityToken != address(0));
        assertTrue(liquidityToken.code.length > 0);
    }

    function test_DFMM_init_RevertsWhenSameTokens() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenX),
            data: ""
        });

        vm.expectRevert(IDFMM.InvalidTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenNotValid() public {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: abi.encode(
                false,
                initialInvariant,
                initialReserveX,
                initialReserveY,
                initialLiquidity
                )
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                IDFMM.Invalid.selector, false, initialInvariant
            )
        );
        dfmm.init(params);
    }
}

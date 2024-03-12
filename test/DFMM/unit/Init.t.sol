// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import { LPToken } from "src/LPToken.sol";
import { DFMMSetUp, IDFMM2 } from "./SetUp.sol";

contract DFMMInit is DFMMSetUp, Script {
    bool valid = true;
    int256 initialInvariant = 1 ether;
    uint256 initialReserveX = 1 ether;
    uint256 initialReserveY = 1 ether;
    uint256 initialLiquidity = 1 ether;
    uint256[] defaultReserves = [initialReserveX, initialReserveY];
    bytes defaultData =
        abi.encode(valid, initialInvariant, defaultReserves, initialLiquidity);

    function test_DFMM_init_StoresStrategyInitialReservesAndLiquidity()
        public
    {
        (uint256 poolId,,) = dfmm.init(getDefaultPoolParams(defaultData));
        (uint256[] memory reserves, uint256 totalLiquidity) =
            dfmm.getReservesAndLiquidity(poolId);
        assertEq(initialLiquidity, totalLiquidity);
        assertEq(initialReserveX, reserves[0]);
        assertEq(initialReserveY, reserves[1]);
    }

    function test_DFMM_init_ReturnsStrategyInitialReserves() public {
        (, uint256[] memory reserves, uint256 totalLiquidity) =
            dfmm.init(getDefaultPoolParams(defaultData));
        // A bit of the liquidity is burnt
        assertEq(initialLiquidity - 1000, totalLiquidity);
        assertEq(initialReserveX, reserves[0]);
        assertEq(initialReserveY, reserves[1]);
    }

    function test_DFMM_init_IncrementsPoolId() public {
        (uint256 poolId,,) = dfmm.init(getDefaultPoolParams(defaultData));
        assertEq(poolId, 0);
        (poolId,,) = dfmm.init(getDefaultPoolParams(defaultData));
        assertEq(poolId, 1);
    }

    function test_DFMM_init_TransfersInitialReserves() public {
        uint256 dfmmPreBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 dfmmPreBalanceY = tokenY.balanceOf(address(dfmm));

        uint256 tokenXPreBalance = tokenX.balanceOf(address(this));
        uint256 tokenYPreBalance = tokenY.balanceOf(address(this));

        (uint256 poolId,,) = dfmm.init(getDefaultPoolParams(defaultData));
        (uint256[] memory reserves,) = dfmm.getReservesAndLiquidity(poolId);

        assertEq(tokenX.balanceOf(address(dfmm)), dfmmPreBalanceX + reserves[0]);
        assertEq(tokenY.balanceOf(address(dfmm)), dfmmPreBalanceY + reserves[1]);

        assertEq(
            tokenX.balanceOf(address(this)), tokenXPreBalance - reserves[0]
        );
        assertEq(
            tokenY.balanceOf(address(this)), tokenYPreBalance - reserves[1]
        );
    }

    function test_dfmm_init_EmitsInitEvent() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        uint256[] memory reserves = new uint256[](2);

        vm.expectEmit(true, true, true, true, address(dfmm));
        emit IDFMM2.Init(
            address(this),
            address(strategy),
            computeCreateAddress(address(dfmm), vm.getNonce(address(dfmm))),
            0,
            tokens,
            reserves,
            initialLiquidity
        );

        dfmm.init(getDefaultPoolParams(defaultData));
    }

    function test_DFMM_init_DeploysLPTokenClone() public initPool {
        IDFMM2.Pool memory pool = dfmm.getPool(POOL_ID);
        assertTrue(pool.liquidityToken != address(0));
        assertTrue(pool.liquidityToken.code.length > 0);
    }

    function test_DFMM_init_SetsLPTokenMetadata() public initPool {
        IDFMM2.Pool memory pool = dfmm.getPool(POOL_ID);
        LPToken lpToken = LPToken(pool.liquidityToken);
        assertEq(lpToken.name(), "DFMM-MockStrategy-TSTX-TSTY-0");
        assertEq(lpToken.symbol(), "DFMM-MockStrategy-TSTX-TSTY-0");
    }

    function test_DFMM_init_MintsLPTokens() public initPool {
        IDFMM2.Pool memory pool = dfmm.getPool(POOL_ID);
        LPToken lpToken = LPToken(pool.liquidityToken);
        assertEq(lpToken.balanceOf(address(this)), initialLiquidity - 1000);
        assertEq(lpToken.balanceOf(address(0)), 1000);
    }

    function test_DFMM_init_RevertsWhenSameTokens() public {
        skip();
        /*
        IDFMM2.InitParams memory params = IDFMM2.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenX),
            data: ""
        });

        vm.expectRevert(IDFMM2.InvalidTokens.selector);
        dfmm.init(params);
        */
    }

    function test_DFMM_init_RevertsWhenNotValid() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM2.InitParams memory params = IDFMM2.InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
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
                IDFMM2.InvalidInvariant.selector, initialInvariant
            )
        );
        dfmm.init(params);
    }
}

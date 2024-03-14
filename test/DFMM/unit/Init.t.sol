// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { LPToken } from "src/LPToken.sol";
import { DFMMSetUp, IDFMM } from "./SetUp.sol";
import { Pool, InitParams } from "src/interfaces/IDFMM.sol";

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
            getReservesAndLiquidity(poolId);
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
        (uint256[] memory reserves,) = getReservesAndLiquidity(poolId);

        assertEq(tokenX.balanceOf(address(dfmm)), dfmmPreBalanceX + reserves[0]);
        assertEq(tokenY.balanceOf(address(dfmm)), dfmmPreBalanceY + reserves[1]);

        assertEq(
            tokenX.balanceOf(address(this)), tokenXPreBalance - reserves[0]
        );
        assertEq(
            tokenY.balanceOf(address(this)), tokenYPreBalance - reserves[1]
        );
    }

    function test_DFMM_init_AcceptsTwoToEightTokens() public {
        for (uint256 i = 2; i < 9; i++) {
            address[] memory tokens = new address[](i);
            uint256[] memory reserves = new uint256[](i);

            for (uint256 j = 0; j < i; j++) {
                MockERC20 token = new MockERC20("", "", 18);
                token.mint(address(this), 1 ether);
                token.approve(address(dfmm), 1 ether);

                tokens[j] = address(token);
                reserves[j] = 1 ether;
            }

            InitParams memory params = InitParams({
                name: "",
                symbol: "",
                strategy: address(strategy),
                tokens: tokens,
                data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
                feeCollector: address(0),
                controllerFee: 0
            });

            dfmm.init(params);
        }
    }

    function test_DFMM_init_AcceptsWETH() public {
        deal(address(weth), address(this), 1 ether);
        weth.approve(address(dfmm), 1 ether);

        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 1 ether;
        reserves[1] = 1 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(1 ether));

        address[] memory tokens = new address[](2);
        tokens[0] = address(weth);
        tokens[1] = address(tokenY);

        uint256 preWETHBalance = weth.balanceOf(address(this));

        (POOL_ID,,) = dfmm.init(
            InitParams({
                name: "Default Pool",
                symbol: "POOL",
                strategy: address(strategy),
                tokens: tokens,
                data: params,
                feeCollector: address(0),
                controllerFee: 0
            })
        );

        assertEq(weth.balanceOf(address(this)), preWETHBalance - 1 ether);
    }

    function test_DFMM_init_WrapsETH() public {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 1 ether;
        reserves[1] = 1 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(1 ether));

        address[] memory tokens = new address[](2);
        tokens[0] = address(weth);
        tokens[1] = address(tokenY);

        uint256 preETHBalance = address(this).balance;

        (POOL_ID,,) = dfmm.init{ value: 1 ether }(
            InitParams({
                name: "Default Pool",
                symbol: "POOL",
                strategy: address(strategy),
                tokens: tokens,
                data: params,
                feeCollector: address(0),
                controllerFee: 0
            })
        );

        assertLe(address(this).balance, preETHBalance - 1 ether);
        assertEq(address(dfmm).balance, 0);
        assertEq(weth.balanceOf(address(dfmm)), 1 ether);
    }

    function test_dfmm_init_EmitsInitEvent() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        vm.expectEmit(true, true, true, true, address(dfmm));
        emit IDFMM.Init(
            address(this),
            address(strategy),
            computeCreateAddress(address(dfmm), vm.getNonce(address(dfmm))),
            0,
            tokens,
            defaultReserves,
            initialLiquidity
        );

        dfmm.init(getDefaultPoolParams(defaultData));
    }

    function test_DFMM_init_DeploysLPTokenClone() public initPool {
        Pool memory pool = dfmm.pools(POOL_ID);
        assertTrue(pool.liquidityToken != address(0));
        assertTrue(pool.liquidityToken.code.length > 0);
    }

    function test_DFMM_init_SetsLPTokenMetadata() public initPool {
        Pool memory pool = dfmm.pools(POOL_ID);
        LPToken lpToken = LPToken(pool.liquidityToken);
        assertEq(lpToken.name(), "Default Pool");
        assertEq(lpToken.symbol(), "POOL");
    }

    function test_DFMM_init_MintsLPTokens() public initPool {
        Pool memory pool = dfmm.pools(POOL_ID);
        LPToken lpToken = LPToken(pool.liquidityToken);
        assertEq(lpToken.balanceOf(address(this)), initialLiquidity - 1000);
        assertEq(lpToken.balanceOf(address(0)), 1000);
    }

    function test_DFMM_init_RevertsWhenETHIsInsufficient() public {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 1 ether;
        reserves[1] = 1 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(1 ether));

        address[] memory tokens = new address[](2);
        tokens[0] = address(weth);
        tokens[1] = address(tokenY);

        vm.expectRevert("TRANSFER_FROM_FAILED");
        (POOL_ID,,) = dfmm.init{ value: 0.5 ether }(
            InitParams({
                name: "Default Pool",
                symbol: "POOL",
                strategy: address(strategy),
                tokens: tokens,
                data: params,
                feeCollector: address(0),
                controllerFee: 0
            })
        );
    }

    function test_DFMM_init_RevertsWhenDecimalsTooLow() public {
        MockERC20 token = new MockERC20("Token", "TKN", 5);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(token);

        uint256[] memory reserves = new uint256[](2);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidTokenDecimals.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenDecimalsTooHigh() public {
        MockERC20 token = new MockERC20("Token", "TKN", 19);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(token);

        uint256[] memory reserves = new uint256[](2);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidTokenDecimals.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenInvalidMinimumTokens() public {
        address[] memory tokens = new address[](1);
        tokens[0] = address(tokenX);

        uint256[] memory reserves = new uint256[](1);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidMinimumTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenInvalidMaximumTokens() public {
        address[] memory tokens = new address[](9);

        for (uint256 i = 0; i < 9; i++) {
            tokens[i] = address(new MockERC20("", "", 18));
        }

        uint256[] memory reserves = new uint256[](9);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidMaximumTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenSameTokens() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenX);

        uint256[] memory reserves = new uint256[](2);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidDuplicateTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenDuplicateTokens() public {
        address[] memory tokens = new address[](3);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);
        tokens[1] = address(tokenX);

        uint256[] memory reserves = new uint256[](3);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(true, int256(1 ether), reserves, uint256(1 ether)),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(IDFMM.InvalidDuplicateTokens.selector);
        dfmm.init(params);
    }

    function test_DFMM_init_RevertsWhenNotValid() public {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory params = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: tokens,
            data: abi.encode(
                false, initialInvariant, defaultReserves, initialLiquidity
                ),
            feeCollector: address(0),
            controllerFee: 0
        });

        vm.expectRevert(
            abi.encodeWithSelector(
                IDFMM.InvalidInvariant.selector, initialInvariant
            )
        );
        dfmm.init(params);
    }
}

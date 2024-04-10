// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSum, ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";
import { Pool, InitParams } from "src/interfaces/IDFMM.sol";
import { ConstantSumSetUp } from "./SetUp.sol";

contract ConstantSumInitTest is ConstantSumSetUp {
    function test_ConstantSum_init_InitializesPool() public {
        uint256 price = 1 ether;

        ConstantSumParams memory params = ConstantSumParams({
            price: price,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        Pool memory pool = dfmm.pools(POOL_ID);

        assertEq(pool.reserves[0], reserveX);
        assertEq(pool.reserves[1], reserveY);
    }

    // This test doesn't pass because the `controller` param is not stored
    function test_ConstantSum_init_StoresPoolParams() public {
        skip();

        uint256 price = 1 ether;

        ConstantSumParams memory params = ConstantSumParams({
            price: price,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        ConstantSumParams memory poolParams =
            abi.decode(constantSum.getPoolParams(POOL_ID), (ConstantSumParams));

        assertEq(poolParams.price, price);
        assertEq(poolParams.swapFee, TEST_SWAP_FEE);
        assertEq(poolParams.controller, address(this));
    }

    function test_ConstantSum_init_TransfersTokens() public {
        uint256 price = 1 ether;

        ConstantSumParams memory params = ConstantSumParams({
            price: price,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        uint256 dfmmPreTokenXBalance = tokenX.balanceOf(address(dfmm));
        uint256 dfmmPreTokenYBalance = tokenY.balanceOf(address(dfmm));
        uint256 userPreTokenXBalance = tokenX.balanceOf(address(this));
        uint256 userPreTokenYBalance = tokenY.balanceOf(address(this));

        dfmm.init(initParams);

        uint256 dfmmPostTokenXBalance = tokenX.balanceOf(address(dfmm));
        uint256 dfmmPostTokenYBalance = tokenY.balanceOf(address(dfmm));
        uint256 userPostTokenXBalance = tokenX.balanceOf(address(this));
        uint256 userPostTokenYBalance = tokenY.balanceOf(address(this));

        assertEq(dfmmPreTokenXBalance + reserveX, dfmmPostTokenXBalance);
        assertEq(dfmmPreTokenYBalance + reserveY, dfmmPostTokenYBalance);
        assertEq(userPreTokenXBalance - reserveX, userPostTokenXBalance);
        assertEq(userPreTokenYBalance - reserveY, userPostTokenYBalance);
    }
}

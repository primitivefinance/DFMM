// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";
import { ConstantSumSetUp, InitParams } from "./SetUp.sol";

contract ConstantSumGetPoolParamsTest is ConstantSumSetUp {
    function test_ConstantSum_getPoolParams_ReturnsPoolParams() public {
        ConstantSumParams memory initPoolParams = ConstantSumParams({
            price: 2 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.prepareInit(reserveX, reserveY, initPoolParams);

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
        assertEq(poolParams.swapFee, initPoolParams.swapFee);
        assertEq(poolParams.price, initPoolParams.price);
        assertEq(poolParams.controller, initPoolParams.controller);
    }
}

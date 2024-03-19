// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp, ConstantSumParams, InitParams } from "./SetUp.sol";

contract ConstantSumGetPoolParamsTest is ConstantSumSetUp {
    function test_ConstantSum_getPoolParams() public {
        ConstantSumParams memory initialParams = ConstantSumParams({
            price: 1 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, initialParams);

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

        dfmm.init(initParams);

        ConstantSumParams memory poolParams =
            abi.decode(constantSum.getPoolParams(0), (ConstantSumParams));

        assertEq(poolParams.price, initialParams.price);
        assertEq(poolParams.swapFee, initialParams.swapFee);
        assertEq(poolParams.controller, initialParams.controller);
    }
}

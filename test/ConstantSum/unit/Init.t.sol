// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp } from "./SetUp.sol";
import { ConstantSum, ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";
import { DFMM, IDFMM, InitParams } from "src/DFMM.sol";

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

        dfmm.init(initParams);
    }
}

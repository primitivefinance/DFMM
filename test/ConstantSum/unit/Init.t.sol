// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp } from "./SetUp.sol";
import { ConstantSum } from "src/ConstantSum/ConstantSum.sol";
import { DFMM, IDFMM } from "src/DFMM.sol";

contract ConstantSumInitTest is ConstantSumSetUp {
    function test_ConstantSum_init_InitializesPool() public {
        uint256 price = 1 ether;

        ConstantSum.ConstantSumParams memory params = ConstantSum
            .ConstantSumParams({
            price: price,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(constantSum),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: initData
        });

        dfmm.init(initParams);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { ConstantSumSetUp, ConstantSumParams } from "./SetUp.sol";
import {
    encodeFeeUpdate,
    encodePriceUpdate,
    encodeControllerUpdate
} from "src/ConstantSum/ConstantSumUtils.sol";

contract ConstantSumUpdateTest is ConstantSumSetUp {
    function test_ConstantSum_update_SetsSwapFee() public defaultPool {
        skip();
        uint256 newSwapFee = 0.004 ether;
        dfmm.update(POOL_ID, encodeFeeUpdate(newSwapFee));
        ConstantSumParams memory poolParams =
            abi.decode(constantSum.getPoolParams(POOL_ID), (ConstantSumParams));
        assertEq(poolParams.swapFee, newSwapFee);
    }

    function test_ConstantSum_update_SetsPrice() public defaultPool {
        skip();
        uint256 newPrice = 3 ether;
        dfmm.update(POOL_ID, encodePriceUpdate(newPrice));
        ConstantSumParams memory poolParams =
            abi.decode(constantSum.getPoolParams(POOL_ID), (ConstantSumParams));
        assertEq(poolParams.price, newPrice);
    }

    function test_ConstantSum_update_SetsController() public defaultPool {
        skip();
        address newController = address(this);
        dfmm.update(POOL_ID, encodeControllerUpdate(newController));
        ConstantSumParams memory poolParams =
            abi.decode(constantSum.getPoolParams(POOL_ID), (ConstantSumParams));
        assertEq(poolParams.controller, newController);
    }
}

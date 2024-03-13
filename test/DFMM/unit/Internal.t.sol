/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { DFMMSetUp, DFMM } from "./SetUp.sol";

contract DFMMInternal is DFMM {
    constructor(address weth_) DFMM(weth_) { }

    function transferFrom(address token, uint256 amount) external payable {
        _transferFrom(token, amount);
    }
}

contract DFMMInternalTest is DFMMSetUp {
    DFMMInternal dfmmInternal;

    function setUp() public override {
        super.setUp();
        dfmmInternal = new DFMMInternal(address(weth));
    }

    function test_DFMM_transferFrom_WrapsETH() public {
        uint256 amount = 1 ether;
        dfmmInternal.transferFrom{ value: amount }(address(0), amount);
        assertEq(weth.balanceOf(address(dfmmInternal)), amount);
    }
}

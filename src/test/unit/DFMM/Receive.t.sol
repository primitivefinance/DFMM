// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";

contract DFMMReceiveTest is DFMMSetUp {
    function test_DFMM_receive_RevertsIfSenderNotWETH() public {
        vm.expectRevert(IDFMM.OnlyWETH.selector);
        payable(address(dfmm)).transfer(1 ether);
    }

    function test_DFMM_receive_CanReceiveETHfromWETH() public {
        vm.deal(address(weth), 1 ether);
        vm.prank(address(weth));
        payable(address(dfmm)).transfer(1 ether);
    }
}

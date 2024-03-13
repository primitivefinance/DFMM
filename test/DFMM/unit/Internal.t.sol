/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { DFMMSetUp, DFMM, IDFMM } from "./SetUp.sol";
import { ERC20WithFees } from "test/utils/ERC20WithFees.sol";

contract DFMMInternal is DFMM {
    constructor(address weth_) DFMM(weth_) { }

    function transferFrom(address token, uint256 amount) external payable {
        _transferFrom(token, amount);
    }
}

contract DFMMInternalTest is DFMMSetUp {
    DFMMInternal dfmmInternal;

    receive() external payable { }

    function setUp() public override {
        super.setUp();
        dfmmInternal = new DFMMInternal(address(weth));
    }

    function test_DFMM_transferFrom_WrapsETH() public {
        uint256 amount = 1 ether;
        dfmmInternal.transferFrom{ value: amount }(address(0), amount);
        assertEq(weth.balanceOf(address(dfmmInternal)), amount);
        assertEq(address(weth).balance, 1 ether);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function test_DFMM_transferFrom_RefundsExtraETH() public {
        uint256 amount = 1 ether;
        dfmmInternal.transferFrom{ value: amount * 2 }(address(0), amount);
        assertEq(weth.balanceOf(address(dfmmInternal)), amount);
        assertEq(address(weth).balance, 1 ether);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function testFuzz_DFMM_transferFrom_TransferTokens(uint256 amount) public {
        vm.assume(
            amount
                <
                115_792_089_237_316_195_423_570_985_008_687_907_853_269_984_665_640_564_039_458
        );
        MockERC20 token = new MockERC20("", "", 18);
        token.mint(address(this), amount);
        token.approve(address(dfmmInternal), amount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(this));
        dfmmInternal.transferFrom(address(token), amount);
        assertEq(
            token.balanceOf(address(dfmmInternal)), preDFMMBalance + amount
        );
        assertEq(token.balanceOf(address(this)), preThisBalance - amount);
    }

    function test_DFMM_transferFrom_ScalesAmount() public {
        uint256 amount = 1_000_000 * 10 ** 18;
        uint256 scaledDownAmount = 1_000_000 * 10 ** 6;
        MockERC20 token = new MockERC20("", "", 6);
        token.mint(address(this), scaledDownAmount);
        token.approve(address(dfmmInternal), scaledDownAmount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(this));
        dfmmInternal.transferFrom(address(token), amount);
        assertEq(
            token.balanceOf(address(dfmmInternal)),
            preDFMMBalance + scaledDownAmount
        );
        assertEq(
            token.balanceOf(address(this)), preThisBalance - scaledDownAmount
        );
    }

    function test_DFMM_transferFrom_RevertsIfBalanceIsInsufficient() public {
        ERC20WithFees token = new ERC20WithFees("", "", 18, 500);
        uint256 amount = 1 ether;
        token.mint(address(this), amount);
        token.approve(address(dfmmInternal), amount);
        vm.expectRevert(IDFMM.InvalidTransfer.selector);
        dfmmInternal.transferFrom(address(token), amount);
    }
}

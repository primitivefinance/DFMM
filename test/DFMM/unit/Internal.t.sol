/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { DFMMSetUp, DFMM, IDFMM } from "./SetUp.sol";
import { ERC20WithFees } from "test/utils/ERC20WithFees.sol";

contract DFMMInternal is DFMM {
    constructor(address weth_) DFMM(weth_) { }

    function transferFrom(
        address[] memory tokens,
        uint256[] memory amounts
    ) external payable {
        _transferFrom(tokens, amounts);
    }

    function transfer(address token, address to, uint256 amount) external {
        _transfer(token, to, amount);
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
        address[] memory tokens = new address[](1);
        tokens[0] = address(weth);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;
        dfmmInternal.transferFrom{ value: amounts[0] }(tokens, amounts);
        assertEq(weth.balanceOf(address(dfmmInternal)), amounts[0]);
        assertEq(address(weth).balance, 1 ether);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function test_DFMM_transferFrom_RefundsExtraETH() public {
        address[] memory tokens = new address[](1);
        tokens[0] = address(weth);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;
        dfmmInternal.transferFrom{ value: amounts[0] * 2 }(tokens, amounts);
        assertEq(weth.balanceOf(address(dfmmInternal)), amounts[0]);
        assertEq(address(weth).balance, 1 ether);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function test_DFMM_transferFrom_UsesWETH() public {
        address[] memory tokens = new address[](1);
        tokens[0] = address(weth);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;
        weth.deposit{ value: amounts[0] }();
        weth.approve(address(dfmmInternal), amounts[0]);
        dfmmInternal.transferFrom(tokens, amounts);
        assertEq(weth.balanceOf(address(dfmmInternal)), amounts[0]);
        assertEq(address(weth).balance, amounts[0]);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function test_DFMM_transferFrom_UsesWETHAndRefunds() public {
        address[] memory tokens = new address[](1);
        tokens[0] = address(weth);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;
        weth.deposit{ value: amounts[0] }();
        weth.approve(address(dfmmInternal), amounts[0]);
        dfmmInternal.transferFrom{ value: amounts[0] - 1 }(tokens, amounts);
        assertEq(weth.balanceOf(address(dfmmInternal)), amounts[0]);
        assertEq(address(weth).balance, amounts[0]);
        assertEq(address(dfmmInternal).balance, 0);
    }

    function testFuzz_DFMM_transferFrom_TransferTokens(uint256 amount) public {
        vm.assume(
            amount
                <
                115_792_089_237_316_195_423_570_985_008_687_907_853_269_984_665_640_564_039_458
        );
        MockERC20 token = new MockERC20("", "", 18);
        address[] memory tokens = new address[](1);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = amount;
        token.mint(address(this), amount);
        token.approve(address(dfmmInternal), amount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(this));
        dfmmInternal.transferFrom(tokens, amounts);
        assertEq(
            token.balanceOf(address(dfmmInternal)), preDFMMBalance + amount
        );
        assertEq(token.balanceOf(address(this)), preThisBalance - amount);
    }

    function test_DFMM_transferFrom_ScalesAmount() public {
        address[] memory tokens = new address[](1);
        MockERC20 token = new MockERC20("", "", 6);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1_000_000 * 10 ** 18;
        uint256 scaledDownAmount = 1_000_000 * 10 ** 6;

        token.mint(address(this), scaledDownAmount);
        token.approve(address(dfmmInternal), scaledDownAmount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(this));
        dfmmInternal.transferFrom(tokens, amounts);
        assertEq(
            token.balanceOf(address(dfmmInternal)),
            preDFMMBalance + scaledDownAmount
        );
        assertEq(
            token.balanceOf(address(this)), preThisBalance - scaledDownAmount
        );
    }

    function test_DFMM_transferFrom_RevertsIfBalanceIsInsufficient() public {
        address[] memory tokens = new address[](1);
        ERC20WithFees token = new ERC20WithFees("", "", 18, 500);
        tokens[0] = address(token);
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1 ether;

        token.mint(address(this), amounts[0]);
        token.approve(address(dfmmInternal), amounts[0]);
        vm.expectRevert(IDFMM.InvalidTransfer.selector);
        dfmmInternal.transferFrom(tokens, amounts);
    }

    function test_DFMM_transfer_UnwrapsETH() public {
        address to = address(this);
        uint256 amount = 1 ether;
        weth.deposit{ value: amount }();
        uint256 preETHBalance = address(to).balance;
        deal(address(weth), address(dfmmInternal), amount);
        dfmmInternal.transfer(address(weth), to, amount);
        assertEq(weth.balanceOf(address(dfmmInternal)), 0);
        assertEq(address(this).balance, preETHBalance + amount);
    }

    function testFuzz_DFMM_transfer_TransferTokens(
        address to,
        uint256 amount
    ) public {
        vm.assume(
            amount
                <
                115_792_089_237_316_195_423_570_985_008_687_907_853_269_984_665_640_564_039_458
        );
        MockERC20 token = new MockERC20("", "", 18);
        token.mint(address(dfmmInternal), amount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(to));
        dfmmInternal.transfer(address(token), address(to), amount);
        assertEq(
            token.balanceOf(address(dfmmInternal)), preDFMMBalance - amount
        );
        assertEq(token.balanceOf(address(to)), preThisBalance + amount);
    }

    function test_DFMM_transfer_ScalesAmount() public {
        uint256 amount = 1_000_000 * 10 ** 18;
        uint256 scaledDownAmount = 1_000_000 * 10 ** 6;
        MockERC20 token = new MockERC20("", "", 6);
        token.mint(address(dfmmInternal), scaledDownAmount);
        uint256 preDFMMBalance = token.balanceOf(address(dfmmInternal));
        uint256 preThisBalance = token.balanceOf(address(this));
        dfmmInternal.transfer(address(token), address(this), amount);
        assertEq(
            token.balanceOf(address(dfmmInternal)),
            preDFMMBalance - scaledDownAmount
        );
        assertEq(
            token.balanceOf(address(this)), preThisBalance + scaledDownAmount
        );
    }
}

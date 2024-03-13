/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";

contract ERC20WithFees is MockERC20 {
    uint256 public immutable fee;

    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_,
        uint256 fee_
    ) MockERC20(name_, symbol_, decimals_) {
        fee = fee_;
    }

    function transfer(
        address to,
        uint256 amount
    ) public override returns (bool) {
        uint256 fees = amount * fee / 10_000;
        super.transfer(address(0), fees);
        return super.transfer(to, amount - fees);
    }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public override returns (bool) {
        uint256 fees = amount * fee / 10_000;
        super.transferFrom(from, address(0), fees);
        return super.transferFrom(from, to, amount - fees);
    }
}

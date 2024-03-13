/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";

contract ERC20WithFees is MockERC20 {
    constructor(
        string memory name_,
        string memory symbol_,
        uint8 decimals_
    ) MockERC20(name_, symbol_, decimals_) { }

    function transfer(
        address to,
        uint256 amount
    ) public override returns (bool) {
        uint256 fee = amount / 50;
        super.transfer(address(0), fee);
        return super.transfer(to, amount - fee);
    }

    function transferFrom(
        address from,
        address to,
        uint256 amount
    ) public override returns (bool) {
        uint256 fee = amount / 50;
        super.transferFrom(from, address(0), fee);
        return super.transferFrom(from, to, amount - fee);
    }
}

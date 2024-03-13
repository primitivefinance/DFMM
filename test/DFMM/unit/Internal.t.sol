/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/DFMM.sol";

contract DFMMInternal is DFMM {
    constructor() DFMM(address(0)) { }

    function transferFrom(address token, uint256 amount) external payable {
        _transferFrom(token, amount);
    }
}

contract DFMMInternalTest is Test {
    DFMMInternal dfmm;

    function setUp() public {
        dfmm = new DFMMInternal();
    }
}

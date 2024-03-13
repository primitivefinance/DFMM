/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "src/LPToken.sol";

contract LPTokenSetUp is Test {
    LPToken lpToken;

    function setUp() public {
        lpToken = new LPToken();
    }
}

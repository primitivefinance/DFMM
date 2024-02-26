// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import "src/test/helpers/SetUp.sol";
import "src/test/helpers/MockStrategy.sol";

contract DFMMSetUp is SetUp {
    MockStrategy strategy;
    MockERC20 tokenX;
    MockERC20 tokenY;

    uint256 public POOL_ID;

    function setUp() public override {
        SetUp.setUp();
        strategy = new MockStrategy(address(dfmm));
        tokenX = new MockERC20("tokenX", "X", 18);
        tokenY = new MockERC20("tokenY", "Y", 18);
    }

    modifier init() {
        IDFMM.InitParams memory params = IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: abi.encode(uint256(2))
        });

        (POOL_ID,,,) = dfmm.init(params);
        _;
    }
}

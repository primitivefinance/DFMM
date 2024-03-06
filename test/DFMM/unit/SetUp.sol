// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { SetUp, IDFMM, DFMM } from "test/utils/SetUp.sol";
import { MockStrategy } from "test/utils/MockStrategy.sol";

contract DFMMSetUp is SetUp {
    MockStrategy strategy;

    uint256 public POOL_ID;

    function setUp() public override {
        SetUp.setUp();
        strategy = new MockStrategy(address(dfmm));
    }

    function getDefaultPoolParams(bytes memory data)
        internal
        view
        returns (IDFMM.InitParams memory)
    {
        return IDFMM.InitParams({
            strategy: address(strategy),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: data
        });
    }

    modifier initPool() {
        bytes memory params = abi.encode(
            true,
            int256(1 ether),
            uint256(1 ether),
            uint256(1 ether),
            uint256(1 ether)
        );
        (POOL_ID,,,) = dfmm.init(getDefaultPoolParams(params));
        _;
    }
}

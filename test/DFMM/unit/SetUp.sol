// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { SetUp, IDFMM, DFMM } from "test/utils/SetUp.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";
import { MockStrategy } from "test/utils/MockStrategy.sol";

contract DFMMSetUp is SetUp {
    MockStrategy strategy;

    uint256 public POOL_ID;

    function setUp() public virtual override {
        SetUp.setUp();
        strategy = new MockStrategy(address(dfmm));
    }

    function getDefaultPoolParams(bytes memory data)
        internal
        view
        returns (InitParams memory)
    {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        return InitParams({
            name: "Default Pool",
            symbol: "POOL",
            strategy: address(strategy),
            tokens: tokens,
            data: data,
            feeCollector: address(0),
            controllerFee: 0
        });
    }

    modifier initPool() {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 1 ether;
        reserves[1] = 1 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(1 ether));
        (POOL_ID,,) = dfmm.init(getDefaultPoolParams(params));
        _;
    }

    modifier initWETHPool() {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 1 ether;
        reserves[1] = 1 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(1 ether));

        address[] memory tokens = new address[](2);
        tokens[0] = address(weth);
        tokens[1] = address(tokenY);

        (POOL_ID,,) = dfmm.init(
            InitParams({
                name: "Default Pool",
                symbol: "POOL",
                strategy: address(strategy),
                tokens: tokens,
                data: params,
                feeCollector: address(0),
                controllerFee: 0
            })
        );
        _;
    }
}

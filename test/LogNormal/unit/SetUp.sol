// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/LogNormal/LogNormal.sol";
import "src/LogNormal/LogNormalSolver.sol";
import "test/utils/SetUp.sol";
import { ONE, TWO } from "src/lib/StrategyLib.sol";

contract LogNormalSetUp is SetUp {
    LogNormal logNormal;
    LogNormalSolver solver;

    uint256 public POOL_ID;

    LogNormalParams defaultParams = LogNormalParams({
        mean: ONE,
        width: ONE,
        swapFee: TEST_SWAP_FEE,
        controller: address(this)
    });

    uint256 defaultReserveX = ONE;
    uint256 defaultPrice = ONE;
    bytes defaultInitialPoolData =
        computeInitialPoolData(defaultReserveX, defaultPrice, defaultParams);

    function setUp() public override {
        SetUp.setUp();
        logNormal = new LogNormal(address(dfmm));
        solver = new LogNormalSolver(address(logNormal));
    }

    modifier init() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM2.InitParams memory defaultInitParams = IDFMM2.InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: defaultInitialPoolData
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }

    modifier initRealistic() {
        vm.warp(0);

        LogNormalParams memory params = LogNormalParams({
            mean: 0,
            width: 0,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM2.InitParams memory defaultInitParams = IDFMM2.InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: computeInitialPoolData(1 ether, 2500 ether, params)
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

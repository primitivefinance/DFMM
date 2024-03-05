// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/LogNormal/LogNormal.sol";
import "src/LogNormal/LogNormalSolver.sol";
import "test/utils/SetUp.sol";

contract LogNormalSetUp is SetUp {
    LogNormal logNormal;
    LogNormalSolver solver;

    uint256 public POOL_ID;

    LogNormal.LogNormalParams defaultParams = LogNormal.LogNormalParams({
        strike: ONE,
        sigma: ONE,
        tau: ONE,
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

        IDFMM.InitParams memory defaultInitParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: defaultInitialPoolData
        });

        (POOL_ID,,,) = dfmm.init(defaultInitParams);

        _;
    }

    modifier initRealistic() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: 2500 ether,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        IDFMM.InitParams memory defaultInitParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: computeInitialPoolData(1 ether, 2500 ether, params)
        });

        (POOL_ID,,,) = dfmm.init(defaultInitParams);

        _;
    }
}

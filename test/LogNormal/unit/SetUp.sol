// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/LogNormal/LogNormal.sol";
import "src/LogNormal/LogNormalSolver.sol";
import "test/utils/SetUp.sol";
import { ONE } from "src/lib/StrategyLib.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";

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

    LogNormalParams defaultParamsDeep = LogNormalParams({
        mean: ONE,
        width: 0.25 ether,
        swapFee: TEST_SWAP_FEE,
        controller: address(this)
    });

    uint256 defaultReserveX = 100 ether;
    uint256 defaultReserveXDeep = ONE * 10_000_000;

    uint256 defaultPrice = ONE;
    bytes defaultInitialPoolData =
        computeInitialPoolData(defaultReserveX, defaultPrice, defaultParams);

    bytes defaultInitialPoolDataDeep = computeInitialPoolData(
        defaultReserveXDeep, defaultPrice, defaultParamsDeep
    );

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

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: defaultInitialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }

    modifier deep() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory defaultInitParamsDeep = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: defaultInitialPoolDataDeep,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParamsDeep);

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

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: computeInitialPoolData(1 ether, 2500 ether, params),
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

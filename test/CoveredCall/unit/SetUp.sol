// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/CoveredCall/CoveredCall.sol";
import "src/CoveredCall/CoveredCallSolver.sol";
import "test/utils/SetUp.sol";
import { ONE } from "src/lib/StrategyLib.sol";
import { YEAR } from "src/CoveredCall/CoveredCallMath.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";
import "forge-std/console2.sol";

contract CoveredCallSetUp is SetUp {
    CoveredCall coveredCall;
    CoveredCallSolver solver;

    uint256 public POOL_ID;

    CoveredCallParams defaultParams = CoveredCallParams({
        mean: ONE,
        width: 0.1 ether,
        maturity: YEAR,
        swapFee: TEST_SWAP_FEE,
        timestamp: block.timestamp,
        controller: address(this)
    });

    CoveredCallParams defaultParamsDeep = CoveredCallParams({
        mean: ONE,
        width: 0.25 ether,
        maturity: YEAR,
        swapFee: TEST_SWAP_FEE,
        timestamp: block.timestamp,
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
        coveredCall = new CoveredCall(address(dfmm));
        solver = new CoveredCallSolver(address(coveredCall));
    }

    modifier init() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
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
            strategy: address(coveredCall),
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

        CoveredCallParams memory params = CoveredCallParams({
            mean: 0,
            width: 0,
            maturity: YEAR,
            swapFee: TEST_SWAP_FEE,
            timestamp: block.timestamp,
            controller: address(this)
        });

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: computeInitialPoolData(1 ether, 2500 ether, params),
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

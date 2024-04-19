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
    uint256 public constant FEE = 0.00001 ether;

    uint256 defaultReserveX = 100 ether;
    uint256 defaultReserveXMil = 1_000_000 ether;
    uint256 defaultReserveXDeep = ONE * 10_000_000;

    uint256 defaultPrice = ONE;
    uint256 defaultPricePoint9Rate = 0.84167999326 ether;

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

        CoveredCallParams memory defaultParams = CoveredCallParams({
            mean: ONE,
            width: 0.1 ether,
            maturity: YEAR,
            swapFee: TEST_SWAP_FEE,
            lastTimestamp: block.timestamp,
            controller: address(this)
        });

        bytes memory initialPoolData = solver.getInitialPoolDataGivenX(
            defaultReserveX, defaultPrice, defaultParams
        );

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }

    modifier init_mil() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        CoveredCallParams memory defaultParamsMil = CoveredCallParams({
            mean: ONE,
            width: 0.05 ether,
            maturity: YEAR * 2,
            swapFee: FEE,
            lastTimestamp: block.timestamp,
            controller: address(this)
        });

        bytes memory initialPoolData = solver.getInitialPoolDataGivenY(
            defaultReserveXMil, defaultPricePoint9Rate, defaultParamsMil
        );

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);
        int256 invariant = solver.getInvariant(POOL_ID);
        console2.log("Invariant at init: {}", invariant);

        _;
    }

    modifier init_no_fee() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        CoveredCallParams memory defaultParamsFeeless = CoveredCallParams({
            mean: ONE,
            width: 0.00001 ether,
            maturity: YEAR,
            swapFee: 0,
            lastTimestamp: block.timestamp,
            controller: address(this)
        });

        bytes memory initialPoolData = solver.getInitialPoolDataGivenX(
            defaultReserveX, defaultPrice, defaultParamsFeeless
        );

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }

    modifier init_quarterly() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        CoveredCallParams memory defaultParamsQuarterly = CoveredCallParams({
            mean: ONE,
            width: 0.1 ether,
            maturity: YEAR / 4,
            swapFee: TEST_SWAP_FEE,
            lastTimestamp: block.timestamp,
            controller: address(this)
        });

        bytes memory initialPoolData = solver.getInitialPoolDataGivenX(
            defaultReserveX, defaultPrice, defaultParamsQuarterly
        );

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
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

        CoveredCallParams memory defaultParamsDeep = CoveredCallParams({
            mean: ONE,
            width: 0.25 ether,
            maturity: YEAR,
            swapFee: TEST_SWAP_FEE,
            lastTimestamp: block.timestamp,
            controller: address(this)
        });

        bytes memory initialPoolData = solver.getInitialPoolDataGivenX(
            defaultReserveXDeep, defaultPrice, defaultParamsDeep
        );

        InitParams memory defaultInitParamsDeep = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
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
            lastTimestamp: block.timestamp,
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

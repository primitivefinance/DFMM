// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {
    GeometricMean,
    GeometricMeanParams
} from "src/GeometricMean/GeometricMean.sol";
import { GeometricMeanSolver } from "src/GeometricMean/GeometricMeanSolver.sol";
import "test/utils/SetUp.sol";
import { computeInitialPoolData } from "src/GeometricMean/G3MMath.sol";
import "solmate/utils/FixedPointMathLib.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";

using FixedPointMathLib for uint256;

contract G3MSetUp is SetUp {
    GeometricMean g3m;
    GeometricMeanSolver solver;

    uint256 public POOL_ID;

    GeometricMeanParams defaultParams = GeometricMeanParams({
        wX: 0.5 ether,
        wY: 0.5 ether,
        swapFee: TEST_SWAP_FEE,
        controller: address(this)
    });
    uint256 defaultReserveX = 1 ether;
    uint256 defaultStrikePrice = 1 ether;
    bytes defaultInitialPoolData = computeInitialPoolData(
        defaultReserveX, defaultStrikePrice, defaultParams
    );

    function setUp() public override {
        SetUp.setUp();
        g3m = new GeometricMean(address(dfmm));
        solver = new GeometricMeanSolver(address(g3m));
    }

    modifier init() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(g3m),
            tokens: tokens,
            data: defaultInitialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

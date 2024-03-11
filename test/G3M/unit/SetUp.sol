// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {
    GeometricMean2,
    GeometricMeanParams
} from "src/GeometricMean/GeometricMean2.sol";
import { GeometricMeanSolver } from "src/GeometricMean/GeometricMeanSolver.sol";
import "test/utils/SetUp.sol";
import { computeInitialPoolData } from "src/GeometricMean/G3MUtils.sol";
import "solmate/utils/FixedPointMathLib.sol";

using FixedPointMathLib for uint256;

contract G3MSetUp is SetUp {
    GeometricMean2 g3m;
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
        g3m = new GeometricMean2(address(dfmm));
        solver = new GeometricMeanSolver(address(g3m));
    }

    modifier init() {
        vm.warp(0);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM2.InitParams memory defaultInitParams = IDFMM2.InitParams({
            name: "",
            symbol: "",
            strategy: address(g3m),
            tokens: tokens,
            data: defaultInitialPoolData
        });

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

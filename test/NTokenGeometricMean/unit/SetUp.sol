// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { SetUp, MockERC20 } from "test/utils/SetUp.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";
import {
    NTokenGeometricMean,
    NTokenGeometricMeanParams
} from "src/NTokenGeometricMean/NTokenGeometricMean.sol";
import { NTokenGeometricMeanSolver } from
    "src/NTokenGeometricMean/NTokenGeometricMeanSolver.sol";
import { computeInitialPoolData } from
    "src/NTokenGeometricMean/NTokenGeometricMeanUtils.sol";

contract NTokenGeometricMeanSetUp is SetUp {
    NTokenGeometricMean strategy;
    NTokenGeometricMeanSolver solver;

    MockERC20[] public tokens;
    uint256 public POOL_ID;

    function setUp() public override {
        SetUp.setUp();
        strategy = new NTokenGeometricMean(address(dfmm));
        solver = new NTokenGeometricMeanSolver(address(strategy));

        for (uint256 i = 0; i < 8; i++) {
            tokens.push(new MockERC20("", "", 18));
            tokens[i].mint(address(this), 1_000_000 ether);
            tokens[i].approve(address(dfmm), type(uint256).max);
        }
    }

    function _prepareInitParams(
        address[] memory initTokens,
        uint256[] memory prices,
        uint256[] memory weights,
        uint256 swapFee,
        address controller
    ) internal view returns (InitParams memory) {
        NTokenGeometricMeanParams memory poolParams = NTokenGeometricMeanParams({
            weights: weights,
            swapFee: swapFee,
            controller: controller
        });

        bytes memory initPoolData =
            computeInitialPoolData(1 ether, prices, poolParams);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(strategy),
            tokens: initTokens,
            data: initPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });

        return initParams;
    }

    modifier initDefaultPool() {
        address[] memory initTokens = new address[](4);
        uint256[] memory weights = new uint256[](4);
        uint256[] memory prices = new uint256[](4);

        for (uint256 i = 0; i < 4; i++) {
            initTokens[i] = address(tokens[i]);
            weights[i] = 1 ether / 4;
            prices[i] = 1 ether;
        }

        InitParams memory defaultInitParams = _prepareInitParams(
            initTokens, prices, weights, TEST_SWAP_FEE, address(this)
        );

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

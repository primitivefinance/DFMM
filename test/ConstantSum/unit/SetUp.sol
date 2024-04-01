// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/ConstantSum/ConstantSum.sol";
import "src/ConstantSum/ConstantSumSolver.sol";
import "test/utils/SetUp.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";

contract ConstantSumSetUp is SetUp {
    ConstantSum constantSum;
    ConstantSumSolver solver;

    uint256 public POOL_ID;

    ConstantSumParams defaultParams = ConstantSumParams({
        price: 2 ether,
        swapFee: TEST_SWAP_FEE,
        controller: address(0)
    });

    ConstantSumParams zeroFeeParams = ConstantSumParams({
        price: 2 ether,
        swapFee: 0,
        controller: address(0)
    });

    function setUp() public override {
        SetUp.setUp();
        constantSum = new ConstantSum(address(dfmm));
        solver = new ConstantSumSolver(address(constantSum));
    }

    modifier defaultPool() {
        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, defaultParams);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);

        _;
    }

    modifier zeroFeePool() {
        uint256 reserveX = 1 ether;
        uint256 reserveY = 1 ether;

        bytes memory initData =
            solver.getInitialPoolData(reserveX, reserveY, zeroFeeParams);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);

        _;
    }
}

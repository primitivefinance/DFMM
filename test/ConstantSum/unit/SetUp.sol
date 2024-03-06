// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/ConstantSum/ConstantSum.sol";
import "src/ConstantSum/ConstantSumSolver.sol";
import "test/utils/SetUp.sol";

contract ConstantSumSetUp is SetUp {
    ConstantSum constantSum;
    ConstantSumSolver solver;

    uint256 public POOL_ID;

    ConstantSum.ConstantSumParams defaultParams = ConstantSum.ConstantSumParams({
        price: 2 ether,
        swapFee: TEST_SWAP_FEE,
        controller: address(0)
    });

    ConstantSum.ConstantSumParams zeroFeeParams = ConstantSum.ConstantSumParams({
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

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(constantSum),
            tokenX: address(tokenX),
            tokenY: address(tokenY),
            data: initData
        });

        (POOL_ID,,,) = dfmm.init(initParams);

        _;
    }
}
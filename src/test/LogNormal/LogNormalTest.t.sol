// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/DFMM.sol";
import "src/LogNormal/LogNormal.sol";
import "src/LogNormal/LogNormalSolver.sol";
import "../helpers/AtomicV2.sol";
import "../helpers/Lex.sol";

contract LogNormalTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    LogNormal logNormal;
    LogNormalSolver solver;
    address tokenX;
    address tokenY;
    Lex lex;
    AtomicV2 atomic;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100_000_000_000 ether);
        MockERC20(tokenY).mint(address(this), 100_000_000_000 ether);

        lex = new Lex(tokenX, tokenY, ONE);
        dfmm = new DFMM(address(0));
        logNormal = new LogNormal(address(dfmm));
        solver = new LogNormalSolver(address(logNormal));
        atomic = new AtomicV2(address(solver), address(dfmm), address(lex), tokenX, tokenY);
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenX).approve(address(atomic), type(uint256).max);
        MockERC20(tokenY).approve(address(atomic), type(uint256).max);
    }

    modifier realisticEth() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: ONE * 2300,
            sigma: ONE,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = ONE * 2345;
        uint256 init_x = ONE * 10;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: TWO,
            sigma: 0.25 ether,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = TWO;
        uint256 init_x = ONE;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    modifier deep_liq() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: ONE,
            sigma: 0.1 ether,
            tau: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = 1.1 ether;
        uint256 init_x = ONE * 100_000_000;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    modifier infinity_case() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: ONE,
            sigma: 100000000000000,
            tau: ONE,
            swapFee: 100000000000000,
            controller: address(0)
        });
        uint256 init_p = ONE;
        uint256 init_x = ONE * 100;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }


    modifier oob_scenario() {
        vm.warp(0);

        LogNormal.LogNormalParams memory params = LogNormal.LogNormalParams({
            strike: ONE,
            sigma: 100_000_000_000_000,
            tau: ONE,
            swapFee: 10000000000000,
            controller: address(0)
        });
        uint256 init_p = 1 ether;
        uint256 init_x = 100 ether;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(logNormal),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);

        _;
    }

    function test_ln_swap_x_in() public basic {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

        dfmm.swap(poolId, swapData);
    }

    function test_ln_swap_y_in() public basic {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

        dfmm.swap(poolId, swapData);
    }

    // todo: write assertApproxEq
    function test_price_formulas() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        (uint256 rx, uint256 ry, uint256 L) =
            solver.getReservesAndLiquidity(poolId);
        uint256 priceGivenY = solver.getPriceGivenYL(poolId, ry, L);
        uint256 priceGivenX = solver.getPriceGivenXL(poolId, rx, L);
        assertApproxEqAbs(priceGivenY, priceGivenX, 100);
    }

    function test_oob_diff_raise() public oob_scenario {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffRaised = solver.calculateDiffRaise(
            poolId, 868046956976567012, 868046956976577012
        );

        console2.log(diffRaised);
    }

    function test_bisect_oob_error() public oob_scenario {
        bool xIn = true;
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountIn = 97506664135616951612;
        (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

        dfmm.swap(poolId, swapData);

        (uint256 rx, uint256 ry, uint256 L) = solver.getReservesAndLiquidity(poolId);
        console2.log(rx);
        console2.log(ry);
        uint256 internalPrice = solver.internalPrice(poolId);
        console2.log(internalPrice);
    }

    function test_ln_diff_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffLowered = solver.calculateDiffLower(
            poolId, 1.9 ether, 171_917_954_632_821_596
        );

        console2.log(diffLowered);
    }

    function test_ln_diff_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffLowered = solver.calculateDiffRaise(
            poolId, 2.1 ether, 326_118_838_819_816_034
        );

        console2.log(diffLowered);
    }

    function test_ln_optimal_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalLower = solver.computeOptimalArbLowerPrice(
            poolId, 1.9 ether, 0.181424 ether
        );

        console2.log(optimalLower);
    }

    function test_ln_optimal_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalRaise = solver.computeOptimalArbRaisePrice(
            poolId, 2.1 ether, 0.345156 ether
        );

        console2.log(optimalRaise);
    }

    function test_atomic_events() public basic {
      atomic.logData(dfmm.nonce() - 1);
    }

    function test_complete_arb_flow_dy() public basic {
      uint256 poolId = dfmm.nonce() - 1;
      uint256 S = 2.1 ether;
      int256 dy = solver.getDyGivenS(poolId, S);

      uint256 optimalRaise = solver.computeOptimalArbRaisePrice(poolId, S, uint256(dy));

      console2.log(optimalRaise);
    }

    function test_complete_arb_flow_dx() public basic {
      uint256 poolId = dfmm.nonce() - 1;
      uint256 S = 1.9 ether;
      int256 dx = solver.getDxGivenS(poolId, S);

      uint256 optimalLower = solver.computeOptimalArbLowerPrice(poolId, S, uint256(dx));

      console2.log(optimalLower);
    }


    function test_compute_dy() public basic {
      uint256 poolId = dfmm.nonce() - 1;
      int256 dy = solver.getDyGivenS(poolId, 2.1 ether);

      console2.log(dy);
    }

    function test_compute_dx() public basic {
      uint256 poolId = dfmm.nonce() - 1;
      int256 dx = solver.getDxGivenS(poolId, 1.9 ether);

      console2.log(dx);
    }

    function test_deep_liq_x_in() public deep_liq {
      bool xIn = true;
      uint256 amountIn = 1 ether;
      uint256 poolId = dfmm.nonce() - 1;
      (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

      (uint256 input, uint256 output) = dfmm.swap(poolId, swapData);

      console2.log(input);
      console2.log(output);
    }

    function test_deep_liq_y_in() public deep_liq {
      bool xIn = false;
      uint256 amountIn = 1 ether;
      uint256 poolId = dfmm.nonce() - 1;
      (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);

      (uint256 input, uint256 output) = dfmm.swap(poolId, swapData);

      console2.log(input);
      console2.log(output);
    }

    function test_inf_scenario() public infinity_case {
      uint256 poolId = dfmm.nonce() - 1;
      bool xIn = true;
      uint256 amountIn = 93184003566910952224;
      (,,, bytes memory swapData) = solver.simulateSwap(poolId, xIn, amountIn);
      dfmm.swap(poolId, swapData);

      (uint256 rx, uint256 ry, uint256 L) = solver.getReservesAndLiquidity(poolId);
      console2.log(rx);
      console2.log(ry);
      console2.log(L);

      int256 raise = solver.calculateDiffRaise(poolId, 1000284890278334600, 193204839774776442304);

      console2.log(raise);
    }

    function test_ierfc_input() public basic {
       int256 res = solver.callIerfc(1999999999999999999);
       console2.log(res);
    }

    function test_solver_tf() public basic {
      
    }

}

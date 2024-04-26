// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/DFMM.sol";
import "src/LogNormal/LogNormal.sol";
import "src/LogNormal/LogNormalSolver.sol";
import { ONE, TWO } from "src/lib/StrategyLib.sol";
import {
    computeNextLiquidity,
    computeLGivenX,
    computeYGivenL,
    computeTradingFunction
} from "src/LogNormal/LogNormalMath.sol";

contract LogNormalTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    LogNormal logNormal;
    LogNormalSolver solver;
    address tokenX;
    address tokenY;

    uint256 POOL_ID;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100_000_000 ether);
        MockERC20(tokenY).mint(address(this), 100_000_000 ether);

        dfmm = new DFMM(address(0));
        logNormal = new LogNormal(address(dfmm));
        solver = new LogNormalSolver(IStrategy(logNormal));
        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    modifier realisticEth() {
        vm.warp(0);

        LogNormalParams memory params = LogNormalParams({
            mean: ONE,
            width: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = ONE * 2345;
        uint256 init_x = ONE * 10;
        bytes memory initData = solver.prepareInit(init_x, init_p, params);

        address[] memory tokens = new address[](2);
        tokens[0] = tokenX;
        tokens[1] = tokenY;

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);

        LogNormalParams memory params = LogNormalParams({
            mean: ONE,
            width: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = TWO;
        uint256 init_x = ONE;
        bytes memory initData = solver.prepareInit(init_x, init_p, params);

        address[] memory tokens = new address[](2);
        tokens[0] = tokenX;
        tokens[1] = tokenY;

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        _;
    }

    modifier revert_scenario() {
        vm.warp(0);

        LogNormalParams memory params = LogNormalParams({
            mean: 0.67323818941934077 ether,
            width: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = 1_329_956_352_651_532_999;
        uint256 init_x = 70.658087306013359413 ether;
        bytes memory initData = solver.prepareInit(init_x, init_p, params);

        address[] memory tokens = new address[](2);
        tokens[0] = tokenX;
        tokens[1] = tokenY;

        InitParams memory initParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(logNormal),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        dfmm.init(initParams);

        _;
    }

    /*

    function test_inital_pool_bisection() public {
        vm.warp(0);

        LogNormalParams memory params = LogNormalParams({
            mean: ONE,
            width: ONE,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 approxL = computeLGivenX(ONE, TWO, params);
        uint256 ry = computeYGivenL(approxL, TWO, params);
        int256 invariant = computeTradingFunction(ONE, ry, approxL, params);
        (uint256 L, uint256 upperInput, uint256 lowerInput) = computeNextLiquidity2(ONE, ry, invariant, approxL, params);
        int256 invUpper = computeTradingFunction(ONE, ry, upperInput, params);
        int256 invLower = computeTradingFunction(ONE, ry, lowerInput, params);

        console2.log(L);
        console2.log("upper", invUpper);
        console2.log("lower", invLower);
    }
    */

    function test_ln_swap_x_in() public basic {
        uint256 amountIn = 0.1 ether;
        (,, bytes memory swapData) = solver.prepareSwap(POOL_ID, 0, 1, amountIn);

        dfmm.swap(POOL_ID, address(this), swapData);
    }

    function test_ln_swap_y_in() public basic {
        uint256 amountIn = 0.1 ether;
        (,, bytes memory swapData) = solver.prepareSwap(POOL_ID, 1, 0, amountIn);

        dfmm.swap(POOL_ID, address(this), swapData);
    }

    // todo: write assertApproxEq
    function test_price_formulas() public basic {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenX = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
        uint256 priceGivenY = solver.getPriceGivenYL(POOL_ID, reserves[1], L);
        assertApproxEqAbs(priceGivenY, priceGivenX, 100);
    }

    /*
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
    */

    // function test_internal_price() public basic {
    //     uint256 internalPrice = solver.internalPrice();

    //     console2.log(internalPrice);
    // }

    // function test_internal_price_post_y_in() public basic {
    //     uint256 internalPrice = solver.internalPrice();
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = false;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);

    //     uint256 postSwapInternalPrice = solver.internalPrice();

    //     assertGt(postSwapInternalPrice, internalPrice);
    // }

    // function test_internal_price_post_x_in() public basic {
    //     uint256 internalPrice = solver.internalPrice();
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = true;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);

    //     uint256 postSwapInternalPrice = solver.internalPrice();

    //     assertLt(postSwapInternalPrice, internalPrice);
    // }

    // function test_swap_eth_backtest() public realisticEth {
    //     uint256 amountIn = 0.1 ether;
    //     bool swapXIn = true;

    //     // Try doing simulate swap to see if we get a similar result.
    //     (bool valid,,, bytes memory payload) =
    //         solver.simulateSwap(swapXIn, amountIn);

    //     assertEq(valid, true);

    //     dfmm.swap(payload);
    // }

    // function test_allocate_multiple_times() public basic {
    //     uint256 amountX = 0.1 ether;
    //     (uint256 rx, uint256 ry, uint256 L) = solver.allocateGivenX(amountX);

    //     uint256 preBalance = dfmm.balanceOf(address(this));
    //     uint256 deltaLiquidity = L - dfmm.totalLiquidity();
    //     bytes memory data = abi.encode(rx, ry, L);
    //     dfmm.allocate(data);
    //     assertEq(preBalance + deltaLiquidity, dfmm.balanceOf(address(this)));

    //     (rx, ry, L) = solver.allocateGivenX(amountX * 2);
    //     deltaLiquidity = L - dfmm.totalLiquidity();
    //     data = abi.encode(rx, ry, L);

    //     MockERC20(tokenX).mint(address(0xbeef), rx);
    //     MockERC20(tokenY).mint(address(0xbeef), ry);

    //     vm.startPrank(address(0xbeef));
    //     MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
    //     MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    //     dfmm.allocate(data);
    //     assertEq(deltaLiquidity, dfmm.balanceOf(address(0xbeef)));
    //     vm.stopPrank();
    // }
}

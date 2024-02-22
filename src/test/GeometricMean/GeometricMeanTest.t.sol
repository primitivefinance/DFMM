// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "solmate/test/utils/mocks/MockERC20.sol";

import "src/GeometricMean/GeometricMean.sol";
import "src/GeometricMean/GeometricMeanSolver.sol";
import "src/DFMM.sol";
import "../helpers/Lex.sol";

contract GeometricMeanTest is Test {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;

    DFMM dfmm;
    GeometricMean g3m;
    GeometricMeanSolver solver;
    address tokenX;
    address tokenY;
    Lex lex;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = address(new MockERC20("tokenX", "X", 18));
        tokenY = address(new MockERC20("tokenY", "Y", 18));
        MockERC20(tokenX).mint(address(this), 100_000_000e18);
        MockERC20(tokenY).mint(address(this), 100_000_000e18);

        lex = new Lex(tokenX, tokenY, ONE);
        dfmm = new DFMM(address(0));
        g3m = new GeometricMean(address(dfmm));
        solver = new GeometricMeanSolver(address(g3m));

        MockERC20(tokenX).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenY).approve(address(dfmm), type(uint256).max);
    }

    function test_G3M_init_18() public {
        uint256 reserveX = 1 ether;
        uint256 price = 2000 * 10 ** 18;

        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: 0,
            controller: address(this)
        });

        dfmm.init(
            IDFMM.InitParams({
                strategy: address(g3m),
                tokenX: tokenX,
                tokenY: tokenY,
                data: computeInitialPoolData(reserveX, price, params)
            })
        );
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = 1 ether;
        uint256 init_x = 1 ether;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    /// @dev Initializes a basic pool in dfmm with 0 swapFee.
    modifier basic_feeless() {
        vm.warp(0);
        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: 0,
            controller: address(0)
        });
        uint256 init_p = 1 ether;
        uint256 init_x = 1 ether;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    /// @dev Initializes a basic pool in dfmm with very deep liquidity.
    modifier deep() {
        vm.warp(0);
        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });
        uint256 init_p = 1 ether;
        uint256 init_x = 1_000_000 ether;
        bytes memory initData =
            solver.getInitialPoolData(init_x, init_p, params);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            strategy: address(g3m),
            tokenX: tokenX,
            tokenY: tokenY,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    function test_g3m_swap() public basic {
        uint256 amountIn = 10 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, true, amountIn);
    }

    function test_g3m_swap_x_in_no_fee() public basic_feeless {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 90_909_090_909_090_907);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 909_090_909_090_909_093);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_g3m_swap_y_in_no_fee() public basic_feeless {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 90_909_090_909_090_907);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 909_090_909_090_909_093);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_g3m_swap_x_in_with_fee() public basic {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 90_636_343_181_818_178);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 909_363_656_818_181_822);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.00015 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_g3m_swap_y_in_with_fee() public basic {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertEq(amountOut, 90_636_343_181_818_178);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 909_363_656_818_181_822);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.00015 ether);

        dfmm.swap(poolId, swapData);
    }

    function test_g3m_swap_x_in_deep() public deep {
        bool xIn = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertApproxEqRel(amountOut, 0.0997 ether, 1e11);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1_000_000.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertApproxEqRel(endReservesRy, 999_999.9003 ether, 1e10);

        console2.log("endReservesL: ", endReservesL);
        assertApproxEqRel(endReservesL, 1_000_000.00015 ether, 1e10);

        dfmm.swap(poolId, swapData);
    }

    function test_g3m_swap_y_in_deep() public deep {
        bool xIn = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, xIn, amountIn);

        console2.log("Valid: ", valid);
        assert(valid);

        console2.log("AmountOut: ", amountOut);
        assertApproxEqRel(amountOut, 0.0997 ether, 1e11);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertApproxEqRel(endReservesRx, 999_999.9003 ether, 1e10);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1_000_000.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertApproxEqRel(endReservesL, 1_000_000.00015 ether, 1e10);

        dfmm.swap(poolId, swapData);
    }

    function test_diff_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffLowered =
            solver.calculateDiffLower(poolId, 0.8 ether, 0.114674 ether);

        console2.log(diffLowered);
    }

    function test_diff_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diffRaised =
            solver.calculateDiffRaise(poolId, 1.2 ether, 0.0921529 ether);

        console2.log(diffRaised);
    }

    function test_optimal_raise() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        int256 diff_min = solver.calculateDiffRaise(poolId, 1.2 ether, 1000);
        int256 diff_max =
            solver.calculateDiffRaise(poolId, 1.2 ether, 0.0954451 ether);
        console2.log("min", diff_min);
        console2.log("max", diff_max);
        uint256 optimalRaise = solver.computeOptimalArbRaisePrice(
            poolId, 1.2 ether, 0.0954451 ether
        );

        (bool valid, uint256 amountOut, uint256 price, bytes memory swapData) =
            solver.simulateSwap(poolId, true, optimalRaise);

        console2.log(valid);
        dfmm.swap(poolId, swapData);
    }

    function test_optimal_lower() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalLower = solver.computeOptimalArbLowerPrice(
            poolId, 0.8 ether, 0.134674 ether
        );

        console2.log(optimalLower);
    }

    function test_optimal_lower_profit() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalLower = solver.computeOptimalArbLowerPrice(
            poolId, 0.98 ether, 0.234674 ether
        );

        (, uint256 amountOut,,) =
            solver.simulateSwap(poolId, true, optimalLower);

        uint256 valueIn = optimalLower.mulWadDown(0.98 ether);
        uint256 valueOut = amountOut;
        uint256 profit = valueOut - valueIn;

        uint256 marginalIncrease = optimalLower + 100_000_000;
        uint256 marginalDecrease = optimalLower - 100_000_000;

        (, uint256 outIncrease,,) =
            solver.simulateSwap(poolId, true, marginalIncrease);

        uint256 valueInIncrease = marginalIncrease.mulWadDown(0.98 ether);
        uint256 valueOutIncrease = outIncrease;
        uint256 profitIncrease = valueOutIncrease - valueInIncrease;

        (, uint256 outDecrease,,) =
            solver.simulateSwap(poolId, true, marginalDecrease);

        uint256 valueInDecrease = marginalDecrease.mulWadDown(0.98 ether);
        uint256 valueOutDecrease = outDecrease;
        uint256 profitDecrease = valueOutDecrease - valueInDecrease;

        console2.log(profitIncrease);
        console2.log(profit);
        console2.log(profitDecrease);

        assertGt(profit, profitIncrease);
        assertGt(profit, profitDecrease);
    }

    function test_optimal_raise_profit() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 optimalRaise = solver.computeOptimalArbRaisePrice(
            poolId, 1.2 ether, 0.0954451 ether
        );

        (, uint256 amountOut,,) =
            solver.simulateSwap(poolId, false, optimalRaise);

        uint256 valueIn = optimalRaise;
        uint256 valueOut = amountOut.mulWadDown(1.2 ether);
        uint256 profit = valueOut - valueIn;

        uint256 marginalIncrease = optimalRaise + 1_000_000_000;
        uint256 marginalDecrease = optimalRaise - 1_000_000_000;

        (, uint256 outIncrease,,) =
            solver.simulateSwap(poolId, false, marginalIncrease);

        uint256 valueInIncrease = marginalIncrease;
        uint256 valueOutIncrease = outIncrease.mulWadDown(1.2 ether);
        uint256 profitIncrease = valueOutIncrease - valueInIncrease;

        (, uint256 outDecrease,,) =
            solver.simulateSwap(poolId, false, marginalDecrease);

        uint256 valueInDecrease = marginalDecrease;
        uint256 valueOutDecrease = outDecrease.mulWadDown(1.2 ether);
        uint256 profitDecrease = valueOutDecrease - valueInDecrease;

        console2.log(profitIncrease);
        console2.log(profit);
        console2.log(profitDecrease);

        assertGt(profit, profitIncrease);
        assertGt(profit, profitDecrease);
    }
}

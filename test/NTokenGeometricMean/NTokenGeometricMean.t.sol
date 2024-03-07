// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "solmate/test/utils/mocks/MockERC20.sol";

import "src/NTokenGeometricMean/NTokenGeometricMean.sol";
import "src/NTokenGeometricMean/NTokenGeometricMeanSolver.sol";
import "src/DFMM2.sol";

contract NTokenGeometricMeanTest is Test {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;

    DFMM2 dfmm;
    NTokenGeometricMean g3m;
    NTokenGeometricMeanSolver solver;
    address tokenA;
    address tokenB;
    address tokenC;
    address tokenD;

    address[] sTokens;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenA = address(new MockERC20("tokenA", "A", 18));
        tokenB = address(new MockERC20("tokenB", "B", 18));
        tokenC = address(new MockERC20("tokenC", "C", 18));
        tokenD = address(new MockERC20("tokenD", "D", 18));

        sTokens.push(tokenA);
        sTokens.push(tokenB);
        sTokens.push(tokenC);
        sTokens.push(tokenD);

        MockERC20(tokenA).mint(address(this), 100_000_000e18);
        MockERC20(tokenB).mint(address(this), 100_000_000e18);
        MockERC20(tokenC).mint(address(this), 100_000_000e18);
        MockERC20(tokenD).mint(address(this), 100_000_000e18);

        dfmm = new DFMM2(address(0));
        g3m = new NTokenGeometricMean(address(dfmm));
        solver = new NTokenGeometricMeanSolver(address(g3m));

        MockERC20(tokenA).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenB).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenC).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenD).approve(address(dfmm), type(uint256).max);
    }

    function test_4_token_init() public {
        uint256 reserveX = 1 ether;
        uint256 price = ONE;
        uint256 w = 0.25 ether;
        uint256[] memory weights = new uint256[](4);
        uint256[] memory prices = new uint256[](4);
        address[] memory tokens = new address[](4);

        for (uint256 i = 0; i < 4; i++) {
          weights[i] = w;
          prices[i] = price;
          tokens[i] = sTokens[i];
        }

        NTokenGeometricMeanParams memory params = NTokenGeometricMeanParams({
            weights: weights,
            swapFee: 0,
            controller: address(this)
        });

        console2.log("addr of strategy", address(g3m));

        dfmm.init(
            DFMM2.InitParams({
                strategy: address(g3m),
                tokens: tokens,
                data: solver.computeInitialPoolData(ONE, prices, params)
            })
        );
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic() {
        vm.warp(0);
        uint256 reserveNumeraire = 1 ether;
        uint256 price = ONE;
        uint256 w = 0.25 ether;
        uint256[] memory weights = new uint256[](4);
        uint256[] memory prices = new uint256[](4);
        address[] memory tokens = new address[](4);

        for (uint256 i = 0; i < 4; i++) {
          weights[i] = w;
          prices[i] = price;
          tokens[i] = sTokens[i];
        }

        NTokenGeometricMeanParams memory params = NTokenGeometricMeanParams({
            weights: weights,
            swapFee: 0,
            controller: address(this)
        });

        bytes memory initData =
            solver.computeInitialPoolData(reserveNumeraire, prices, params);
        DFMM2.InitParams memory initParams = DFMM2.InitParams({
            strategy: address(g3m),
            tokens: tokens,
            data: initData
        });


        dfmm.init(initParams);
        _;
    }

    function getTokens() public view returns (address[] memory) {
        address[] memory tokens = new address[](sTokens.length);
        for (uint256 i = 0; i < sTokens.length; i++) {
          tokens[i] = sTokens[i];
        }
        return tokens;
    }

    function createTokenDeltas(uint256 delta) public view returns (uint256[] memory) {
        uint256[] memory deltas = new uint256[](sTokens.length);
        for (uint256 i = 0; i < sTokens.length; i++) {
          deltas[i] = delta;
        }
        return deltas;
    }


    function test_4_token_allocate_basic() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 maxTokenDelta = 100e18;
        uint256[] memory maxDeltas = createTokenDeltas(maxTokenDelta);
        uint256 deltaL = ONE;

        bytes memory data = abi.encode(maxDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) = dfmm.getReservesAndLiquidity(poolId);
        console2.log(preReserves[0]);
        console2.log(preL);

        dfmm.allocate(poolId, data);

        (uint256[] memory postReserves, uint256 postL) = dfmm.getReservesAndLiquidity(poolId);
        console2.log(postReserves[0]);
        console2.log(postL);

    }

    function test_4_token_deallocate_basic() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 minTokenDelta = 0.3 ether;
        uint256[] memory minDeltas = createTokenDeltas(minTokenDelta);
        uint256 deltaL = 0.6 ether;

        bytes memory data = abi.encode(minDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) = dfmm.getReservesAndLiquidity(poolId);
        console2.log(preReserves[0]);
        console2.log(preL);

        dfmm.deallocate(poolId, data);

        (uint256[] memory postReserves, uint256 postL) = dfmm.getReservesAndLiquidity(poolId);
        console2.log(postReserves[0]);
        console2.log(postL);

    }


    /*
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
    */
}

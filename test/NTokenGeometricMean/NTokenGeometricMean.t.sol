// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console2.sol";
import "solmate/test/utils/mocks/MockERC20.sol";

import "src/NTokenGeometricMean/NTokenGeometricMean.sol";
import "src/NTokenGeometricMean/NTokenGeometricMeanSolver.sol";
import "src/interfaces/IDFMM.sol";
import "src/DFMM.sol";

contract NTokenGeometricMeanTest is Test {
    using stdStorage for StdStorage;
    using FixedPointMathLib for uint256;

    DFMM dfmm;
    NTokenGeometricMean g3m;
    NTokenGeometricMeanSolver solver;
    address tokenA;
    address tokenB;
    address tokenC;
    address tokenD;

    address[] sTokens;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    uint256 POOL_ID;

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

        dfmm = new DFMM(address(0));
        g3m = new NTokenGeometricMean(address(dfmm));
        solver = new NTokenGeometricMeanSolver(address(g3m));

        MockERC20(tokenA).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenB).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenC).approve(address(dfmm), type(uint256).max);
        MockERC20(tokenD).approve(address(dfmm), type(uint256).max);
    }

    function test_4_token_init() public {
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

        (POOL_ID,,) = dfmm.init(
            InitParams({
                name: "4-token-LP",
                symbol: "4T",
                strategy: address(g3m),
                tokens: tokens,
                data: solver.getInitialPoolData(ONE * 10, prices, params),
                feeCollector: address(0),
                controllerFee: 0
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
            solver.getInitialPoolData(reserveNumeraire, prices, params);
        InitParams memory initParams = InitParams({
            name: "4-token-LP",
            symbol: "4T",
            strategy: address(g3m),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        _;
    }

    /// @dev Initializes a basic pool in dfmm.
    modifier basic_70_10_10_10() {
        vm.warp(0);
        uint256 reserveNumeraire = 2 ether;
        uint256 price = ONE;
        uint256 w = 0.25 ether;
        uint256[] memory weights = new uint256[](4);
        uint256[] memory prices = new uint256[](4);
        address[] memory tokens = new address[](4);

        for (uint256 i = 0; i < 4; i++) {
            prices[i] = price;
            tokens[i] = sTokens[i];
        }
        weights[0] = 0.7 ether;
        weights[1] = 0.1 ether;
        weights[2] = 0.1 ether;
        weights[3] = 0.1 ether;

        NTokenGeometricMeanParams memory params = NTokenGeometricMeanParams({
            weights: weights,
            swapFee: 0,
            controller: address(this)
        });

        bytes memory initData =
            solver.getInitialPoolData(reserveNumeraire, prices, params);
        InitParams memory initParams = InitParams({
            name: "4-token-LP",
            symbol: "4T",
            strategy: address(g3m),
            tokens: tokens,
            data: initData,
            feeCollector: address(0),
            controllerFee: 0
        });

        (POOL_ID,,) = dfmm.init(initParams);
        _;
    }

    function getTokens() public view returns (address[] memory) {
        address[] memory tokens = new address[](sTokens.length);
        for (uint256 i = 0; i < sTokens.length; i++) {
            tokens[i] = sTokens[i];
        }
        return tokens;
    }

    function createTokenDeltas(uint256 delta)
        public
        view
        returns (uint256[] memory)
    {
        uint256[] memory deltas = new uint256[](sTokens.length);
        for (uint256 i = 0; i < sTokens.length; i++) {
            deltas[i] = delta;
        }
        return deltas;
    }

    function test_4_token_allocate_basic() public basic {
        uint256 maxTokenDelta = 100e18;
        uint256[] memory maxDeltas = createTokenDeltas(maxTokenDelta);
        uint256 deltaL = ONE;

        bytes memory data = abi.encode(maxDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(preReserves[0]);
        console2.log(preL);

        dfmm.allocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(postReserves[0]);
        console2.log(postL);
    }

    function test_4_token_allocate_given_delta_t() public basic {
        (uint256[] memory dReserves, uint256 dLiquidity) =
            solver.getAllocationDeltasGivenDeltaT(POOL_ID, 1, ONE);

        bytes memory data = abi.encode(dReserves, dLiquidity);

        dfmm.allocate(POOL_ID, data);
    }

    function test_4_token_deallocate_given_delta_t() public basic {
        (uint256[] memory dReserves, uint256 dLiquidity) =
            solver.getAllocationDeltasGivenDeltaT(POOL_ID, 1, 0.5 ether);

        bytes memory data = abi.encode(dReserves, dLiquidity);

        dfmm.deallocate(POOL_ID, data);
    }

    function test_4_token_deallocate_basic() public basic {
        uint256 minTokenDelta = 0.3 ether;
        uint256[] memory minDeltas = createTokenDeltas(minTokenDelta);
        uint256 deltaL = 0.6 ether;

        bytes memory data = abi.encode(minDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(preReserves[0]);
        console2.log(preL);

        dfmm.deallocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(postReserves[0]);
        console2.log(postL);
    }

    function test_4_token_simulate_swap() public basic {
        uint256 amountIn = 0.1 ether;
        uint256 tokenInIndex = 0;
        uint256 tokenOutIndex = 1;

        (bool valid, uint256 amountOut, bytes memory data) =
            solver.simulateSwap(POOL_ID, tokenInIndex, tokenOutIndex, amountIn);
        console2.log("amountOut", amountOut);
        console2.log("valid", valid);
        dfmm.swap(POOL_ID, address(this), data);
    }

    function test_4_token_compute_price() public basic {
        uint256 tIndex = 0;

        (uint256[] memory reserves,) = solver.getReservesAndLiquidity(POOL_ID);
        NTokenGeometricMeanParams memory params = solver.getPoolParams(POOL_ID);

        uint256 price = solver.computePriceOfToken(
            reserves[tIndex],
            reserves[reserves.length - 1],
            params.weights[tIndex],
            params.weights[reserves.length - 1]
        );
    }

    function test_4_token_allocate_basic_non_uniform()
        public
        basic_70_10_10_10
    {
        uint256 maxTokenDelta = 100e18;
        uint256[] memory maxDeltas = createTokenDeltas(maxTokenDelta);
        uint256 deltaL = ONE;

        bytes memory data = abi.encode(maxDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) =
            solver.getReservesAndLiquidity(POOL_ID);

        dfmm.allocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postL) =
            solver.getReservesAndLiquidity(POOL_ID);
    }

    function test_4_token_allocate_given_delta_t_non_uniform()
        public
        basic_70_10_10_10
    {
        (uint256[] memory dReserves, uint256 dLiquidity) =
            solver.getAllocationDeltasGivenDeltaT(POOL_ID, 1, ONE);

        console2.log(dReserves[0]);
        console2.log(dReserves[1]);
        console2.log(dReserves[2]);
        console2.log(dReserves[3]);

        bytes memory data = abi.encode(dReserves, dLiquidity);

        dfmm.allocate(POOL_ID, data);
    }

    function test_4_token_deallocate_given_delta_t_non_uniform()
        public
        basic_70_10_10_10
    {
        (uint256[] memory dReserves, uint256 dLiquidity) =
            solver.getAllocationDeltasGivenDeltaT(POOL_ID, 1, 0.2 ether);

        bytes memory data = abi.encode(dReserves, dLiquidity);

        dfmm.deallocate(POOL_ID, data);
    }

    function test_4_token_deallocate_basic_non_uniform()
        public
        basic_70_10_10_10
    {
        uint256 minTokenDelta = 0.1 ether;
        uint256[] memory minDeltas = createTokenDeltas(minTokenDelta);
        uint256 deltaL = 0.4 ether;

        bytes memory data = abi.encode(minDeltas, deltaL);

        (uint256[] memory preReserves, uint256 preL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(preReserves[0]);
        console2.log(preL);

        dfmm.deallocate(POOL_ID, data);

        (uint256[] memory postReserves, uint256 postL) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log(postReserves[0]);
        console2.log(postL);
    }

    function test_4_token_simulate_swap_non_uniform()
        public
        basic_70_10_10_10
    {
        uint256 amountIn = 0.1 ether;
        uint256 tokenInIndex = 0;
        uint256 tokenOutIndex = 1;

        (bool valid, uint256 amountOut, bytes memory data) =
            solver.simulateSwap(POOL_ID, tokenInIndex, tokenOutIndex, amountIn);

        dfmm.swap(POOL_ID, address(this), data);
    }

    function test_4_token_compute_price_non_uniform()
        public
        basic_70_10_10_10
    {
        uint256 tIndex = 0;

        (uint256[] memory reserves,) = solver.getReservesAndLiquidity(POOL_ID);
        NTokenGeometricMeanParams memory params = solver.getPoolParams(POOL_ID);

        uint256 price = solver.computePriceOfToken(
            reserves[tIndex],
            reserves[reserves.length - 1],
            params.weights[tIndex],
            params.weights[reserves.length - 1]
        );
    }
}

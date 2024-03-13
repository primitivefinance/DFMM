// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { DFMM, IDFMM, InitParams, Pool } from "src/DFMM.sol";
import { GeometricMean } from "src/GeometricMean/GeometricMean.sol";
import {
    GeometricMeanSolver,
    GeometricMeanParams
} from "src/GeometricMean/GeometricMeanSolver.sol";
import {
    computePrice,
    computeY,
    computeLGivenX
} from "src/GeometricMean/G3MMath.sol";
import { computeInitialPoolData } from "src/GeometricMean/G3MUtils.sol";

contract SetUp is Test {
    DFMM dfmm;
    MockERC20 tokenX;
    MockERC20 tokenY;
    WETH weth;
    GeometricMean g3m;
    GeometricMeanSolver solver;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    uint256 POOL_ID;

    function setUp() public virtual {
        tokenX = new MockERC20("Test Token X", "TSTX", 18);
        tokenY = new MockERC20("Test Token Y", "TSTY", 18);
        tokenX.mint(address(this), 100e18);
        tokenY.mint(address(this), 100e18);

        weth = new WETH();
        dfmm = new DFMM(address(weth));
        g3m = new GeometricMean(address(dfmm));
        solver = new GeometricMeanSolver(address(g3m));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function test_G3M2_init() public {
        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 0.5 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        bytes memory defaultInitialPoolData =
            computeInitialPoolData(1 ether, 1 ether, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        InitParams memory initParams = InitParams({
            name: "Test Pool",
            symbol: "TPOOL",
            strategy: address(g3m),
            tokens: tokens,
            data: defaultInitialPoolData
        });

        (POOL_ID,,) = dfmm.init(initParams);
    }

    function test_G3M2_allocate() public {
        test_G3M2_init();

        Pool memory pool = dfmm.getPool(POOL_ID);

        console.log(pool.reserves[0]);
        console.log(pool.reserves[1]);

        uint256 maxDeltaX = 0.1 ether;

        GeometricMeanParams memory params = solver.getPoolParams(POOL_ID);

        uint256 S = computePrice(pool.reserves[0], pool.reserves[1], params);
        uint256 deltaLiquidity = computeLGivenX(maxDeltaX, S, params);
        uint256 maxDeltaY = computeY(maxDeltaX, S, params);

        bytes memory data = abi.encode(maxDeltaX, maxDeltaY, deltaLiquidity);
        (uint256[] memory deltas) = dfmm.allocate(POOL_ID, data);
    }

    function test_G3M2_deallocate() public {
        test_G3M2_allocate();

        Pool memory pool = dfmm.getPool(POOL_ID);

        console.log(pool.reserves[0]);
        console.log(pool.reserves[1]);

        uint256 minDeltaX = 0.1 ether;

        GeometricMeanParams memory params = solver.getPoolParams(POOL_ID);

        uint256 S = computePrice(pool.reserves[0], pool.reserves[1], params);
        uint256 deltaLiquidity = computeLGivenX(minDeltaX, S, params);
        uint256 minDeltaY = computeY(minDeltaX, S, params);

        bytes memory data =
            abi.encode(minDeltaX - 1, minDeltaY - 1, deltaLiquidity);
        (uint256[] memory deltas) = dfmm.deallocate(POOL_ID, data);
    }

    function getPoolLiquidityToken(uint256 poolId)
        public
        view
        returns (address)
    {
        Pool memory pool = dfmm.getPool(poolId);
        return pool.liquidityToken;
    }

    function skip() public {
        vm.skip(true);
    }
}

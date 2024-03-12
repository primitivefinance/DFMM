// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "solmate/test/utils/mocks/MockERC20.sol";
import "src/DFMM.sol";
import "src/ConstantSum/ConstantSum.sol";
import "src/ConstantSum/ConstantSumSolver.sol";

contract ConstantSumTest is Test {
    using stdStorage for StdStorage;

    DFMM dfmm;
    ConstantSum constantSum;
    ConstantSumSolver solver;
    MockERC20 tokenX;
    MockERC20 tokenY;

    uint256 public constant TEST_ZERO_FEE = 0;
    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public {
        tokenX = new MockERC20("tokenX", "X", 18);
        tokenY = new MockERC20("tokenY", "Y", 18);
        tokenX.mint(address(this), 100_000_000 ether);
        tokenY.mint(address(this), 100_000_000 ether);

        dfmm = new DFMM(address(0));
        constantSum = new ConstantSum(address(dfmm));
        solver = new ConstantSumSolver(address(constantSum));
        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    modifier basic_feeless() {
        vm.warp(0);

        ConstantSumParams memory params = ConstantSumParams({
            price: ONE * 2,
            swapFee: 0,
            controller: address(0)
        });

        uint256 init_x = ONE * 1;
        uint256 init_y = ONE * 1;

        bytes memory initData =
            solver.getInitialPoolData(init_x, init_y, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    modifier basic() {
        vm.warp(0);

        ConstantSumParams memory params = ConstantSumParams({
            price: ONE * 2,
            swapFee: TEST_SWAP_FEE,
            controller: address(0)
        });

        uint256 init_x = ONE * 1;
        uint256 init_y = ONE * 1;

        bytes memory initData =
            solver.getInitialPoolData(init_x, init_y, params);

        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM.InitParams memory initParams = IDFMM.InitParams({
            name: "",
            symbol: "",
            strategy: address(constantSum),
            tokens: tokens,
            data: initData
        });

        dfmm.init(initParams);
        _;
    }

    function test_init() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        (ConstantSumParams memory params) =
            abi.decode(constantSum.getPoolParams(poolId), (ConstantSumParams));
        assertEq(params.price, 2 ether);
        assertEq(params.swapFee, 0.003 ether);
        assertEq(params.controller, address(0));

        (uint256[] memory reserves, uint256 initL) =
            DFMM(dfmm).getReservesAndLiquidity(poolId);

        assertEq(reserves[0], 1 ether);
        assertEq(reserves[1], 1 ether);
        assertEq(initL, 1.5 ether);
    }

    function test_constant_sum_swap_x_in_no_fee() public basic_feeless {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,, bytes memory swapData) =
            solver.simulateSwap(poolId, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(poolId, swapData);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenY.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }

    function test_constant_sum_swap_y_in_no_fee() public basic_feeless {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,, bytes memory swapData) =
            solver.simulateSwap(poolId, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(poolId, swapData);

        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceX - outputAmount
        );
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY + inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceX + outputAmount
        );
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY - inputAmount);
    }

    function test_constant_sum_swap_x_in_invalid() public basic_feeless {
        bool xIn = true;
        uint256 amountIn = 1.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateSwap(poolId, xIn, amountIn);
    }

    function test_constant_sum_swap_y_in_invalid() public basic_feeless {
        bool xIn = false;
        uint256 amountIn = 2.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateSwap(poolId, xIn, amountIn);
    }

    function test_constant_sum_swap_x_in_with_fee() public basic {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = true;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,, bytes memory swapData) =
            solver.simulateSwap(poolId, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(poolId, swapData);

        assertEq(tokenX.balanceOf(address(dfmm)), preDfmmBalanceX + inputAmount);
        assertEq(
            tokenY.balanceOf(address(dfmm)), preDfmmBalanceY - outputAmount
        );
        assertEq(tokenX.balanceOf(address(this)), preUserBalanceX - inputAmount);
        assertEq(
            tokenY.balanceOf(address(this)), preUserBalanceY + outputAmount
        );
    }

    function test_constant_sum_swap_y_in_with_fee() public basic {
        uint256 preDfmmBalanceX = tokenX.balanceOf(address(dfmm));
        uint256 preDfmmBalanceY = tokenY.balanceOf(address(dfmm));
        uint256 preUserBalanceX = tokenX.balanceOf(address(this));
        uint256 preUserBalanceY = tokenY.balanceOf(address(this));

        bool isSwapXForY = false;
        uint256 amountIn = 0.1 ether;
        uint256 poolId = dfmm.nonce() - 1;
        (,, bytes memory swapData) =
            solver.simulateSwap(poolId, isSwapXForY, amountIn);
        (,, uint256 inputAmount, uint256 outputAmount) =
            dfmm.swap(poolId, swapData);

        assertEq(
            tokenX.balanceOf(address(dfmm)), preDfmmBalanceX - outputAmount
        );
        assertEq(tokenY.balanceOf(address(dfmm)), preDfmmBalanceY + inputAmount);
        assertEq(
            tokenX.balanceOf(address(this)), preUserBalanceX + outputAmount
        );
        assertEq(tokenY.balanceOf(address(this)), preUserBalanceY - inputAmount);
    }
    /*
    function test_constant_sum_allocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 0.1 ether;
        uint256 amountY = 0.1 ether;
        (bool valid, bytes memory swapData) =
            solver.simulateAllocate(poolId, amountX, amountY);
        console2.log("Valid: ", valid);
        assert(valid);

        (uint256 endReservesRx, uint256 endReservesRy, uint256 endReservesL) =
            abi.decode(swapData, (uint256, uint256, uint256));

        console2.log("endReservesRx: ", endReservesRx);
        assertEq(endReservesRx, 1.1 ether);

        console2.log("endReservesRy: ", endReservesRy);
        assertEq(endReservesRy, 1.1 ether);

        console2.log("endReservesL: ", endReservesL);
        assertEq(endReservesL, 1.65 ether);

        dfmm.allocate(poolId, swapData);
    }

    function test_constant_sum_deallocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 0.1 ether;
        uint256 amountY = 0.1 ether;
        (, bytes memory swapData) =
            solver.simulateDeallocate(poolId, amountX, amountY);
        dfmm.deallocate(poolId, swapData);
    }

    function test_constant_sum_fail_deallocate() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 amountX = 1.2 ether;
        uint256 amountY = 1.2 ether;
        vm.expectRevert(ConstantSumSolver.NotEnoughLiquidity.selector);
        solver.simulateDeallocate(poolId, amountX, amountY);
    }

    function test_constant_sum_price_update() public basic {
        uint256 poolId = dfmm.nonce() - 1;
        uint256 newPrice = 3 ether;
        bytes memory data = encodePriceUpdate(newPrice);

        vm.prank(address(0));
        DFMM(dfmm).update(poolId, data);

        (ConstantSumParams memory newParams) = abi.decode(
            constantSum.getPoolParams(poolId), (ConstantSumParams)
        );
        assertEq(newParams.price, 3 ether);
    }
    */
}

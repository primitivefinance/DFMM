// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import "forge-std/Test.sol";
import "forge-std/console2.sol";

contract CoveredCallInitTest is CoveredCallSetUp {
    function test_CoveredCall_init_ReturnsPriceOfOne() public init_quarterly {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenYL = solver.getPriceGivenYL(POOL_ID, reserves[1], L);
        uint256 priceGivenXL = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
        console2.log("priceGivenYL", priceGivenYL);
        console2.log("priceGivenXL", priceGivenXL);

        assertApproxEqAbs(priceGivenXL, ONE, 10);
        assertApproxEqAbs(priceGivenYL, ONE, 10);
    }

    function test_CoveredCall_init_log_state() public init_mil {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        uint256 priceGivenYL = solver.getPriceGivenYL(POOL_ID, reserves[1], L);
        uint256 priceGivenXL = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
        console2.log("priceGivenYL", priceGivenYL);
        console2.log("priceGivenXL", priceGivenXL);

        assertApproxEqAbs(priceGivenXL, ONE, 10);
        assertApproxEqAbs(priceGivenYL, ONE, 10);
    }

    function test_CoveredCall_init_capital_efficiency() public init_mil {
        (uint256[] memory reserves, uint256 L) =
            solver.getReservesAndLiquidity(POOL_ID);
        console2.log("rXinit", reserves[0]);
        console2.log("rYinit", reserves[1]);
        uint256 defaultPricePoin11Rate = 0.81162243324 ether;
        uint256 maxRange = 0.69444444444 ether;
        uint256 amountIn = 5000 ether;
        bool xIn = true;

        uint256 price = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
        console2.log("initial price", price);

        int256 invariant = solver.getInvariant(POOL_ID);
        console2.log("initial invariant", invariant);
        uint256 acc = 0;
        while (price > defaultPricePoin11Rate) {
            (reserves, L) = solver.getReservesAndLiquidity(POOL_ID);
            (,,, bytes memory data) =
                solver.simulateSwap(POOL_ID, xIn, amountIn);
            dfmm.swap(POOL_ID, address(this), data);
            price = solver.getPriceGivenXL(POOL_ID, reserves[0], L);
            invariant = solver.getInvariant(POOL_ID);
            acc += amountIn;
            console2.log("invariant", invariant);
            console2.log("rX", reserves[0]);
            console2.log("rY", reserves[1]);
            console2.log("acc", acc);
            console2.log("price post swap", price);
            console2.log("price gt defaultPricePoin11Rate", price > defaultPricePoin11Rate);
        }
    }
}

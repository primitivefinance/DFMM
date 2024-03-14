// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { LPToken, Pool } from "src/DFMM.sol";
import { DFMMSetUp } from "./SetUp.sol";

contract DFMMSwapTest is DFMMSetUp {
    function test_DFMM_swap_IncreasesTotalLiquidity() public {
        skip();
    }

    function test_DFMM_swap_UpdatesReserves() public {
        skip();
    }

    function test_DFMM_swap_TransfersTokens18Decimals() public {
        skip();
    }

    function test_DFMM_swap_TransfersTokens6Decimals() public {
        skip();
    }

    function test_DFMM_swap_EmitsSwapEvent() public {
        skip();
    }

    function test_DFMM_swap_RevertsWhenInvalid() public {
        skip();
    }

    function test_DFMM_swap_RevertsWhenDeltaXGreaterThanReserveX() public {
        skip();
    }

    function test_DFMM_swap_RevertsWhenDeltaYGreaterThanReserveY() public {
        skip();
    }

    function test_DFMM_swap_MintsLPTokensToFeeCollector() public initPool {
        Pool memory pool = dfmm.pools(POOL_ID);
        LPToken token = LPToken(pool.liquidityToken);

        uint256 preBalance = token.balanceOf(address(this));
        uint256 deltaLiquidity = 1 ether;
        uint256 fees = deltaLiquidity * pool.controllerFee / 1 ether;
        uint256 feesInToken =
            fees * token.totalSupply() / (pool.totalLiquidity + deltaLiquidity);
        dfmm.swap(
            POOL_ID,
            address(this),
            abi.encode(true, 0, 0, 1, 1 ether, 1 ether, 1 ether)
        );
        assertEq(token.balanceOf(address(this)), preBalance + feesInToken);
    }
}

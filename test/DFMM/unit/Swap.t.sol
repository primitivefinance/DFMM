// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { LPToken, Pool, ERC20, IDFMM } from "src/DFMM.sol";
import { ISwapCallback } from "src/interfaces/ISwapCallback.sol";
import { computeScalingFactor, downscaleUp } from "src/lib/ScalingLib.sol";
import { DFMMSetUp } from "./SetUp.sol";

contract DFMMSwapTest is DFMMSetUp, ISwapCallback {
    using FixedPointMathLib for uint256;

    enum SwapCallbackType {
        Valid,
        Invalid
    }

    function swapCallback(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        bytes calldata data
    ) external {
        SwapCallbackType swapCallbackType = abi.decode(data, (SwapCallbackType));

        if (swapCallbackType == SwapCallbackType.Valid) {
            uint256 downscaledAmountIn =
                downscaleUp(amountIn, computeScalingFactor(tokenIn));
            ERC20(tokenIn).transfer(msg.sender, downscaledAmountIn);
        } else if (swapCallbackType == SwapCallbackType.Invalid) {
            return;
        }
    }

    function test_DFMM_swap_ForwardsSwapCallbackData() public {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 10 ether;
        reserves[1] = 10 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(10 ether));
        (POOL_ID,,) = dfmm.init(getDefaultPoolParams(params));

        dfmm.swap(
            POOL_ID,
            address(this),
            abi.encode(true, 1 ether, 0, 1, 1 ether, 1 ether, 1 ether),
            abi.encode(SwapCallbackType.Valid)
        );
    }

    function test_DFMM_swap_RevertsIfInvalidTransfer() public {
        uint256[] memory reserves = new uint256[](2);
        reserves[0] = 10 ether;
        reserves[1] = 10 ether;

        bytes memory params =
            abi.encode(true, int256(1 ether), reserves, uint256(10 ether));
        (POOL_ID,,) = dfmm.init(getDefaultPoolParams(params));

        vm.expectRevert(IDFMM.InvalidTransfer.selector);
        dfmm.swap(
            POOL_ID,
            address(this),
            abi.encode(true, 1 ether, 0, 1, 1 ether, 1 ether, 1 ether),
            abi.encode(SwapCallbackType.Invalid)
        );
    }

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
        uint256 fees = deltaLiquidity.mulWadDown(pool.controllerFee);
        uint256 feesInToken = fees * token.totalSupply()
            / (pool.totalLiquidity + deltaLiquidity - fees);
        dfmm.swap(
            POOL_ID,
            address(this),
            abi.encode(true, 0, 0, 1, 1 ether, 1 ether, 1 ether),
            ""
        );
        assertEq(token.balanceOf(address(this)), preBalance + feesInToken);
    }
}

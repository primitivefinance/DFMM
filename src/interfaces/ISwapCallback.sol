// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.5.0;

interface ISwapCallback {
    /// @notice              Triggered when swapping tokens in DFMM.
    /// @param  tokenIn      Token to swap from
    /// @param  tokenOut     Token to swap to
    /// @param  amountIn     Amount of tokenIn to swap
    /// @param  amountOut    Amount of tokenOut received
    /// @param  data         Calldata passed on swap function call
    function swapCallback(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        uint256 amountOut,
        bytes calldata data
    ) external;
}

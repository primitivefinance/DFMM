// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

interface ISolver {
    function prepareInit(
        bytes calldata params,
        bytes calldata poolParams
    ) external view returns (bytes memory);

    function prepareAllocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 delta
    ) external view returns (bytes memory);

    function prepareDeallocation(
        uint256 poolId,
        uint256 tokenIndex,
        uint256 delta
    ) external view returns (bytes memory);

    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) external view returns (bool, uint256, bytes memory);

    function getPrice(uint256 poolId) external view returns (uint256);

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256[] memory reserves, uint256 totalLiquidity);
}

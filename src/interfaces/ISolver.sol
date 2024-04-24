// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

error InvalidTokenIndex();

/**
 * @title Solver Interface
 * @author Primitive
 */
interface ISolver {
    function prepareAllocation(
        uint256 poolId,
        uint256[] memory deltas
    ) external view returns (bytes memory);

    function prepareDeallocation(
        uint256 poolId,
        uint256 liquidityDelta
    ) external view returns (bytes memory);

    function prepareSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) external view returns (bool, uint256, bytes memory);

    function getEstimatedPrice(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex
    ) external view returns (uint256);

    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256[] memory reserves, uint256 totalLiquidity);

    // TODO: This function might get removed because `swapFee` might be
    // merged into default DFMM pool parameters.
    // function getSwapFee(uint256 poolId) external view returns (uint256);
}

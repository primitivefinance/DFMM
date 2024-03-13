// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IDFMM, Pool } from "src/interfaces/IDFMM.sol";

/**
 * @title Strategy Interface.
 * @author Primitive
 * @notice All the strategy contracts must implement this interface.
 */
interface IStrategy {
    // Errors

    /// @dev Thrown when the update code is invalid.
    error InvalidUpdateCode();

    /// @dev Thrown when the sender is not the DFMM contract.
    error NotDFMM();

    /// @dev Thrown when the sender is authorized.
    error InvalidSender();

    error DeltaError(uint256 expected, uint256 actual);

    // Setters

    /**
     * @notice Intializes a new pool.
     * @param sender Address that called the DFMM contract.
     * @param poolId Id of the pool to initialize.
     * @param data Pool parameters encoded as bytes.
     * @return valid True if the initialization is valid.
     * @return invariant Initial swap growth.
     * @return reserves Initial reserves of the pool.
     * @return totalLiquidity Initial liquidity of the pool.
     */
    function init(
        address sender,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        );

    // Getters

    /**
     * @notice Returns the name of the strategy.
     * @dev The name of the strategy is included in the name of
     * the liquidity token.
     */
    function name() external view returns (string memory);

    function validateAllocate(
        address sender,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        );

    function validateDeallocate(
        address sender,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        );

    function validateSwap(
        address sender,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 tokenInIndex,
            uint256 tokenOutIndex,
            uint256 amountIn,
            uint256 amountOut,
            uint256 deltaLiquidity
        );

    function update(
        address sender,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    ) external;

    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) external view returns (int256);

    function dfmm() external view returns (address);

    function getPoolParams(uint256 poolId)
        external
        view
        returns (bytes calldata params);
}

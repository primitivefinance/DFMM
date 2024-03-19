// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

/**
 * @dev Parameters of a DFMM pool.
 * @param strategy Address of the associated strategy contract.
 * @param tokens Array of token addresses in the pool.
 * @param reserves Array of token reserves in the pool in WAD.
 * @param totalLiquidity Total liquidity in the pool.
 * @param liquidityToken Address of the LP token contract.
 */
struct Pool {
    address strategy;
    address[] tokens;
    uint256[] reserves;
    uint256 totalLiquidity;
    address liquidityToken;
    address feeCollector;
    uint256 controllerFee;
}

/**
 * @dev Parameters used to initialize a new DFMM pool.
 * @param name Name of the LP token.
 * @param symbol Symbol of the LP token.
 * @param strategy Address of the associated strategy contract.
 * @param tokens Array of token addresses in the pool.
 * @param data An array of bytes used by the strategy contract.
 */
struct InitParams {
    string name;
    string symbol;
    address strategy;
    address[] tokens;
    bytes data;
    address feeCollector;
    uint256 controllerFee;
}

/**
 * @title DFMM Interface
 * @author Primitive
 */
interface IDFMM {
    // Errors

    /// @dev Thrown when the caller is not the WETH contract.
    error OnlyWETH();

    /// @dev Thrown when a token transfer fails.
    error InvalidTransfer();

    /// @dev Thrown when the invariant is invalid.
    error InvalidInvariant(int256 invariant);

    /// @dev Thrown when pool tokens are identical.
    error InvalidDuplicateTokens();

    /// @dev Thrown when a strategy is returning an invalid amount of reserves.
    error InvalidReserves();

    /// @dev Thrown when a pool is being initalized with less than two tokens.
    error InvalidMinimumTokens();

    /// @dev Thrown when a pool is being initalized with more than eight tokens.
    error InvalidMaximumTokens();

    /// @dev Thrown when a token has more than 18 decimals or less than 6.
    error InvalidTokenDecimals();

    /// @dev Thrown when a new call is made during a locked state.
    error Locked();

    /// @dev Thrown when a clone contract could not be deployed.
    error ERC1167FailedCreateClone();

    /// @dev Thrown when the caller is not the pool controller.
    error NotController();

    // Events

    /**
     * @notice Emitted when a pool is initialized.
     * @param account Address initializing the pool.
     * @param strategy Address of the associated strategy contract.
     * @param lpToken Address of the LP token contract.
     * @param poolId Id of the newly initialized pool.
     * @param tokens Array of token addresses in the pool.
     * @param reserves Array of token reserves in the pool in WAD.
     * @param totalLiquidity Initial liquidity in the pool.
     */
    event Init(
        address indexed account,
        address strategy,
        address lpToken,
        uint256 poolId,
        address[] indexed tokens,
        uint256[] reserves,
        uint256 totalLiquidity
    );

    /**
     * @notice Emitted when liquidity is allocated into a pool.
     * @param account Address allocating liquidity.
     * @param poolId Id of the pool receiving liquidity.
     * @param deltas Array of amounts (in WAD) deposited into the pool.
     * @param deltaL Amount of liquidity received by the allocator.
     */
    event Allocate(
        address indexed account,
        uint256 poolId,
        uint256[] deltas,
        uint256 deltaL
    );

    /**
     * @notice Emitted when liquidity is deallocated from a pool.
     * @param account Address deallocating liquidity.
     * @param poolId Id of the pool losing liquidity.
     * @param deltas Array of amounts (in WAD) withdrawn from the pool.
     * @param deltaL Amount of liquidity being deallocated.
     */
    event Deallocate(
        address indexed account,
        uint256 poolId,
        uint256[] indexed deltas,
        uint256 deltaL
    );

    /**
     * @notice Emitted when a swap is made.
     * @param account Address making the swap.
     * @param poolId Id of the pool where the swap is happening.
     * @param tokenIn Address of the token being sent.
     * @param tokenOut Address of the token being received.
     * @param inputAmount Amount of token sent by the swapper.
     * @param outputAmount Amount of token received by the swapper.
     */
    event Swap(
        address indexed account,
        uint256 indexed poolId,
        address recipient,
        address tokenIn,
        address tokenOut,
        uint256 inputAmount,
        uint256 outputAmount
    );

    // Setters

    /**
     * @notice Intializes a new pool.
     * @param params A struct containing the initialization parameters.
     * @return poolId Id of the newly initialized pool.
     * @return reserves Initial reserves of the pool in WAD.
     * @return totalLiquidity Initial amount of liquidity in the pool.
     */
    function init(InitParams calldata params)
        external
        payable
        returns (
            uint256 poolId,
            uint256[] memory reserves,
            uint256 totalLiquidity
        );

    /**
     * @notice Allocates liquidity into the pool `poolId`.
     * @param poolId Id of the pool to allocate into.
     * @param data An array of bytes used by the strategy contract.
     * @return deltas Array of amounts (in WAD) deposited into the pool.
     */
    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external payable returns (uint256[] memory deltas);

    /**
     * @notice Deallocates liquidity from the pool `poolId`.
     * @param poolId Id of the pool to deallocate from.
     * @param data An array of bytes used by the strategy contract.
     * @return deltas Array of amounts (in WAD) withdrawn from the pool.
     */
    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external returns (uint256[] memory deltas);

    /**
     * @notice Swaps tokens into pool `poolId`.
     * @param poolId Id of the pool to swap tokens into.
     * @param recipient Address receiving the output tokens.
     * @param data An array of bytes used by the strategy contract.
     * @return tokenIn Address of the token being sent.
     * @return tokenOut Address of the token being received.
     * @return amountIn Amount of token sent by the swapper.
     * @return amountOut Amount of token received by the swapper.
     */
    function swap(
        uint256 poolId,
        address recipient,
        bytes calldata data
    )
        external
        payable
        returns (
            address tokenIn,
            address tokenOut,
            uint256 amountIn,
            uint256 amountOut
        );

    /**
     * @notice Updates pool `poolId` by calling the associated strategy.
     * @param poolId Id of the pool to update.
     * @param data An array of bytes used by the strategy contract.
     */
    function update(uint256 poolId, bytes calldata data) external;

    // Getters

    /// @notice Address of the implementation of the LPToken contract.
    function lpTokenImplementation() external view returns (address);

    /// @notice Address of the WETH contract.
    function weth() external view returns (address);

    /// @notice Returns the pool parameters of pool `poolId`.
    /// @return pool A struct containing the pool parameters.
    function pools(uint256 poolId) external view returns (Pool memory pool);
}

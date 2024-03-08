// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { IDFMM2 } from "src/interfaces/IDFMM2.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { SafeTransferLib, ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { IStrategy2 } from "./interfaces/IStrategy2.sol";
import {
    computeScalingFactor,
    downscaleDown,
    downscaleUp
} from "./lib/ScalingLib.sol";
import { LPToken } from "./LPToken.sol";
import "forge-std/console2.sol";

/**
 * @title DFMM
 * @author Primitive
 * @notice Dynamic Function Market Maker
 */
contract DFMM2 is IDFMM2 {
    using FixedPointMathLib for uint256;

    Pool[] public pools;

    address public immutable lpTokenImplementation;

    address public immutable weth;

    /// @dev Part of the reentrancy lock, 1 = unlocked, 2 = locked.
    uint256 private _locked = 1;

    /// @dev Amount of liquidity that is burnt on initialization.
    uint256 private constant BURNT_LIQUIDITY = 1000;

    /// @dev Prevents reentrancy.
    modifier lock() {
        if (_locked == 2) revert Locked();
        _locked = 2;
        _;
        _locked = 1;
    }

    receive() external payable {
        if (msg.sender != weth) revert OnlyWETH();
    }

    /**
     * @dev The implementation of the LPToken contract is also
     * deployed at the same time. It'll be used later to deploy
     * new LPTokens using the [clone factory pattern](https://eips.ethereum.org/EIPS/eip-1167).
     */
    constructor(address weth_) {
        weth = weth_;
        lpTokenImplementation = address(new LPToken());
        LPToken(lpTokenImplementation).initialize("", "");
    }

    function init(InitParams calldata params)
        external
        payable
        lock
        returns (uint256, uint256[] memory, uint256)
    {
        Pool memory pool = Pool({
            strategy: params.strategy,
            tokens: params.tokens,
            reserves: new uint256[](params.tokens.length),
            totalLiquidity: 0,
            liquidityToken: address(0)
        });

        (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        ) = IStrategy2(params.strategy).init(
            msg.sender, pools.length, pool, params.data
        );

        if (!valid) revert InvalidInvariant(invariant);

        LPToken liquidityToken = LPToken(clone(lpTokenImplementation));

        liquidityToken.initialize(params.name, params.symbol);
        liquidityToken.mint(msg.sender, totalLiquidity - BURNT_LIQUIDITY);
        liquidityToken.mint(address(0), BURNT_LIQUIDITY);
        console2.log("issued tokens", totalLiquidity - BURNT_LIQUIDITY);

        pool.reserves = reserves;
        pool.totalLiquidity = totalLiquidity;
        pool.liquidityToken = address(liquidityToken);

        pools.push(pool);
        uint256 poolId = pools.length - 1;

        for (uint256 i = 0; i < params.tokens.length; i++) {
            if (params.tokens[i] != address(0)) {
                _transferFrom(params.tokens[i], reserves[i]);
            }
        }

        emit Init(
            msg.sender,
            pool.strategy,
            address(liquidityToken),
            poolId,
            pool.tokens,
            pool.reserves,
            pool.totalLiquidity
        );

        return (poolId, reserves, totalLiquidity - BURNT_LIQUIDITY);
    }

    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external payable lock returns (uint256[] memory) {
        (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        ) = IStrategy2(pools[poolId].strategy).validateAllocate(
            msg.sender, poolId, pools[poolId], data
        );

        if (!valid) revert InvalidInvariant(invariant);

        for (uint256 i = 0; i < pools[poolId].tokens.length; i++) {
            if (pools[poolId].tokens[i] != address(0)) {
                pools[poolId].reserves[i] += deltas[i];
            }
        }
        pools[poolId].totalLiquidity += deltaLiquidity;

        _manageTokens(poolId, true, deltaLiquidity);

        for (uint256 i = 0; i < pools[poolId].tokens.length; i++) {
            if (pools[poolId].tokens[i] != address(0)) {
                _transferFrom(pools[poolId].tokens[i], deltas[i]);
            }
        }

        emit Allocate(msg.sender, poolId, deltas, deltaLiquidity);
        return deltas;
    }

    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external lock returns (uint256[] memory) {
        (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        ) = IStrategy2(pools[poolId].strategy).validateDeallocate(
            msg.sender, poolId, pools[poolId], data
        );

        if (!valid) revert InvalidInvariant(invariant);

        for (uint256 i = 0; i < pools[poolId].tokens.length; i++) {
            if (pools[poolId].tokens[i] != address(0)) {
                pools[poolId].reserves[i] -= deltas[i];
            }
        }
        pools[poolId].totalLiquidity -= deltaLiquidity;

        _manageTokens(poolId, false, deltaLiquidity);

        for (uint256 i = 0; i < pools[poolId].tokens.length; i++) {
            if (pools[poolId].tokens[i] != address(0)) {
                _transfer(pools[poolId].tokens[i], msg.sender, deltas[i]);
            }
        }

        emit Deallocate(msg.sender, poolId, deltas, deltaLiquidity);
        return deltas;
    }

    struct SwapState {
        bool valid;
        int256 invariant;
        uint256 tokenInIndex;
        uint256 tokenOutIndex;
        uint256 amountIn;
        uint256 amountOut;
        uint256 deltaLiquidity;
    }

    function swap(
        uint256 poolId,
        bytes calldata data
    ) external lock returns (address, address, uint256, uint256) {
        SwapState memory state;

        (
            state.valid,
            state.invariant,
            state.tokenInIndex,
            state.tokenOutIndex,
            state.amountIn,
            state.amountOut,
            state.deltaLiquidity
        ) = IStrategy2(pools[poolId].strategy).validateSwap(
            msg.sender, poolId, pools[poolId], data
        );

        if (!state.valid) revert InvalidInvariant(state.invariant);

        pools[poolId].totalLiquidity += state.deltaLiquidity;

        pools[poolId].reserves[state.tokenInIndex] += state.amountIn;
        pools[poolId].reserves[state.tokenOutIndex] -= state.amountOut;

        address tokenIn = pools[poolId].tokens[state.tokenInIndex];
        address tokenOut = pools[poolId].tokens[state.tokenOutIndex];

        _transferFrom(tokenIn, state.amountIn);
        _transfer(tokenOut, msg.sender, state.amountOut);

        emit Swap(
            msg.sender,
            poolId,
            tokenIn,
            tokenOut,
            state.amountIn,
            state.amountOut
        );

        return (tokenIn, tokenOut, state.amountIn, state.amountOut);
    }

    function update(uint256 poolId, bytes calldata data) external lock {
        IStrategy2(pools[poolId].strategy).update(
            msg.sender, poolId, pools[poolId], data
        );
    }

    // Internals

    /**
     * @dev Transfers `amount` of `token` from the sender to the contract.
     * Note that if ETH is present in the contract, it will be wrapped to WETH.
     * @param token Address of the token to transfer.
     * @param amount Amount to transfer expressed in WAD.
     */
    function _transferFrom(address token, uint256 amount) internal {
        if (address(this).balance >= amount) {
            WETH(payable(weth)).deposit{ value: amount }();

            if (address(this).balance > 0) {
                SafeTransferLib.safeTransferETH(
                    msg.sender, address(this).balance
                );
            }
        } else {
            uint256 downscaledAmount =
                downscaleUp(amount, computeScalingFactor(token));
            uint256 preBalance = ERC20(token).balanceOf(address(this));
            SafeTransferLib.safeTransferFrom(
                ERC20(token), msg.sender, address(this), downscaledAmount
            );
            uint256 postBalance = ERC20(token).balanceOf(address(this));
            if (postBalance < preBalance + downscaledAmount) {
                revert InvalidTransfer();
            }
        }
    }

    /**
     * @dev Transfers `amount of `token` from the contract to the recipient
     * `to`. Note that WETH is automatically unwrapped to ETH.
     * @param token Address of the token to transfer.
     * @param to Address of the recipient.
     * @param amount Amount to transfer expressed in WAD.
     */
    function _transfer(address token, address to, uint256 amount) internal {
        if (token == weth) {
            WETH(payable(weth)).withdraw(amount);
            SafeTransferLib.safeTransferETH(to, amount);
        } else {
            uint256 downscaledAmount =
                downscaleDown(amount, computeScalingFactor(token));
            uint256 preBalance = ERC20(token).balanceOf(address(this));
            SafeTransferLib.safeTransfer(ERC20(token), to, downscaledAmount);
            uint256 postBalance = ERC20(token).balanceOf(address(this));
            if (postBalance < preBalance - downscaledAmount) {
                revert InvalidTransfer();
            }
        }
    }

    /**
     * @dev Mints or burns liquidity tokens.
     */
    function _manageTokens(
        uint256 poolId,
        bool isAllocate,
        uint256 deltaL
    ) internal {
        LPToken liquidityToken = LPToken(pools[poolId].liquidityToken);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = pools[poolId].totalLiquidity;
        console2.log("total tokens", totalSupply);
        console2.log("total liquidity", totalLiquidity);
        console2.log(
            "muldiv", deltaL.mulWadUp(totalSupply.divWadUp(totalLiquidity))
        );

        if (isAllocate) {
            uint256 amount =
                deltaL.mulWadDown(totalSupply.divWadDown(totalLiquidity));
            liquidityToken.mint(msg.sender, amount);
        } else {
            uint256 amount =
                deltaL.mulWadUp(totalSupply.divWadUp(totalLiquidity));
            liquidityToken.burn(msg.sender, amount);
        }
    }

    /**
     * @dev Deploys and returns the address of a clone that mimics the behaviour of `implementation`.
     * This function uses the create opcode, which should never revert.
     * This function is taken from https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/proxy/Clones.sol#L23.
     */
    function clone(address implementation)
        internal
        returns (address instance)
    {
        /// @solidity memory-safe-assembly
        assembly {
            // Cleans the upper 96 bits of the `implementation` word, then packs the first 3 bytes
            // of the `implementation` address with the bytecode before the address.
            mstore(
                0x00,
                or(
                    shr(0xe8, shl(0x60, implementation)),
                    0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000
                )
            )
            // Packs the remaining 17 bytes of `implementation` with the bytecode after the address.
            mstore(
                0x20,
                or(shl(0x78, implementation), 0x5af43d82803e903d91602b57fd5bf3)
            )
            instance := create(0, 0x09, 0x37)
        }
        if (instance == address(0)) {
            revert ERC1167FailedCreateClone();
        }
    }

    // Lens

    /// @notice Returns the amount of initialized pools.
    function nonce() external view returns (uint256) {
        return pools.length;
    }

    /// @notice Returns the pool `poolId` as a Pool struct.
    function getPool(uint256 poolId) external view returns (Pool memory) {
        return pools[poolId];
    }

    /// @notice Returns the reserves and liquidity of pool `poolId`.
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256[] memory, uint256)
    {
        return (pools[poolId].reserves, pools[poolId].totalLiquidity);
    }

    /**
     * @notice Returns the amount of liquidity owned by `account` for
     * the pool `poolId`.
     * @dev This function should NOT be used in a non-view call, as the
     * values can be manipulated. In the future this function might be
     * removed.
     */
    function liquidityOf(
        address account,
        uint256 poolId
    ) public view returns (uint256) {
        LPToken liquidityToken = LPToken(pools[poolId].liquidityToken);
        uint256 balance = liquidityToken.balanceOf(account);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = pools[poolId].totalLiquidity;
        return balance.mulWadDown(totalLiquidity.divWadDown(totalSupply));
    }
}

// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { IDFMM, Pool, InitParams } from "src/interfaces/IDFMM.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { SafeTransferLib, ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { IStrategy } from "./interfaces/IStrategy.sol";
import {
    computeScalingFactor,
    downscaleDown,
    downscaleUp
} from "./lib/ScalingLib.sol";
import { LPToken } from "./LPToken.sol";

/**
 * @title DFMM
 * @author Primitive
 * @notice Dynamic Function Market Maker
 */
contract DFMM is IDFMM {
    using FixedPointMathLib for uint256;

    Pool[] internal _pools;

    /// @inheritdoc IDFMM
    address public immutable lpTokenImplementation;

    /// @inheritdoc IDFMM
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

    /// @dev Only the WETH contract can send ETH to this contract.
    receive() external payable {
        if (msg.sender != weth) revert OnlyWETH();
    }

    /**
     * @dev The implementation of the LPToken contract is also
     * deployed at the same time. It'll be used later to deploy
     * new LPTokens using the [clone factory pattern](https://eips.ethereum.org/EIPS/eip-1167).
     * @param weth_ The address of the WETH contract.
     */
    constructor(address weth_) {
        weth = weth_;
        lpTokenImplementation = address(new LPToken());
        LPToken(lpTokenImplementation).initialize("", "");
    }

    /// @inheritdoc IDFMM
    function init(InitParams calldata params)
        external
        payable
        lock
        returns (uint256, uint256[] memory, uint256)
    {
        if (params.tokens.length < 2) revert InvalidMinimumTokens();
        if (params.tokens.length > 8) revert InvalidMaximumTokens();

        Pool memory pool = Pool({
            strategy: params.strategy,
            tokens: params.tokens,
            reserves: new uint256[](params.tokens.length),
            totalLiquidity: 0,
            liquidityToken: address(0),
            feeCollector: params.feeCollector,
            controllerFee: params.controllerFee
        });

        (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        ) = IStrategy(params.strategy).init(
            msg.sender, _pools.length, pool, params.data
        );

        if (!valid) revert InvalidInvariant(invariant);

        LPToken liquidityToken = LPToken(clone(lpTokenImplementation));

        liquidityToken.initialize(params.name, params.symbol);
        liquidityToken.mint(msg.sender, totalLiquidity - BURNT_LIQUIDITY);
        liquidityToken.mint(address(0), BURNT_LIQUIDITY);

        pool.reserves = reserves;
        pool.totalLiquidity = totalLiquidity;
        pool.liquidityToken = address(liquidityToken);

        _pools.push(pool);
        uint256 poolId = _pools.length - 1;

        // TODO: Improve this code.
        uint256 tokensLength = params.tokens.length;
        for (uint256 i = 0; i < tokensLength; i++) {
            address token = params.tokens[i];

            for (uint256 j = 0; j < tokensLength; j++) {
                if (i != j && token == params.tokens[j]) {
                    revert InvalidDuplicateTokens();
                }
            }
        }

        for (uint256 i = 0; i < params.tokens.length; i++) {
            uint256 decimals = ERC20(params.tokens[i]).decimals();

            if (decimals > 18 || decimals < 6) {
                revert InvalidTokenDecimals();
            }

            _transferFrom(params.tokens[i], reserves[i]);
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

    /// @inheritdoc IDFMM
    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external payable lock returns (uint256[] memory) {
        (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        ) = IStrategy(_pools[poolId].strategy).validateAllocate(
            msg.sender, poolId, _pools[poolId], data
        );

        if (!valid) revert InvalidInvariant(invariant);

        for (uint256 i = 0; i < _pools[poolId].tokens.length; i++) {
            _pools[poolId].reserves[i] += deltas[i];
        }

        _pools[poolId].totalLiquidity += deltaLiquidity;
        _manageTokens(msg.sender, poolId, true, deltaLiquidity);

        for (uint256 i = 0; i < _pools[poolId].tokens.length; i++) {
            _transferFrom(_pools[poolId].tokens[i], deltas[i]);
        }

        emit Allocate(msg.sender, poolId, deltas, deltaLiquidity);
        return deltas;
    }

    /// @inheritdoc IDFMM
    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external lock returns (uint256[] memory) {
        (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        ) = IStrategy(_pools[poolId].strategy).validateDeallocate(
            msg.sender, poolId, _pools[poolId], data
        );

        if (!valid) revert InvalidInvariant(invariant);

        for (uint256 i = 0; i < _pools[poolId].tokens.length; i++) {
            _pools[poolId].reserves[i] -= deltas[i];
        }

        _manageTokens(msg.sender, poolId, false, deltaLiquidity);
        _pools[poolId].totalLiquidity -= deltaLiquidity;

        for (uint256 i = 0; i < _pools[poolId].tokens.length; i++) {
            _transfer(_pools[poolId].tokens[i], msg.sender, deltas[i]);
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
        address recipient,
        bytes calldata data
    ) external payable lock returns (address, address, uint256, uint256) {
        SwapState memory state;

        (
            state.valid,
            state.invariant,
            state.tokenInIndex,
            state.tokenOutIndex,
            state.amountIn,
            state.amountOut,
            state.deltaLiquidity
        ) = IStrategy(_pools[poolId].strategy).validateSwap(
            msg.sender, poolId, _pools[poolId], data
        );

        if (!state.valid) revert InvalidInvariant(state.invariant);

        _pools[poolId].totalLiquidity += state.deltaLiquidity;

        if (_pools[poolId].feeCollector != address(0)) {
            uint256 fees = state.deltaLiquidity * _pools[poolId].controllerFee
                / FixedPointMathLib.WAD;
            if (fees == 0) ++fees;
            _manageTokens(_pools[poolId].feeCollector, poolId, true, fees);
        }

        _pools[poolId].reserves[state.tokenInIndex] += state.amountIn;
        _pools[poolId].reserves[state.tokenOutIndex] -= state.amountOut;

        address tokenIn = _pools[poolId].tokens[state.tokenInIndex];
        address tokenOut = _pools[poolId].tokens[state.tokenOutIndex];

        _transferFrom(tokenIn, state.amountIn);
        _transfer(tokenOut, recipient, state.amountOut);

        emit Swap(
            msg.sender,
            poolId,
            recipient,
            tokenIn,
            tokenOut,
            state.amountIn,
            state.amountOut
        );

        return (tokenIn, tokenOut, state.amountIn, state.amountOut);
    }

    /// @inheritdoc IDFMM
    function update(uint256 poolId, bytes calldata data) external lock {
        IStrategy(_pools[poolId].strategy).update(
            msg.sender, poolId, _pools[poolId], data
        );
    }

    // Internals

    /**
     * @dev Transfers `amount` of `token` from the sender to the contract. Note
     * that if ETH is present in the contract, it will be wrapped to WETH. Any
     * excess of ETH will be sent back to the sender.
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
     * @dev Mints or burns liquidity tokens. Note that the amount of minted
     * or burnt tokens is NOT the same as the amount of liquidity added or
     * removed from the pool.
     * @param recipient Address receiving the minted LP tokens.
     * @param poolId Id of the associated pool.
     * @param isAllocate True if tokens will be minted, false otherwise.
     * @param deltaL Amount of liquidity added or removed from the pool.
     */
    function _manageTokens(
        address recipient,
        uint256 poolId,
        bool isAllocate,
        uint256 deltaL
    ) internal {
        LPToken liquidityToken = LPToken(_pools[poolId].liquidityToken);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = _pools[poolId].totalLiquidity;

        if (isAllocate) {
            uint256 amount =
                deltaL.mulWadDown(totalSupply.divWadDown(totalLiquidity));
            liquidityToken.mint(recipient, amount);
        } else {
            uint256 amount =
                deltaL.mulWadUp(totalSupply.divWadUp(totalLiquidity));
            liquidityToken.burn(msg.sender, amount);
        }
    }

    /**
     * @dev Deploys and returns the address of a clone contract that mimics
     * the behaviour of the contract deployed at the address `implementation`.
     * This function uses the `CREATE` opcode, which should never revert.
     * This function was taken from https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/proxy/Clones.sol#L23.
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

    // Getters

    /// @inheritdoc IDFMM
    function nonce() external view returns (uint256) {
        return _pools.length;
    }

    /// @inheritdoc IDFMM
    function pools(uint256 poolId) external view returns (Pool memory) {
        return _pools[poolId];
    }

    /// @inheritdoc IDFMM
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256[] memory, uint256)
    {
        return (_pools[poolId].reserves, _pools[poolId].totalLiquidity);
    }
}

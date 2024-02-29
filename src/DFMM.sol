// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { SafeTransferLib, ERC20 } from "solmate/utils/SafeTransferLib.sol";
import { LibString } from "solmate/utils/LibString.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { abs } from "solstat/Units.sol";
import { IDFMM } from "./interfaces/IDFMM.sol";
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

    /// @inheritdoc IDFMM
    Pool[] public pools;

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

    /// @inheritdoc IDFMM
    function init(InitParams calldata params)
        external
        payable
        lock
        returns (uint256, uint256, uint256, uint256)
    {
        if (params.tokenX == params.tokenY) revert InvalidTokens();

        Pool memory pool = Pool({
            strategy: params.strategy,
            tokenX: params.tokenX,
            tokenY: params.tokenY,
            reserveX: 0,
            reserveY: 0,
            totalLiquidity: 0,
            liquidityToken: address(0)
        });

        (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        ) = IStrategy(params.strategy).init(
            msg.sender, pools.length, pool, params.data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        LPToken liquidityToken = LPToken(clone(lpTokenImplementation));

        string memory tokenMetadata =
            _prepareTokenMetadata(params.strategy, params.tokenX, params.tokenY);
        liquidityToken.initialize(tokenMetadata, tokenMetadata);
        liquidityToken.mint(msg.sender, totalLiquidity - BURNT_LIQUIDITY);
        liquidityToken.mint(address(0), BURNT_LIQUIDITY);

        pool.reserveX = reserveX;
        pool.reserveY = reserveY;
        pool.totalLiquidity = totalLiquidity;
        pool.liquidityToken = address(liquidityToken);

        pools.push(pool);
        uint256 poolId = pools.length - 1;

        _transferFrom(params.tokenX, reserveX);
        _transferFrom(params.tokenY, reserveY);

        emitInit(poolId, address(liquidityToken));

        return (poolId, reserveX, reserveY, totalLiquidity - BURNT_LIQUIDITY);
    }

    function emitInit(uint256 poolId, address lpToken) private {
        Pool memory pool = pools[poolId];

        emit Init(
            msg.sender,
            pool.strategy,
            lpToken,
            pool.tokenX,
            pool.tokenY,
            poolId,
            pool.reserveX,
            pool.reserveY,
            pool.totalLiquidity
        );
    }

    function _prepareTokenMetadata(
        address strategy,
        address tokenX,
        address tokenY
    ) internal view returns (string memory) {
        return string.concat(
            "DFMM-",
            IStrategy(strategy).name(),
            "-",
            ERC20(tokenX).symbol(),
            "-",
            ERC20(tokenY).symbol(),
            "-",
            LibString.toString(pools.length)
        );
    }

    /// @inheritdoc IDFMM
    function allocate(
        uint256 poolId,
        bytes calldata data
    ) external payable lock returns (uint256, uint256) {
        (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        ) = IStrategy(pools[poolId].strategy).validateAllocate(
            msg.sender, poolId, pools[poolId], data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        pools[poolId].reserveX += deltaX;
        pools[poolId].reserveY += deltaY;
        pools[poolId].totalLiquidity += deltaLiquidity;

        _manageTokens(poolId, true, deltaLiquidity);

        _transferFrom(pools[poolId].tokenX, deltaX);
        _transferFrom(pools[poolId].tokenY, deltaY);

        emit Allocate(msg.sender, poolId, deltaX, deltaY, deltaLiquidity);
        return (deltaX, deltaY);
    }

    /// @inheritdoc IDFMM
    function deallocate(
        uint256 poolId,
        bytes calldata data
    ) external lock returns (uint256, uint256) {
        (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        ) = IStrategy(pools[poolId].strategy).validateDeallocate(
            msg.sender, poolId, pools[poolId], data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        pools[poolId].reserveX -= deltaX;
        pools[poolId].reserveY -= deltaY;
        pools[poolId].totalLiquidity -= deltaLiquidity;

        _manageTokens(poolId, false, deltaLiquidity);

        _transfer(pools[poolId].tokenX, msg.sender, deltaX);
        _transfer(pools[poolId].tokenY, msg.sender, deltaY);

        emit Deallocate(msg.sender, poolId, deltaX, deltaY, deltaLiquidity);
        return (deltaX, deltaY);
    }

    /// @inheritdoc IDFMM
    function swap(
        uint256 poolId,
        bytes calldata data
    ) external lock returns (uint256, uint256) {
        (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity,
            bool isSwapXForY
        ) = IStrategy(pools[poolId].strategy).validateSwap(
            msg.sender, poolId, pools[poolId], data
        );

        if (!valid) {
            revert Invalid(invariant < 0, abs(invariant));
        }

        pools[poolId].totalLiquidity += deltaLiquidity;

        if (isSwapXForY) {
            pools[poolId].reserveX += deltaX;
            pools[poolId].reserveY -= deltaY;
            _transferFrom(pools[poolId].tokenX, deltaX);
            _transfer(pools[poolId].tokenY, msg.sender, deltaY);
        } else {
            pools[poolId].reserveX -= deltaX;
            pools[poolId].reserveY += deltaY;
            _transferFrom(pools[poolId].tokenY, deltaY);
            _transfer(pools[poolId].tokenX, msg.sender, deltaX);
        }

        emit Swap(msg.sender, poolId, isSwapXForY, deltaX, deltaX);

        return (deltaX, deltaY);
    }

    /// @inheritdoc IDFMM
    function update(uint256 poolId, bytes calldata data) external lock {
        IStrategy(pools[poolId].strategy).update(
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
        returns (uint256, uint256, uint256)
    {
        return (
            pools[poolId].reserveX,
            pools[poolId].reserveY,
            pools[poolId].totalLiquidity
        );
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

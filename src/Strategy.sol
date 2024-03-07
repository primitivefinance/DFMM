/// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/interfaces/IStrategy.sol";

/**
 * @title Strategy base contract for DFMM.
 * @author Primitive
 */
abstract contract Strategy is IStrategy {
    /// @inheritdoc IStrategy
    address public immutable dfmm;

    int256 public constant EPSILON = 20;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    /// @dev Restricts the caller to the DFMM contract.
    modifier onlyDFMM() {
        if (msg.sender != address(dfmm)) revert NotDFMM();
        _;
    }

    /// @inheritdoc IStrategy
    function validateAllocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 maxDeltaX, uint256 maxDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        // TODO: This is a small trick because `deltaLiquidity` cannot be used
        // directly, let's fix this later.
        deltaLiquidity = deltaL;
        deltaX = _computeDeltaXGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );
        deltaY = _computeDeltaYGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );

        if (deltaX > maxDeltaX) {
            revert DeltaError(maxDeltaX, deltaX);
        }

        if (deltaY > maxDeltaY) {
            revert DeltaError(maxDeltaY, deltaY);
        }

        uint256 poolId = poolId;

        invariant = tradingFunction(
            pool.reserveX + deltaX,
            pool.reserveY + deltaY,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    /// @inheritdoc IStrategy
    function validateDeallocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (uint256 minDeltaX, uint256 minDeltaY, uint256 deltaL) =
            abi.decode(data, (uint256, uint256, uint256));

        deltaLiquidity = deltaL;
        deltaX = _computeDeltaXGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );
        deltaY = _computeDeltaYGivenDeltaL(
            deltaLiquidity, pool, getPoolParams(poolId)
        );

        if (minDeltaX > deltaX) {
            revert DeltaError(minDeltaX, deltaX);
        }

        if (minDeltaY > deltaY) {
            revert DeltaError(minDeltaY, deltaY);
        }

        uint256 poolId = poolId;

        invariant = tradingFunction(
            pool.reserveX - deltaX,
            pool.reserveY - deltaY,
            pool.totalLiquidity - deltaLiquidity,
            getPoolParams(poolId)
        );

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function validateSwap(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes memory data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity,
            bool isSwapXForY
        )
    {
        bytes memory params = getPoolParams(poolId);

        (deltaX, deltaY, deltaLiquidity, isSwapXForY) =
            abi.decode(data, (uint256, uint256, uint256, bool));

        if (isSwapXForY) {
            invariant = tradingFunction(
                pool.reserveX + deltaX,
                pool.reserveY - deltaY,
                pool.totalLiquidity + deltaLiquidity,
                params
            );
        } else {
            invariant = tradingFunction(
                pool.reserveX - deltaX,
                pool.reserveY + deltaY,
                pool.totalLiquidity + deltaLiquidity,
                params
            );
        }

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        virtual
        returns (bytes memory);

    function tradingFunction(
        uint256 reserveX,
        uint256 reserveY,
        uint256 totalLiquidity,
        bytes memory params
    ) public view virtual returns (int256);

    function _computeDeltaXGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory data
    ) internal view virtual returns (uint256);

    function _computeDeltaYGivenDeltaL(
        uint256 deltaLiquidity,
        IDFMM.Pool calldata pool,
        bytes memory data
    ) internal view virtual returns (uint256);
}

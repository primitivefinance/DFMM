// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./ConstantSum.sol";
import "./ConstantSumLib.sol";
import "src/interfaces/IStrategy.sol";
import "src/interfaces/IDFMM.sol";
import "src/lib/StrategyLib.sol";
import "solmate/tokens/ERC20.sol";

contract ConstantSumSolver {
    error NotEnoughLiquidity();

    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    address public strategy;

    constructor(address strategy_) {
        strategy = strategy_;
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 ry,
        ConstantSum.ConstantSumParams memory params
    ) public pure returns (bytes memory) {
        // The pool can be initialized with any non-negative amount of rx, and ry.
        // so we have to allow a user to pass an amount of both even if one is zero.
        uint256 L = rx + ry.divWadUp(params.price);
        return abi.encode(rx, ry, L, params);
    }

    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
        ConstantSum.ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (ConstantSum.ConstantSumParams)
        );
        uint256 amountOut;
        if (swapXIn) {
            uint256 deltaL = amountIn.mulWadUp(poolParams.swapFee);

            amountOut = amountIn.mulWadDown(poolParams.price).mulWadDown(
                ONE - poolParams.swapFee
            );

            endReserves.rx = startReserves.rx + amountIn;
            endReserves.L = startReserves.L + deltaL;

            if (startReserves.ry < amountOut) {
                revert NotEnoughLiquidity();
            }
            endReserves.ry = startReserves.ry - amountOut;
        } else {
            uint256 deltaL =
                amountIn.mulWadUp(poolParams.swapFee).divWadUp(poolParams.price);

            amountOut = (ONE - poolParams.swapFee).mulWadDown(amountIn)
                .divWadDown(poolParams.price);

            endReserves.ry = startReserves.ry + amountIn;
            endReserves.L = startReserves.L + deltaL;

            if (startReserves.rx < amountOut) {
                revert NotEnoughLiquidity();
            }
            endReserves.rx = startReserves.rx - amountOut;
        }

        IDFMM.Pool memory pool;
        pool.reserveX = startReserves.rx;
        pool.reserveY = startReserves.rx;
        pool.totalLiquidity = startReserves.L;

        bytes memory swapData = abi.encode(endReserves);
        (bool valid,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (valid, amountOut, swapData);
    }

    function simulateAllocate(
        uint256 poolId,
        uint256 amountX,
        uint256 amountY
    ) public view returns (bool, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
        ConstantSum.ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (ConstantSum.ConstantSumParams)
        );

        endReserves.rx = startReserves.rx + amountX;
        endReserves.ry = startReserves.ry + amountY;
        endReserves.L =
            endReserves.rx + endReserves.ry.divWadUp(poolParams.price);

        IDFMM.Pool memory pool;
        pool.reserveX = startReserves.rx;
        pool.reserveY = startReserves.ry;
        pool.totalLiquidity = startReserves.L;

        bytes memory allocateData = abi.encode(endReserves);
        (bool valid,,,,) = IStrategy(strategy).validateAllocate(
            address(this), poolId, pool, allocateData
        );
        return (valid, allocateData);
    }

    function simulateDeallocate(
        uint256 poolId,
        uint256 amountX,
        uint256 amountY
    ) public view returns (bool, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
        ConstantSum.ConstantSumParams memory poolParams = abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (ConstantSum.ConstantSumParams)
        );

        if (startReserves.rx < amountX || startReserves.ry < amountY) {
            revert NotEnoughLiquidity();
        }
        endReserves.rx = startReserves.rx - amountX;
        endReserves.ry = startReserves.ry - amountY;
        endReserves.L =
            endReserves.rx + endReserves.ry.divWadUp(poolParams.price);

        IDFMM.Pool memory pool;
        pool.reserveX = startReserves.rx;
        pool.reserveY = startReserves.ry;
        pool.totalLiquidity = startReserves.L;

        bytes memory deallocateData = abi.encode(endReserves);
        (bool valid,,,,) = IStrategy(strategy).validateDeallocate(
            address(this), poolId, pool, deallocateData
        );
        return (valid, deallocateData);
}
    function preparePriceUpdate(uint256 newPrice) public pure returns (bytes memory) {
        return ConstantSumLib.encodePriceUpdate(newPrice);
    }
}

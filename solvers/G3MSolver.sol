// SPDX-LICENSE-Identifier: MIT
pragma solidity ^0.8.13;

import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";
import "../lib/BisectionLib.sol";
import "../lib/g3m/G3MExtendedLib.sol";
import "../interfaces/IStrategyLike.sol";

contract G3MSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    uint256 public constant BISECTION_EPSILON = 1;
    uint256 public constant MAX_BISECTION_ITERS = 90;

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (G3mParameters memory)
    {
        return G3MStrategyLike(strategy).dynamicSlotInternal(poolId);
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return IStrategyLike(strategy).getReservesAndLiquidity(poolId);
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        G3mParameters memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function allocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(true, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (nextRx, nextRy, nextL);
    }

    function allocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(true, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (nextRx, nextRy, nextL);
    }

    function getNextLiquidity(
        uint256 poolId,
        uint256 rx,
        uint256 ry
    ) public view returns (uint256) {
        return computeNextLiquidity(rx, ry, getPoolParams(poolId));
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRx(ry, L, getPoolParams(poolId));
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRy(rx, L, getPoolParams(poolId));
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        Reserves memory startReserves;
        Reserves memory endReserves;
        (startReserves.rx, startReserves.ry, startReserves.L) =
            IStrategyLike(strategy).getReservesAndLiquidity(poolId);
        G3mParameters memory poolParams = getPoolParams(poolId);

        uint256 amountOut;
        {
            uint256 swapFee = IStrategyLike(strategy).swapFee();
            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.ry.divWadUp(startReserves.rx)).powWad(
                        int256(poolParams.wy)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startReserves.L + deltaL;

                endReserves.ry =
                    getNextReserveY(poolId, endReserves.rx, endReserves.L);
                endReserves.ry += 1;

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                uint256 fees = amountIn.mulWadUp(swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.rx.divWadUp(startReserves.ry)).powWad(
                        int256(poolParams.wx)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startReserves.L + deltaL;

                endReserves.rx =
                    getNextReserveX(poolId, endReserves.ry, endReserves.L);
                endReserves.rx += 1;

                require(
                    endReserves.rx < startReserves.rx,
                    "invalid swap: x reserve increased!"
                );
                amountOut = startReserves.rx - endReserves.rx;
            }
        }

        bytes memory swapData =
            abi.encode(endReserves.rx, endReserves.ry, endReserves.L);
        (bool valid,,,,,) =
            IStrategyLike(strategy).validateSwap(poolId, swapData);
        return (
            valid,
            amountOut,
            computePrice(endReserves.rx, endReserves.ry, poolParams),
            swapData
        );
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        G3mParameters memory params = getPoolParams(poolId);
        (uint256 rx, uint256 ry,) =
            IStrategyLike(strategy).getReservesAndLiquidity(poolId);
        price = computePrice(rx, ry, params);
    }
}

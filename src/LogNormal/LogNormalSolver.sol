// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./LogNormalExtendedLib.sol";
import "../lib/BisectionLib.sol";
import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "solmate/tokens/ERC20.sol";
import "solstat/Gaussian.sol";

contract LogNormalSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    uint256 public constant BISECTION_EPSILON = 0;
    uint256 public constant MAX_BISECTION_ITERS = 120;

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (LogNormal.LogNormalParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId),
            (LogNormal.LogNormalParams)
        );
    }

    function prepareFeeUpdate(uint256 swapFee)
        external
        pure
        returns (bytes memory)
    {
        return LogNormalLib.encodeFeeUpdate(swapFee);
    }

    function prepareStrikeUpdate(
        uint256 targetStrike,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return LogNormalLib.encodeStrikeUpdate(targetStrike, targetTimestamp);
    }

    function prepareSigmaUpdate(
        uint256 targetSigma,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return LogNormalLib.encodeSigmaUpdate(targetSigma, targetTimestamp);
    }

    function prepareTauUpdate(
        uint256 targetTau,
        uint256 targetTimestamp
    ) external pure returns (bytes memory) {
        return LogNormalLib.encodeTauUpdate(targetTau, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        external
        pure
        returns (bytes memory)
    {
        return LogNormalLib.encodeControllerUpdate(controller);
    }

    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256, uint256, uint256)
    {
        return IDFMM(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
    }

    function getInitialPoolData(
        uint256 rx,
        uint256 S,
        LogNormal.LogNormalParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
    }

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256 deltaY, uint256 deltaLiquidity) {
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 adjustedReserveX, uint256 adjustedLiquidity) =
            computeAllocationGivenX(true, deltaX, reserveX, liquidity);
        uint256 approximatedPrice =
            getPriceGivenXL(poolId, adjustedReserveX, adjustedLiquidity);
        uint256 adjustedReserveY = getNextReserveY(
            poolId, adjustedReserveX, adjustedLiquidity, approximatedPrice
        );
        deltaY = adjustedReserveY - reserveY;
        deltaLiquidity = adjustedLiquidity - liquidity;
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256 deltaX, uint256 deltaLiquidity) {
        (uint256 reserveX, uint256 reserveY, uint256 liquidity) =
            getReservesAndLiquidity(poolId);
        (uint256 adjustedReserveY, uint256 adjustedLiquidity) =
            computeAllocationGivenY(true, deltaY, reserveY, liquidity);
        uint256 approximatedPrice =
            getPriceGivenYL(poolId, adjustedReserveY, adjustedLiquidity);
        uint256 adjustedReserveX = getNextReserveX(
            poolId, adjustedReserveY, adjustedLiquidity, approximatedPrice
        );
        deltaX = adjustedReserveX - reserveX;
        deltaLiquidity = adjustedLiquidity - liquidity;
    }

    function allocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(true, amountX, rx, L);
        uint256 approximatedPrice = getPriceGivenXL(poolId, nextRx, nextL);
        uint256 nextRy =
            getNextReserveY(poolId, nextRx, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function allocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(true, amountY, ry, L);
        uint256 approximatedPrice = getPriceGivenYL(poolId, nextRy, nextL);
        uint256 nextRx =
            getNextReserveX(poolId, nextRy, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenX(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, rx, L);
        uint256 approximatedPrice = getPriceGivenXL(poolId, nextRx, nextL);
        uint256 nextRy =
            getNextReserveY(poolId, nextRx, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function deallocateGivenY(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, ry, L);
        uint256 approximatedPrice = getPriceGivenYL(poolId, nextRy, nextL);
        uint256 nextRx =
            getNextReserveX(poolId, nextRy, nextL, approximatedPrice);
        return (nextRx, nextRy, nextL);
    }

    function getNextLiquidity(
        uint256 poolId,
        uint256 rx,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        bytes memory data = abi.encode(rx, ry, L);
        int256 invariant = IStrategy(strategy).computeSwapConstant(poolId, data);
        return
            computeNextLiquidity(rx, ry, invariant, L, fetchPoolParams(poolId));
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        uint256 approximatedRx = computeXGivenL(L, S, fetchPoolParams(poolId));
        bytes memory data = abi.encode(approximatedRx, ry, L);
        int256 invariant = IStrategy(strategy).computeSwapConstant(poolId, data);
        return computeNextRx(
            ry, L, invariant, approximatedRx, fetchPoolParams(poolId)
        );
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L,
        uint256 S
    ) public view returns (uint256) {
        uint256 approximatedRy = computeYGivenL(L, S, fetchPoolParams(poolId));
        bytes memory data = abi.encode(rx, approximatedRy, L);
        int256 invariant = IStrategy(strategy).computeSwapConstant(poolId, data);
        return computeNextRy(
            rx, L, invariant, approximatedRy, fetchPoolParams(poolId)
        );
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
            getReservesAndLiquidity(poolId);
        LogNormal.LogNormalParams memory poolParams = fetchPoolParams(poolId);

        uint256 amountOut;
        {
            uint256 startComputedL = getNextLiquidity(
                poolId, startReserves.rx, startReserves.ry, startReserves.L
            );

            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedL).divWadUp(startReserves.rx);
                deltaL += 1;

                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startComputedL + deltaL;
                uint256 approxPrice =
                    getPriceGivenXL(poolId, endReserves.rx, endReserves.L);

                endReserves.ry = getNextReserveY(
                    poolId, endReserves.rx, endReserves.L, approxPrice
                );
                endReserves.ry += 1;

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
                uint256 deltaL =
                    fees.mulWadUp(startComputedL).divWadUp(startReserves.ry);
                deltaL += 1;

                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startComputedL + deltaL;
                uint256 approxPrice =
                    getPriceGivenYL(poolId, endReserves.ry, endReserves.L);

                endReserves.rx = getNextReserveX(
                    poolId, endReserves.ry, endReserves.L, approxPrice
                );
                endReserves.rx += 1;

                require(
                    endReserves.rx < startReserves.rx,
                    "invalid swap: x reserve increased!"
                );
                amountOut = startReserves.rx - endReserves.rx;
            }
        }

        IDFMM.Pool memory pool;
        pool.reserveX = startReserves.rx;
        pool.reserveY = startReserves.ry;
        pool.totalLiquidity = startReserves.L;

        bytes memory swapData =
            abi.encode(endReserves.rx, endReserves.ry, endReserves.L);

        uint256 poolId = poolId;
        (bool valid,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (
            valid,
            amountOut,
            LogNormalLib.computePriceGivenX({
                rx: endReserves.rx,
                L: endReserves.L,
                params: poolParams
            }),
            swapData
        );
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        price = LogNormalLib.computePriceGivenX(rx, L, fetchPoolParams(poolId));
    }

    function getPriceGivenYL(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256 price) {
        price = LogNormalLib.computePriceGivenY(ry, L, fetchPoolParams(poolId));
    }

    function getPriceGivenXL(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256 price) {
        price = LogNormalLib.computePriceGivenX(rx, L, fetchPoolParams(poolId));
    }

    function calculateDiffLower(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffLower(S, rx, ry, L, v, params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffRaise(S, rx, ry, L, v, params);
    }

    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalLower(S, rx, ry, L, vUpper, params);
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalRaise(S, rx, ry, L, vUpper, params);
    }
}

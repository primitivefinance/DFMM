// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./G3MExtendedLib.sol";
import "solmate/tokens/ERC20.sol";
import "src/interfaces/IStrategy.sol";
import "src/interfaces/IDFMM.sol";

contract GeometricMeanSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256 rx;
        uint256 ry;
        uint256 L;
    }

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return GeometricMeanLib.encodeFeeUpdate(swapFee);
    }

    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return
            GeometricMeanLib.encodeWeightXUpdate(targetWeightX, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        public
        pure
        returns (bytes memory)
    {
        return GeometricMeanLib.encodeControllerUpdate(controller);
    }

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (GeometricMeanParams)
        );
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
        GeometricMeanParams memory params
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

    function allocateGivenDeltaX(
        uint256 poolId,
        uint256 deltaX
    ) public view returns (uint256, uint256) {
        (uint256 reserveX, uint256 reserveY,) = getReservesAndLiquidity(poolId);
        GeometricMeanParams memory params = getPoolParams(poolId);
        uint256 S = computePrice(reserveX, reserveY, params);
        uint256 deltaLiquidity = computeLGivenX(deltaX, S, params);
        uint256 deltaY = computeY(deltaX, S, params);
        return (deltaY, deltaLiquidity);
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

    function deallocateGivenXReturnDeltas(
        uint256 poolId,
        uint256 amountX
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRx, uint256 nextL) =
            computeAllocationGivenX(false, amountX, rx, L);
        uint256 nextRy = getNextReserveY(poolId, nextRx, nextL);
        return (rx - nextRx, ry - nextRy, L - nextL);
    }

    function deallocateGivenYReturnDeltas(
        uint256 poolId,
        uint256 amountY
    ) public view returns (uint256, uint256, uint256) {
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        (uint256 nextRy, uint256 nextL) =
            computeAllocationGivenX(false, amountY, ry, L);
        uint256 nextRx = getNextReserveX(poolId, nextRy, nextL);
        return (rx - nextRx, ry - nextRy, L - nextL);
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
        return GeometricMeanLib.computeNextLiquidity(
            rx, ry, fetchPoolParams(poolId)
        );
    }

    function getNextReserveX(
        uint256 poolId,
        uint256 ry,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRx(ry, L, fetchPoolParams(poolId));
    }

    function getNextReserveY(
        uint256 poolId,
        uint256 rx,
        uint256 L
    ) public view returns (uint256) {
        return computeNextRy(rx, L, fetchPoolParams(poolId));
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
        GeometricMeanParams memory poolParams = fetchPoolParams(poolId);

        uint256 amountOut;

        uint256 startComputedL = GeometricMeanLib.computeNextLiquidity(
            startReserves.rx, startReserves.ry, fetchPoolParams(poolId)
        );

        {
            if (swapXIn) {
                uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.ry.divWadUp(startReserves.rx)).powWad(
                        int256(poolParams.wY)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startComputedL + deltaL;

                endReserves.ry =
                    getNextReserveY(poolId, endReserves.rx, endReserves.L);
                endReserves.ry += 1;

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                uint256 fees = amountIn.mulWadUp(poolParams.swapFee);
                uint256 weightedPrice = uint256(
                    int256(startReserves.rx.divWadUp(startReserves.ry)).powWad(
                        int256(poolParams.wX)
                    )
                );
                uint256 deltaL = fees.mulWadUp(weightedPrice);
                deltaL += 1;

                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startComputedL + deltaL;

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

        IDFMM.Pool memory pool;
        pool.reserveX = startReserves.rx;
        pool.reserveY = startReserves.ry;
        pool.totalLiquidity = startComputedL;

        uint256 poolId = poolId;
        (bool valid,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );
        return (
            valid,
            amountOut,
            computePrice(endReserves.rx, endReserves.ry, poolParams),
            swapData
        );
    }

    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalLower(S, rx, ry, L, vUpper, params);
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalRaise(S, rx, ry, L, vUpper, params);
    }

    function calculateDiffLower(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffLower(S, rx, ry, L, v, params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffRaise(S, rx, ry, L, v, params);
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry,) = getReservesAndLiquidity(poolId);
        price = computePrice(rx, ry, params);
    }

    function checkSwapConstant(
        uint256 poolId,
        bytes calldata data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        GeometricMeanParams memory params = fetchPoolParams(poolId);
        return GeometricMeanLib.tradingFunction(rx, ry, L, params);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (GeometricMeanParams memory params)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (GeometricMeanParams)
        );
    }
}

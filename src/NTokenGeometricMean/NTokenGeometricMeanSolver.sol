// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./NTokenGeometricMeanExtendedLib.sol";
import "solmate/tokens/ERC20.sol";
import "src/interfaces/IStrategy2.sol";
import "forge-std/console2.sol";
import { IDFMM2 } from "src/interfaces/IDFMM2.sol";

contract NTokenGeometricMeanSolver {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    /// @dev Structure to hold reserve information
    struct Reserves {
        uint256[] reserves;
        uint256 L;
    }

    address public strategy;

    constructor(address _strategy) {
        strategy = _strategy;
    }

    function computeInitialPoolData(
        uint256 numeraireAmount,
        uint256[] memory prices,
        NTokenGeometricMeanParams memory params
    ) public view returns (bytes memory) {

        uint256[] memory reserves = new uint256[](prices.length);
        uint256 numerairePrice = prices[prices.length - 1];
        uint256 numeraireWeight = params.weights[params.weights.length - 1];
        for (uint256 i = 0; i < prices.length - 1; i++) {
          // compute the amount of reserves for token T given numeraireAmount and weights wT and wNumeraire
          uint256 amountT = computeReserveFromNumeraire(numeraireAmount, numerairePrice, params.weights[i], numeraireWeight);
          reserves[i] = amountT;
        }
        reserves[prices.length - 1] = ONE;

        uint256 L = computeNextLiquidity(reserves, params);

        return
            abi.encode(reserves, L, params.weights, params.swapFee, params.controller);
    }

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (NTokenGeometricMeanParams memory)
    {
        return abi.decode(
            IStrategy2(strategy).getPoolParams(poolId), (NTokenGeometricMeanParams)
        );
    }

    struct SimulateSwapState {
        uint256 amountIn;
        uint256 amountOut;
        uint256 inReserve;
        uint256 outReserve;
        uint256 inWeight;
        uint256 outWeight;
        uint256 deltaLiquidity;
        uint256 fees;
    }


    function simulateSwap(
        uint256 poolId,
        uint256 tokenInIndex,
        uint256 tokenOutIndex,
        uint256 amountIn
    ) public view returns (bool, uint256, bytes memory) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        IDFMM2.Pool memory pool =
            IDFMM2(IStrategy2(strategy).dfmm()).getPool(poolId);

        SimulateSwapState memory state;

        state.inReserve = pool.reserves[tokenInIndex];
        state.outReserve = pool.reserves[tokenOutIndex];
        state.inWeight = params.weights[tokenInIndex];
        state.outWeight = params.weights[tokenOutIndex];

        state.fees = amountIn.mulWadUp(params.swapFee);
        state.deltaLiquidity = pool.totalLiquidity.divWadUp(state.inReserve).mulWadUp(state.fees).mulWadUp(state.inWeight);
        {
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 accumulator = ONE;
            for (uint256 i = 0; i < pool.reserves.length; i++) {
              if (i != tokenOutIndex && i != tokenInIndex) {
                uint256 di = uint256(int256(pool.reserves[i]).powWad(int256(params.weights[i])));
                accumulator = accumulator.mulWadUp(di);
              }
            }
            uint256 d = uint256(
                int256((state.inReserve + amountIn)).powWad(int256(state.inWeight))
            );
            uint256 a = uint256(
                int256(n.divWadUp(d.mulWadUp(accumulator))).powWad(
                    int256(ONE.divWadUp(state.outWeight))
                )
            );

            state.amountOut = state.outReserve - a;
        }

        bytes memory swapData = abi.encode(tokenInIndex, tokenOutIndex, amountIn, state.amountOut, state.deltaLiquidity);

        (bool valid,,,,,,) = IStrategy2(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (
            valid,
            state.amountOut,
            swapData
        );
    }


    /*
    function getReservesAndLiquidity(uint256 poolId)
        public
        view
        returns (uint256[] memory, uint256)
    {
        return DFMM2(IStrategy(strategy).dfmm()).getReservesAndLiquidity(poolId);
    }
    */


    /*

    function prepareFeeUpdate(uint256 swapFee)
        public
        pure
        returns (bytes memory data)
    {
        return NTokenGeometricMeanLib.encodeFeeUpdate(swapFee);
    }

    function prepareWeightXUpdate(
        uint256 targetWeightX,
        uint256 targetTimestamp
    ) public pure returns (bytes memory) {
        return
            NTokenGeometricMeanLib.encodeWeightXUpdate(targetWeightX, targetTimestamp);
    }

    function prepareControllerUpdate(address controller)
        public
        pure
        returns (bytes memory)
    {
        return NTokenGeometricMeanLib.encodeControllerUpdate(controller);
    }

    function fetchPoolParams(uint256 poolId)
        public
        view
        returns (NTokenGeometricMeanParams memory)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (NTokenGeometricMeanParams)
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
        uint256 numeraireAmount,
        uint256[] calldata prices,
        NTokenGeometricMeanParams memory params
    ) public pure returns (bytes memory) {
        return computeInitialPoolDataFromPrices(numeraireAmount, prices, params);
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
        NTokenGeometricMeanParams memory params = getPoolParams(poolId);
        uint256 S = computePrice(reserveX, reserveY, params);
        uint256 deltaLiquidity = computeLGivenX(deltaX, S, params);
        uint256 deltaY = computeY(deltaX, S, params);
        return (deltaY, deltaLiquidity);
    }

    function allocateGivenDeltaY(
        uint256 poolId,
        uint256 deltaY
    ) public view returns (uint256, uint256) {
        (uint256 reserveX, uint256 reserveY,) = getReservesAndLiquidity(poolId);
        NTokenGeometricMeanParams memory params = getPoolParams(poolId);
        uint256 S = computePrice(reserveX, reserveY, params);
        uint256 deltaLiquidity = computeLGivenY(deltaY, S, params);
        uint256 deltaX = computeReserveFromNumeraire(deltaY, S, params);
        return (deltaX, deltaLiquidity);
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
        return NTokenGeometricMeanLib.computeNextLiquidity(
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

    struct SimulateSwapState {
        uint256 amountIn;
        uint256 amountOut;
        uint256 deltaLiquidity;
        uint256 fees;
    }

    /// @dev Estimates a swap's reserves and adjustments and returns its validity.
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    ) public view returns (bool, uint256, uint256, bytes memory) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        IDFMM.Pool memory pool =
            IDFMM(IStrategy(strategy).dfmm()).getPool(poolId);

        SimulateSwapState memory state;

        state.fees = amountIn.mulWadUp(params.swapFee);

        if (swapXIn) {
            state.deltaLiquidity = pool.totalLiquidity.divWadUp(pool.reserveX)
                .mulWadUp(state.fees).mulWadUp(params.wX);
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 d = uint256(
                int256((pool.reserveX + amountIn)).powWad(int256(params.wX))
            );
            uint256 a = uint256(
                int256(n.divWadUp(d)).powWad(
                    int256(FixedPointMathLib.WAD.divWadUp(params.wY))
                )
            );
            state.amountOut = pool.reserveY - a;
        } else {
            state.deltaLiquidity = pool.totalLiquidity.divWadUp(pool.reserveY)
                .mulWadUp(state.fees).mulWadUp(params.wY);
            uint256 n = (pool.totalLiquidity + state.deltaLiquidity);
            uint256 d = uint256(
                int256((pool.reserveY + amountIn)).powWad(int256(params.wY))
            );
            uint256 a = uint256(
                int256(n.divWadUp(d)).powWad(
                    int256(FixedPointMathLib.WAD.divWadUp(params.wX))
                )
            );
            state.amountOut = pool.reserveX - a;
        }

        bytes memory swapData;

        if (swapXIn) {
            swapData = abi.encode(
                amountIn, state.amountOut, state.deltaLiquidity, true
            );
        } else {
            swapData = abi.encode(
                state.amountOut, amountIn, state.deltaLiquidity, false
            );
        }

        (bool valid,,,,,) = IStrategy(strategy).validateSwap(
            address(this), poolId, pool, swapData
        );

        return (
            valid,
            state.amountOut,
            computePrice(pool.reserveX, pool.reserveY, params),
            swapData
        );
    }

    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalLower(S, rx, ry, L, vUpper, params);
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalRaise(S, rx, ry, L, vUpper, params);
    }

    function calculateDiffLower(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffLower(S, rx, ry, L, v, params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffRaise(S, rx, ry, L, v, params);
    }

    /// @dev Computes the internal price using this strategie's slot parameters.
    function internalPrice(uint256 poolId)
        public
        view
        returns (uint256 price)
    {
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        (uint256 rx, uint256 ry,) = getReservesAndLiquidity(poolId);
        price = computePrice(rx, ry, params);
    }

    function checkSwapConstant(
        uint256 poolId,
        bytes calldata data
    ) public view returns (int256) {
        (uint256 rx, uint256 ry, uint256 L) =
            abi.decode(data, (uint256, uint256, uint256));
        NTokenGeometricMeanParams memory params = fetchPoolParams(poolId);
        return NTokenGeometricMeanLib.tradingFunction(rx, ry, L, params);
    }

    function getPoolParams(uint256 poolId)
        public
        view
        returns (NTokenGeometricMeanParams memory params)
    {
        return abi.decode(
            IStrategy(strategy).getPoolParams(poolId), (NTokenGeometricMeanParams)
        );
    }
    */
}

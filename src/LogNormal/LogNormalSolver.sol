// SPDX-License-Identifier: GPL-3.0-or-latersolver
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
        uint256 approxPrice;
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
    ) public view returns (bytes memory) {
        return computeInitialPoolData(rx, S, params);
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
        console2.log("rx in nextL", rx);
        console2.log("L in nextL", L);

        int256 invariant = IStrategy(strategy).computeSwapConstant(poolId, data);
        console2.log("invariant", invariant);
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

    function computeDeltaL(
      uint256 amountIn,
      uint256 swapFee,
      uint256 L,
      uint256 reserve
    ) public view returns (uint256 deltaL) {
      deltaL = amountIn.mulWadUp(swapFee).mulWadUp(L).divWadUp(reserve).mulWadUp(HALF);
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
            console2.log("start rx", startReserves.rx);
            console2.log("start ry", startReserves.ry);
            console2.log("start L", startReserves.L);
            console2.log("computing next L");
            uint256 startComputedL = getNextLiquidity(
                poolId, startReserves.rx, startReserves.ry, startReserves.L
            );
            console2.log("computed next L");

            if (swapXIn) {
                endReserves.rx = startReserves.rx + amountIn;
                endReserves.L = startComputedL + computeDeltaL(amountIn, poolParams.swapFee, startReserves.L, startReserves.rx);
                endReserves.approxPrice =
                    getPriceGivenXL(poolId, endReserves.rx, endReserves.L);

            console2.log("computing next Y");
                endReserves.ry = getNextReserveY(
                    poolId, endReserves.rx, endReserves.L, endReserves.approxPrice
                );
            console2.log("computed next Y");

                require(
                    endReserves.ry < startReserves.ry,
                    "invalid swap: y reserve increased!"
                );
                amountOut = startReserves.ry - endReserves.ry;
            } else {
                endReserves.ry = startReserves.ry + amountIn;
                endReserves.L = startComputedL + computeDeltaL(amountIn, poolParams.swapFee, startReserves.L, startReserves.ry);
                endReserves.approxPrice =
                    getPriceGivenYL(poolId, endReserves.ry, endReserves.L);

              console2.log("computing next X");
                endReserves.rx = getNextReserveX(
                    poolId, endReserves.ry, endReserves.L, endReserves.approxPrice
                );
              console2.log("computed next X");

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
            IStrategy(strategy).validateSwap(address(this), poolId, swapData);
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
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        return diffLower(int256(S), int256(rx), int256(L), int256(v), params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return diffRaise(int256(S), int256(ry), int256(L), int256(v), params);
    }

    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalLower(int256(S), int256(rx), int256(L), vUpper, params);
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeOptimalRaise(int256(S), int256(ry), int256(L), vUpper, params);
    }

    function getDyGivenS(
      uint256 poolId,
      uint256 S
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = getReservesAndLiquidity(poolId);
        return computeDy(int256(S), int256(ry), int256(L), params);
    }

    function getDxGivenS(
      uint256 poolId,
      uint256 S
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = fetchPoolParams(poolId);
        (uint256 rx,, uint256 L) = getReservesAndLiquidity(poolId);
        return computeDx(int256(S), int256(rx), int256(L), params);
    }

    function callIerfc(int256 x) public view returns (int256) {
      return testIerfc(x);
    }
    
    function solverTradingFunction(
        uint256 rx,
        uint256 ry,
        uint256 L,
        LogNormal.LogNormalParams memory params
    ) public view returns (int256) {
        require(rx < L, "tradingFunction: invalid x");

        int256 AAAAA;
        int256 BBBBB;
        if (FixedPointMathLib.divWadDown(rx, L) >= ONE) {
            AAAAA = int256(2 ** 255 - 1);
        } else {
            AAAAA = Gaussian.ppf(int256(FixedPointMathLib.divWadDown(rx, L)));
            console2.log("a", AAAAA);
        }
        if (
            FixedPointMathLib.divWadDown(
                ry, FixedPointMathLib.mulWadDown(params.strike, L)
            ) >= ONE
        ) {
            console2.log("in max int branch");
            BBBBB = int256(2 ** 255 - 1);
            console2.log("b", BBBBB);
        } else {
            BBBBB = Gaussian.ppf(
                int256(
                    FixedPointMathLib.divWadDown(
                        ry, FixedPointMathLib.mulWadDown(params.strike, L)
                    )
                )
            );
        }

        int256 CCCCC = int256(computeSigmaSqrtTau(params.sigma, params.tau));
        console2.log("c", CCCCC);

        console2.log("a + c", AAAAA + CCCCC);
        console2.log("a + b + c", AAAAA + BBBBB + CCCCC);

        return AAAAA + BBBBB + CCCCC;
    }

}

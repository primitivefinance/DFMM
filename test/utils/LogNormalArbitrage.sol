// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "../../src/LogNormal/LogNormalExtendedLib.sol";

interface SolverLike {
    function simulateSwap(
        uint256 poolId,
        bool swapXIn,
        uint256 amountIn
    )
        external
        view
        returns (
            bool valid,
            uint256 estimatedOut,
            uint256 estimatedPrice,
            bytes memory payload
        );
    function internalPrice(uint256 poolId) external view returns (uint256);
    function getReservesAndLiquidity(uint256 poolId)
        external
        view
        returns (uint256, uint256, uint256);
    function strategy() external view returns (address);
    function fetchPoolParams(uint256 poolId)
        external
        view
        returns (LogNormal.LogNormalParams memory);
}

contract LogNormalArbitrage {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;

    SolverLike solver;

    constructor(address solver_) {
        solver = SolverLike(solver);
    }

    function calculateDiffLower(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (uint256 rx,, uint256 L) = solver.getReservesAndLiquidity(poolId);
        return diffLower(int256(S), int256(rx), int256(L), int256(v), params);
    }

    function calculateDiffRaise(
        uint256 poolId,
        uint256 S,
        uint256 v
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = solver.getReservesAndLiquidity(poolId);
        return diffRaise(int256(S), int256(ry), int256(L), int256(v), params);
    }

    function computeOptimalArbLowerPrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (uint256 rx,, uint256 L) = solver.getReservesAndLiquidity(poolId);
        return computeOptimalLower(
            int256(S), int256(rx), int256(L), vUpper, params
        );
    }

    function computeOptimalArbRaisePrice(
        uint256 poolId,
        uint256 S,
        uint256 vUpper
    ) public view returns (uint256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = solver.getReservesAndLiquidity(poolId);
        return computeOptimalRaise(
            int256(S), int256(ry), int256(L), vUpper, params
        );
    }

    function getDyGivenS(
        uint256 poolId,
        uint256 S
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (, uint256 ry, uint256 L) = solver.getReservesAndLiquidity(poolId);
        int256 dy = computeDy(int256(S), int256(ry), int256(L), params);
        return dy;
    }

    function getDxGivenS(
        uint256 poolId,
        uint256 S
    ) public view returns (int256) {
        LogNormal.LogNormalParams memory params = solver.fetchPoolParams(poolId);
        (uint256 rx,, uint256 L) = solver.getReservesAndLiquidity(poolId);
        return computeDx(int256(S), int256(rx), int256(L), params);
    }

    function findRootLower(
        bytes memory data,
        uint256 v
    ) internal pure returns (int256) {
        (
            uint256 S,
            uint256 rX,
            uint256 L,
            LogNormal.LogNormalParams memory params
        ) = abi.decode(
            data, (uint256, uint256, uint256, LogNormal.LogNormalParams)
        );
        return diffLower({
            S: int256(S),
            rX: int256(rX),
            L: int256(L),
            v: int256(v),
            params: params
        });
    }

    function findRootRaise(
        bytes memory data,
        uint256 v
    ) internal pure returns (int256) {
        (
            uint256 S,
            uint256 rY,
            uint256 L,
            LogNormal.LogNormalParams memory params
        ) = abi.decode(
            data, (uint256, uint256, uint256, LogNormal.LogNormalParams)
        );
        return diffRaise({
            S: int256(S),
            rY: int256(rY),
            L: int256(L),
            v: int256(v),
            params: params
        });
    }

    struct DiffLowerStruct {
        int256 ierfcResult;
        int256 strike;
        int256 sigma;
        int256 tau;
        int256 gamma;
        int256 rX;
        int256 L;
        int256 v;
        int256 S;
        int256 sqrtTau;
        int256 sqrtTwo;
    }

    function createDiffLowerStruct(
        int256 S,
        int256 rx,
        int256 L,
        int256 gamma,
        int256 v,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (DiffLowerStruct memory) {
        int256 a = I_TWO.wadMul(v + rx);
        int256 b = L + v - v.wadMul(gamma);
        int256 ierfcRes = Gaussian.ierfc(a.wadDiv(b));

        int256 sqrtTwo = int256(FixedPointMathLib.sqrt(TWO) * 1e9);
        int256 sqrtTau = int256(FixedPointMathLib.sqrt(params.tau) * 1e9);

        DiffLowerStruct memory ints = DiffLowerStruct({
            ierfcResult: ierfcRes,
            strike: int256(params.strike),
            sigma: int256(params.sigma),
            tau: int256(params.tau),
            gamma: gamma,
            rX: rx,
            L: L,
            v: v,
            S: S,
            sqrtTwo: sqrtTwo,
            sqrtTau: sqrtTau
        });

        return ints;
    }

    function computeLowerA(DiffLowerStruct memory params)
        internal
        pure
        returns (int256)
    {
        int256 firstExp = -(params.sigma.wadMul(params.sigma).wadMul(params.tau).wadDiv(I_TWO));
        int256 secondExp = params.sqrtTwo.wadMul(params.sigma).wadMul(
            params.sqrtTau
        ).wadMul(params.ierfcResult);

        int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
        int256 second = params.strike.wadMul(
            params.L + params.rX.wadMul(-I_ONE + params.gamma)
        );

        int256 firstNum = first.wadMul(second);
        int256 firstDen = params.L + params.v - params.v.wadMul(params.gamma);
        return firstNum.wadDiv(firstDen);
    }

    function computeLowerB(DiffLowerStruct memory params)
        internal
        pure
        returns (int256)
    {
        int256 a = I_HALF.wadMul(params.strike).wadMul(-I_ONE + params.gamma);
        int256 b = params.sigma.wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
        return a.wadMul(Gaussian.erfc(b - params.ierfcResult));
    }

    function diffLower(
        int256 S,
        int256 rX,
        int256 L,
        int256 v,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (int256) {
        int256 gamma = I_ONE - int256(params.swapFee);
        DiffLowerStruct memory ints =
            createDiffLowerStruct(S, rX, L, gamma, v, params);
        int256 a = computeLowerA(ints);
        int256 b = computeLowerB(ints);

        return -ints.S + a + b;
    }

    struct DiffRaiseStruct {
        int256 ierfcResult;
        int256 strike;
        int256 sigma;
        int256 tau;
        int256 gamma;
        int256 rY;
        int256 L;
        int256 v;
        int256 S;
        int256 sqrtTwo;
        int256 sqrtTau;
    }

    function createDiffRaiseStruct(
        int256 S,
        int256 ry,
        int256 L,
        int256 gamma,
        int256 v,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (DiffRaiseStruct memory) {
        int256 a = I_TWO.wadMul(v + ry);
        int256 b = int256(params.strike).wadMul(L) + v - v.wadMul(gamma);
        int256 ierfcRes = Gaussian.ierfc(a.wadDiv(b));

        int256 sqrtTwo = int256(FixedPointMathLib.sqrt(TWO) * 1e9);
        int256 sqrtTau = int256(FixedPointMathLib.sqrt(params.tau) * 1e9);

        DiffRaiseStruct memory ints = DiffRaiseStruct({
            ierfcResult: ierfcRes,
            strike: int256(params.strike),
            sigma: int256(params.sigma),
            tau: int256(params.tau),
            gamma: gamma,
            rY: ry,
            L: L,
            S: S,
            v: v,
            sqrtTwo: sqrtTwo,
            sqrtTau: sqrtTau
        });

        return ints;
    }

    function computeRaiseA(DiffRaiseStruct memory params)
        internal
        pure
        returns (int256)
    {
        int256 firstExp = -(params.sigma.wadMul(params.sigma).wadMul(params.tau).wadDiv(I_TWO));
        int256 secondExp = params.sqrtTwo.wadMul(params.sigma).wadMul(
            params.sqrtTau
        ).wadMul(params.ierfcResult);
        int256 first = FixedPointMathLib.expWad(firstExp + secondExp);
        int256 second = params.S.wadMul(
            params.strike.wadMul(params.L)
                + params.rY.wadMul(-I_ONE + params.gamma)
        );

        int256 num = first.wadMul(second);
        int256 den = params.strike.wadMul(
            params.strike.wadMul(params.L) + params.v
                - params.v.wadMul(params.gamma)
        );
        return num.wadDiv(den);
    }

    function computeRaiseB(DiffRaiseStruct memory params)
        internal
        pure
        returns (int256)
    {
        int256 first = params.S.wadMul(-I_ONE + params.gamma);
        int256 erfcFirst =
            params.sigma.wadMul(params.sqrtTau).wadDiv(params.sqrtTwo);
        int256 num = first.wadMul(Gaussian.erfc(erfcFirst - params.ierfcResult));
        int256 den = I_TWO.wadMul(params.strike);
        return num.wadDiv(den);
    }

    function diffRaise(
        int256 S,
        int256 rY,
        int256 L,
        int256 v,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (int256) {
        int256 gamma = I_ONE - int256(params.swapFee);
        DiffRaiseStruct memory ints =
            createDiffRaiseStruct(S, rY, L, gamma, v, params);
        int256 a = computeRaiseA(ints);
        int256 b = computeRaiseB(ints);

        return -I_ONE + a + b;
    }

    function computeDy(
        int256 S,
        int256 rY,
        int256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (int256 dy) {
        int256 gamma = I_ONE - int256(params.swapFee);
        int256 strike = int256(params.strike);
        int256 sigma = int256(params.sigma);

        int256 lnSDivK = computeLnSDivK(uint256(S), params.strike);
        int256 a = lnSDivK.wadDiv(sigma) - sigma.wadDiv(I_TWO);
        int256 cdfA = Gaussian.cdf(a);

        int256 delta = L.wadMul(strike).wadMul(cdfA);
        dy = delta - rY;
    }

    function computeDx(
        int256 S,
        int256 rX,
        int256 L,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (int256 dx) {
        int256 gamma = I_ONE - int256(params.swapFee);
        int256 sigma = int256(params.sigma);

        int256 lnSDivK = computeLnSDivK(uint256(S), params.strike);
        int256 a = Gaussian.cdf(lnSDivK.wadDiv(sigma) + sigma.wadDiv(I_TWO));

        int256 delta = L.wadMul(I_ONE - a);
        dx = delta - rX;
    }

    function computeOptimalLower(
        int256 S,
        int256 rX,
        int256 L,
        uint256 vUpper,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256 v) {
        uint256 upper = vUpper;
        uint256 lower = 1;
        int256 lowerBoundOutput = diffLower(S, rX, L, int256(lower), params);
        if (lowerBoundOutput < 0) {
            return 0;
        }
        v = bisection(
            abi.encode(S, rX, L, params),
            lower,
            upper,
            uint256(1),
            256,
            findRootLower
        );
    }

    function computeOptimalRaise(
        int256 S,
        int256 rY,
        int256 L,
        uint256 vUpper,
        LogNormal.LogNormalParams memory params
    ) internal pure returns (uint256 v) {
        uint256 upper = vUpper;
        uint256 lower = 1;
        int256 lowerBoundOutput = diffRaise(S, rY, L, int256(lower), params);
        if (lowerBoundOutput < 0) {
            return 0;
        }
        v = bisection(
            abi.encode(S, rY, L, params),
            lower,
            upper,
            uint256(1),
            256,
            findRootRaise
        );
    }
}

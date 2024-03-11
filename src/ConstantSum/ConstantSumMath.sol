// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";

uint256 constant ONE = 1 ether;

using FixedPointMathLib for uint256;
using FixedPointMathLib for int256;

function computeTradingFunction(
    uint256 reserveX,
    uint256 reserveY,
    uint256 totalLiquidity,
    uint256 price
) pure returns (int256) {
    return int256(reserveX.divWadUp(totalLiquidity))
        + int256(reserveY.divWadUp(totalLiquidity.mulWadUp(price)))
        - int256(ONE);
}

function computeInitialPoolData(
    uint256 rx,
    uint256 ry,
    ConstantSum.ConstantSumParams memory params
) pure returns (bytes memory) {
    // The pool can be initialized with any non-negative amount of rx, and ry.
    // so we have to allow a user to pass an amount of both even if one is zero.
    uint256 L = rx + ry.divWadUp(params.price);
    return abi.encode(rx, ry, L, params);
}

function computeDeallocateGivenDeltaX(
    uint256 poolId,
    uint256 deltaX
) view returns (bool, bytes memory) {
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

    bytes memory deallocateData =
        abi.encode(amountX, amountY, startReserves.L - endReserves.L);
    (bool valid,,,,) = IStrategy(strategy).validateDeallocate(
        address(this), poolId, pool, deallocateData
    );
    return (valid, deallocateData);
}


function computeDeltaGivenDeltaLRoundUp(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulWadUp(deltaLiquidity.divWadUp(totalLiquidity));
}

function computeDeltaGivenDeltaLRoundDown(
    uint256 reserve,
    uint256 deltaLiquidity,
    uint256 totalLiquidity
) pure returns (uint256) {
    return reserve.mulWadDown(deltaLiquidity.divWadDown(totalLiquidity));
}

function computeLGivenX(
    uint256 x,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    int256 a = int256(params.wY.divWadUp(params.wX).mulWadUp(S));
    int256 b = a.powWad(int256(params.wY));
    return x.mulWadUp(uint256(b));
}

function computeLGivenY(
    uint256 y,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return y.mulWadUp(params.wX).divWadUp(params.wY.mulWadUp(S));
}

function computeXGivenL(
    uint256 L,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wX.mulWadUp(L).divWadUp(params.wY.mulWadUp(S));
}

function computeYGivenL(
    uint256 L,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wY.mulWadUp(L).divWadUp(params.wX.mulWadUp(S));
}

function computeAllocationGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadUp(rX);
    deltaY = a.mulWadUp(rY);
    deltaL = a.mulWadUp(totalLiquidity);
}

function computeAllocationGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadUp(rY);
    deltaX = a.mulWadUp(rX);
    deltaL = a.mulWadUp(totalLiquidity);
}

function computeDeallocationGivenDeltaX(
    uint256 deltaX,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaY, uint256 deltaL) {
    uint256 a = deltaX.divWadDown(rX);
    deltaY = a.mulWadDown(rY);
    deltaL = a.mulWadDown(totalLiquidity);
}

function computeDeallocationGivenDeltaY(
    uint256 deltaY,
    uint256 rX,
    uint256 rY,
    uint256 totalLiquidity
) pure returns (uint256 deltaX, uint256 deltaL) {
    uint256 a = deltaY.divWadDown(rY);
    deltaX = a.mulWadDown(rX);
    deltaL = a.mulWadDown(totalLiquidity);
}

function computeY(
    uint256 x,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wY.divWadDown(params.wX).mulWadDown(S).mulWadDown(x);
}

function computeX(
    uint256 y,
    uint256 S,
    GeometricMeanParams memory params
) pure returns (uint256) {
    return params.wX.divWadDown(params.wY.mulWadDown(S)).mulWadDown(y);
}

function computeL(
    uint256 x,
    uint256 y,
    GeometricMeanParams memory params
) pure returns (uint256) {
    uint256 a = uint256(int256(x).powWad(int256(params.wX)));
    uint256 b = uint256(int256(y).powWad(int256(params.wY)));

    return a.mulWadUp(b);
}

/// @dev Finds the root of the swapConstant given the independent variable rX.
function computeNextRy(
    uint256 rX,
    uint256 liquidity,
    GeometricMeanParams memory params
) pure returns (uint256 rY) {
    rY = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rX).powWad(int256(params.wX))))
        ).powWad(int256(ONE.divWadUp(params.wY)))
    );
}

/// @dev Finds the root of the swapConstant given the independent variable rY.
function computeNextRx(
    uint256 rY,
    uint256 liquidity,
    GeometricMeanParams memory params
) pure returns (uint256 rX) {
    rX = uint256(
        int256(
            liquidity.divWadUp(uint256(int256(rY).powWad(int256(params.wY))))
        ).powWad(int256(ONE.divWadUp(params.wX)))
    );
}

/// @dev Computes the approximated spot price given current reserves and liquidity.
function computePrice(
    uint256 rX,
    uint256 rY,
    GeometricMeanParams memory params
) pure returns (uint256 price) {
    uint256 n = rY.divWadDown(params.wY);
    uint256 d = rX.divWadDown(params.wX);
    price = n.divWadDown(d);
}

/// @dev Finds the root of the swapConstant given the independent variable liquidity.
function computeNextLiquidity(
    uint256 rX,
    uint256 rY,
    GeometricMeanParams memory params
) pure returns (uint256 L) {
    return uint256(int256(rX).powWad(int256(params.wX))).mulWadUp(
        uint256(int256(rY).powWad(int256(params.wY)))
    );
}

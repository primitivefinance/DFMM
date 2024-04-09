// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import { ConstantSumParams } from "src/ConstantSum/ConstantSum.sol";
import { ONE } from "src/lib/StrategyLib.sol";

using FixedPointMathLib for uint256;

function computeTradingFunction(
    uint256[] memory reserves,
    uint256 totalLiquidity,
    uint256 price
) pure returns (int256) {
    return int256(
        price.mulWadUp(reserves[0].divWadUp(totalLiquidity))
            + reserves[1].divWadUp(totalLiquidity)
    ) - int256(ONE);
}

function computeInitialPoolData(
    uint256 rx,
    uint256 ry,
    ConstantSumParams memory params
) pure returns (bytes memory) {
    // The pool can be initialized with any non-negative amount of rx, and ry.
    // so we have to allow a user to pass an amount of both even if one is zero.
    uint256[] memory reserves = new uint256[](2);
    reserves[0] = rx;
    reserves[1] = ry;
    return abi.encode(reserves, params);
}

function computeDeltaLiquidity(
    uint256 deltaX,
    uint256 deltaY,
    uint256 price
) pure returns (uint256) {
    return price.mulWadUp(deltaX) + deltaY;
}

function computeSwapDeltaLiquidity(
    uint256 delta,
    ConstantSumParams memory params,
    bool isSwapXForY
) pure returns (uint256) {
    if (isSwapXForY) {
        return params.swapFee.mulWadUp(delta.mulWadUp(params.price));
    } else {
        return params.swapFee.mulWadUp(delta);
    }
}

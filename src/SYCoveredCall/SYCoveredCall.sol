// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity 0.8.22;

import { Pool } from "src/interfaces/IDFMM.sol";
import { PairStrategy, IStrategy } from "src/PairStrategy.sol";
import { IDFMM } from "src/interfaces/IDFMM.sol";
import { DynamicParamLib, DynamicParam } from "src/lib/DynamicParamLib.sol";
import { FixedPointMathLib } from "solmate/utils/FixedPointMathLib.sol";
import {
    computeTradingFunction,
    computeDeltaGivenDeltaLRoundUp,
    computeDeltaGivenDeltaLRoundDown,
    computeDeltaLXIn,
    computeDeltaLYIn,
    computeTau,
    computeKGivenLastPrice
} from "src/SYCoveredCall/SYCoveredCallMath.sol";
import {
    decodeFeeUpdate,
    decodeControllerUpdate
} from "src/SYCoveredCall/SYCoveredCallUtils.sol";
import { EPSILON } from "src/lib/StrategyLib.sol";
import { IPPrincipalToken } from "pendle/interfaces/IPPrincipalToken.sol";
import { IStandardizedYield } from "pendle/interfaces/IStandardizedYield.sol";
import { IPYieldToken } from "pendle/interfaces/IPYieldToken.sol";

enum UpdateCode {
    Invalid,
    SwapFee,
    Controller
}

struct InternalParams {
    uint256 mean;
    uint256 width;
    uint256 maturity;
    uint256 swapFee;
    address controller;
    uint256 lastTimestamp;
    IStandardizedYield SY;
    IPPrincipalToken PT;
    IPYieldToken YT;
}

/// @dev Parameterization of the Log Normal curve.
struct SYCoveredCallParams {
    uint256 mean;
    uint256 width;
    uint256 maturity;
    uint256 swapFee;
    address controller;
    uint256 lastTimestamp;
    IStandardizedYield SY;
    IPPrincipalToken PT;
    IPYieldToken YT;
}

/// @dev Thrown when the mean parameter is not within the allowed bounds.
error InvalidMean();

/// @dev Thrown when the width parameter is not within the allowed bounds.
error InvalidWidth();

/// @dev Thrown when the maturity parameter is not later than the current block.timestamp.
error InvalidMaturity();

/// @dev Thrown when the pool SY token is not associated with the pool PT token.
error InvalidPair();

/// @dev Thrown when meanAnchor <= ONE.
error InvalidMeanAnchor();

error InvalidTimestamp();

error InvalidComputedK();

/// @dev Thrown when the computedL passed to swap does not satisfy the invariant check
error InvalidComputedLiquidity(int256 invariant);

uint256 constant MIN_WIDTH = 1;
uint256 constant MAX_WIDTH = uint256(type(int256).max);
uint256 constant MIN_MEAN = 1;
uint256 constant MAX_MEAN = uint256(type(int256).max);
uint256 constant T_EPSILON = 200;

/**
 * @title SYCoveredCall Strategy for DFMM.
 * @author Primitive
 */
contract SYCoveredCall is PairStrategy {
    using FixedPointMathLib for int256;
    /// @inheritdoc IStrategy

    string public constant override name = "SYCoveredCall";

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) PairStrategy(dfmm_) { }

    /// @inheritdoc IStrategy
    function init(
        address,
        uint256 poolId,
        Pool calldata pool,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        )
    {
        SYCoveredCallParams memory params;

        (reserves, totalLiquidity, params) =
            abi.decode(data, (uint256[], uint256, SYCoveredCallParams));

        IStandardizedYield SY = IStandardizedYield(pool.tokens[0]);
        IPPrincipalToken PT = IPPrincipalToken(pool.tokens[1]);
        params.lastTimestamp = block.timestamp;

        int256 tau = int256(computeTau(params));

        if (PT.SY() != address(SY)) {
            revert InvalidPair();
        }

        if (PT.expiry() <= block.timestamp) {
            revert InvalidMaturity();
        }

        if (params.mean <= 1 ether) {
            revert InvalidMeanAnchor();
        }

        if (pool.reserves.length != 2 || reserves.length != 2) {
            revert InvalidReservesLength();
        }

        internalParams[poolId].SY = SY;
        internalParams[poolId].PT = PT;
        internalParams[poolId].YT = IPYieldToken(PT.YT());

        internalParams[poolId].maturity = internalParams[poolId].PT.expiry();
        internalParams[poolId].mean = params.mean;
        internalParams[poolId].width = params.width;
        internalParams[poolId].swapFee = params.swapFee;
        internalParams[poolId].controller = params.controller;

        invariant =
            tradingFunction(reserves, totalLiquidity, abi.encode(params));
        valid = invariant >= 0 && invariant <= EPSILON;
    }

    /// @inheritdoc IStrategy
    function update(
        address sender,
        uint256 poolId,
        Pool calldata,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        UpdateCode updateCode = abi.decode(data, (UpdateCode));
        if (updateCode == UpdateCode.SwapFee) {
            internalParams[poolId].swapFee = decodeFeeUpdate(data);
        } else if (updateCode == UpdateCode.Controller) {
            internalParams[poolId].controller = decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    /// @inheritdoc IStrategy
    function getPoolParams(uint256 poolId)
        public
        view
        override
        returns (bytes memory)
    {
        SYCoveredCallParams memory params;

        params.width = internalParams[poolId].width;
        params.mean = internalParams[poolId].mean;
        params.swapFee = internalParams[poolId].swapFee;
        params.maturity = internalParams[poolId].maturity;
        params.lastTimestamp = internalParams[poolId].lastTimestamp;

        return abi.encode(params);
    }

    /// @inheritdoc IStrategy
    function validateSwap(
        address,
        uint256 poolId,
        Pool memory pool,
        bytes memory data
    )
        external
        view
        override
        returns (
            bool valid,
            int256 invariant,
            uint256 tokenInIndex,
            uint256 tokenOutIndex,
            uint256 amountIn,
            uint256 amountOut,
            uint256 deltaLiquidity,
            bytes memory params
        )
    {
        // fetch computedL and swapTimestamp from data
        params = getPoolParams(poolId);
        SYCoveredCallParams memory ccParams =
            abi.decode(params, (SYCoveredCallParams));

        uint256 computedL;
        uint256 swapTimestamp;
        (
            tokenInIndex,
            tokenOutIndex,
            amountIn,
            amountOut,
            computedL,
            swapTimestamp
        ) = abi.decode(
            data, (uint256, uint256, uint256, uint256, uint256, uint256)
        );

        if (
            swapTimestamp < internalParams[poolId].lastTimestamp
                || swapTimestamp < block.timestamp - T_EPSILON
                || swapTimestamp > block.timestamp + T_EPSILON
        ) {
            revert InvalidTimestamp();
        }

        // if timestamp is valid, append it to the poolParams for validation check
        ccParams.lastTimestamp = swapTimestamp;

        // compute new K
        ccParams.mean =
            computeKGivenLastPrice(pool.reserves[0], computedL, ccParams);

        int256 computedInvariant =
            tradingFunction(pool.reserves, computedL, abi.encode(params));

        if (computedInvariant < 0 || computedInvariant > EPSILON) {
            revert InvalidComputedLiquidity(computedInvariant);
        }

        deltaLiquidity = _computeSwapDeltaLiquidity(
            pool,
            abi.encode(params),
            tokenInIndex,
            tokenOutIndex,
            amountIn,
            amountOut
        );

        pool.reserves[tokenInIndex] += amountIn;
        pool.reserves[tokenOutIndex] -= amountOut;

        invariant = tradingFunction(
            pool.reserves, computedL + deltaLiquidity, abi.encode(params)
        );

        params = abi.encode(ccParams);
        valid = invariant >= 0;
        //valid = invariant >= 0 && invariant <= EPSILON;
    }

    function postSwapHook(
        address,
        uint256 poolId,
        Pool memory,
        bytes memory params
    ) external override onlyDFMM {
        SYCoveredCallParams memory ccParams =
            abi.decode(params, (SYCoveredCallParams));

        internalParams[poolId].lastTimestamp = ccParams.lastTimestamp;
        internalParams[poolId].mean = ccParams.mean;
    }

    /// @inheritdoc IStrategy
    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure override returns (int256) {
        SYCoveredCallParams memory poolParams =
            abi.decode(params, (SYCoveredCallParams));
        return computeTradingFunction(
            reserves[0], reserves[1], totalLiquidity, poolParams
        );
    }

    /// @inheritdoc PairStrategy
    function _computeAllocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        uint256[] memory deltas = new uint256[](2);

        deltas[0] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundUp(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );

        return deltas;
    }

    /// @inheritdoc PairStrategy
    function _computeDeallocateDeltasGivenDeltaL(
        uint256 deltaLiquidity,
        Pool memory pool,
        bytes memory
    ) internal pure override returns (uint256[] memory) {
        uint256[] memory deltas = new uint256[](2);

        deltas[0] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[0], deltaLiquidity, pool.totalLiquidity
        );

        deltas[1] = computeDeltaGivenDeltaLRoundDown(
            pool.reserves[1], deltaLiquidity, pool.totalLiquidity
        );
        return deltas;
    }

    function _computeSwapDeltaLiquidity(
        Pool memory pool,
        bytes memory params,
        uint256 tokenInIndex,
        uint256,
        uint256 amountIn,
        uint256
    ) internal pure override returns (uint256) {
        SYCoveredCallParams memory poolParams =
            abi.decode(params, (SYCoveredCallParams));

        if (tokenInIndex == 0) {
            return computeDeltaLXIn(
                amountIn,
                pool.reserves[0],
                pool.reserves[1],
                pool.totalLiquidity,
                poolParams
            );
        }

        return computeDeltaLYIn(
            amountIn,
            pool.reserves[0],
            pool.reserves[1],
            pool.totalLiquidity,
            poolParams
        );
    }
}
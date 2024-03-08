// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import { DynamicParam, DynamicParamLib } from "src/lib/DynamicParamLib.sol";
import { IDFMM2 } from "src/interfaces/IDFMM2.sol";
import {
    NTokenGeometricMeanLib,
    FixedPointMathLib
} from "./NTokenGeometricMeanLib.sol";
import { ONE, EPSILON } from "src/lib/StrategyLib.sol";
import "forge-std/console2.sol";

/// @dev Parameterization of the GeometricMean curve.
struct NTokenGeometricMeanParams {
    uint256[] weights;
    uint256 swapFee;
    address controller;
}

/**
 * @notice Geometric Mean Market Maker.
 */
contract NTokenGeometricMean {
    using FixedPointMathLib for uint256;
    using FixedPointMathLib for int256;
    using DynamicParamLib for DynamicParam;

    /// @dev Thrown when the update code is invalid.
    error InvalidUpdateCode();

    /// @dev Thrown when the sender is not the DFMM contract.
    error NotDFMM();

    /// @dev Thrown when the sender is authorized.
    error InvalidSender();

    error DeltaError(uint256 expected, uint256 actual);

    struct InternalParams {
        DynamicParam[] weights;
        uint256 swapFee;
        address controller;
    }

    string public constant name = "NTokenGeometricMean";

    address public immutable dfmm;

    mapping(uint256 => InternalParams) public internalParams;

    /// @param dfmm_ Address of the DFMM contract.
    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    error InvalidWeights(uint256 totalWeight);
    error InvalidConfiguration(uint256 reservesLength, uint256 weightsLength);

    function init(
        address,
        uint256 poolId,
        IDFMM2.Pool calldata,
        bytes calldata data
    )
        external
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserves, totalLiquidity,,,) =
            _decodeInit(poolId, data);
    }

    function _decodeInit(
        uint256 poolId,
        bytes calldata data
    )
        private
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity,
            uint256[] memory weights,
            uint256 swapFee,
            address controller
        )
    {
        (reserves, totalLiquidity, weights, swapFee, controller) =
            abi.decode(data, (uint256[], uint256, uint256[], uint256, address));

        if (reserves.length != weights.length) {
            revert InvalidConfiguration(reserves.length, weights.length);
        }

        uint256 weightAccumulator;
        for (uint256 i = 0; i < weights.length; i++) {
            weightAccumulator += weights[i];
            internalParams[poolId].weights.push(
                DynamicParam({
                    lastComputedValue: weights[i],
                    updateEnd: 0,
                    updatePerSecond: 0,
                    lastUpdateAt: 0
                })
            );
        }

        if (weightAccumulator != ONE) {
            revert InvalidWeights(weightAccumulator);
        }

        internalParams[poolId].swapFee = swapFee;
        internalParams[poolId].controller = controller;

        invariant = NTokenGeometricMeanLib.tradingFunction(
            reserves,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (NTokenGeometricMeanParams))
        );

        // todo: should the be EXACTLY 0? just positive? within an epsilon?
        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function update(
        address sender,
        uint256 poolId,
        IDFMM2.Pool calldata,
        bytes calldata data
    ) external {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        NTokenGeometricMeanLib.GeometricMeanUpdateCode updateCode =
            abi.decode(data, (NTokenGeometricMeanLib.GeometricMeanUpdateCode));

        if (
            updateCode == NTokenGeometricMeanLib.GeometricMeanUpdateCode.SwapFee
        ) {
            internalParams[poolId].swapFee =
                NTokenGeometricMeanLib.decodeFeeUpdate(data);
        }
        /*
        else if (
            updateCode == NTokenGeometricMeanLib.GeometricMeanUpdateCode.WeightX
        ) {
            (uint256 targetWeightX, uint256 targetTimestamp) =
                NTokenGeometricMeanLib.decodeWeightXUpdate(data);
            internalParams[poolId].wX.set(targetWeightX, targetTimestamp);
        } 
        */
        else if (
            updateCode
                == NTokenGeometricMeanLib.GeometricMeanUpdateCode.Controller
        ) {
            internalParams[poolId].controller =
                NTokenGeometricMeanLib.decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        NTokenGeometricMeanParams memory params;

        uint256[] memory weights =
            new uint256[](internalParams[poolId].weights.length);

        for (uint256 i = 0; i < params.weights.length; i++) {
            weights[i] = internalParams[poolId].weights[i].actualized();
        }

        params.weights = weights;
        params.swapFee = internalParams[poolId].swapFee;
        params.controller = internalParams[poolId].controller;

        return abi.encode(params);
    }

    function tradingFunction(
        uint256[] memory reserves,
        uint256 totalLiquidity,
        bytes memory params
    ) public pure returns (int256) {
        return NTokenGeometricMeanLib.tradingFunction(
            reserves,
            totalLiquidity,
            abi.decode(params, (NTokenGeometricMeanParams))
        );
    }

    function computeAllocationGivenNumeraire(
        bool add,
        uint256 amountNumeraire,
        uint256[] calldata reserves,
        uint256 L
    ) public pure returns (uint256 nextRNumeraire, uint256 nextL) {
        uint256 rNumeraire = reserves[reserves.length - 1];
        uint256 liquidityPerRNumeraire = L.divWadUp(rNumeraire);
        uint256 deltaL = amountNumeraire.mulWadUp(liquidityPerRNumeraire);
        nextRNumeraire =
            add ? rNumeraire + amountNumeraire : rNumeraire - amountNumeraire;
        nextL = add ? L + deltaL : L - deltaL;
    }

    function validateAllocate(
        address,
        uint256 poolId,
        IDFMM2.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory tokenDeltas,
            uint256 deltaLiquidity
        )
    {
        (uint256[] memory maxTokenDeltas, uint256 deltaL) =
            abi.decode(data, (uint256[], uint256));

        // TODO: This is a small trick because `deltaLiquidity` cannot be used
        // directly, let's fix this later.
        uint256[] memory nextReserves = new uint256[](pool.reserves.length);
        tokenDeltas = new uint256[](pool.reserves.length);
        deltaLiquidity = deltaL;
        for (uint256 i = 0; i < pool.reserves.length; i++) {
            uint256 deltaT = pool.reserves[i].mulWadDown(
                deltaLiquidity.divWadDown(pool.totalLiquidity)
            );
            if (deltaT > maxTokenDeltas[i]) {
                revert DeltaError(maxTokenDeltas[i], deltaT);
            }
            nextReserves[i] = pool.reserves[i] + deltaT;
            tokenDeltas[i] = deltaT;
        }

        uint256 poolId = poolId;

        invariant = tradingFunction(
            nextReserves,
            pool.totalLiquidity + deltaLiquidity,
            getPoolParams(poolId)
        );
        console2.log("invariant", invariant);

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function validateDeallocate(
        address,
        uint256 poolId,
        IDFMM2.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        virtual
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory tokenDeltas,
            uint256 deltaLiquidity
        )
    {
        (uint256[] memory minTokenDeltas, uint256 deltaL) =
            abi.decode(data, (uint256[], uint256));

        uint256[] memory nextReserves = new uint256[](pool.reserves.length);
        tokenDeltas = new uint256[](pool.reserves.length);
        deltaLiquidity = deltaL;
        for (uint256 i = 0; i < pool.reserves.length; i++) {
            uint256 deltaT = pool.reserves[i].mulWadDown(
                deltaLiquidity.divWadDown(pool.totalLiquidity)
            );
            if (deltaT < minTokenDeltas[i]) {
                revert DeltaError(minTokenDeltas[i], deltaT);
            }
            nextReserves[i] = pool.reserves[i] - deltaT;
            tokenDeltas[i] = deltaT;
        }

        uint256 poolId = poolId;

        invariant = tradingFunction(
            nextReserves,
            pool.totalLiquidity - deltaLiquidity,
            getPoolParams(poolId)
        );

        console2.log("invariant", invariant);
        console2.log("deltaL", deltaLiquidity);
        console2.log("nextL", pool.totalLiquidity - deltaLiquidity);

        valid = -(EPSILON) < invariant && invariant < EPSILON;
    }

    function _computeDeltaTokenGivenDeltaL(
        uint256 deltaLiquidity,
        uint256 reserve,
        IDFMM2.Pool calldata pool,
        bytes memory
    ) internal pure returns (uint256) {
        return
            reserve.mulWadDown(deltaLiquidity.divWadDown(pool.totalLiquidity));
    }
    /*

    function _computeDeltaYGivenDeltaL(
        uint256 deltaLiquidity,
        DFMM2.Pool calldata pool,
        bytes memory
    ) internal pure returns (uint256) {
        return pool.reserveY.mulWadDown(
            deltaLiquidity.divWadDown(pool.totalLiquidity)
        );
    }
    */
}

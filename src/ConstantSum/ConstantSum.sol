// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.13;

import "./ConstantSumLib.sol";
import "src/interfaces/IDFMM.sol";
import "src/interfaces/IStrategy.sol";
import "forge-std/Test.sol";

contract ConstantSum is IStrategy {
    using FixedPointMathLib for uint256;

    struct InternalParams {
        uint256 price;
        uint256 swapFee;
        address controller;
    }

    struct ConstantSumParams {
        uint256 price;
        uint256 swapFee;
        address controller;
    }

    /// @inheritdoc IStrategy
    address public dfmm;

    /// @inheritdoc IStrategy
    string public constant name = "ConstantSum";

    mapping(uint256 => InternalParams) public internalParams;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    modifier onlyDFMM() {
        if (msg.sender != dfmm) revert NotDFMM();
        _;
    }

    function init(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        public
        onlyDFMM
        returns (
            bool valid,
            int256 invariant,
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        ConstantSumParams memory params;
        (reserveX, reserveY, totalLiquidity, params) =
            abi.decode(data, (uint256, uint256, uint256, ConstantSumParams));

        internalParams[poolId].price = params.price;
        internalParams[poolId].swapFee = params.swapFee;

        // Get the trading function and check this is valid
        invariant = ConstantSumLib.tradingFunction(
            reserveX, reserveY, totalLiquidity, params.price
        );

        valid = -EPSILON < invariant && invariant < EPSILON;

        return (valid, invariant, reserveX, reserveY, totalLiquidity);
    }

    function validateSwap(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            int256 liquidityDelta,
            uint256 nextRx,
            uint256 nextRy,
            uint256 nextL
        )
    {
        ConstantSumParams memory params =
            abi.decode(getPoolParams(poolId), (ConstantSumParams));

        (nextRx, nextRy, nextL) = abi.decode(data, (uint256, uint256, uint256));

        uint256 minLiquidityDelta;
        uint256 amountIn;
        uint256 fees;
        if (nextRx > pool.reserveX) {
            amountIn = nextRx - pool.reserveX;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta += fees;
        } else if (nextRy > pool.reserveY) {
            amountIn = nextRy - pool.reserveY;
            fees = amountIn.mulWadUp(params.swapFee);
            minLiquidityDelta += fees.divWadUp(params.price);
        } else {
            revert("invalid swap: inputs x and y have the same sign!");
        }

        liquidityDelta = int256(nextL) - int256(pool.totalLiquidity);
        assert(liquidityDelta >= int256(minLiquidityDelta));

        invariant =
            ConstantSumLib.tradingFunction(nextRx, nextRy, nextL, params.price);

        valid = -EPSILON < invariant && invariant < EPSILON;
    }

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) {
        (uint256 reserveX, uint256 reserveY, uint256 totalLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));
        return ConstantSumLib.tradingFunction(
            reserveX,
            reserveY,
            totalLiquidity,
            abi.decode(getPoolParams(poolId), (ConstantSumParams)).price
        );
    }

    // This should literally always work lol
    function validateAllocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        invariant = ConstantSumLib.tradingFunction(
            pool.reserveX + deltaX,
            pool.reserveX + deltaY,
            pool.totalLiquidity + deltaLiquidity,
            abi.decode(getPoolParams(poolId), (ConstantSumParams)).price
        );

        valid = -EPSILON < invariant && invariant < EPSILON;
    }

    // This should literally always work lol
    function validateDeallocate(
        address,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
        view
        returns (
            bool valid,
            int256 invariant,
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (uint256, uint256, uint256));

        invariant = ConstantSumLib.tradingFunction(
            pool.reserveX - deltaX,
            pool.reserveY - deltaY,
            pool.totalLiquidity - deltaLiquidity,
            abi.decode(getPoolParams(poolId), (ConstantSumParams)).price
        );

        valid = -EPSILON < invariant && invariant < EPSILON;
    }

    function update(
        address sender,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    ) external onlyDFMM {
        if (sender != internalParams[poolId].controller) revert InvalidSender();
        ConstantSumLib.ConstantSumUpdateCode updateCode =
            abi.decode(data, (ConstantSumLib.ConstantSumUpdateCode));

        if (updateCode == ConstantSumLib.ConstantSumUpdateCode.Price) {
            internalParams[poolId].price =
                ConstantSumLib.decodePriceUpdate(data);
        } else if (updateCode == ConstantSumLib.ConstantSumUpdateCode.SwapFee) {
            internalParams[poolId].swapFee =
                ConstantSumLib.decodeFeeUpdate(data);
        } else if (
            updateCode == ConstantSumLib.ConstantSumUpdateCode.Controller
        ) {
            internalParams[poolId].controller =
                ConstantSumLib.decodeControllerUpdate(data);
        } else {
            revert InvalidUpdateCode();
        }
    }

    function getPoolParams(uint256 poolId) public view returns (bytes memory) {
        ConstantSumParams memory params;

        params.price = internalParams[poolId].price;
        params.swapFee = internalParams[poolId].swapFee;

        return abi.encode(params);
    }
}

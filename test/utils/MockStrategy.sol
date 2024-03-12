// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IDFMM } from "src/interfaces/IDFMM.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";

contract MockStrategy is IStrategy {
    string public constant name = "MockStrategy";
    address public immutable dfmm;

    constructor(address dfmm_) {
        dfmm = dfmm_;
    }

    function equals(
        string memory a,
        string memory b
    ) internal pure returns (bool) {
        return (
            keccak256(abi.encodePacked((a))) == keccak256(abi.encodePacked((b)))
        );
    }

    function init(
        address,
        uint256,
        IDFMM.Pool calldata,
        bytes calldata data
    )
        external
        pure
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory reserves,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserves, totalLiquidity) =
            abi.decode(data, (bool, int256, uint256[], uint256));
    }

    function validateAllocate(
        address,
        uint256,
        IDFMM.Pool calldata,
        bytes calldata data
    )
        external
        pure
        override
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        (valid, invariant, deltas, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256[], uint256));
    }

    function validateDeallocate(
        address,
        uint256,
        IDFMM.Pool calldata,
        bytes calldata data
    )
        external
        pure
        override
        returns (
            bool valid,
            int256 invariant,
            uint256[] memory deltas,
            uint256 deltaLiquidity
        )
    {
        (valid, invariant, deltas, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256[], uint256));
    }

    function validateSwap(
        address,
        uint256,
        IDFMM.Pool calldata,
        bytes calldata data
    )
        external
        pure
        override
        returns (
            bool valid,
            int256 invariant,
            uint256 tokenInIndex,
            uint256 tokenOutIndex,
            uint256 amountIn,
            uint256 amountOut,
            uint256 deltaLiquidity
        )
    {
        (
            valid,
            invariant,
            tokenInIndex,
            tokenOutIndex,
            amountIn,
            amountOut,
            deltaLiquidity
        ) = abi.decode(
            data, (bool, int256, uint256, uint256, uint256, uint256, uint256)
        );
    }

    function update(
        address sender,
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    ) external { }

    function computeSwapConstant(
        uint256 poolId,
        bytes memory data
    ) external view returns (int256) { }

    function getPoolParams(uint256 poolId)
        public
        view
        override
        returns (bytes memory params)
    { }

    function tradingFunction(
        uint256[] memory,
        uint256,
        bytes memory
    ) external pure returns (int256) {
        return int256(0);
    }
}

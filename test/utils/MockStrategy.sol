// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IDFMM } from "src/interfaces/IDFMM.sol";
import { Strategy } from "src/Strategy.sol";

contract MockStrategy is Strategy {
    string public constant name = "MockStrategy";

    constructor(address dfmm_) Strategy(dfmm_) { }

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
            uint256 reserveX,
            uint256 reserveY,
            uint256 totalLiquidity
        )
    {
        (valid, invariant, reserveX, reserveY, totalLiquidity) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256));
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
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (valid, invariant, deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256));
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
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity
        )
    {
        (valid, invariant, deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256));
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
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity,
            bool isSwapXForY
        )
    {
        (valid, invariant, deltaX, deltaY, deltaLiquidity, isSwapXForY) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256, bool));
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
        uint256,
        uint256,
        uint256,
        bytes memory
    ) public pure override returns (int256) {
        return int256(0);
    }

    function _computeDeltaXGivenDeltaL(
        uint256,
        IDFMM.Pool calldata,
        bytes memory
    ) internal pure override returns (uint256) {
        return 0;
    }

    function _computeDeltaYGivenDeltaL(
        uint256,
        IDFMM.Pool calldata,
        bytes memory
    ) internal pure override returns (uint256) {
        return 0;
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { IDFMM } from "src/interfaces/IDFMM.sol";
import { IStrategy } from "src/interfaces/IStrategy.sol";

contract MockStrategy is IStrategy {
    address public immutable dfmm;

    string public constant name = "MockStrategy";

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
        uint256 poolId,
        IDFMM.Pool calldata pool,
        bytes calldata data
    )
        external
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
        (valid, invariant, deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256));
    }

    function validateDeallocate(
        address sender,
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
        (valid, invariant, deltaX, deltaY, deltaLiquidity) =
            abi.decode(data, (bool, int256, uint256, uint256, uint256));
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
            uint256 deltaX,
            uint256 deltaY,
            uint256 deltaLiquidity,
            bool isSwapXForY
        )
    { }

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
        external
        view
        returns (bytes memory params)
    { }
}

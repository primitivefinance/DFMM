// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./SetUp.sol";
import { DynamicParamLib, DynamicParam } from "src/lib/DynamicParamLib.sol";
import { IStrategy2 } from "src/interfaces/IStrategy2.sol";

contract G3MInitTest is G3MSetUp {
    using DynamicParamLib for DynamicParam;

    function test_G3M_init_SetInternalParams() public init {
        (DynamicParam memory wX, uint256 swapFee, address controller) =
            g3m.internalParams(POOL_ID);

        assertEq(wX.actualized(), defaultParams.wX);
        assertEq(swapFee, defaultParams.swapFee);
        assertEq(controller, defaultParams.controller);
    }

    function test_G3M_init_RevertsWhenInvalidWeightX() public {
        GeometricMeanParams memory params = GeometricMeanParams({
            wX: 1.1 ether,
            wY: 0.5 ether,
            swapFee: TEST_SWAP_FEE,
            controller: address(this)
        });

        bytes memory defaultInitialPoolData =
            computeInitialPoolData(defaultReserveX, defaultStrikePrice, params);

        address[] memory tokens = new address[](2);

        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        IDFMM2.InitParams memory initParams = IDFMM2.InitParams({
            name: "",
            symbol: "",
            strategy: address(g3m),
            tokens: tokens,
            data: defaultInitialPoolData
        });

        vm.expectRevert(GeometricMean2.InvalidWeightX.selector);
        dfmm.init(initParams);
    }

    function test_G3M_init_RevertsWhenSenderNotDFMM() public {
        bytes memory empty;
        IDFMM2.Pool memory pool;
        vm.expectRevert(IStrategy2.NotDFMM.selector);
        g3m.init(address(0), 0, pool, empty);
    }
}

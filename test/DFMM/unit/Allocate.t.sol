// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import { stdError } from "forge-std/StdError.sol";
import { DFMMSetUp, DFMM } from "./SetUp.sol";

contract DFMMAllocateTest is DFMMSetUp {
    function test_DFMM_allocate_RevertsWhenPoolNotInitialized() public {
        bytes memory empty;
        vm.expectRevert(stdError.indexOOBError);
        dfmm.allocate(32, empty);
    }

    function test_DFMM_allocate_RevertsWhenNoPoolWasInitialized() public {
        dfmm = new DFMM();
        bytes memory empty;
        vm.expectRevert(stdError.indexOOBError);
        dfmm.allocate(0, empty);
    }

    function test_DFMM_allocate_EmitsAllocateEvent() public {
        skip();
    }
}

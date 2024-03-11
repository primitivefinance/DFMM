// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { DFMM2, IDFMM2 } from "src/DFMM2.sol";

contract SetUp is Test {
    DFMM2 dfmm;
    MockERC20 tokenX;
    MockERC20 tokenY;
    WETH weth;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public virtual {
        tokenX = new MockERC20("Test Token X", "TSTX", 18);
        tokenY = new MockERC20("Test Token Y", "TSTY", 18);
        tokenX.mint(address(this), 100e18);
        tokenY.mint(address(this), 100e18);

        weth = new WETH();
        dfmm = new DFMM2(address(weth));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function getPoolLiquidityToken(uint256 poolId)
        public
        view
        returns (address)
    {
        IDFMM2.Pool memory pool = dfmm.getPool(poolId);
        return pool.liquidityToken;
    }

    function skip() public {
        vm.skip(true);
    }
}

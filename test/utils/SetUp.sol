// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import { MockERC20 } from "solmate/test/utils/mocks/MockERC20.sol";
import { WETH } from "solmate/tokens/WETH.sol";
import { DFMM, IDFMM, Pool, LPToken } from "src/DFMM.sol";

contract SetUp is Test {
    DFMM dfmm;
    MockERC20 tokenX;
    MockERC20 tokenY;
    WETH weth;

    uint256 public constant TEST_SWAP_FEE = 0.003 ether;

    function setUp() public virtual {
        tokenX = new MockERC20("Test Token X", "TSTX", 18);
        tokenY = new MockERC20("Test Token Y", "TSTY", 18);
        tokenX.mint(address(this), 10_000_000_000_000e18);
        tokenY.mint(address(this), 10_000_000_000_000e18);

        weth = new WETH();
        dfmm = new DFMM(address(weth));

        tokenX.approve(address(dfmm), type(uint256).max);
        tokenY.approve(address(dfmm), type(uint256).max);
    }

    function getPoolLiquidityToken(uint256 poolId)
        public
        view
        returns (address)
    {
        Pool memory pool = dfmm.pools(poolId);
        return pool.liquidityToken;
    }

    function skip() public {
        vm.skip(true);
    }

    function liquidityOf(
        address account,
        uint256 poolId
    ) public view returns (uint256) {
        LPToken liquidityToken = LPToken(dfmm.pools(poolId).liquidityToken);
        uint256 balance = liquidityToken.balanceOf(account);
        uint256 totalSupply = liquidityToken.totalSupply();
        uint256 totalLiquidity = dfmm.pools(poolId).totalLiquidity;
        uint256 liquidityOwned = balance * totalLiquidity / totalSupply;
        uint256 remainder = balance * totalLiquidity % totalSupply;
        if (remainder == 0) {
            return liquidityOwned;
        } else {
            return liquidityOwned + 1;
        }
    }
}

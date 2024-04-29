// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "src/SYCoveredCall/SYCoveredCall.sol";
import "src/SYCoveredCall/SYCoveredCallSolver.sol";
import "test/utils/SetUp.sol";
import { ONE } from "src/lib/StrategyLib.sol";
import { YEAR } from "src/SYCoveredCall/SYCoveredCallMath.sol";
import { InitParams } from "src/interfaces/IDFMM.sol";
import "forge-std/console2.sol";
import { IPMarket } from "pendle/interfaces/IPMarket.sol";
import "pendle/core/Market/MarketMathCore.sol";
import "pendle/interfaces/IPAllActionV3.sol";

contract SYCoveredCallSetUp is SetUp {
    using MarketMathCore for MarketState;
    using MarketMathCore for int256;
    using MarketMathCore for uint256;

    SYCoveredCall coveredCall;
    SYCoveredCallSolver solver;

    uint256 public POOL_ID;
    uint256 public constant FEE = 0.00001 ether;

    uint256 defaultReserveX = 100 ether;
    uint256 defaultReserveXMil = 1_000_000 ether;
    uint256 defaultReserveXDeep = ONE * 10_000_000;

    uint256 defaultPrice = ONE;
    uint256 defaultPricePoint9Rate = 0.84167999326 ether;

    address public constant wstETH = 0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0; //real wsteth
    IPMarket public constant market =
        IPMarket(0xC374f7eC85F8C7DE3207a10bB1978bA104bdA3B2);
    IPAllActionV3 public constant router =
        IPAllActionV3(0x00000000005BBB0EF59571E58418F9a4357b68A0);

    IStandardizedYield public SY;
    IPPrincipalToken public PT;
    IPYieldToken public YT;
    MarketState public pendleMarketState;
    int256 pendleRateAnchor;
    int256 pendleRateScalar;
    uint256 timeToExpiry;

    function mintSY(uint256 amount) public {
        IERC20(wstETH).approve(address(SY), type(uint256).max);
        SY.deposit(address(this), address(wstETH), amount, 1);
    }

    function mintPtYt(uint256 amount) public {
        SY.transfer(address(YT), amount);
        YT.mintPY(address(this), address(this));
    }

    function setUp() public override {
        vm.createSelectFork({ urlOrAlias: "mainnet", blockNumber: 19_662_269 });
        SetUp.setUp();
        (SY, PT, YT) = IPMarket(market).readTokens();
        pendleMarketState = market.readState(address(router));
        coveredCall = new SYCoveredCall(address(dfmm));
        solver = new SYCoveredCallSolver(coveredCall);
        timeToExpiry = pendleMarketState.expiry - block.timestamp;
        pendleRateScalar = pendleMarketState._getRateScalar(timeToExpiry);

        pendleRateAnchor = pendleMarketState.totalPt._getRateAnchor(
            pendleMarketState.lastLnImpliedRate,
            pendleMarketState.totalSy,
            pendleRateScalar,
            timeToExpiry
        );

        deal(wstETH, address(this), 1_000_000e18);

        mintSY(100_000 ether);
        mintPtYt(50_000 ether);

        IERC20(wstETH).approve(address(dfmm), type(uint256).max);
        IERC20(SY).approve(address(dfmm), type(uint256).max);
        IERC20(PT).approve(address(dfmm), type(uint256).max);
        IERC20(YT).approve(address(dfmm), type(uint256).max);

        IERC20(wstETH).approve(address(router), type(uint256).max);
        IERC20(SY).approve(address(router), type(uint256).max);
        IERC20(PT).approve(address(router), type(uint256).max);
        IERC20(YT).approve(address(router), type(uint256).max);
        IERC20(market).approve(address(router), type(uint256).max);
    }

    modifier init() {
        address[] memory tokens = new address[](2);
        tokens[0] = address(tokenX);
        tokens[1] = address(tokenY);

        SYCoveredCallParams memory defaultParams = SYCoveredCallParams({
            mean: uint256(pendleRateAnchor),
            width: 0.1 ether,
            maturity: PT.expiry(),
            swapFee: 0.0005 ether,
            lastTimestamp: block.timestamp,
            controller: address(this),
            SY: SY,
            PT: PT,
            YT: YT
        });

        bytes memory initialPoolData =
            solver.prepareInit(defaultReserveX, defaultPrice, defaultParams);

        InitParams memory defaultInitParams = InitParams({
            name: "",
            symbol: "",
            strategy: address(coveredCall),
            tokens: tokens,
            data: initialPoolData,
            feeCollector: address(0),
            controllerFee: 0
        });
        console2.log("timestamp", block.timestamp);

        (POOL_ID,,) = dfmm.init(defaultInitParams);

        _;
    }
}

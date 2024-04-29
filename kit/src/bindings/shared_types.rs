/// `DynamicParam(uint256,uint256,int256,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct DynamicParam {
    pub last_computed_value: ::ethers::core::types::U256,
    pub update_end: ::ethers::core::types::U256,
    pub update_per_second: ::ethers::core::types::I256,
    pub last_update_at: ::ethers::core::types::U256,
}
/// `InitParams(string,string,address,address[],bytes,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct InitParams {
    pub name: ::std::string::String,
    pub symbol: ::std::string::String,
    pub strategy: ::ethers::core::types::Address,
    pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    pub data: ::ethers::core::types::Bytes,
    pub fee_collector: ::ethers::core::types::Address,
    pub controller_fee: ::ethers::core::types::U256,
}
/// `LogNormalParams(uint256,uint256,uint256,address)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct LogNormalParams {
    pub mean: ::ethers::core::types::U256,
    pub width: ::ethers::core::types::U256,
    pub swap_fee: ::ethers::core::types::U256,
    pub controller: ::ethers::core::types::Address,
}
/// `Pool(address,address[],uint256[],uint256,address,address,uint256)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct Pool {
    pub strategy: ::ethers::core::types::Address,
    pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
    pub reserves: ::std::vec::Vec<::ethers::core::types::U256>,
    pub total_liquidity: ::ethers::core::types::U256,
    pub liquidity_token: ::ethers::core::types::Address,
    pub fee_collector: ::ethers::core::types::Address,
    pub controller_fee: ::ethers::core::types::U256,
}
/// `FuzzInterface(address,string[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FuzzInterface {
    pub addr: ::ethers::core::types::Address,
    pub artifacts: ::std::vec::Vec<::std::string::String>,
}
/// `FuzzSelector(address,bytes4[])`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash,
)]
pub struct FuzzSelector {
    pub addr: ::ethers::core::types::Address,
    pub selectors: ::std::vec::Vec<[u8; 4]>,
}

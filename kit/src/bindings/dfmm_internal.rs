pub use dfmm_internal::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod dfmm_internal {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("weth_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                ::std::boxed::Box::new(
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ),
                            ),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256[]"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("init"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::String,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct InitParams"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lpTokenImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpTokenImplementation",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct Pool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("token"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address[]"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amounts"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256[]"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("update"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("data"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("weth"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Allocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltas"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltas"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Init"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Init"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("strategy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("lpToken"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokens"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Address,
                                    ),
                                ),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserves"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("recipient"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("inputAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("outputAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidDuplicateTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidDuplicateTokens",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidInvariant"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidInvariant"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("invariant"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMaximumTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMaximumTokens",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMinimumTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMinimumTokens",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReserves"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidReserves"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokenDecimals"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTokenDecimals",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTransfer"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Locked"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotController"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotController"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static DFMMINTERNAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`\x01\x80U4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0=\x0E8\x03\x80b\0=\x0E\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\0\xFFV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0R`@Q\x81\x90b\0\0U\x90b\0\0\xF1V[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0rW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01RcL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\0\xD0W`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\0\xE5W=`\0\x80>=`\0\xFD[PPPPPPb\0\x011V[a\x0E\xB9\x80b\0.U\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x01\x12W`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01*W`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa,\xD5b\0\x01\x80`\09`\0\x81\x81`\xA5\x01R\x81\x81a\x01\x7F\x01R\x81\x81a\x1A\xFF\x01R\x81\x81a\x1BE\x01R\x81\x81a\x1C\x83\x01Ra\x1C\xD0\x01R`\0\x81\x81a\x02+\x01Ra\x11)\x01Ra,\xD5`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x95W`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0YW\x80c\x9D\x94/\x9A\x14a\x01\xCCW\x80c\xACJ\xFA8\x14a\x01\xECW\x80c\xB4b\xCD%\x14a\x02\x19W\x80c\xBE\xAB\xAC\xC8\x14a\x02MW\x80c\xEB&\xF3h\x14a\x02mW`\0\x80\xFD[\x80c\x02\x16\xB88\x14a\0\xEAW\x80c\x1Cm\xA7$\x14a\x01\nW\x80c.\xC3\x81\x88\x14a\x01MW\x80c?\xC8\xCE\xF3\x14a\x01mW\x80cw\xEA\xBBI\x14a\x01\xB9W`\0\x80\xFD[6a\0\xE5W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xE3W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xF6W`\0\x80\xFD[Pa\0\xE3a\x01\x056`\x04a\"lV[a\x02\x8FV[a\x01\x1Da\x01\x186`\x04a\"\xCFV[a\x03nV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x94\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xF3[a\x01`a\x01[6`\x04a\"lV[a\t\x0FV[`@Qa\x01D\x91\x90a#eV[4\x80\x15a\x01yW`\0\x80\xFD[Pa\x01\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01DV[a\0\xE3a\x01\xC76`\x04a$RV[a\x0B\xF6V[4\x80\x15a\x01\xD8W`\0\x80\xFD[Pa\x01`a\x01\xE76`\x04a\"lV[a\x0C\x04V[4\x80\x15a\x01\xF8W`\0\x80\xFD[Pa\x02\x0Ca\x02\x076`\x04a%\x12V[a\x0E\xF9V[`@Qa\x01D\x91\x90a%\xF7V[4\x80\x15a\x02%W`\0\x80\xFD[Pa\x01\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02YW`\0\x80\xFD[Pa\0\xE3a\x02h6`\x04a&\nV[a\x10\x84V[a\x02\x80a\x02{6`\x04a&FV[a\x10\x94V[`@Qa\x01D\x93\x92\x91\x90a&\x88V[`\x01T`\x02\x03a\x02\xB2W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x02\xCBWa\x02\xCBa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xD8\xB5\xED\x12\x913\x91\x87\x91\x82\x90\x81\x10a\x03\x03Wa\x03\x03a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x86\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03aW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x03\x97W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UPa\x03\xE1`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x89\x81T\x81\x10a\x03\xF4Wa\x03\xF4a&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cu\xE6D\x0F\x913\x91\x8D\x91\x82\x90\x81\x10a\x04,Wa\x04,a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\\\x95\x94\x93\x92\x91\x90a'eV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9D\x91\x90a(LV[`\xC0\x88\x01R`\xA0\x87\x01R`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x15\x15\x80\x82Ra\x04\xEEW\x80` \x01Q`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x01a\x04\xE5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x8A\x81T\x81\x10a\x05\x02Wa\x05\x02a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01T\x11\x15a\x06\x1FW`\0a\x05W`\0\x8B\x81T\x81\x10a\x052Wa\x052a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01T\x83`\xC0\x01Qa\x18O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x82`\xC0\x01Qa\x05i\x91\x90a(\xBDV[`\0\x8B\x81T\x81\x10a\x05|Wa\x05|a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x05\x9C\x91\x90a(\xD0V[\x92PP\x81\x90UPa\x05\xDF`\0\x8B\x81T\x81\x10a\x05\xB9Wa\x05\xB9a&\xB1V[`\0\x91\x82R` \x90\x91 `\x05`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01\x84a\x18mV[\x80`\0\x8B\x81T\x81\x10a\x05\xF3Wa\x05\xF3a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x06\x13\x91\x90a(\xD0V[\x90\x91UPa\x06]\x91PPV[\x80`\xC0\x01Q`\0\x8A\x81T\x81\x10a\x067Wa\x067a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x06W\x91\x90a(\xD0V[\x90\x91UPP[\x80`\x80\x01Q`\0\x8A\x81T\x81\x10a\x06uWa\x06ua&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82`@\x01Q\x81T\x81\x10a\x06\x9BWa\x06\x9Ba&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\x06\xB4\x91\x90a(\xD0V[\x90\x91UPP`\xA0\x81\x01Q`\0\x80T\x8B\x90\x81\x10a\x06\xD2Wa\x06\xD2a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82``\x01Q\x81T\x81\x10a\x06\xF8Wa\x06\xF8a&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\x07\x11\x91\x90a(\xBDV[\x92PP\x81\x90UP`\0\x80\x8A\x81T\x81\x10a\x07,Wa\x07,a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x82`@\x01Q\x81T\x81\x10a\x07RWa\x07Ra&\xB1V[`\0\x91\x82R` \x82 \x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92P\x81\x90\x8C\x90\x81\x10a\x07~Wa\x07~a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x83``\x01Q\x81T\x81\x10a\x07\xA4Wa\x07\xA4a&\xB1V[`\0\x91\x82R` \x82 \x01T`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x07\xF1Wa\x07\xF1a&\xB1V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x84`\x80\x01Q\x81`\0\x81Q\x81\x10a\x08FWa\x08Fa&\xB1V[` \x02` \x01\x01\x81\x81RPPa\x08\\\x82\x82a\x1A0V[a\x08k\x83\x8D\x87`\xA0\x01Qa\x1C\x81V[\x8C3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA6\xD7\x8D\xC7\x9Fm\x8C\x83\xD5\xB7\x15E\xED.\xDDS\x8F]K\xA7^Ru*dV\xF2\xBDD\xAD\xF9\x06\x8E\x87\x87\x8A`\x80\x01Q\x8B`\xA0\x01Q`@Qa\x08\xE4\x95\x94\x93\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA3PP`\x80\x83\x01Q`\xA0\x90\x93\x01Q`\x01\x80U\x91\x9B\x90\x9AP\x91\x98P\x96P\x94PPPPPV[```\x01T`\x02\x03a\t4W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x88\x81T\x81\x10a\tUWa\tUa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c|\x10\x12D\x913\x91\x8C\x91\x82\x90\x81\x10a\t\x8DWa\t\x8Da&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xBD\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x02\x91\x90\x81\x01\x90a(\xE3V[\x93P\x93P\x93P\x93P\x83a\n+W`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\0\x80\x89\x81T\x81\x10a\n?Wa\n?a&\xB1V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\n\xD2W\x83\x81\x81Q\x81\x10a\noWa\noa&\xB1V[` \x02` \x01\x01Q`\0\x8B\x81T\x81\x10a\n\x8AWa\n\x8Aa&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82\x81T\x81\x10a\n\xACWa\n\xACa&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\n\xC5\x91\x90a(\xD0V[\x90\x91UPP`\x01\x01a\nUV[Pa\n\xE03\x8A`\x01\x85a\x18mV[\x81`\0\x8A\x81T\x81\x10a\n\xF4Wa\n\xF4a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0B\x14\x91\x90a(\xD0V[\x92PP\x81\x90UPa\x0B\xA1`\0\x8A\x81T\x81\x10a\x0B1Wa\x0B1a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x96W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BxW[PPPPP\x84a\x1A0V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F[\xD9&0pCI\x9E\x1E\xFF\xF9\xC4Ei\x85\x84\xA1\xB8^2t\n\xD2\x04\xCB\xE7\xC9\x083\xFA2\x97\x8A\x85\x85`@Qa\x0B\xDE\x93\x92\x91\x90a&\x88V[`@Q\x80\x91\x03\x90\xA2PP`\x01\x80U\x96\x95PPPPPPV[a\x0C\0\x82\x82a\x1A0V[PPV[```\x01T`\x02\x03a\x0C)W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x88\x81T\x81\x10a\x0CJWa\x0CJa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x04\r\x95\x1E\x913\x91\x8C\x91\x82\x90\x81\x10a\x0C\x82Wa\x0C\x82a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xB2\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xF7\x91\x90\x81\x01\x90a(\xE3V[\x93P\x93P\x93P\x93P\x83a\r W`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\0\x80\x89\x81T\x81\x10a\r4Wa\r4a&\xB1V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\r\xC7W\x83\x81\x81Q\x81\x10a\rdWa\rda&\xB1V[` \x02` \x01\x01Q`\0\x8B\x81T\x81\x10a\r\x7FWa\r\x7Fa&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82\x81T\x81\x10a\r\xA1Wa\r\xA1a&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\r\xBA\x91\x90a(\xBDV[\x90\x91UPP`\x01\x01a\rJV[Pa\r\xD53\x8A`\0\x85a\x18mV[\x81`\0\x8A\x81T\x81\x10a\r\xE9Wa\r\xE9a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0E\t\x91\x90a(\xBDV[\x90\x91UP`\0\x90P[\x81\x81\x10\x15a\x0E\x98Wa\x0E\x90`\0\x8B\x81T\x81\x10a\x0E0Wa\x0E0a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x82\x81T\x81\x10a\x0ERWa\x0ERa&\xB1V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x86\x84\x81Q\x81\x10a\x0E\x83Wa\x0E\x83a&\xB1V[` \x02` \x01\x01Qa\x1C\x81V[`\x01\x01a\x0E\x12V[P\x82`@Qa\x0E\xA7\x91\x90a)\x96V[`@\x80Q\x91\x82\x90\x03\x82 \x8B\x83R` \x83\x01\x85\x90R\x913\x91\x7F\xED\xDA\xCF\x8A\x7F\xCA\xC4\x16\xBF\x1B{O4\xA2\xA3\xC9\xDF\xAE:\xD3q9\xE0[\x91;w\xAB\x9D\xC3\x9C\x90\x91\x01`@Q\x80\x91\x03\x90\xA3PP`\x01\x80U\x96\x95PPPPPPV[a\x0FT`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`\0\x82\x81T\x81\x10a\x0FgWa\x0Fga&\xB1V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0F\xE8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0F\xCAW[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x10@W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10,W[PPP\x91\x83RPP`\x03\x82\x01T` \x82\x01R`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x05\x83\x01T\x16``\x82\x01R`\x06\x90\x91\x01T`\x80\x90\x91\x01R\x92\x91PPV[a\x10\x8F\x83\x83\x83a\x1C\x81V[PPPV[`\0```\0`\x01T`\x02\x03a\x10\xBDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90Ua\x10\xD1``\x86\x01\x86a)\xCCV[\x90P\x10\x15a\x10\xF2W`@Qc*wA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08a\x11\x01``\x86\x01\x86a)\xCCV[\x90P\x11\x15a\x11\"W`@Qc@\x9E\x14\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1ElV[\x90P`\0`@Q\x80`\xE0\x01`@R\x80\x87`@\x01` \x81\x01\x90a\x11o\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x11\x8A``\x89\x01\x89a)\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x11\xCE``\x89\x01\x89a)\xCCV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE8Wa\x11\xE8a#xV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x82\x01R``\x01a\x12=`\xC0\x89\x01`\xA0\x8A\x01a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x90\x91\x01R\x90P`\0\x80\x80\x80a\x12j``\x8B\x01`@\x8C\x01a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16cO\x17\xD9\x133`\0\x80T\x90P\x88\x8E\x80`\x80\x01\x90a\x12\x90\x91\x90a*1V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB0\x95\x94\x93\x92\x91\x90a*xV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xF7\x91\x90\x81\x01\x90a(\xE3V[\x92\x96P\x90\x94P\x92P\x90Pa\x13\x0E``\x8B\x01\x8Ba)\xCCV[\x90P\x82Q\x14a\x130W`@Qc=\xCED\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x13QW`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\x01`\x01`\xA0\x1B\x03\x86\x16cL\xD8\x8Bva\x13j\x8C\x80a*1V[a\x13w` \x8F\x01\x8Fa*1V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x96\x94\x93\x92\x91\x90a*\xB2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xC4W=`\0\x80>=`\0\xFD[PPPP\x85`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x84a\x13\xE6\x91\x90a(\xBDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14@W=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPP`@\x85\x01\x82\x90R``\x85\x01\x81\x90R`\0\x80T`\x01\x81\x01\x82U\x90\x80R\x85Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x07\x90\x92\x02\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x80\x88\x01Q\x80Q\x89\x94a\x15C\x93\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x90\x91\x01\x92\x01\x90a!nV[P`@\x82\x01Q\x80Qa\x15_\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a!\xD3V[P``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Q`\x04\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\xA0\x84\x01Q`\x05\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xC0\x90\x91\x01Q`\x06\x90\x91\x01U`\0\x80Ta\x15\xC0\x90`\x01\x90a(\xBDV[\x90P`\0a\x15\xD1``\x8D\x01\x8Da)\xCCV[\x90P\x90P`\0[\x81\x81\x10\x15a\x16\xA6W`\0a\x15\xEF``\x8F\x01\x8Fa)\xCCV[\x83\x81\x81\x10a\x15\xFFWa\x15\xFFa&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16\x14\x91\x90a*\x16V[\x90P`\0a\x16#\x83`\x01a(\xD0V[\x90P[\x83\x81\x10\x15a\x16\x9CW\x8E\x80``\x01\x90a\x16>\x91\x90a)\xCCV[\x82\x81\x81\x10a\x16NWa\x16Na&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16c\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x16\x94W`@Qc\x85c\x1EW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x16&V[PP`\x01\x01a\x15\xD8V[P`\0[\x81\x81\x10\x15a\x17\x82W`\0a\x16\xC1``\x8F\x01\x8Fa)\xCCV[\x83\x81\x81\x10a\x16\xD1Wa\x16\xD1a&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16\xE6\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17G\x91\x90a*\xE4V[`\xFF\x16\x90P`\x12\x81\x11\x80a\x17[WP`\x06\x81\x10[\x15a\x17yW`@Qchm6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x16\xAAV[Pa\x17\xCCa\x17\x93``\x8E\x01\x8Ea)\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92Pa\x1A0\x91PPV[\x86` \x01Q`@Qa\x17\xDE\x91\x90a+\x07V[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F)\xADRC\xFF\x81\xE7O*\x02\xB9W\xC0\xD8;{V \xEB\xF0\xBE\x8B0\x99\xD21\xC9\xF4\x98\xF6>\xE2\x89`\0\x01Q\x8B\x86\x8C`@\x01Q\x8D``\x01Q`@Qa\x183\x95\x94\x93\x92\x91\x90a+:V[`@Q\x80\x91\x03\x90\xA3P`\x01\x80U\x9A\x91\x99P\x97P\x95PPPPPPV[`\0a\x18d\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xDEV[\x90P[\x92\x91PPV[`\0\x80\x84\x81T\x81\x10a\x18\x81Wa\x18\x81a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x0C\x91\x90a+\x7FV[\x90P`\0\x80\x86\x81T\x81\x10a\x19\"Wa\x19\"a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P\x84\x15a\x19\xB5W`\0a\x19I\x85\x84\x84a\x1F\x0CV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x90\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x97W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xABW=`\0\x80>=`\0\xFD[PPPPPa\x1A'V[`\0a\x19\xC2\x85\x84\x84a\x1E\xDEV[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A!W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPPV[\x81Q`\0[\x81\x81\x10\x15a\x1CpW`\0\x84\x82\x81Q\x81\x10a\x1AQWa\x1AQa&\xB1V[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10a\x1AoWa\x1Aoa&\xB1V[` \x02` \x01\x01Q\x90P`\0a\x1A\x8D\x82a\x1A\x88\x85a\x1F+V[a\x1F\xC9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xFB\x91\x90a+\x7FV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\x1B>WP\x82G\x10\x15[\x15a\x1B\xBCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1B\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xB2W=`\0\x80>=`\0\xFD[PPPPPa\x1B\xC8V[a\x1B\xC8\x8430\x85a\x1F\xD5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C3\x91\x90a+\x7FV[\x90Pa\x1C?\x83\x83a(\xD0V[\x81\x10\x15a\x1C_W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x90\x93\x01\x92Pa\x1A5\x91PPV[PG\x15a\x10\x8FWa\x10\x8F3Ga cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1D>W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D0W=`\0\x80>=`\0\xFD[PPPPa\x10\x8F\x82\x82a cV[`\0a\x1DR\x82a\x1DM\x86a\x1F+V[a \xB4V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC0\x91\x90a+\x7FV[\x90Pa\x1D\xCD\x85\x85\x84a \xC0V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E8\x91\x90a+\x7FV[\x90Pa\x1ED\x83\x83a(\xBDV[\x81\x10\x15a\x1EdW`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xD9W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1E\xF6W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F$W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x90\x91\x90a*\xE4V[`\xFF\x16\x90P`\0a\x1F\xA2\x82`\x12a(\xBDV[\x90Pa\x1F\xAF\x81`\na,|V[a\x1F\xC1\x90g\r\xE0\xB6\xB3\xA7d\0\0a,\x88V[\x94\x93PPPPV[`\0a\x18d\x83\x83a!DV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a \\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x04\xE5V[PPPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x10\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x04\xE5V[`\0a\x18d\x83\x83a!YV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a!>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x04\xE5V[PPPPV[`\0a\x18d\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xDEV[`\0a\x18d\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1F\x0CV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a!\xC3W\x91` \x02\x82\x01[\x82\x81\x11\x15a!\xC3W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a!\x8EV[Pa!\xCF\x92\x91Pa\"\x0EV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a!\xC3W\x91` \x02\x82\x01[\x82\x81\x11\x15a!\xC3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a!\xF3V[[\x80\x82\x11\x15a!\xCFW`\0\x81U`\x01\x01a\"\x0FV[`\0\x80\x83`\x1F\x84\x01\x12a\"5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"MW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"eW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\x81W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x9FW`\0\x80\xFD[a\"\xAB\x86\x82\x87\x01a\"#V[\x94\x97\x90\x96P\x93\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E\xD9W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\"\xE5W`\0\x80\xFD[\x845\x93Pa\"\xF5` \x86\x01a\"\xB8V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\x11W`\0\x80\xFD[a#\x1D\x87\x82\x88\x01a\"#V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a#ZW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#>V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x18d` \x83\x01\x84a#)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#\xB7Wa#\xB7a#xV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#\xD9Wa#\xD9a#xV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a#\xF4W`\0\x80\xFD[\x815` a$\ta$\x04\x83a#\xBFV[a#\x8EV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a$+W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a$GW\x805\x83R\x91\x83\x01\x91\x83\x01a$0V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a$eW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$}W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$\x91W`\0\x80\xFD[\x815` a$\xA1a$\x04\x83a#\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a$\xC0W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xE5Wa$\xD6\x86a\"\xB8V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xC5V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a$\xFBW`\0\x80\xFD[Pa%\x08\x85\x82\x86\x01a#\xE3V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a%$W`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q`\xE0\x82\x86\x01\x81\x90R\x81Q\x90\x86\x01\x81\x90R`\0\x93\x91\x83\x01\x92\x90\x84\x90a\x01\0\x88\x01\x90[\x80\x83\x10\x15a%\x83W\x85Q\x85\x16\x82R\x94\x83\x01\x94`\x01\x92\x90\x92\x01\x91\x90\x83\x01\x90a%aV[P`@\x87\x01Q\x94P\x87\x81\x03`@\x89\x01Ra%\x9D\x81\x86a#)V[\x94PPPPP``\x83\x01Q``\x85\x01R`\x80\x83\x01Qa%\xC7`\x80\x86\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa%\xE2`\xA0\x86\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a\x18d` \x83\x01\x84a%+V[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x1FW`\0\x80\xFD[a&(\x84a\"\xB8V[\x92Pa&6` \x85\x01a\"\xB8V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&XW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&oW`\0\x80\xFD[\x82\x01`\xE0\x81\x85\x03\x12\x15a&\x81W`\0\x80\xFD[\x93\x92PPPV[\x83\x81R``` \x82\x01R`\0a&\xA1``\x83\x01\x85a#)V[\x90P\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R` `\0 `\0[\x83\x81\x10\x15a#ZW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95`\x01\x91\x82\x01\x91\x01a&\xE1V[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R` `\0 `\0[\x83\x81\x10\x15a#ZW\x81T\x87R\x95\x82\x01\x95`\x01\x91\x82\x01\x91\x01a' V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\x80`@\x82\x01Ra'\xA2`\x80\x82\x01a'\x95\x86T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\xE0`\xA0\x82\x01R`\0a'\xBCa\x01`\x83\x01`\x01\x87\x01a&\xC7V[\x82\x81\x03`\x7F\x19\x01`\xC0\x84\x01Ra'\xD5\x81`\x02\x88\x01a'\x06V[\x90P`\x03\x86\x01T`\xE0\x84\x01Ra'\xF5`\x04\x87\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\0\x85\x01R`\x05\x87\x01T\x16a\x01 \x84\x01R`\x06\x86\x01Ta\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra(0\x81\x85\x87a'<V[\x98\x97PPPPPPPPV[\x80Q\x80\x15\x15\x81\x14a\x1E\xD9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a(gW`\0\x80\xFD[a(p\x88a(<V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18gWa\x18ga(\xA7V[\x80\x82\x01\x80\x82\x11\x15a\x18gWa\x18ga(\xA7V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(\xF9W`\0\x80\xFD[a)\x02\x85a(<V[\x93P` \x80\x86\x01Q\x93P`@\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)&W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a)7W`\0\x80\xFD[\x80Qa)Ea$\x04\x82a#\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15a)dW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a)\x82W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a)iV[``\x99\x90\x99\x01Q\x97\x9A\x96\x99PPPPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a)\xC0W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a)\xA4V[P\x92\x96\x95PPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a)\xE3W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xFEW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\"eW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a*(W`\0\x80\xFD[a\x18d\x82a\"\xB8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a*HW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*cW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\"eW`\0\x80\xFD[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\x80`@\x82\x01R`\0a*\x9F`\x80\x83\x01\x86a%+V[\x82\x81\x03``\x84\x01Ra(0\x81\x85\x87a'<V[`@\x81R`\0a*\xC6`@\x83\x01\x86\x88a'<V[\x82\x81\x03` \x84\x01Ra*\xD9\x81\x85\x87a'<V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xF6W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a&\x81W`\0\x80\xFD[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a)\xC0W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a+\x15V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R`\xA0``\x82\x01\x81\x90R`\0\x90a+m\x90\x83\x01\x85a#)V[\x90P\x82`\x80\x83\x01R\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a+\x91W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81[\x80\x85\x11\x15a+\xD3W\x81`\0\x19\x04\x82\x11\x15a+\xB9Wa+\xB9a(\xA7V[\x80\x85\x16\x15a+\xC6W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a+\x9DV[P\x92P\x92\x90PV[`\0\x82a+\xEAWP`\x01a\x18gV[\x81a+\xF7WP`\0a\x18gV[\x81`\x01\x81\x14a,\rW`\x02\x81\x14a,\x17Wa,3V[`\x01\x91PPa\x18gV[`\xFF\x84\x11\x15a,(Wa,(a(\xA7V[PP`\x01\x82\x1Ba\x18gV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a,VWP\x81\x81\na\x18gV[a,`\x83\x83a+\x98V[\x80`\0\x19\x04\x82\x11\x15a,tWa,ta(\xA7V[\x02\x93\x92PPPV[`\0a\x18d\x83\x83a+\xDBV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18gWa\x18ga(\xA7V\xFE\xA2dipfsX\"\x12 \xF6\x89\x8A\x0F\xDE\xDB\x04sUN\xEFd\x87\x92\x95\xEE\xD9\x83\xCC[\xE1\x8A0\xB6>G\xA7\x93\xD44\xEC\x9AdsolcC\0\x08\x16\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0E\x99\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cL\xD8\x8Bv\x11a\0\xA2W\x80c\x9D\xC2\x9F\xAC\x11a\0qW\x80c\x9D\xC2\x9F\xAC\x14a\x02!W\x80c\xA9\x05\x9C\xBB\x14a\x024W\x80c\xAF\xBA\x13\xC4\x14a\x02GW\x80c\xD5\x05\xAC\xCF\x14a\x02rW\x80c\xDDb\xED>\x14a\x02\x85W`\0\x80\xFD[\x80cL\xD8\x8Bv\x14a\x01\xC6W\x80cp\xA0\x821\x14a\x01\xD9W\x80c~\xCE\xBE\0\x14a\x01\xF9W\x80c\x95\xD8\x9BA\x14a\x02\x19W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\0\xDEW\x80c#\xB8r\xDD\x14a\x01|W\x80c1<\xE5g\x14a\x01\x8FW\x80c6D\xE5\x15\x14a\x01\xA9W\x80c@\xC1\x0F\x19\x14a\x01\xB1W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x10W\x80c\t^\xA7\xB3\x14a\x01.W\x80c\x15\x8E\xF9>\x14a\x01QW\x80c\x18\x16\r\xDD\x14a\x01eW[`\0\x80\xFD[a\x01\x18a\x02\xB0V[`@Qa\x01%\x91\x90a\t\xA6V[`@Q\x80\x91\x03\x90\xF3[a\x01Aa\x01<6`\x04a\n\x11V[a\x03>V[`@Q\x90\x15\x15\x81R` \x01a\x01%V[`\x08Ta\x01A\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x01n`\x02T\x81V[`@Q\x90\x81R` \x01a\x01%V[a\x01Aa\x01\x8A6`\x04a\n;V[a\x03\xABV[a\x01\x97`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01%V[a\x01na\x04\x8BV[a\x01\xC4a\x01\xBF6`\x04a\n\x11V[a\x04\xAAV[\0[a\x01\xC4a\x01\xD46`\x04a\x0B\x1AV[a\x04\xE3V[a\x01na\x01\xE76`\x04a\x0B~V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x01na\x02\x076`\x04a\x0B~V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x18a\x05_V[a\x01\xC4a\x02/6`\x04a\n\x11V[a\x05lV[a\x01Aa\x02B6`\x04a\n\x11V[a\x05\xA1V[`\x08Ta\x02Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01%V[a\x01\xC4a\x02\x806`\x04a\x0B\xA0V[a\x06\x07V[a\x01na\x02\x936`\x04a\x0C\x13V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\0\x80Ta\x02\xBD\x90a\x0CFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xE9\x90a\x0CFV[\x80\x15a\x036W\x80`\x1F\x10a\x03\x0BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x036V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x03\x99\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\x07Wa\x03\xE2\x83\x82a\x0C\x96V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x04/\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90a\x04x\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x05TF\x14a\x04\xA3Wa\x04\x9Ea\x08PV[\x90P\x90V[P`\x06T\x90V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xD5W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xDF\x82\x82a\x08\xEAV[PPV[`\x08T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\rW`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\0a\x05+\x83\x82a\x0C\xFAV[P`\x01a\x058\x82\x82a\x0C\xFAV[PF`\x05Ua\x05Ea\x08PV[`\x06UPP`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x02\xBD\x90a\x0CFV[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x97W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xDF\x82\x82a\tDV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x05\xC2\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90a\x03\x99\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01a\x06ha\x04\x8BV[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07tW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x07\xAAWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x07\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x06SV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x08\x82\x91\x90a\r\xBAV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x08\xFC\x91\x90a\x0E0V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x0ED\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\tl\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90` \x01a\t8V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\t\xD4W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t\xB8V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x0CW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\n$W`\0\x80\xFD[a\n-\x83a\t\xF5V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\nPW`\0\x80\xFD[a\nY\x84a\t\xF5V[\x92Pa\ng` \x85\x01a\t\xF5V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\n\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xB9Wa\n\xB9a\nwV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\n\xE1Wa\n\xE1a\nwV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BEW`\0\x80\xFD[a\x0BQ\x86\x83\x87\x01a\n\x8DV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0BgW`\0\x80\xFD[Pa\x0Bt\x85\x82\x86\x01a\n\x8DV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x90W`\0\x80\xFD[a\x0B\x99\x82a\t\xF5V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xBBW`\0\x80\xFD[a\x0B\xC4\x88a\t\xF5V[\x96Pa\x0B\xD2` \x89\x01a\t\xF5V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\xF6W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C&W`\0\x80\xFD[a\x0C/\x83a\t\xF5V[\x91Pa\x0C=` \x84\x01a\t\xF5V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0CZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0CzWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xA5Wa\x03\xA5a\x0C\x80V[`\x1F\x82\x11\x15a\x0C\xF5W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xD2WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xF1W\x82\x81U`\x01\x01a\x0C\xDEV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x14Wa\r\x14a\nwV[a\r(\x81a\r\"\x84Ta\x0CFV[\x84a\x0C\xA9V[` \x80`\x1F\x83\x11`\x01\x81\x14a\r]W`\0\x84\x15a\rEWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xF1V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r\x8CW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\rmV[P\x85\x82\x10\x15a\r\xAAW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\r\xC8\x81a\x0CFV[`\x01\x82\x81\x16\x80\x15a\r\xE0W`\x01\x81\x14a\r\xF5Wa\x0E$V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0E$V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0E\x1BW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x0E\x02V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03\xA5Wa\x03\xA5a\x0C\x80V\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x9D\xBD\xD5\xF38`H)\x96Ly\x01@\x04\x16\x97@\xEDix\xAB\0\xA1\xBBd\xF7c\xA2\x93\xF0\x8B\xEBdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DFMMINTERNAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x95W`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0YW\x80c\x9D\x94/\x9A\x14a\x01\xCCW\x80c\xACJ\xFA8\x14a\x01\xECW\x80c\xB4b\xCD%\x14a\x02\x19W\x80c\xBE\xAB\xAC\xC8\x14a\x02MW\x80c\xEB&\xF3h\x14a\x02mW`\0\x80\xFD[\x80c\x02\x16\xB88\x14a\0\xEAW\x80c\x1Cm\xA7$\x14a\x01\nW\x80c.\xC3\x81\x88\x14a\x01MW\x80c?\xC8\xCE\xF3\x14a\x01mW\x80cw\xEA\xBBI\x14a\x01\xB9W`\0\x80\xFD[6a\0\xE5W3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xE3W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\0\xF6W`\0\x80\xFD[Pa\0\xE3a\x01\x056`\x04a\"lV[a\x02\x8FV[a\x01\x1Da\x01\x186`\x04a\"\xCFV[a\x03nV[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x94\x90\x93\x16` \x85\x01R\x91\x83\x01R``\x82\x01R`\x80\x01[`@Q\x80\x91\x03\x90\xF3[a\x01`a\x01[6`\x04a\"lV[a\t\x0FV[`@Qa\x01D\x91\x90a#eV[4\x80\x15a\x01yW`\0\x80\xFD[Pa\x01\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01DV[a\0\xE3a\x01\xC76`\x04a$RV[a\x0B\xF6V[4\x80\x15a\x01\xD8W`\0\x80\xFD[Pa\x01`a\x01\xE76`\x04a\"lV[a\x0C\x04V[4\x80\x15a\x01\xF8W`\0\x80\xFD[Pa\x02\x0Ca\x02\x076`\x04a%\x12V[a\x0E\xF9V[`@Qa\x01D\x91\x90a%\xF7V[4\x80\x15a\x02%W`\0\x80\xFD[Pa\x01\xA1\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x02YW`\0\x80\xFD[Pa\0\xE3a\x02h6`\x04a&\nV[a\x10\x84V[a\x02\x80a\x02{6`\x04a&FV[a\x10\x94V[`@Qa\x01D\x93\x92\x91\x90a&\x88V[`\x01T`\x02\x03a\x02\xB2W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x02\xCBWa\x02\xCBa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\xD8\xB5\xED\x12\x913\x91\x87\x91\x82\x90\x81\x10a\x03\x03Wa\x03\x03a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x86\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x033\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x03MW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x03aW=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x03\x97W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UPa\x03\xE1`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x89\x81T\x81\x10a\x03\xF4Wa\x03\xF4a&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91cu\xE6D\x0F\x913\x91\x8D\x91\x82\x90\x81\x10a\x04,Wa\x04,a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\\\x95\x94\x93\x92\x91\x90a'eV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04yW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x9D\x91\x90a(LV[`\xC0\x88\x01R`\xA0\x87\x01R`\x80\x86\x01R``\x85\x01R`@\x84\x01R` \x83\x01R\x15\x15\x80\x82Ra\x04\xEEW\x80` \x01Q`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x01a\x04\xE5\x91\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xFD[`\0\x80\x8A\x81T\x81\x10a\x05\x02Wa\x05\x02a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01T\x11\x15a\x06\x1FW`\0a\x05W`\0\x8B\x81T\x81\x10a\x052Wa\x052a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01T\x83`\xC0\x01Qa\x18O\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P\x80\x82`\xC0\x01Qa\x05i\x91\x90a(\xBDV[`\0\x8B\x81T\x81\x10a\x05|Wa\x05|a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x05\x9C\x91\x90a(\xD0V[\x92PP\x81\x90UPa\x05\xDF`\0\x8B\x81T\x81\x10a\x05\xB9Wa\x05\xB9a&\xB1V[`\0\x91\x82R` \x90\x91 `\x05`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x8B`\x01\x84a\x18mV[\x80`\0\x8B\x81T\x81\x10a\x05\xF3Wa\x05\xF3a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x06\x13\x91\x90a(\xD0V[\x90\x91UPa\x06]\x91PPV[\x80`\xC0\x01Q`\0\x8A\x81T\x81\x10a\x067Wa\x067a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x06W\x91\x90a(\xD0V[\x90\x91UPP[\x80`\x80\x01Q`\0\x8A\x81T\x81\x10a\x06uWa\x06ua&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82`@\x01Q\x81T\x81\x10a\x06\x9BWa\x06\x9Ba&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\x06\xB4\x91\x90a(\xD0V[\x90\x91UPP`\xA0\x81\x01Q`\0\x80T\x8B\x90\x81\x10a\x06\xD2Wa\x06\xD2a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82``\x01Q\x81T\x81\x10a\x06\xF8Wa\x06\xF8a&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\x07\x11\x91\x90a(\xBDV[\x92PP\x81\x90UP`\0\x80\x8A\x81T\x81\x10a\x07,Wa\x07,a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x82`@\x01Q\x81T\x81\x10a\x07RWa\x07Ra&\xB1V[`\0\x91\x82R` \x82 \x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92P\x81\x90\x8C\x90\x81\x10a\x07~Wa\x07~a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x83``\x01Q\x81T\x81\x10a\x07\xA4Wa\x07\xA4a&\xB1V[`\0\x91\x82R` \x82 \x01T`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x93P\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x82\x81`\0\x81Q\x81\x10a\x07\xF1Wa\x07\xF1a&\xB1V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01R`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R`\0\x91\x81` \x01` \x82\x02\x806\x837\x01\x90PP\x90P\x84`\x80\x01Q\x81`\0\x81Q\x81\x10a\x08FWa\x08Fa&\xB1V[` \x02` \x01\x01\x81\x81RPPa\x08\\\x82\x82a\x1A0V[a\x08k\x83\x8D\x87`\xA0\x01Qa\x1C\x81V[\x8C3`\x01`\x01`\xA0\x1B\x03\x16\x7F\xA6\xD7\x8D\xC7\x9Fm\x8C\x83\xD5\xB7\x15E\xED.\xDDS\x8F]K\xA7^Ru*dV\xF2\xBDD\xAD\xF9\x06\x8E\x87\x87\x8A`\x80\x01Q\x8B`\xA0\x01Q`@Qa\x08\xE4\x95\x94\x93\x92\x91\x90`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x81R\x93\x85\x16` \x85\x01R\x91\x90\x93\x16`@\x83\x01R``\x82\x01\x92\x90\x92R`\x80\x81\x01\x91\x90\x91R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA3PP`\x80\x83\x01Q`\xA0\x90\x93\x01Q`\x01\x80U\x91\x9B\x90\x9AP\x91\x98P\x96P\x94PPPPPV[```\x01T`\x02\x03a\t4W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x88\x81T\x81\x10a\tUWa\tUa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c|\x10\x12D\x913\x91\x8C\x91\x82\x90\x81\x10a\t\x8DWa\t\x8Da&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xBD\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xDAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x02\x91\x90\x81\x01\x90a(\xE3V[\x93P\x93P\x93P\x93P\x83a\n+W`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\0\x80\x89\x81T\x81\x10a\n?Wa\n?a&\xB1V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\n\xD2W\x83\x81\x81Q\x81\x10a\noWa\noa&\xB1V[` \x02` \x01\x01Q`\0\x8B\x81T\x81\x10a\n\x8AWa\n\x8Aa&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82\x81T\x81\x10a\n\xACWa\n\xACa&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\n\xC5\x91\x90a(\xD0V[\x90\x91UPP`\x01\x01a\nUV[Pa\n\xE03\x8A`\x01\x85a\x18mV[\x81`\0\x8A\x81T\x81\x10a\n\xF4Wa\n\xF4a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0B\x14\x91\x90a(\xD0V[\x92PP\x81\x90UPa\x0B\xA1`\0\x8A\x81T\x81\x10a\x0B1Wa\x0B1a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x0B\x96W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0BxW[PPPPP\x84a\x1A0V[3`\x01`\x01`\xA0\x1B\x03\x16\x7F[\xD9&0pCI\x9E\x1E\xFF\xF9\xC4Ei\x85\x84\xA1\xB8^2t\n\xD2\x04\xCB\xE7\xC9\x083\xFA2\x97\x8A\x85\x85`@Qa\x0B\xDE\x93\x92\x91\x90a&\x88V[`@Q\x80\x91\x03\x90\xA2PP`\x01\x80U\x96\x95PPPPPPV[a\x0C\0\x82\x82a\x1A0V[PPV[```\x01T`\x02\x03a\x0C)W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x88\x81T\x81\x10a\x0CJWa\x0CJa&\xB1V[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x04\r\x95\x1E\x913\x91\x8C\x91\x82\x90\x81\x10a\x0C\x82Wa\x0C\x82a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01\x8B\x8B`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\xB2\x95\x94\x93\x92\x91\x90a'eV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xF7\x91\x90\x81\x01\x90a(\xE3V[\x93P\x93P\x93P\x93P\x83a\r W`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\0\x80\x89\x81T\x81\x10a\r4Wa\r4a&\xB1V[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x91P[\x81\x81\x10\x15a\r\xC7W\x83\x81\x81Q\x81\x10a\rdWa\rda&\xB1V[` \x02` \x01\x01Q`\0\x8B\x81T\x81\x10a\r\x7FWa\r\x7Fa&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x02\x01\x82\x81T\x81\x10a\r\xA1Wa\r\xA1a&\xB1V[\x90`\0R` `\0 \x01`\0\x82\x82Ta\r\xBA\x91\x90a(\xBDV[\x90\x91UPP`\x01\x01a\rJV[Pa\r\xD53\x8A`\0\x85a\x18mV[\x81`\0\x8A\x81T\x81\x10a\r\xE9Wa\r\xE9a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0E\t\x91\x90a(\xBDV[\x90\x91UP`\0\x90P[\x81\x81\x10\x15a\x0E\x98Wa\x0E\x90`\0\x8B\x81T\x81\x10a\x0E0Wa\x0E0a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x01\x01\x82\x81T\x81\x10a\x0ERWa\x0ERa&\xB1V[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x163\x86\x84\x81Q\x81\x10a\x0E\x83Wa\x0E\x83a&\xB1V[` \x02` \x01\x01Qa\x1C\x81V[`\x01\x01a\x0E\x12V[P\x82`@Qa\x0E\xA7\x91\x90a)\x96V[`@\x80Q\x91\x82\x90\x03\x82 \x8B\x83R` \x83\x01\x85\x90R\x913\x91\x7F\xED\xDA\xCF\x8A\x7F\xCA\xC4\x16\xBF\x1B{O4\xA2\xA3\xC9\xDF\xAE:\xD3q9\xE0[\x91;w\xAB\x9D\xC3\x9C\x90\x91\x01`@Q\x80\x91\x03\x90\xA3PP`\x01\x80U\x96\x95PPPPPPV[a\x0FT`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`\0\x82\x81T\x81\x10a\x0FgWa\x0Fga&\xB1V[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x16\x83R`\x01\x81\x01\x80T\x83Q\x81\x87\x02\x81\x01\x87\x01\x90\x94R\x80\x84R\x93\x94\x91\x93\x85\x83\x01\x93\x92\x83\x01\x82\x82\x80\x15a\x0F\xE8W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T`\x01`\x01`\xA0\x1B\x03\x16\x81R`\x01\x90\x91\x01\x90` \x01\x80\x83\x11a\x0F\xCAW[PPPPP\x81R` \x01`\x02\x82\x01\x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80T\x80\x15a\x10@W` \x02\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R` \x01\x90`\x01\x01\x90\x80\x83\x11a\x10,W[PPP\x91\x83RPP`\x03\x82\x01T` \x82\x01R`\x04\x82\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`@\x83\x01R`\x05\x83\x01T\x16``\x82\x01R`\x06\x90\x91\x01T`\x80\x90\x91\x01R\x92\x91PPV[a\x10\x8F\x83\x83\x83a\x1C\x81V[PPPV[`\0```\0`\x01T`\x02\x03a\x10\xBDW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90Ua\x10\xD1``\x86\x01\x86a)\xCCV[\x90P\x10\x15a\x10\xF2W`@Qc*wA1`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08a\x11\x01``\x86\x01\x86a)\xCCV[\x90P\x11\x15a\x11\"W`@Qc@\x9E\x14\xF5`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11M\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1ElV[\x90P`\0`@Q\x80`\xE0\x01`@R\x80\x87`@\x01` \x81\x01\x90a\x11o\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x11\x8A``\x89\x01\x89a)\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RPPP\x90\x82RP` \x01a\x11\xCE``\x89\x01\x89a)\xCCV[\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x11\xE8Wa\x11\xE8a#xV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x12\x11W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`\0` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16`@\x82\x01R``\x01a\x12=`\xC0\x89\x01`\xA0\x8A\x01a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x81R`\xC0\x88\x015` \x90\x91\x01R\x90P`\0\x80\x80\x80a\x12j``\x8B\x01`@\x8C\x01a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16cO\x17\xD9\x133`\0\x80T\x90P\x88\x8E\x80`\x80\x01\x90a\x12\x90\x91\x90a*1V[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x12\xB0\x95\x94\x93\x92\x91\x90a*xV[`\0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x12\xCFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x12\xF7\x91\x90\x81\x01\x90a(\xE3V[\x92\x96P\x90\x94P\x92P\x90Pa\x13\x0E``\x8B\x01\x8Ba)\xCCV[\x90P\x82Q\x14a\x130W`@Qc=\xCED\x8B`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83a\x13QW`@Qc\n\x8DQ\x9B`\xE2\x1B\x81R`\x04\x81\x01\x84\x90R`$\x01a\x04\xE5V[`\x01`\x01`\xA0\x1B\x03\x86\x16cL\xD8\x8Bva\x13j\x8C\x80a*1V[a\x13w` \x8F\x01\x8Fa*1V[`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x13\x96\x94\x93\x92\x91\x90a*\xB2V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\xB0W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xC4W=`\0\x80>=`\0\xFD[PPPP\x85`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x84a\x13\xE6\x91\x90a(\xBDV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14,W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14@W=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x89\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x8EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\xA2W=`\0\x80>=`\0\xFD[PPPP`@\x85\x01\x82\x90R``\x85\x01\x81\x90R`\0\x80T`\x01\x81\x01\x82U\x90\x80R\x85Q\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c`\x07\x90\x92\x02\x91\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x81U` \x80\x88\x01Q\x80Q\x89\x94a\x15C\x93\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5d\x90\x91\x01\x92\x01\x90a!nV[P`@\x82\x01Q\x80Qa\x15_\x91`\x02\x84\x01\x91` \x90\x91\x01\x90a!\xD3V[P``\x82\x01Q`\x03\x82\x01U`\x80\x82\x01Q`\x04\x82\x01\x80T`\x01`\x01`\xA0\x1B\x03\x92\x83\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x90\x91U`\xA0\x84\x01Q`\x05\x84\x01\x80T\x91\x90\x93\x16\x91\x16\x17\x90U`\xC0\x90\x91\x01Q`\x06\x90\x91\x01U`\0\x80Ta\x15\xC0\x90`\x01\x90a(\xBDV[\x90P`\0a\x15\xD1``\x8D\x01\x8Da)\xCCV[\x90P\x90P`\0[\x81\x81\x10\x15a\x16\xA6W`\0a\x15\xEF``\x8F\x01\x8Fa)\xCCV[\x83\x81\x81\x10a\x15\xFFWa\x15\xFFa&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16\x14\x91\x90a*\x16V[\x90P`\0a\x16#\x83`\x01a(\xD0V[\x90P[\x83\x81\x10\x15a\x16\x9CW\x8E\x80``\x01\x90a\x16>\x91\x90a)\xCCV[\x82\x81\x81\x10a\x16NWa\x16Na&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16c\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16\x82`\x01`\x01`\xA0\x1B\x03\x16\x03a\x16\x94W`@Qc\x85c\x1EW`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x01a\x16&V[PP`\x01\x01a\x15\xD8V[P`\0[\x81\x81\x10\x15a\x17\x82W`\0a\x16\xC1``\x8F\x01\x8Fa)\xCCV[\x83\x81\x81\x10a\x16\xD1Wa\x16\xD1a&\xB1V[\x90P` \x02\x01` \x81\x01\x90a\x16\xE6\x91\x90a*\x16V[`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x17#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x17G\x91\x90a*\xE4V[`\xFF\x16\x90P`\x12\x81\x11\x80a\x17[WP`\x06\x81\x10[\x15a\x17yW`@Qchm6\x07`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[P`\x01\x01a\x16\xAAV[Pa\x17\xCCa\x17\x93``\x8E\x01\x8Ea)\xCCV[\x80\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83` \x02\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x88\x92Pa\x1A0\x91PPV[\x86` \x01Q`@Qa\x17\xDE\x91\x90a+\x07V[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F)\xADRC\xFF\x81\xE7O*\x02\xB9W\xC0\xD8;{V \xEB\xF0\xBE\x8B0\x99\xD21\xC9\xF4\x98\xF6>\xE2\x89`\0\x01Q\x8B\x86\x8C`@\x01Q\x8D``\x01Q`@Qa\x183\x95\x94\x93\x92\x91\x90a+:V[`@Q\x80\x91\x03\x90\xA3P`\x01\x80U\x9A\x91\x99P\x97P\x95PPPPPPV[`\0a\x18d\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xDEV[\x90P[\x92\x91PPV[`\0\x80\x84\x81T\x81\x10a\x18\x81Wa\x18\x81a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xE8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x19\x0C\x91\x90a+\x7FV[\x90P`\0\x80\x86\x81T\x81\x10a\x19\"Wa\x19\"a&\xB1V[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P\x84\x15a\x19\xB5W`\0a\x19I\x85\x84\x84a\x1F\x0CV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\x04\x83\x01R`$\x82\x01\x83\x90R\x91\x92P\x90\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\x97W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xABW=`\0\x80>=`\0\xFD[PPPPPa\x1A'V[`\0a\x19\xC2\x85\x84\x84a\x1E\xDEV[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\rW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A!W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPPV[\x81Q`\0[\x81\x81\x10\x15a\x1CpW`\0\x84\x82\x81Q\x81\x10a\x1AQWa\x1AQa&\xB1V[` \x02` \x01\x01Q\x90P`\0\x84\x83\x81Q\x81\x10a\x1AoWa\x1Aoa&\xB1V[` \x02` \x01\x01Q\x90P`\0a\x1A\x8D\x82a\x1A\x88\x85a\x1F+V[a\x1F\xC9V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A\xD7W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1A\xFB\x91\x90a+\x7FV[\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x80\x15a\x1B>WP\x82G\x10\x15[\x15a\x1B\xBCW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x84`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x1B\x9EW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xB2W=`\0\x80>=`\0\xFD[PPPPPa\x1B\xC8V[a\x1B\xC8\x8430\x85a\x1F\xD5V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\x0FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1C3\x91\x90a+\x7FV[\x90Pa\x1C?\x83\x83a(\xD0V[\x81\x10\x15a\x1C_W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PP`\x01\x90\x93\x01\x92Pa\x1A5\x91PPV[PG\x15a\x10\x8FWa\x10\x8F3Ga cV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1D>W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\x1CW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1D0W=`\0\x80>=`\0\xFD[PPPPa\x10\x8F\x82\x82a cV[`\0a\x1DR\x82a\x1DM\x86a\x1F+V[a \xB4V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1D\x9CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1D\xC0\x91\x90a+\x7FV[\x90Pa\x1D\xCD\x85\x85\x84a \xC0V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1E\x14W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E8\x91\x90a+\x7FV[\x90Pa\x1ED\x83\x83a(\xBDV[\x81\x10\x15a\x1EdW`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x1E\xD9W`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91\x90PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1E\xF6W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1F$W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1FlW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F\x90\x91\x90a*\xE4V[`\xFF\x16\x90P`\0a\x1F\xA2\x82`\x12a(\xBDV[\x90Pa\x1F\xAF\x81`\na,|V[a\x1F\xC1\x90g\r\xE0\xB6\xB3\xA7d\0\0a,\x88V[\x94\x93PPPPV[`\0a\x18d\x83\x83a!DV[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a \\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x04\xE5V[PPPPPV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x10\x8FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x04\xE5V[`\0a\x18d\x83\x83a!YV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a!>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x04\xE5V[PPPPV[`\0a\x18d\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1E\xDEV[`\0a\x18d\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1F\x0CV[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a!\xC3W\x91` \x02\x82\x01[\x82\x81\x11\x15a!\xC3W\x82Q\x82T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x82U` \x90\x92\x01\x91`\x01\x90\x91\x01\x90a!\x8EV[Pa!\xCF\x92\x91Pa\"\x0EV[P\x90V[\x82\x80T\x82\x82U\x90`\0R` `\0 \x90\x81\x01\x92\x82\x15a!\xC3W\x91` \x02\x82\x01[\x82\x81\x11\x15a!\xC3W\x82Q\x82U\x91` \x01\x91\x90`\x01\x01\x90a!\xF3V[[\x80\x82\x11\x15a!\xCFW`\0\x81U`\x01\x01a\"\x0FV[`\0\x80\x83`\x1F\x84\x01\x12a\"5W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"MW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"eW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\"\x81W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\"\x9FW`\0\x80\xFD[a\"\xAB\x86\x82\x87\x01a\"#V[\x94\x97\x90\x96P\x93\x94PPPPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E\xD9W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a\"\xE5W`\0\x80\xFD[\x845\x93Pa\"\xF5` \x86\x01a\"\xB8V[\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a#\x11W`\0\x80\xFD[a#\x1D\x87\x82\x88\x01a\"#V[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a#ZW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a#>V[P\x94\x95\x94PPPPPV[` \x81R`\0a\x18d` \x83\x01\x84a#)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#\xB7Wa#\xB7a#xV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#\xD9Wa#\xD9a#xV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a#\xF4W`\0\x80\xFD[\x815` a$\ta$\x04\x83a#\xBFV[a#\x8EV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a$+W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a$GW\x805\x83R\x91\x83\x01\x91\x83\x01a$0V[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a$eW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a$}W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a$\x91W`\0\x80\xFD[\x815` a$\xA1a$\x04\x83a#\xBFV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x89\x84\x11\x15a$\xC0W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a$\xE5Wa$\xD6\x86a\"\xB8V[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a$\xC5V[\x96PP\x86\x015\x92PP\x80\x82\x11\x15a$\xFBW`\0\x80\xFD[Pa%\x08\x85\x82\x86\x01a#\xE3V[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a%$W`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q`\xE0\x82\x86\x01\x81\x90R\x81Q\x90\x86\x01\x81\x90R`\0\x93\x91\x83\x01\x92\x90\x84\x90a\x01\0\x88\x01\x90[\x80\x83\x10\x15a%\x83W\x85Q\x85\x16\x82R\x94\x83\x01\x94`\x01\x92\x90\x92\x01\x91\x90\x83\x01\x90a%aV[P`@\x87\x01Q\x94P\x87\x81\x03`@\x89\x01Ra%\x9D\x81\x86a#)V[\x94PPPPP``\x83\x01Q``\x85\x01R`\x80\x83\x01Qa%\xC7`\x80\x86\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x83\x01Qa%\xE2`\xA0\x86\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xC0\x83\x01Q`\xC0\x85\x01R\x80\x91PP\x92\x91PPV[` \x81R`\0a\x18d` \x83\x01\x84a%+V[`\0\x80`\0``\x84\x86\x03\x12\x15a&\x1FW`\0\x80\xFD[a&(\x84a\"\xB8V[\x92Pa&6` \x85\x01a\"\xB8V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a&XW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a&oW`\0\x80\xFD[\x82\x01`\xE0\x81\x85\x03\x12\x15a&\x81W`\0\x80\xFD[\x93\x92PPPV[\x83\x81R``` \x82\x01R`\0a&\xA1``\x83\x01\x85a#)V[\x90P\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R` `\0 `\0[\x83\x81\x10\x15a#ZW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x87R\x95\x82\x01\x95`\x01\x91\x82\x01\x91\x01a&\xE1V[`\0\x81T\x80\x84R` \x80\x85\x01\x94P\x83`\0R` `\0 `\0[\x83\x81\x10\x15a#ZW\x81T\x87R\x95\x82\x01\x95`\x01\x91\x82\x01\x91\x01a' V[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\x80`@\x82\x01Ra'\xA2`\x80\x82\x01a'\x95\x86T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[`\xE0`\xA0\x82\x01R`\0a'\xBCa\x01`\x83\x01`\x01\x87\x01a&\xC7V[\x82\x81\x03`\x7F\x19\x01`\xC0\x84\x01Ra'\xD5\x81`\x02\x88\x01a'\x06V[\x90P`\x03\x86\x01T`\xE0\x84\x01Ra'\xF5`\x04\x87\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16a\x01\0\x85\x01R`\x05\x87\x01T\x16a\x01 \x84\x01R`\x06\x86\x01Ta\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra(0\x81\x85\x87a'<V[\x98\x97PPPPPPPPV[\x80Q\x80\x15\x15\x81\x14a\x1E\xD9W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a(gW`\0\x80\xFD[a(p\x88a(<V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x18gWa\x18ga(\xA7V[\x80\x82\x01\x80\x82\x11\x15a\x18gWa\x18ga(\xA7V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a(\xF9W`\0\x80\xFD[a)\x02\x85a(<V[\x93P` \x80\x86\x01Q\x93P`@\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a)&W`\0\x80\xFD[\x86\x01`\x1F\x81\x01\x88\x13a)7W`\0\x80\xFD[\x80Qa)Ea$\x04\x82a#\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x8A\x83\x11\x15a)dW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a)\x82W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a)iV[``\x99\x90\x99\x01Q\x97\x9A\x96\x99PPPPPPPV[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a)\xC0W\x81Q\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a)\xA4V[P\x92\x96\x95PPPPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a)\xE3W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a)\xFEW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\"eW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a*(W`\0\x80\xFD[a\x18d\x82a\"\xB8V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a*HW`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a*cW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a\"eW`\0\x80\xFD[`\x01\x80`\xA0\x1B\x03\x86\x16\x81R\x84` \x82\x01R`\x80`@\x82\x01R`\0a*\x9F`\x80\x83\x01\x86a%+V[\x82\x81\x03``\x84\x01Ra(0\x81\x85\x87a'<V[`@\x81R`\0a*\xC6`@\x83\x01\x86\x88a'<V[\x82\x81\x03` \x84\x01Ra*\xD9\x81\x85\x87a'<V[\x97\x96PPPPPPPV[`\0` \x82\x84\x03\x12\x15a*\xF6W`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a&\x81W`\0\x80\xFD[\x81Q`\0\x90\x82\x90` \x80\x86\x01\x84[\x83\x81\x10\x15a)\xC0W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x93\x82\x01\x93\x90\x82\x01\x90`\x01\x01a+\x15V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R\x85\x16` \x82\x01R`@\x81\x01\x84\x90R`\xA0``\x82\x01\x81\x90R`\0\x90a+m\x90\x83\x01\x85a#)V[\x90P\x82`\x80\x83\x01R\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a+\x91W`\0\x80\xFD[PQ\x91\x90PV[`\x01\x81\x81[\x80\x85\x11\x15a+\xD3W\x81`\0\x19\x04\x82\x11\x15a+\xB9Wa+\xB9a(\xA7V[\x80\x85\x16\x15a+\xC6W\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a+\x9DV[P\x92P\x92\x90PV[`\0\x82a+\xEAWP`\x01a\x18gV[\x81a+\xF7WP`\0a\x18gV[\x81`\x01\x81\x14a,\rW`\x02\x81\x14a,\x17Wa,3V[`\x01\x91PPa\x18gV[`\xFF\x84\x11\x15a,(Wa,(a(\xA7V[PP`\x01\x82\x1Ba\x18gV[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a,VWP\x81\x81\na\x18gV[a,`\x83\x83a+\x98V[\x80`\0\x19\x04\x82\x11\x15a,tWa,ta(\xA7V[\x02\x93\x92PPPV[`\0a\x18d\x83\x83a+\xDBV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x18gWa\x18ga(\xA7V\xFE\xA2dipfsX\"\x12 \xF6\x89\x8A\x0F\xDE\xDB\x04sUN\xEFd\x87\x92\x95\xEE\xD9\x83\xCC[\xE1\x8A0\xB6>G\xA7\x93\xD44\xEC\x9AdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DFMMINTERNAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct DFMMInternal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DFMMInternal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DFMMInternal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DFMMInternal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DFMMInternal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DFMMInternal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMMInternal<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DFMMINTERNAL_ABI.clone(),
                client,
            ))
        }
        /// Constructs the general purpose `Deployer` instance based on the
        /// provided constructor arguments and sends it. Returns a new
        /// instance of a deployer that returns an instance of this contract
        /// after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the
        ///   argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract
        /// instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the
        /// `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                DFMMINTERNAL_ABI.clone(),
                DFMMINTERNAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `allocate` (0x2ec38188) function
        pub fn allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([46, 195, 129, 136], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `deallocate` (0x9d942f9a) function
        pub fn deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([157, 148, 47, 154], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `init` (0xeb26f368) function
        pub fn init(
            &self,
            params: InitParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([235, 38, 243, 104], (params,))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `lpTokenImplementation` (0xb462cd25) function
        pub fn lp_token_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([180, 98, 205, 37], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pool> {
            self.0
                .method_hash([172, 74, 250, 56], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `swap` (0x1c6da724) function
        pub fn swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            recipient: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([28, 109, 167, 36], (pool_id, recipient, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `transfer` (0xbeabacc8) function
        pub fn transfer(
            &self,
            token: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 171, 172, 200], (token, to, amount))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `transferFrom` (0x77eabb49) function
        pub fn transfer_from(
            &self,
            tokens: ::std::vec::Vec<::ethers::core::types::Address>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 234, 187, 73], (tokens, amounts))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllocateFilter> {
            self.0.event()
        }
        /// Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeallocateFilter> {
            self.0.event()
        }
        /// Gets the contract's `Init` event
        pub fn init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitFilter> {
            self.0.event()
        }
        /// Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DFMMInternalEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DFMMInternal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `ERC1167FailedCreateClone` with signature
    /// `ERC1167FailedCreateClone()` and selector `0xc2f868f4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "ERC1167FailedCreateClone", abi = "ERC1167FailedCreateClone()")]
    pub struct ERC1167FailedCreateClone;
    /// Custom Error type `InvalidDuplicateTokens` with signature
    /// `InvalidDuplicateTokens()` and selector `0x85631e57`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidDuplicateTokens", abi = "InvalidDuplicateTokens()")]
    pub struct InvalidDuplicateTokens;
    /// Custom Error type `InvalidInvariant` with signature
    /// `InvalidInvariant(int256)` and selector `0x2a35466c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidInvariant", abi = "InvalidInvariant(int256)")]
    pub struct InvalidInvariant {
        pub invariant: ::ethers::core::types::I256,
    }
    /// Custom Error type `InvalidMaximumTokens` with signature
    /// `InvalidMaximumTokens()` and selector `0x409e14f5`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidMaximumTokens", abi = "InvalidMaximumTokens()")]
    pub struct InvalidMaximumTokens;
    /// Custom Error type `InvalidMinimumTokens` with signature
    /// `InvalidMinimumTokens()` and selector `0xa9dd04c4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidMinimumTokens", abi = "InvalidMinimumTokens()")]
    pub struct InvalidMinimumTokens;
    /// Custom Error type `InvalidReserves` with signature `InvalidReserves()`
    /// and selector `0x7b9c8916`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidReserves", abi = "InvalidReserves()")]
    pub struct InvalidReserves;
    /// Custom Error type `InvalidTokenDecimals` with signature
    /// `InvalidTokenDecimals()` and selector `0x686d3607`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTokenDecimals", abi = "InvalidTokenDecimals()")]
    pub struct InvalidTokenDecimals;
    /// Custom Error type `InvalidTransfer` with signature `InvalidTransfer()`
    /// and selector `0x2f352531`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidTransfer", abi = "InvalidTransfer()")]
    pub struct InvalidTransfer;
    /// Custom Error type `Locked` with signature `Locked()` and selector
    /// `0x0f2e5b6c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "Locked", abi = "Locked()")]
    pub struct Locked;
    /// Custom Error type `NotController` with signature `NotController()` and
    /// selector `0x23019e67`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NotController", abi = "NotController()")]
    pub struct NotController;
    /// Custom Error type `OnlyWETH` with signature `OnlyWETH()` and selector
    /// `0x01f180c9`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "OnlyWETH", abi = "OnlyWETH()")]
    pub struct OnlyWETH;
    /// Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum DFMMInternalErrors {
        ERC1167FailedCreateClone(ERC1167FailedCreateClone),
        InvalidDuplicateTokens(InvalidDuplicateTokens),
        InvalidInvariant(InvalidInvariant),
        InvalidMaximumTokens(InvalidMaximumTokens),
        InvalidMinimumTokens(InvalidMinimumTokens),
        InvalidReserves(InvalidReserves),
        InvalidTokenDecimals(InvalidTokenDecimals),
        InvalidTransfer(InvalidTransfer),
        Locked(Locked),
        NotController(NotController),
        OnlyWETH(OnlyWETH),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMInternalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <ERC1167FailedCreateClone as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1167FailedCreateClone(decoded));
            }
            if let Ok(decoded) =
                <InvalidDuplicateTokens as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidDuplicateTokens(decoded));
            }
            if let Ok(decoded) = <InvalidInvariant as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidInvariant(decoded));
            }
            if let Ok(decoded) =
                <InvalidMaximumTokens as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidMaximumTokens(decoded));
            }
            if let Ok(decoded) =
                <InvalidMinimumTokens as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidMinimumTokens(decoded));
            }
            if let Ok(decoded) = <InvalidReserves as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidReserves(decoded));
            }
            if let Ok(decoded) =
                <InvalidTokenDecimals as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidTokenDecimals(decoded));
            }
            if let Ok(decoded) = <InvalidTransfer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTransfer(decoded));
            }
            if let Ok(decoded) = <Locked as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <NotController as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotController(decoded));
            }
            if let Ok(decoded) = <OnlyWETH as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OnlyWETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMInternalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC1167FailedCreateClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidDuplicateTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidInvariant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidMaximumTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMinimumTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReserves(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidTokenDecimals(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTransfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotController(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyWETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMInternalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1167FailedCreateClone as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidDuplicateTokens as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidInvariant as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMaximumTokens as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidMinimumTokens as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidReserves as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokenDecimals as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Locked as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotController as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OnlyWETH as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMInternalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1167FailedCreateClone(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDuplicateTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidInvariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMaximumTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMinimumTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReserves(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokenDecimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotController(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DFMMInternalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC1167FailedCreateClone> for DFMMInternalErrors {
        fn from(value: ERC1167FailedCreateClone) -> Self {
            Self::ERC1167FailedCreateClone(value)
        }
    }
    impl ::core::convert::From<InvalidDuplicateTokens> for DFMMInternalErrors {
        fn from(value: InvalidDuplicateTokens) -> Self {
            Self::InvalidDuplicateTokens(value)
        }
    }
    impl ::core::convert::From<InvalidInvariant> for DFMMInternalErrors {
        fn from(value: InvalidInvariant) -> Self {
            Self::InvalidInvariant(value)
        }
    }
    impl ::core::convert::From<InvalidMaximumTokens> for DFMMInternalErrors {
        fn from(value: InvalidMaximumTokens) -> Self {
            Self::InvalidMaximumTokens(value)
        }
    }
    impl ::core::convert::From<InvalidMinimumTokens> for DFMMInternalErrors {
        fn from(value: InvalidMinimumTokens) -> Self {
            Self::InvalidMinimumTokens(value)
        }
    }
    impl ::core::convert::From<InvalidReserves> for DFMMInternalErrors {
        fn from(value: InvalidReserves) -> Self {
            Self::InvalidReserves(value)
        }
    }
    impl ::core::convert::From<InvalidTokenDecimals> for DFMMInternalErrors {
        fn from(value: InvalidTokenDecimals) -> Self {
            Self::InvalidTokenDecimals(value)
        }
    }
    impl ::core::convert::From<InvalidTransfer> for DFMMInternalErrors {
        fn from(value: InvalidTransfer) -> Self {
            Self::InvalidTransfer(value)
        }
    }
    impl ::core::convert::From<Locked> for DFMMInternalErrors {
        fn from(value: Locked) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<NotController> for DFMMInternalErrors {
        fn from(value: NotController) -> Self {
            Self::NotController(value)
        }
    }
    impl ::core::convert::From<OnlyWETH> for DFMMInternalErrors {
        fn from(value: OnlyWETH) -> Self {
            Self::OnlyWETH(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "Allocate", abi = "Allocate(address,uint256,uint256[],uint256)")]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub deltas: ::std::vec::Vec<::ethers::core::types::U256>,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(address,uint256,uint256[],uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub deltas: ::ethers::core::types::H256,
        pub delta_l: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Init",
        abi = "Init(address,address,address,uint256,address[],uint256[],uint256)"
    )]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub lp_token: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub tokens: ::ethers::core::types::H256,
        pub reserves: ::std::vec::Vec<::ethers::core::types::U256>,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(
        name = "Swap",
        abi = "Swap(address,uint256,address,address,address,uint256,uint256)"
    )]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
    }
    /// Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum DFMMInternalEvents {
        AllocateFilter(AllocateFilter),
        DeallocateFilter(DeallocateFilter),
        InitFilter(InitFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for DFMMInternalEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(DFMMInternalEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(DFMMInternalEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(DFMMInternalEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(DFMMInternalEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DFMMInternalEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for DFMMInternalEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for DFMMInternalEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<InitFilter> for DFMMInternalEvents {
        fn from(value: InitFilter) -> Self {
            Self::InitFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for DFMMInternalEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    /// Container type for all input parameters for the `allocate` function with
    /// signature `allocate(uint256,bytes)` and selector `0x2ec38188`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "allocate", abi = "allocate(uint256,bytes)")]
    pub struct AllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `deallocate` function
    /// with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "deallocate", abi = "deallocate(uint256,bytes)")]
    pub struct DeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `init` function with
    /// signature `init((string,string,address,address[],bytes,address,
    /// uint256))` and selector `0xeb26f368`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "init",
        abi = "init((string,string,address,address[],bytes,address,uint256))"
    )]
    pub struct InitCall {
        pub params: InitParams,
    }
    /// Container type for all input parameters for the `lpTokenImplementation`
    /// function with signature `lpTokenImplementation()` and selector
    /// `0xb462cd25`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lpTokenImplementation", abi = "lpTokenImplementation()")]
    pub struct LpTokenImplementationCall;
    /// Container type for all input parameters for the `pools` function with
    /// signature `pools(uint256)` and selector `0xac4afa38`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `swap` function with
    /// signature `swap(uint256,address,bytes)` and selector `0x1c6da724`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "swap", abi = "swap(uint256,address,bytes)")]
    pub struct SwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub recipient: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `transfer` function with
    /// signature `transfer(address,address,uint256)` and selector `0xbeabacc8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transfer", abi = "transfer(address,address,uint256)")]
    pub struct TransferCall {
        pub token: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `transferFrom` function
    /// with signature `transferFrom(address[],uint256[])` and selector
    /// `0x77eabb49`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "transferFrom", abi = "transferFrom(address[],uint256[])")]
    pub struct TransferFromCall {
        pub tokens: ::std::vec::Vec<::ethers::core::types::Address>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    /// Container type for all input parameters for the `update` function with
    /// signature `update(uint256,bytes)` and selector `0x0216b838`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `weth` function with
    /// signature `weth()` and selector `0x3fc8cef3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
    /// Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum DFMMInternalCalls {
        Allocate(AllocateCall),
        Deallocate(DeallocateCall),
        Init(InitCall),
        LpTokenImplementation(LpTokenImplementationCall),
        Pools(PoolsCall),
        Swap(SwapCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Update(UpdateCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMInternalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) =
                <LpTokenImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LpTokenImplementation(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMInternalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deallocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LpTokenImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DFMMInternalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpTokenImplementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for DFMMInternalCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for DFMMInternalCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<InitCall> for DFMMInternalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<LpTokenImplementationCall> for DFMMInternalCalls {
        fn from(value: LpTokenImplementationCall) -> Self {
            Self::LpTokenImplementation(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for DFMMInternalCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMInternalCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<TransferCall> for DFMMInternalCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for DFMMInternalCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for DFMMInternalCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<WethCall> for DFMMInternalCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    /// Container type for all return fields from the `allocate` function with
    /// signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    pub struct AllocateReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    /// Container type for all return fields from the `deallocate` function with
    /// signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    pub struct DeallocateReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    /// Container type for all return fields from the `init` function with
    /// signature `init((string,string,address,address[],bytes,address,
    /// uint256))` and selector `0xeb26f368`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `lpTokenImplementation`
    /// function with signature `lpTokenImplementation()` and selector
    /// `0xb462cd25`
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
    pub struct LpTokenImplementationReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `pools` function with
    /// signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn(pub Pool);
    /// Container type for all return fields from the `swap` function with
    /// signature `swap(uint256,address,bytes)` and selector `0x1c6da724`
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
    pub struct SwapReturn(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `weth` function with
    /// signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
}

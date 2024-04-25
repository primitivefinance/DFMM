pub use log_normal::*;
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
pub mod log_normal {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dfmm"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("init"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserves"),
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
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("mean"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("width"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("controller"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tradingFunction"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tradingFunction"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserves"),
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
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("update"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
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
                    ::std::borrow::ToOwned::to_owned("validateAllocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateAllocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltas"),
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
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateDeallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltas"),
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
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("invariant"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenInIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tokenOutIndex"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DeltaError"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DeltaError"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("expected"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("actual"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidMean"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidMean"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidReservesLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidReservesLength",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWidth"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidWidth"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Min"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NegativeInfinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotDFMM"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotDFMM"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OutOfBounds"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0(r8\x03\x80b\0(r\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0FV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Rb\0\0xV[`\0` \x82\x84\x03\x12\x15b\0\0YW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0qW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa'\xD0b\0\0\xA2`\09`\0\x81\x81a\x02\x92\x01R\x81\x81a\x04\xA8\x01Ra\x08\xEA\x01Ra'\xD0`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x02YW\x80c\x8D\xDA\0=\x14a\x02lW\x80c\xAF\xBA\x13\xC4\x14a\x02\x8DW\x80c\xD8\xB5\xED\x12\x14a\x02\xCCW\x80c\xDC\x17\x83U\x14a\x02\xE1W`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x04W\x80cO\x17\xD9\x13\x14a\x01\xFCW\x80cu\xE6D\x0F\x14a\x02\x0FW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a \xADV[a\x02\xF4V[`@Qa\0\xC6\x94\x93\x92\x91\x90a!3V[`@Q\x80\x91\x03\x90\xF3[a\0\xF7`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a!\xDBV[a\x01\x98a\x01\x126`\x04a!\xEEV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x80\x82\x01\x84R\x82T\x82R`\x01\x83\x01T\x82\x86\x01R`\x02\x83\x01T\x82\x85\x01R`\x03\x83\x01T``\x80\x84\x01\x91\x90\x91R\x84Q\x91\x82\x01\x85R`\x04\x84\x01T\x82R`\x05\x84\x01T\x95\x82\x01\x95\x90\x95R`\x06\x83\x01T\x93\x81\x01\x93\x90\x93R`\x07\x82\x01T\x93\x83\x01\x93\x90\x93R`\x08\x81\x01T`\t\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R``\x96\x87\x01Q\x87\x83\x01R\x85Q`\x80\x83\x01R\x85\x01Q`\xA0\x82\x01R\x90\x84\x01Q`\xC0\x82\x01R\x93\x90\x92\x01Q`\xE0\x84\x01Ra\x01\0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x82\x01Ra\x01@\x01a\0\xC6V[a\0\xB6a\x02\n6`\x04a\"\x07V[a\x04\x97V[a\x02\"a\x02\x1D6`\x04a\"\xE6V[a\x06YV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x02g6`\x04a \xADV[a\x075V[a\x02\x7Fa\x02z6`\x04a#eV[a\x08\x82V[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02\xDFa\x02\xDA6`\x04a\"\x07V[a\x08\xDFV[\0[a\0\xF7a\x02\xEF6`\x04a!\xEEV[a\x0B\xF1V[`\0\x80``\x81\x80\x80\x80a\x03\t\x88\x8A\x01\x8Aa#\xD1V[\x92P\x92P\x92P\x80\x93Pa\x03%\x84\x8Ba\x03 \x8Ea\x0B\xF1V[a\r)V[\x94P\x84`\0\x81Q\x81\x10a\x03:Wa\x03:a#\xFDV[` \x02` \x01\x01Q\x83\x11\x15a\x03\x92W\x82\x85`\0\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[\x84`\x01\x81Q\x81\x10a\x03\xA5Wa\x03\xA5a#\xFDV[` \x02` \x01\x01Q\x82\x11\x15a\x03\xC8W\x81\x85`\x01\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x84`\0\x81Q\x81\x10a\x03\xDBWa\x03\xDBa#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x03\xFAWa\x03\xFAa#\xFDV[` \x02` \x01\x01\x81\x81Qa\x04\x0E\x91\x90a$)V[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x04&Wa\x04&a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x04EWa\x04Ea#\xFDV[` \x02` \x01\x01\x81\x81Qa\x04Y\x91\x90a$)V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x04\x7F\x91\x90a\x04v\x90\x87\x90a$)V[a\x02z\x8Ea\x0B\xF1V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE6W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x1A`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x05&\x86\x88\x01\x88a$<V[\x80Q\x92\x95P\x90\x93P\x91P`\x01\x11\x80a\x05EWP\x80Q`\x01`\x01`\xFF\x1B\x03\x10[\x15a\x05cW`@Qc5\x10;\xF5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81` \x01Q\x10\x80a\x05\x80WP`\x01`\x01`\xFF\x1B\x03\x81` \x01Q\x11[\x15a\x05\x9EW`@Qc\xA2<\x95E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xAB`@\x89\x01\x89a$\xD3V[\x90P`\x02\x14\x15\x80a\x05\xBEWP\x82Q`\x02\x14\x15[\x15a\x05\xDCW`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x8A\x81R` \x81\x81R`@\x91\x82\x90 \x92\x83U\x83\x01Q`\x04\x83\x01U\x82\x01Q`\x08\x82\x01U``\x82\x01Q`\t\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90Ua\x066\x83\x83a\x02z\x8Ca\x0B\xF1V[\x93P`\0\x84\x12\x15\x80\x15a\x06JWP`\x1E\x84\x13\x15[\x94PP\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x06n\x8Ba\x0B\xF1V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x06\x84\x91\x90a%\x1CV[\x92\x98P\x90\x96P\x94P\x92Pa\x06\x9C\x8A\x82\x88\x88\x88\x88a\r\xDAV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x06\xB5Wa\x06\xB5a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x06\xC9\x91\x90a%RV[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x06\xE6Wa\x06\xE6a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x06\xFA\x91\x90a$)V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x07\x1D\x91\x90a\x07\x17\x90\x85\x90a%RV[\x83a\x08\x82V[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x07J\x88\x8A\x01\x8Aa#\xD1V[\x92P\x92P\x92P\x80\x93Pa\x07f\x84\x8Ba\x07a\x8Ea\x0B\xF1V[a\x0E\xABV[\x94P\x82\x85`\0\x81Q\x81\x10a\x07|Wa\x07|a#\xFDV[` \x02` \x01\x01Q\x11\x15a\x07\x9EW\x82\x85`\0\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x81\x85`\x01\x81Q\x81\x10a\x07\xB2Wa\x07\xB2a#\xFDV[` \x02` \x01\x01Q\x11\x15a\x07\xD4W\x81\x85`\x01\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x84`\0\x81Q\x81\x10a\x07\xE7Wa\x07\xE7a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x08\x06Wa\x08\x06a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x08\x1A\x91\x90a%RV[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x082Wa\x082a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x08QWa\x08Qa#\xFDV[` \x02` \x01\x01\x81\x81Qa\x08e\x91\x90a%RV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x04\x7F\x91\x90a\x04v\x90\x87\x90a%RV[`\0a\x08\xD7\x84`\0\x81Q\x81\x10a\x08\x9AWa\x08\x9Aa#\xFDV[` \x02` \x01\x01Q\x85`\x01\x81Q\x81\x10a\x08\xB5Wa\x08\xB5a#\xFDV[` \x02` \x01\x01Q\x85\x85\x80` \x01\x90Q\x81\x01\x90a\x08\xD2\x91\x90a%eV[a\x0F6V[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t(W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\t\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\teW`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\ts\x82\x84\x01\x84a%\xBEV[\x90P`\x01\x81`\x04\x81\x11\x15a\t\x89Wa\t\x89a%\xDBV[\x03a\t\xE4Wa\t\xCD\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\x9A\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x08\x01Ua\x0B\xE9V[`\x02\x81`\x04\x81\x11\x15a\t\xF8Wa\t\xF8a%\xDBV[\x03a\n\x99W`\0\x80a\n?\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xB1\x92PPPV[\x91P\x91P`\x01\x82\x10\x80a\nXWP`\x01`\x01`\xFF\x1B\x03\x82\x11[\x15a\nvW`@Qc\xA2<\x95E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\n\x92\x90`\x04\x01\x83\x83a\x0F\xD3V[PPa\x0B\xE9V[`\x03\x81`\x04\x81\x11\x15a\n\xADWa\n\xADa%\xDBV[\x03a\x0BDW`\0\x80a\n\xF4\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xB1\x92PPPV[\x91P\x91P`\x01\x82\x10\x80a\x0B\rWP`\x01`\x01`\xFF\x1B\x03\x82\x11[\x15a\x0B+W`@Qc5\x10;\xF5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\n\x92\x90\x83\x83a\x0F\xD3V[`\x04\x81`\x04\x81\x11\x15a\x0BXWa\x0BXa%\xDBV[\x03a\x0B\xD0Wa\x0B\x9C\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10\x99\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\t\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0B\xE9V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\x0C'`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x0Cw`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x10\xB6V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x01T``\x82\x01Ra\x0C\xC4\x90a\x10\xB6V[\x81R`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 `\x08\x01T\x82\x84\x01\x90\x81R\x82Q\x84Q\x81\x84\x01R\x91\x84\x01Q\x92\x82\x01\x92\x90\x92R\x90Q``\x80\x83\x01\x91\x90\x91R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\r{\x84`@\x01Q`\0\x81Q\x81\x10a\rhWa\rha#\xFDV[` \x02` \x01\x01Q\x86\x86``\x01Qa\x11GV[\x81`\0\x81Q\x81\x10a\r\x8EWa\r\x8Ea#\xFDV[` \x02` \x01\x01\x81\x81RPPa\r\xB4\x84`@\x01Q`\x01\x81Q\x81\x10a\rhWa\rha#\xFDV[\x81`\x01\x81Q\x81\x10a\r\xC7Wa\r\xC7a#\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x94\x93PPPPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r\xF1\x91\x90a%eV[\x90P\x85`\0\x03a\x0EPWa\x0EH\x84\x89`@\x01Q`\0\x81Q\x81\x10a\x0E\x16Wa\x0E\x16a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x0E5Wa\x0E5a#\xFDV[` \x02` \x01\x01Q\x8B``\x01Q\x85a\x11TV[\x91PPa\x0E\xA1V[a\x0E\x9D\x84\x89`@\x01Q`\0\x81Q\x81\x10a\x0EkWa\x0Eka#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x0E\x8AWa\x0E\x8Aa#\xFDV[` \x02` \x01\x01Q\x8B``\x01Q\x85a\x11\xBDV[\x91PP[\x96\x95PPPPPPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0E\xFD\x84`@\x01Q`\0\x81Q\x81\x10a\x0E\xEAWa\x0E\xEAa#\xFDV[` \x02` \x01\x01Q\x86\x86``\x01Qa\x12\nV[\x81`\0\x81Q\x81\x10a\x0F\x10Wa\x0F\x10a#\xFDV[` \x02` \x01\x01\x81\x81RPPa\r\xB4\x84`@\x01Q`\x01\x81Q\x81\x10a\x0E\xEAWa\x0E\xEAa#\xFDV[`\0\x80a\x0FKa\x0FF\x87\x86a\x12\x17V[a\x125V[\x90P`\0a\x0Fsa\x0FFa\x0Fl\x86`\0\x01Q\x88a\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x12\x17V[` \x85\x01Q\x90\x91Pa\x0F\x85\x82\x84a%\xF1V[a\x0F\x8F\x91\x90a%\xF1V[\x97\x96PPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD7\x91\x90a&\x19V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0F\xC8\x91\x90a&GV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x0F\xF3W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xFC\x83a\x12\xF0V[`\0a\x10\x08B\x83a$)V[\x84T\x90\x91P`\0\x90a\x10\x1A\x90\x85a&~V[\x90P`\0a\x10(\x83\x83a&\xBBV[\x90P`\0a\x106\x84\x84a&\xE9V[\x90P`\0\x81\x13\x15a\x10`W\x80\x87`\0\x01`\0\x82\x82Ta\x10U\x91\x90a%RV[\x90\x91UPa\x10\x82\x90PV[a\x10i\x81a&\xFDV[\x87`\0\x01`\0\x82\x82Ta\x10|\x91\x90a$)V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x10\xAF\x91\x90a'\x19V[\x93\x92PPPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x10\xCCWPQ\x90V[`\0\x82` \x01QB\x11a\x10\xDFWBa\x10\xE5V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x10\xF9\x91\x90a$)V[\x90P`\0\x84`@\x01Q\x13\x15a\x11#W`@\x84\x01Qa\x11\x17\x90\x82a'SV[\x84Qa\x08\xD7\x91\x90a%RV[\x83`@\x01Qa\x111\x90a&\xFDV[a\x11;\x90\x82a'SV[\x84Qa\x08\xD7\x91\x90a$)V[`\0a\x08\xD7\x84\x84\x84a\x132V[`\0\x80a\x11n\x87\x84`@\x01Qa\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11}\x87\x86\x86a\x13QV[\x90Pa\x11\xB1\x86a\x11\x8D\x83\x8Aa\x13\xC1V[a\x11\x97\x91\x90a%RV[a\x11\xAB\x84a\x11\xA5\x85\x8Aa\x12\xDBV[\x90a\x12\xDBV[\x90a\x12\x17V[\x98\x97PPPPPPPPV[`\0\x80a\x11\xD7\x87\x84`@\x01Qa\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xE6\x87\x86\x86a\x13QV[\x90Pa\x11\xB1\x86a\x11\xF6\x83\x8Aa\x13\xC1V[a\x12\0\x91\x90a%RV[a\x11\xAB\x87\x85a\x12\xDBV[`\0a\x08\xD7\x84\x84\x84a\x13\xD6V[`\0a\x12,\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x132V[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x12NWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x12vW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x12\x97W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xA4\x83`\x02a'jV[\x90P`\0a\x12\xB1\x82a\x14\x04V[\x90P`\0a\x12\xC7g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x16\x82V[\x90Pa\x12\xD2\x81a&\xFDV[\x95\x94PPPPPV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xD6V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x13'\x90a\x10\xB6V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13JW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x13a\x83` \x01Qa\x16\x97V[\x90P`\0a\x13\x84a\x13r\x87\x87a\x12\x17V[a\x0FF\x90g\r\xE0\xB6\xB3\xA7d\0\0a$)V[\x90P`\0a\x13\xB2\x83a\x13\xA3\x87` \x01Q\x85a\x16\xB5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x13\xAD\x91\x90a&~V[a\x16\xE8V[\x85Q\x90\x91Pa\x0F\x8F\x90\x82a\x12\xDBV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x132V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xEEW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x12\x80a\x14\x1BWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x149W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14ZW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x14\x82W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x14\x8DW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x14\xB5Wa\x14\xB0\x83g\x1B\xC1mgN\xC8\0\0a&~V[a\x14\xB7V[\x82[\x90P`\0a\x14\xCD\x82g\x1B\xC1mgN\xC8\0\0a\x18\x91V[\x90P\x80`\0\x03a\x14\xF0W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xFB\x82a\x18\xA6V[\x90P`\0c;\x9A\xCA\0a\x15&a\x15!a\x15\x1Bg\x1B\xC1mgN\xC8\0\0a&\xFDV[\x85a\x16\x82V[a\x1A\x81V[a\x150\x91\x90a'jV[\x90P`\0\x80a\x15G\x83g\x03\xC1f\\z\xAB \0a\x16\x82V[a\x15Y\x90g \x05\xFEO&\x8E\xA0\0a%\xF1V[\x90P`\0a\x15\x89\x84a\x15r\x86f\x9F2u$b\xA0\0a\x16\x82V[a\x15\x84\x90g\r\xC5R\x7Fd, \0a%\xF1V[a\x16\x82V[a\x15\x9B\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\xF1V[\x90Pa\x15\xBFg\t\xD0(\xCCo _\xFF\x19\x85a\x15\xB5\x85\x85a\x18\x91V[a\x15\x84\x91\x90a&~V[\x92PPP`\0[`\x02\x81\x10\x15a\x16ZW`\0\x86a\x15\xDB\x84a\x1B%V[a\x15\xE5\x91\x90a&~V[\x90P`\0a\x15\xF3\x84\x85a\x16\x82V[a\x15\xFC\x90a&\xFDV[\x90P`\0a\x16\t\x82a\x16\xE8V[\x90P`\0a\x16\x17\x86\x85a\x16\x82V[a\x16)g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x16\x82V[a\x163\x91\x90a&~V[\x90Pa\x16?\x84\x82a\x18\x91V[a\x16I\x90\x87a%\xF1V[\x95P\x84`\x01\x01\x94PPPPPa\x15\xC6V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x16wWa\x16r\x82a&\xFDV[a\x11\xB1V[P\x96\x95PPPPPPV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\tV[`\0a\x12/a\x16\xA6\x83\x80a\x12\xDBV[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x13\xC1V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x16\xD7W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x17\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x17JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\x89V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x12,\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1D\tV[`\0\x80\x82\x13a\x18\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x89V[`\0``a\x18\xF0\x84a\x1D(V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1A\x9AW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1A\xB6W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1A\xCEW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1A\xE4W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x81`\0\x03a\x1B>WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1BUWP`\0\x91\x90PV[a\x1BfgV\x98\xEE\xF0fp\0\0a&\xFDV[\x82\x13a\x1B{WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1B\x86\x83a\x1D\xCFV[\x90P`\0a\x1B\xBFg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xA8\x84g\x1B\xC1mgN\xC8\0\0a\x12\x17V[a\x1B\xBA\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\xF1V[a\x18\x91V[\x90P`\0\x80\x82a\x1C\x1B\x81a\x1C\x08\x81a\x1B\xF6\x81a\x1B\xE3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x16\x82V[a\x15\x84\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a%\xF1V[a\x15\x84\x90g\x14\xA8EL\x19\xE1\xAC\0a%\xF1V[a\x15\x84\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a%\xF1V[a\x1C-\x90g\x03\xDE\xBD\x08;\x8C|\0a%\xF1V[\x91P\x83\x90Pa\x1C\x95\x81a\x1C\x83\x81a\x1Cq\x81a\x1C_\x81a\x1CL\x81\x8Ba\x16\x82V[a\x15\x84\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a%\xF1V[a\x15\x84\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a%\xF1V[a\x15\x84\x90g\x051\n\xA7\xD5!0\0a%\xF1V[a\x15\x84\x90g\r\xE0\xCC=\x15a\0\0a%\xF1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1C\xAB\x87\x88a\x16\x82V[a\x1C\xB7\x90`\0\x19a'jV[a\x1C\xC1\x91\x90a&~V[a\x1C\xCB\x91\x90a%\xF1V[\x92PP`\0a\x1C\xD9\x83a\x16\xE8V[\x90P`\0a\x1C\xE7\x85\x83a\x16\x82V[\x90P`\0\x88\x12a\x1C\xF7W\x80a\x11\xB1V[a\x11\xB1\x81g\x1B\xC1mgN\xC8\0\0a&~V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1D!W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1DeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x89V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1D\xF5W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1E\x06WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E#W`\0\x80\xFD[PV[\x805a\x1E\t\x81a\x1E\x0EV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1EiWa\x1Eia\x1E1V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1EiWa\x1Eia\x1E1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xB9Wa\x1E\xB9a\x1E1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\xDAWa\x1E\xDAa\x1E1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\xF5W`\0\x80\xFD[\x815` a\x1F\na\x1F\x05\x83a\x1E\xC1V[a\x1E\x91V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1F,W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16wW\x805a\x1FD\x81a\x1E\x0EV[\x83R\x91\x83\x01\x91\x83\x01a\x1F1V[`\0\x82`\x1F\x83\x01\x12a\x1FbW`\0\x80\xFD[\x815` a\x1Fra\x1F\x05\x83a\x1E\xC1V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1F\x94W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16wW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1F\x99V[`\0`\xE0\x82\x84\x03\x12\x15a\x1F\xC2W`\0\x80\xFD[a\x1F\xCAa\x1EGV[\x90Pa\x1F\xD5\x82a\x1E&V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xF1W`\0\x80\xFD[a\x1F\xFD\x85\x83\x86\x01a\x1E\xE4V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a \x16W`\0\x80\xFD[Pa #\x84\x82\x85\x01a\x1FQV[`@\x83\x01RP``\x82\x015``\x82\x01Ra ?`\x80\x83\x01a\x1E&V[`\x80\x82\x01Ra P`\xA0\x83\x01a\x1E&V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a wW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \x8EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \xA6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a \xC5W`\0\x80\xFD[\x855a \xD0\x81a\x1E\x0EV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xF3W`\0\x80\xFD[a \xFF\x89\x83\x8A\x01a\x1F\xB0V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a!\x15W`\0\x80\xFD[Pa!\"\x88\x82\x89\x01a eV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a!~W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a!bV[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a!\xBBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a!\x9FV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12,` \x83\x01\x84a!\x95V[`\0` \x82\x84\x03\x12\x15a\"\0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\x1FW`\0\x80\xFD[\x855a\"*\x81a\x1E\x0EV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"MW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\"aW`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a!\x15W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\"\x88W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA1Wa\"\xA1a\x1E1V[a\"\xB4`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xC9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\"\xFCW`\0\x80\xFD[\x845a#\x07\x81a\x1E\x0EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#*W`\0\x80\xFD[a#6\x88\x83\x89\x01a\x1F\xB0V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a#LW`\0\x80\xFD[Pa#Y\x87\x82\x88\x01a\"wV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#zW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\x91W`\0\x80\xFD[a#\x9D\x87\x83\x88\x01a\x1FQV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a#\xBAW`\0\x80\xFD[Pa#\xC7\x86\x82\x87\x01a\"wV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xE6W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12/Wa\x12/a$\x13V[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a$RW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a$hW`\0\x80\xFD[a$t\x87\x82\x88\x01a\x1FQV[\x94PP` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a$\x90W`\0\x80\xFD[Pa$\x99a\x1EoV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a$\xC2\x81a\x1E\x0EV[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\xEAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a%\x04W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a \xA6W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%2W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x12/Wa\x12/a$\x13V[`\0`\x80\x82\x84\x03\x12\x15a%wW`\0\x80\xFD[a%\x7Fa\x1EoV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa%\xA5\x81a\x1E\x0EV[``\x82\x01R\x93\x92PPPV[`\x05\x81\x10a\x1E#W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a%\xD0W`\0\x80\xFD[\x815a\x10\xAF\x81a%\xB1V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&\x11Wa&\x11a$\x13V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a&,W`\0\x80\xFD[\x82Qa&7\x81a%\xB1V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\\W`\0\x80\xFD[\x83Qa&g\x81a%\xB1V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\x9EWa&\x9Ea$\x13V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&\xCAWa&\xCAa&\xA5V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\xE4Wa&\xE4a$\x13V[P\x05\x90V[`\0\x82a&\xF8Wa&\xF8a&\xA5V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a'\x12Wa'\x12a$\x13V[P`\0\x03\x90V[`\0\x80`@\x83\x85\x03\x12\x15a',W`\0\x80\xFD[\x82Qa'7\x81a%\xB1V[` \x84\x01Q\x90\x92Pa'H\x81a\x1E\x0EV[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12/Wa\x12/a$\x13V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a'\x86Wa'\x86a$\x13V[\x81\x81\x05\x83\x14\x82\x15\x17a\x12/Wa\x12/a$\x13V\xFE\xA2dipfsX\"\x12 }\xE8U\xB8x\xAAzlD\xF7\xD0&\xDF\xE9^5\x16\xFB)K+\x9D\xC3\xE9*\x9C\xE5\xA0\x90\x91S\xB9dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x02YW\x80c\x8D\xDA\0=\x14a\x02lW\x80c\xAF\xBA\x13\xC4\x14a\x02\x8DW\x80c\xD8\xB5\xED\x12\x14a\x02\xCCW\x80c\xDC\x17\x83U\x14a\x02\xE1W`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x04W\x80cO\x17\xD9\x13\x14a\x01\xFCW\x80cu\xE6D\x0F\x14a\x02\x0FW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a \xADV[a\x02\xF4V[`@Qa\0\xC6\x94\x93\x92\x91\x90a!3V[`@Q\x80\x91\x03\x90\xF3[a\0\xF7`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a!\xDBV[a\x01\x98a\x01\x126`\x04a!\xEEV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x80\x82\x01\x84R\x82T\x82R`\x01\x83\x01T\x82\x86\x01R`\x02\x83\x01T\x82\x85\x01R`\x03\x83\x01T``\x80\x84\x01\x91\x90\x91R\x84Q\x91\x82\x01\x85R`\x04\x84\x01T\x82R`\x05\x84\x01T\x95\x82\x01\x95\x90\x95R`\x06\x83\x01T\x93\x81\x01\x93\x90\x93R`\x07\x82\x01T\x93\x83\x01\x93\x90\x93R`\x08\x81\x01T`\t\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84V[`@\x80Q\x85Q\x81R` \x80\x87\x01Q\x81\x83\x01R\x86\x83\x01Q\x82\x84\x01R``\x96\x87\x01Q\x87\x83\x01R\x85Q`\x80\x83\x01R\x85\x01Q`\xA0\x82\x01R\x90\x84\x01Q`\xC0\x82\x01R\x93\x90\x92\x01Q`\xE0\x84\x01Ra\x01\0\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x82\x01Ra\x01@\x01a\0\xC6V[a\0\xB6a\x02\n6`\x04a\"\x07V[a\x04\x97V[a\x02\"a\x02\x1D6`\x04a\"\xE6V[a\x06YV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x02g6`\x04a \xADV[a\x075V[a\x02\x7Fa\x02z6`\x04a#eV[a\x08\x82V[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\xB4\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02\xDFa\x02\xDA6`\x04a\"\x07V[a\x08\xDFV[\0[a\0\xF7a\x02\xEF6`\x04a!\xEEV[a\x0B\xF1V[`\0\x80``\x81\x80\x80\x80a\x03\t\x88\x8A\x01\x8Aa#\xD1V[\x92P\x92P\x92P\x80\x93Pa\x03%\x84\x8Ba\x03 \x8Ea\x0B\xF1V[a\r)V[\x94P\x84`\0\x81Q\x81\x10a\x03:Wa\x03:a#\xFDV[` \x02` \x01\x01Q\x83\x11\x15a\x03\x92W\x82\x85`\0\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03\x89\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[\x84`\x01\x81Q\x81\x10a\x03\xA5Wa\x03\xA5a#\xFDV[` \x02` \x01\x01Q\x82\x11\x15a\x03\xC8W\x81\x85`\x01\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x84`\0\x81Q\x81\x10a\x03\xDBWa\x03\xDBa#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x03\xFAWa\x03\xFAa#\xFDV[` \x02` \x01\x01\x81\x81Qa\x04\x0E\x91\x90a$)V[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x04&Wa\x04&a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x04EWa\x04Ea#\xFDV[` \x02` \x01\x01\x81\x81Qa\x04Y\x91\x90a$)V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x04\x7F\x91\x90a\x04v\x90\x87\x90a$)V[a\x02z\x8Ea\x0B\xF1V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\xE6W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x1A`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x05&\x86\x88\x01\x88a$<V[\x80Q\x92\x95P\x90\x93P\x91P`\x01\x11\x80a\x05EWP\x80Q`\x01`\x01`\xFF\x1B\x03\x10[\x15a\x05cW`@Qc5\x10;\xF5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01\x81` \x01Q\x10\x80a\x05\x80WP`\x01`\x01`\xFF\x1B\x03\x81` \x01Q\x11[\x15a\x05\x9EW`@Qc\xA2<\x95E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\xAB`@\x89\x01\x89a$\xD3V[\x90P`\x02\x14\x15\x80a\x05\xBEWP\x82Q`\x02\x14\x15[\x15a\x05\xDCW`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x8A\x81R` \x81\x81R`@\x91\x82\x90 \x92\x83U\x83\x01Q`\x04\x83\x01U\x82\x01Q`\x08\x82\x01U``\x82\x01Q`\t\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90Ua\x066\x83\x83a\x02z\x8Ca\x0B\xF1V[\x93P`\0\x84\x12\x15\x80\x15a\x06JWP`\x1E\x84\x13\x15[\x94PP\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x06n\x8Ba\x0B\xF1V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x06\x84\x91\x90a%\x1CV[\x92\x98P\x90\x96P\x94P\x92Pa\x06\x9C\x8A\x82\x88\x88\x88\x88a\r\xDAV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x06\xB5Wa\x06\xB5a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x06\xC9\x91\x90a%RV[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x06\xE6Wa\x06\xE6a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x06\xFA\x91\x90a$)V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x07\x1D\x91\x90a\x07\x17\x90\x85\x90a%RV[\x83a\x08\x82V[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x07J\x88\x8A\x01\x8Aa#\xD1V[\x92P\x92P\x92P\x80\x93Pa\x07f\x84\x8Ba\x07a\x8Ea\x0B\xF1V[a\x0E\xABV[\x94P\x82\x85`\0\x81Q\x81\x10a\x07|Wa\x07|a#\xFDV[` \x02` \x01\x01Q\x11\x15a\x07\x9EW\x82\x85`\0\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x81\x85`\x01\x81Q\x81\x10a\x07\xB2Wa\x07\xB2a#\xFDV[` \x02` \x01\x01Q\x11\x15a\x07\xD4W\x81\x85`\x01\x81Q\x81\x10a\x03]Wa\x03]a#\xFDV[\x84`\0\x81Q\x81\x10a\x07\xE7Wa\x07\xE7a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x08\x06Wa\x08\x06a#\xFDV[` \x02` \x01\x01\x81\x81Qa\x08\x1A\x91\x90a%RV[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x082Wa\x082a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x08QWa\x08Qa#\xFDV[` \x02` \x01\x01\x81\x81Qa\x08e\x91\x90a%RV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x04\x7F\x91\x90a\x04v\x90\x87\x90a%RV[`\0a\x08\xD7\x84`\0\x81Q\x81\x10a\x08\x9AWa\x08\x9Aa#\xFDV[` \x02` \x01\x01Q\x85`\x01\x81Q\x81\x10a\x08\xB5Wa\x08\xB5a#\xFDV[` \x02` \x01\x01Q\x85\x85\x80` \x01\x90Q\x81\x01\x90a\x08\xD2\x91\x90a%eV[a\x0F6V[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t(W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\t\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\teW`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\ts\x82\x84\x01\x84a%\xBEV[\x90P`\x01\x81`\x04\x81\x11\x15a\t\x89Wa\t\x89a%\xDBV[\x03a\t\xE4Wa\t\xCD\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\x9A\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x08\x01Ua\x0B\xE9V[`\x02\x81`\x04\x81\x11\x15a\t\xF8Wa\t\xF8a%\xDBV[\x03a\n\x99W`\0\x80a\n?\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xB1\x92PPPV[\x91P\x91P`\x01\x82\x10\x80a\nXWP`\x01`\x01`\xFF\x1B\x03\x82\x11[\x15a\nvW`@Qc\xA2<\x95E`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\n\x92\x90`\x04\x01\x83\x83a\x0F\xD3V[PPa\x0B\xE9V[`\x03\x81`\x04\x81\x11\x15a\n\xADWa\n\xADa%\xDBV[\x03a\x0BDW`\0\x80a\n\xF4\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0F\xB1\x92PPPV[\x91P\x91P`\x01\x82\x10\x80a\x0B\rWP`\x01`\x01`\xFF\x1B\x03\x82\x11[\x15a\x0B+W`@Qc5\x10;\xF5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\n\x92\x90\x83\x83a\x0F\xD3V[`\x04\x81`\x04\x81\x11\x15a\x0BXWa\x0BXa%\xDBV[\x03a\x0B\xD0Wa\x0B\x9C\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10\x99\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\t\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x0B\xE9V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\x0C'`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x0Cw`\0\x80\x85\x81R` \x01\x90\x81R` \x01`\0 `\x04\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x10\xB6V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x01T``\x82\x01Ra\x0C\xC4\x90a\x10\xB6V[\x81R`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 `\x08\x01T\x82\x84\x01\x90\x81R\x82Q\x84Q\x81\x84\x01R\x91\x84\x01Q\x92\x82\x01\x92\x90\x92R\x90Q``\x80\x83\x01\x91\x90\x91R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x80\x82\x01R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\r{\x84`@\x01Q`\0\x81Q\x81\x10a\rhWa\rha#\xFDV[` \x02` \x01\x01Q\x86\x86``\x01Qa\x11GV[\x81`\0\x81Q\x81\x10a\r\x8EWa\r\x8Ea#\xFDV[` \x02` \x01\x01\x81\x81RPPa\r\xB4\x84`@\x01Q`\x01\x81Q\x81\x10a\rhWa\rha#\xFDV[\x81`\x01\x81Q\x81\x10a\r\xC7Wa\r\xC7a#\xFDV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x94\x93PPPPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r\xF1\x91\x90a%eV[\x90P\x85`\0\x03a\x0EPWa\x0EH\x84\x89`@\x01Q`\0\x81Q\x81\x10a\x0E\x16Wa\x0E\x16a#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x0E5Wa\x0E5a#\xFDV[` \x02` \x01\x01Q\x8B``\x01Q\x85a\x11TV[\x91PPa\x0E\xA1V[a\x0E\x9D\x84\x89`@\x01Q`\0\x81Q\x81\x10a\x0EkWa\x0Eka#\xFDV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x0E\x8AWa\x0E\x8Aa#\xFDV[` \x02` \x01\x01Q\x8B``\x01Q\x85a\x11\xBDV[\x91PP[\x96\x95PPPPPPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0E\xFD\x84`@\x01Q`\0\x81Q\x81\x10a\x0E\xEAWa\x0E\xEAa#\xFDV[` \x02` \x01\x01Q\x86\x86``\x01Qa\x12\nV[\x81`\0\x81Q\x81\x10a\x0F\x10Wa\x0F\x10a#\xFDV[` \x02` \x01\x01\x81\x81RPPa\r\xB4\x84`@\x01Q`\x01\x81Q\x81\x10a\x0E\xEAWa\x0E\xEAa#\xFDV[`\0\x80a\x0FKa\x0FF\x87\x86a\x12\x17V[a\x125V[\x90P`\0a\x0Fsa\x0FFa\x0Fl\x86`\0\x01Q\x88a\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x12\x17V[` \x85\x01Q\x90\x91Pa\x0F\x85\x82\x84a%\xF1V[a\x0F\x8F\x91\x90a%\xF1V[\x97\x96PPPPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD7\x91\x90a&\x19V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x0F\xC8\x91\x90a&GV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x0F\xF3W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0F\xFC\x83a\x12\xF0V[`\0a\x10\x08B\x83a$)V[\x84T\x90\x91P`\0\x90a\x10\x1A\x90\x85a&~V[\x90P`\0a\x10(\x83\x83a&\xBBV[\x90P`\0a\x106\x84\x84a&\xE9V[\x90P`\0\x81\x13\x15a\x10`W\x80\x87`\0\x01`\0\x82\x82Ta\x10U\x91\x90a%RV[\x90\x91UPa\x10\x82\x90PV[a\x10i\x81a&\xFDV[\x87`\0\x01`\0\x82\x82Ta\x10|\x91\x90a$)V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x10\xAF\x91\x90a'\x19V[\x93\x92PPPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x10\xCCWPQ\x90V[`\0\x82` \x01QB\x11a\x10\xDFWBa\x10\xE5V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x10\xF9\x91\x90a$)V[\x90P`\0\x84`@\x01Q\x13\x15a\x11#W`@\x84\x01Qa\x11\x17\x90\x82a'SV[\x84Qa\x08\xD7\x91\x90a%RV[\x83`@\x01Qa\x111\x90a&\xFDV[a\x11;\x90\x82a'SV[\x84Qa\x08\xD7\x91\x90a$)V[`\0a\x08\xD7\x84\x84\x84a\x132V[`\0\x80a\x11n\x87\x84`@\x01Qa\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11}\x87\x86\x86a\x13QV[\x90Pa\x11\xB1\x86a\x11\x8D\x83\x8Aa\x13\xC1V[a\x11\x97\x91\x90a%RV[a\x11\xAB\x84a\x11\xA5\x85\x8Aa\x12\xDBV[\x90a\x12\xDBV[\x90a\x12\x17V[\x98\x97PPPPPPPPV[`\0\x80a\x11\xD7\x87\x84`@\x01Qa\x12\xDB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\xE6\x87\x86\x86a\x13QV[\x90Pa\x11\xB1\x86a\x11\xF6\x83\x8Aa\x13\xC1V[a\x12\0\x91\x90a%RV[a\x11\xAB\x87\x85a\x12\xDBV[`\0a\x08\xD7\x84\x84\x84a\x13\xD6V[`\0a\x12,\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x132V[\x90P[\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x12NWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x12vW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x12\x97W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x12\xA4\x83`\x02a'jV[\x90P`\0a\x12\xB1\x82a\x14\x04V[\x90P`\0a\x12\xC7g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x16\x82V[\x90Pa\x12\xD2\x81a&\xFDV[\x95\x94PPPPPV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xD6V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x13'\x90a\x10\xB6V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13JW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x13a\x83` \x01Qa\x16\x97V[\x90P`\0a\x13\x84a\x13r\x87\x87a\x12\x17V[a\x0FF\x90g\r\xE0\xB6\xB3\xA7d\0\0a$)V[\x90P`\0a\x13\xB2\x83a\x13\xA3\x87` \x01Q\x85a\x16\xB5\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x13\xAD\x91\x90a&~V[a\x16\xE8V[\x85Q\x90\x91Pa\x0F\x8F\x90\x82a\x12\xDBV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x132V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13\xEEW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x12\x80a\x14\x1BWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x149W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14ZW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x14\x82W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x14\x8DW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x14\xB5Wa\x14\xB0\x83g\x1B\xC1mgN\xC8\0\0a&~V[a\x14\xB7V[\x82[\x90P`\0a\x14\xCD\x82g\x1B\xC1mgN\xC8\0\0a\x18\x91V[\x90P\x80`\0\x03a\x14\xF0W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x14\xFB\x82a\x18\xA6V[\x90P`\0c;\x9A\xCA\0a\x15&a\x15!a\x15\x1Bg\x1B\xC1mgN\xC8\0\0a&\xFDV[\x85a\x16\x82V[a\x1A\x81V[a\x150\x91\x90a'jV[\x90P`\0\x80a\x15G\x83g\x03\xC1f\\z\xAB \0a\x16\x82V[a\x15Y\x90g \x05\xFEO&\x8E\xA0\0a%\xF1V[\x90P`\0a\x15\x89\x84a\x15r\x86f\x9F2u$b\xA0\0a\x16\x82V[a\x15\x84\x90g\r\xC5R\x7Fd, \0a%\xF1V[a\x16\x82V[a\x15\x9B\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\xF1V[\x90Pa\x15\xBFg\t\xD0(\xCCo _\xFF\x19\x85a\x15\xB5\x85\x85a\x18\x91V[a\x15\x84\x91\x90a&~V[\x92PPP`\0[`\x02\x81\x10\x15a\x16ZW`\0\x86a\x15\xDB\x84a\x1B%V[a\x15\xE5\x91\x90a&~V[\x90P`\0a\x15\xF3\x84\x85a\x16\x82V[a\x15\xFC\x90a&\xFDV[\x90P`\0a\x16\t\x82a\x16\xE8V[\x90P`\0a\x16\x17\x86\x85a\x16\x82V[a\x16)g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x16\x82V[a\x163\x91\x90a&~V[\x90Pa\x16?\x84\x82a\x18\x91V[a\x16I\x90\x87a%\xF1V[\x95P\x84`\x01\x01\x94PPPPPa\x15\xC6V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x16wWa\x16r\x82a&\xFDV[a\x11\xB1V[P\x96\x95PPPPPPV[`\0a\x12,\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1D\tV[`\0a\x12/a\x16\xA6\x83\x80a\x12\xDBV[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x13\xC1V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x16\xD7W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x17\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x17JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\x89V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x12,\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1D\tV[`\0\x80\x82\x13a\x18\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x89V[`\0``a\x18\xF0\x84a\x1D(V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1A\x9AW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1A\xB6W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1A\xCEW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1A\xE4W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0\x81`\0\x03a\x1B>WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1BUWP`\0\x91\x90PV[a\x1BfgV\x98\xEE\xF0fp\0\0a&\xFDV[\x82\x13a\x1B{WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1B\x86\x83a\x1D\xCFV[\x90P`\0a\x1B\xBFg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xA8\x84g\x1B\xC1mgN\xC8\0\0a\x12\x17V[a\x1B\xBA\x90g\r\xE0\xB6\xB3\xA7d\0\0a%\xF1V[a\x18\x91V[\x90P`\0\x80\x82a\x1C\x1B\x81a\x1C\x08\x81a\x1B\xF6\x81a\x1B\xE3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x16\x82V[a\x15\x84\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a%\xF1V[a\x15\x84\x90g\x14\xA8EL\x19\xE1\xAC\0a%\xF1V[a\x15\x84\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a%\xF1V[a\x1C-\x90g\x03\xDE\xBD\x08;\x8C|\0a%\xF1V[\x91P\x83\x90Pa\x1C\x95\x81a\x1C\x83\x81a\x1Cq\x81a\x1C_\x81a\x1CL\x81\x8Ba\x16\x82V[a\x15\x84\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a%\xF1V[a\x15\x84\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a%\xF1V[a\x15\x84\x90g\x051\n\xA7\xD5!0\0a%\xF1V[a\x15\x84\x90g\r\xE0\xCC=\x15a\0\0a%\xF1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1C\xAB\x87\x88a\x16\x82V[a\x1C\xB7\x90`\0\x19a'jV[a\x1C\xC1\x91\x90a&~V[a\x1C\xCB\x91\x90a%\xF1V[\x92PP`\0a\x1C\xD9\x83a\x16\xE8V[\x90P`\0a\x1C\xE7\x85\x83a\x16\x82V[\x90P`\0\x88\x12a\x1C\xF7W\x80a\x11\xB1V[a\x11\xB1\x81g\x1B\xC1mgN\xC8\0\0a&~V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1D!W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x1DeW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\x89V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1D\xF5W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1E\x06WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1E#W`\0\x80\xFD[PV[\x805a\x1E\t\x81a\x1E\x0EV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1EiWa\x1Eia\x1E1V[`@R\x90V[`@Q`\x80\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1EiWa\x1Eia\x1E1V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x1E\xB9Wa\x1E\xB9a\x1E1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1E\xDAWa\x1E\xDAa\x1E1V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x1E\xF5W`\0\x80\xFD[\x815` a\x1F\na\x1F\x05\x83a\x1E\xC1V[a\x1E\x91V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1F,W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16wW\x805a\x1FD\x81a\x1E\x0EV[\x83R\x91\x83\x01\x91\x83\x01a\x1F1V[`\0\x82`\x1F\x83\x01\x12a\x1FbW`\0\x80\xFD[\x815` a\x1Fra\x1F\x05\x83a\x1E\xC1V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1F\x94W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16wW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1F\x99V[`\0`\xE0\x82\x84\x03\x12\x15a\x1F\xC2W`\0\x80\xFD[a\x1F\xCAa\x1EGV[\x90Pa\x1F\xD5\x82a\x1E&V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\xF1W`\0\x80\xFD[a\x1F\xFD\x85\x83\x86\x01a\x1E\xE4V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a \x16W`\0\x80\xFD[Pa #\x84\x82\x85\x01a\x1FQV[`@\x83\x01RP``\x82\x015``\x82\x01Ra ?`\x80\x83\x01a\x1E&V[`\x80\x82\x01Ra P`\xA0\x83\x01a\x1E&V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a wW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a \x8EW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a \xA6W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a \xC5W`\0\x80\xFD[\x855a \xD0\x81a\x1E\x0EV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a \xF3W`\0\x80\xFD[a \xFF\x89\x83\x8A\x01a\x1F\xB0V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a!\x15W`\0\x80\xFD[Pa!\"\x88\x82\x89\x01a eV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a!~W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a!bV[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a!\xBBW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a!\x9FV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12,` \x83\x01\x84a!\x95V[`\0` \x82\x84\x03\x12\x15a\"\0W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\"\x1FW`\0\x80\xFD[\x855a\"*\x81a\x1E\x0EV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\"MW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\"aW`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a!\x15W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\"\x88W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\"\xA1Wa\"\xA1a\x1E1V[a\"\xB4`\x1F\x82\x01`\x1F\x19\x16` \x01a\x1E\x91V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\"\xC9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\"\xFCW`\0\x80\xFD[\x845a#\x07\x81a\x1E\x0EV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#*W`\0\x80\xFD[a#6\x88\x83\x89\x01a\x1F\xB0V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a#LW`\0\x80\xFD[Pa#Y\x87\x82\x88\x01a\"wV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#zW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a#\x91W`\0\x80\xFD[a#\x9D\x87\x83\x88\x01a\x1FQV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a#\xBAW`\0\x80\xFD[Pa#\xC7\x86\x82\x87\x01a\"wV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xE6W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12/Wa\x12/a$\x13V[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a$RW`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x81\x11\x15a$hW`\0\x80\xFD[a$t\x87\x82\x88\x01a\x1FQV[\x94PP` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a$\x90W`\0\x80\xFD[Pa$\x99a\x1EoV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a$\xC2\x81a\x1E\x0EV[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a$\xEAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a%\x04W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a \xA6W`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a%2W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x12/Wa\x12/a$\x13V[`\0`\x80\x82\x84\x03\x12\x15a%wW`\0\x80\xFD[a%\x7Fa\x1EoV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa%\xA5\x81a\x1E\x0EV[``\x82\x01R\x93\x92PPPV[`\x05\x81\x10a\x1E#W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a%\xD0W`\0\x80\xFD[\x815a\x10\xAF\x81a%\xB1V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&\x11Wa&\x11a$\x13V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a&,W`\0\x80\xFD[\x82Qa&7\x81a%\xB1V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a&\\W`\0\x80\xFD[\x83Qa&g\x81a%\xB1V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a&\x9EWa&\x9Ea$\x13V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a&\xCAWa&\xCAa&\xA5V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\xE4Wa&\xE4a$\x13V[P\x05\x90V[`\0\x82a&\xF8Wa&\xF8a&\xA5V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a'\x12Wa'\x12a$\x13V[P`\0\x03\x90V[`\0\x80`@\x83\x85\x03\x12\x15a',W`\0\x80\xFD[\x82Qa'7\x81a%\xB1V[` \x84\x01Q\x90\x92Pa'H\x81a\x1E\x0EV[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12/Wa\x12/a$\x13V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a'\x86Wa'\x86a$\x13V[\x81\x81\x05\x83\x14\x82\x15\x17a\x12/Wa\x12/a$\x13V\xFE\xA2dipfsX\"\x12 }\xE8U\xB8x\xAAzlD\xF7\xD0&\xDF\xE9^5\x16\xFB)K+\x9D\xC3\xE9*\x9C\xE5\xA0\x90\x91S\xB9dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMAL_ABI.clone(),
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
                LOGNORMAL_ABI.clone(),
                LOGNORMAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `dfmm` (0xafba13c4) function
        pub fn dfmm(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `init` (0x4f17d913) function
        pub fn init(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            pool: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([79, 23, 217, 19], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                DynamicParam,
                DynamicParam,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `tradingFunction` (0x8dda003d) function
        pub fn trading_function(
            &self,
            reserves: ::std::vec::Vec<::ethers::core::types::U256>,
            total_liquidity: ::ethers::core::types::U256,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([141, 218, 0, 61], (reserves, total_liquidity, params))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `update` (0xd8b5ed12) function
        pub fn update(
            &self,
            sender: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            p2: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 181, 237, 18], (sender, pool_id, p2, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateAllocate` (0x7c101244) function
        pub fn validate_allocate(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            pool: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([124, 16, 18, 68], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateDeallocate` (0x040d951e) function
        pub fn validate_deallocate(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            pool: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([4, 13, 149, 30], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateSwap` (0x75e6440f) function
        pub fn validate_swap(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            pool: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([117, 230, 68, 15], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LogNormal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `DeltaError` with signature
    /// `DeltaError(uint256,uint256)` and selector `0x6d685fa7`
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
    #[etherror(name = "DeltaError", abi = "DeltaError(uint256,uint256)")]
    pub struct DeltaError {
        pub expected: ::ethers::core::types::U256,
        pub actual: ::ethers::core::types::U256,
    }
    /// Custom Error type `Infinity` with signature `Infinity()` and selector
    /// `0x07a02127`
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
    #[etherror(name = "Infinity", abi = "Infinity()")]
    pub struct Infinity;
    /// Custom Error type `InvalidMean` with signature `InvalidMean()` and
    /// selector `0xd440efd4`
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
    #[etherror(name = "InvalidMean", abi = "InvalidMean()")]
    pub struct InvalidMean;
    /// Custom Error type `InvalidReservesLength` with signature
    /// `InvalidReservesLength()` and selector `0xc7f63e5e`
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
    #[etherror(name = "InvalidReservesLength", abi = "InvalidReservesLength()")]
    pub struct InvalidReservesLength;
    /// Custom Error type `InvalidSender` with signature `InvalidSender()` and
    /// selector `0xddb5de5e`
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
    #[etherror(name = "InvalidSender", abi = "InvalidSender()")]
    pub struct InvalidSender;
    /// Custom Error type `InvalidUpdateCode` with signature
    /// `InvalidUpdateCode()` and selector `0x235d2b3d`
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
    #[etherror(name = "InvalidUpdateCode", abi = "InvalidUpdateCode()")]
    pub struct InvalidUpdateCode;
    /// Custom Error type `InvalidUpdateEnd` with signature `InvalidUpdateEnd()`
    /// and selector `0xcde205da`
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
    #[etherror(name = "InvalidUpdateEnd", abi = "InvalidUpdateEnd()")]
    pub struct InvalidUpdateEnd;
    /// Custom Error type `InvalidWidth` with signature `InvalidWidth()` and
    /// selector `0xa23c9545`
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
    #[etherror(name = "InvalidWidth", abi = "InvalidWidth()")]
    pub struct InvalidWidth;
    /// Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
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
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    /// Custom Error type `NegativeInfinity` with signature `NegativeInfinity()`
    /// and selector `0x8bb56614`
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
    #[etherror(name = "NegativeInfinity", abi = "NegativeInfinity()")]
    pub struct NegativeInfinity;
    /// Custom Error type `NotDFMM` with signature `NotDFMM()` and selector
    /// `0x6853cba7`
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
    #[etherror(name = "NotDFMM", abi = "NotDFMM()")]
    pub struct NotDFMM;
    /// Custom Error type `OutOfBounds` with signature `OutOfBounds()` and
    /// selector `0xb4120f14`
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
    #[etherror(name = "OutOfBounds", abi = "OutOfBounds()")]
    pub struct OutOfBounds;
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
    pub enum LogNormalErrors {
        DeltaError(DeltaError),
        Infinity(Infinity),
        InvalidMean(InvalidMean),
        InvalidReservesLength(InvalidReservesLength),
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWidth(InvalidWidth),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NotDFMM(NotDFMM),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DeltaError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DeltaError(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <InvalidMean as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMean(decoded));
            }
            if let Ok(decoded) =
                <InvalidReservesLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidReservesLength(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) = <InvalidWidth as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidWidth(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeltaError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidMean(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidReservesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidWidth(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DeltaError as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidMean as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidReservesLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateCode as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidUpdateEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidWidth as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeltaError(element) => ::core::fmt::Display::fmt(element, f),
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidMean(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReservesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWidth(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeltaError> for LogNormalErrors {
        fn from(value: DeltaError) -> Self {
            Self::DeltaError(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<InvalidMean> for LogNormalErrors {
        fn from(value: InvalidMean) -> Self {
            Self::InvalidMean(value)
        }
    }
    impl ::core::convert::From<InvalidReservesLength> for LogNormalErrors {
        fn from(value: InvalidReservesLength) -> Self {
            Self::InvalidReservesLength(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for LogNormalErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for LogNormalErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for LogNormalErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWidth> for LogNormalErrors {
        fn from(value: InvalidWidth) -> Self {
            Self::InvalidWidth(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for LogNormalErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    /// Container type for all input parameters for the `dfmm` function with
    /// signature `dfmm()` and selector `0xafba13c4`
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
    #[ethcall(name = "dfmm", abi = "dfmm()")]
    pub struct DfmmCall;
    /// Container type for all input parameters for the `getPoolParams` function
    /// with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    #[ethcall(name = "getPoolParams", abi = "getPoolParams(uint256)")]
    pub struct GetPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `init` function with
    /// signature `init(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x4f17d913`
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
        abi = "init(address,uint256,(address,address[],uint256[],uint256,address,address,uint256),bytes)"
    )]
    pub struct InitCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `internalParams`
    /// function with signature `internalParams(uint256)` and selector
    /// `0x1edb71e5`
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
    #[ethcall(name = "internalParams", abi = "internalParams(uint256)")]
    pub struct InternalParamsCall(pub ::ethers::core::types::U256);
    /// Container type for all input parameters for the `name` function with
    /// signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    /// Container type for all input parameters for the `tradingFunction`
    /// function with signature `tradingFunction(uint256[],uint256,bytes)` and
    /// selector `0x8dda003d`
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
        name = "tradingFunction",
        abi = "tradingFunction(uint256[],uint256,bytes)"
    )]
    pub struct TradingFunctionCall {
        pub reserves: ::std::vec::Vec<::ethers::core::types::U256>,
        pub total_liquidity: ::ethers::core::types::U256,
        pub params: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `update` function with
    /// signature `update(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0xd8b5ed12`
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
        name = "update",
        abi = "update(address,uint256,(address,address[],uint256[],uint256,address,address,uint256),bytes)"
    )]
    pub struct UpdateCall {
        pub sender: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub p2: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateAllocate`
    /// function with signature
    /// `validateAllocate(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x7c101244`
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
        name = "validateAllocate",
        abi = "validateAllocate(address,uint256,(address,address[],uint256[],uint256,address,address,uint256),bytes)"
    )]
    pub struct ValidateAllocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateDeallocate`
    /// function with signature
    /// `validateDeallocate(address,uint256,(address,address[],uint256[],
    /// uint256,address,address,uint256),bytes)` and selector `0x040d951e`
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
        name = "validateDeallocate",
        abi = "validateDeallocate(address,uint256,(address,address[],uint256[],uint256,address,address,uint256),bytes)"
    )]
    pub struct ValidateDeallocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateSwap` function
    /// with signature
    /// `validateSwap(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x75e6440f`
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
        name = "validateSwap",
        abi = "validateSwap(address,uint256,(address,address[],uint256[],uint256,address,address,uint256),bytes)"
    )]
    pub struct ValidateSwapCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
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
    pub enum LogNormalCalls {
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Name(NameCall),
        TradingFunction(TradingFunctionCall),
        Update(UpdateCall),
        ValidateAllocate(ValidateAllocateCall),
        ValidateDeallocate(ValidateDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) =
                <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalParams(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) =
                <TradingFunctionCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TradingFunction(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) =
                <ValidateAllocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateAllocate(decoded));
            }
            if let Ok(decoded) =
                <ValidateDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TradingFunction(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::TradingFunction(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateDeallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DfmmCall> for LogNormalCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for LogNormalCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for LogNormalCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<TradingFunctionCall> for LogNormalCalls {
        fn from(value: TradingFunctionCall) -> Self {
            Self::TradingFunction(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for LogNormalCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateCall> for LogNormalCalls {
        fn from(value: ValidateAllocateCall) -> Self {
            Self::ValidateAllocate(value)
        }
    }
    impl ::core::convert::From<ValidateDeallocateCall> for LogNormalCalls {
        fn from(value: ValidateDeallocateCall) -> Self {
            Self::ValidateDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for LogNormalCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    /// Container type for all return fields from the `dfmm` function with
    /// signature `dfmm()` and selector `0xafba13c4`
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
    pub struct DfmmReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `getPoolParams` function
    /// with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    pub struct GetPoolParamsReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `init` function with
    /// signature `init(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x4f17d913`
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
    pub struct InitReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserves: ::std::vec::Vec<::ethers::core::types::U256>,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `internalParams` function
    /// with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    pub struct InternalParamsReturn {
        pub mean: DynamicParam,
        pub width: DynamicParam,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
    /// Container type for all return fields from the `name` function with
    /// signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    /// Container type for all return fields from the `tradingFunction` function
    /// with signature `tradingFunction(uint256[],uint256,bytes)` and selector
    /// `0x8dda003d`
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
    pub struct TradingFunctionReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the `validateAllocate`
    /// function with signature
    /// `validateAllocate(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x7c101244`
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
    pub struct ValidateAllocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub deltas: ::std::vec::Vec<::ethers::core::types::U256>,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `validateDeallocate`
    /// function with signature
    /// `validateDeallocate(address,uint256,(address,address[],uint256[],
    /// uint256,address,address,uint256),bytes)` and selector `0x040d951e`
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
    pub struct ValidateDeallocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub deltas: ::std::vec::Vec<::ethers::core::types::U256>,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `validateSwap` function
    /// with signature
    /// `validateSwap(address,uint256,(address,address[],uint256[],uint256,
    /// address,address,uint256),bytes)` and selector `0x75e6440f`
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
    pub struct ValidateSwapReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub token_in_index: ::ethers::core::types::U256,
        pub token_out_index: ::ethers::core::types::U256,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
}

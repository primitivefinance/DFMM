pub use geometric_mean::*;
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
pub mod geometric_mean {
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
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
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
                                name: ::std::borrow::ToOwned::to_owned("wX"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static GEOMETRICMEAN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1E\xF98\x03\x80a\x1E\xF9\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x1E`a\0\x99`\09`\0\x81\x81a\x02I\x01R\x81\x81a\x04_\x01Ra\t\x1E\x01Ra\x1E``\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x02\x10W\x80c\x8D\xDA\0=\x14a\x02#W\x80c\xAF\xBA\x13\xC4\x14a\x02DW\x80c\xD8\xB5\xED\x12\x14a\x02\x83W\x80c\xDC\x17\x83U\x14a\x02\x98W`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x08W\x80cO\x17\xD9\x13\x14a\x01\xB3W\x80cu\xE6D\x0F\x14a\x01\xC6W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x17zV[a\x02\xABV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\0\xFB`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x18\xA8V[a\x01ka\x01\x166`\x04a\x18\xBBV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01R\x84\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x93\x84\x01Q\x93\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a\0\xC6V[a\0\xB6a\x01\xC16`\x04a\x18\xD4V[a\x04NV[a\x01\xD9a\x01\xD46`\x04a\x19\xB3V[a\x06\x92V[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x02\x1E6`\x04a\x17zV[a\x07nV[a\x026a\x0216`\x04a\x1A2V[a\x08\xBBV[`@Q\x90\x81R` \x01a\0\xC6V[a\x02k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02\x96a\x02\x916`\x04a\x18\xD4V[a\t\x13V[\0[a\0\xFBa\x02\xA66`\x04a\x18\xBBV[a\n\xE3V[`\0\x80``\x81\x80\x80\x80a\x02\xC0\x88\x8A\x01\x8Aa\x1A\x9EV[\x92P\x92P\x92P\x80\x93Pa\x02\xDC\x84\x8Ba\x02\xD7\x8Ea\n\xE3V[a\x0C\x02V[\x94P\x84`\0\x81Q\x81\x10a\x02\xF1Wa\x02\xF1a\x1A\xCAV[` \x02` \x01\x01Q\x83\x11\x15a\x03IW\x82\x85`\0\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[\x84`\x01\x81Q\x81\x10a\x03\\Wa\x03\\a\x1A\xCAV[` \x02` \x01\x01Q\x82\x11\x15a\x03\x7FW\x81\x85`\x01\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x84`\0\x81Q\x81\x10a\x03\x92Wa\x03\x92a\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x03\xB1Wa\x03\xB1a\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x03\xC5\x91\x90a\x1A\xF6V[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x03\xDDWa\x03\xDDa\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x03\xFCWa\x03\xFCa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x04\x10\x91\x90a\x1A\xF6V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x046\x91\x90a\x04-\x90\x87\x90a\x1A\xF6V[a\x021\x8Ea\n\xE3V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x9DW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xE8`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81RP\x90V[a\x04\xF4\x86\x88\x01\x88a\x1B\tV[`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x87\x01\x91\x90\x91R``\x86\x01\x91\x90\x91R`\x80\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91Ra\x052\x90\x89\x01\x89a\x1BwV[\x90P`\x02\x14\x15\x80a\x05IWP\x80`\xA0\x01QQ`\x02\x14\x15[\x15a\x05gW`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q\x15\x80a\x05\x84WPg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x80\x01Q\x10\x15[\x15a\x05\xA2W`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\0\x8A\x81R` \x81\x90R`@\x80\x82 \x92\x83U``\x84\x01Q`\x04\x84\x01U\x83\x01Q`\x05\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U`\xA0\x82\x01Q\x80Qa\x06N\x92\x90a\x06\x01Wa\x06\x01a\x1A\xCAV[` \x02` \x01\x01Q\x82`\xA0\x01Q`\x01\x81Q\x81\x10a\x06 Wa\x06 a\x1A\xCAV[` \x02` \x01\x01Q\x83`\xC0\x01Qa\x066\x8Da\n\xE3V[\x80` \x01\x90Q\x81\x01\x90a\x06I\x91\x90a\x1B\xC0V[a\x0C\xAEV[` \x82\x01\x81\x90R`\0\x13\x80\x15\x90a\x06jWP`\x1E\x81` \x01Q\x13\x15[\x15\x15\x80\x82R` \x82\x01Q`\xA0\x83\x01Q`\xC0\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x06\xA7\x8Ba\n\xE3V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x06\xBD\x91\x90a\x1C+V[\x92\x98P\x90\x96P\x94P\x92Pa\x06\xD5\x8A\x82\x88\x88\x88\x88a\r\x12V[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x06\xEEWa\x06\xEEa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x07\x02\x91\x90a\x1CaV[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x07\x1FWa\x07\x1Fa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x073\x91\x90a\x1A\xF6V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x07V\x91\x90a\x07P\x90\x85\x90a\x1CaV[\x83a\x08\xBBV[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x07\x83\x88\x8A\x01\x8Aa\x1A\x9EV[\x92P\x92P\x92P\x80\x93Pa\x07\x9F\x84\x8Ba\x07\x9A\x8Ea\n\xE3V[a\r\xB7V[\x94P\x82\x85`\0\x81Q\x81\x10a\x07\xB5Wa\x07\xB5a\x1A\xCAV[` \x02` \x01\x01Q\x11\x15a\x07\xD7W\x82\x85`\0\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x81\x85`\x01\x81Q\x81\x10a\x07\xEBWa\x07\xEBa\x1A\xCAV[` \x02` \x01\x01Q\x11\x15a\x08\rW\x81\x85`\x01\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x84`\0\x81Q\x81\x10a\x08 Wa\x08 a\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x08?Wa\x08?a\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x08S\x91\x90a\x1CaV[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x08kWa\x08ka\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x08\x8AWa\x08\x8Aa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x08\x9E\x91\x90a\x1CaV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x046\x91\x90a\x04-\x90\x87\x90a\x1CaV[`\0a\t\x0B\x84`\0\x81Q\x81\x10a\x08\xD3Wa\x08\xD3a\x1A\xCAV[` \x02` \x01\x01Q\x85`\x01\x81Q\x81\x10a\x08\xEEWa\x08\xEEa\x1A\xCAV[` \x02` \x01\x01Q\x85\x85\x80` \x01\x90Q\x81\x01\x90a\x06I\x91\x90a\x1B\xC0V[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\\W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x05\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\t\x99W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\xA7\x82\x84\x01\x84a\x1C\x83V[\x90P`\x01\x81`\x03\x81\x11\x15a\t\xBDWa\t\xBDa\x1C\x9EV[\x03a\t\xE6Wa\t\xCE\x82\x84\x01\x84a\x1C\xB4V[`\0\x87\x81R` \x81\x90R`@\x90 `\x04\x01UPa\n\xDBV[`\x02\x81`\x03\x81\x11\x15a\t\xFAWa\t\xFAa\x1C\x9EV[\x03a\nhW`\0\x80a\n\x0E\x84\x86\x01\x86a\x1C\xDEV[\x92P\x92PP\x81`\0\x14\x80a\n*WPg\r\xE0\xB6\xB3\xA7d\0\0\x82\x10\x15[\x15a\nHW`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\na\x90\x83\x83a\x0E=V[PPa\n\xDBV[`\x03\x81`\x03\x81\x11\x15a\n|Wa\n|a\x1C\x9EV[\x03a\n\xC2Wa\n\x8D\x82\x84\x01\x84a\x1D\x11V[`\0\x87\x81R` \x81\x90R`@\x90 `\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\n\xDBV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\x0B\x19`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01Ra\x0Ba\x90a\x0F\x03V[\x80\x82Ra\x0Bv\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF6V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x80\x82 `\x04\x81\x01T\x82\x86\x01R\x86\x83R\x91\x83R`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01RQa\x0B\xEB\x91\x83\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0CO\x83`@\x01Q`\0\x81Q\x81\x10a\x0C<Wa\x0C<a\x1A\xCAV[` \x02` \x01\x01Q\x85\x85``\x01Qa\x0F\x94V[\x81`\0\x81Q\x81\x10a\x0CbWa\x0Cba\x1A\xCAV[` \x02` \x01\x01\x81\x81RPPa\x0C\x88\x83`@\x01Q`\x01\x81Q\x81\x10a\x0C<Wa\x0C<a\x1A\xCAV[\x81`\x01\x81Q\x81\x10a\x0C\x9BWa\x0C\x9Ba\x1A\xCAV[` \x02` \x01\x01\x81\x81RPP\x93\x92PPPV[\x80Q`\0\x90\x81\x90a\x0C\xC9\x90a\x0C\xC3\x88\x87a\x0F\xA1V[\x90a\x0F\xBFV[\x90P`\0a\x0C\xE8\x84` \x01Qa\x0C\xC3\x87\x89a\x0F\xA1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xFD\x83\x83a\x0F\xF0V[a\r\x07\x91\x90a\x1DHV[\x97\x96PPPPPPPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r)\x91\x90a\x1B\xC0V[\x90P\x85`\0\x03a\rrWa\rj\x84\x89`@\x01Q`\0\x81Q\x81\x10a\rNWa\rNa\x1A\xCAV[` \x02` \x01\x01Q\x8A``\x01Q\x84`\0\x01Q\x85`@\x01Qa\x10\x05V[\x91PPa\r\xADV[a\r\xA9\x84\x89`@\x01Q`\x01\x81Q\x81\x10a\r\x8DWa\r\x8Da\x1A\xCAV[` \x02` \x01\x01Q\x8A``\x01Q\x84` \x01Q\x85`@\x01Qa\x10\x05V[\x91PP[\x96\x95PPPPPPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0E\x04\x83`@\x01Q`\0\x81Q\x81\x10a\r\xF1Wa\r\xF1a\x1A\xCAV[` \x02` \x01\x01Q\x85\x85``\x01Qa\x10&V[\x81`\0\x81Q\x81\x10a\x0E\x17Wa\x0E\x17a\x1A\xCAV[` \x02` \x01\x01\x81\x81RPPa\x0C\x88\x83`@\x01Q`\x01\x81Q\x81\x10a\r\xF1Wa\r\xF1a\x1A\xCAV[B\x81\x11a\x0E]W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ef\x83a\x103V[`\0a\x0ErB\x83a\x1A\xF6V[\x84T\x90\x91P`\0\x90a\x0E\x84\x90\x85a\x1DHV[\x90P`\0a\x0E\x92\x83\x83a\x1D\x85V[\x90P`\0a\x0E\xA0\x84\x84a\x1D\xB3V[\x90P`\0\x81\x13\x15a\x0E\xCAW\x80\x87`\0\x01`\0\x82\x82Ta\x0E\xBF\x91\x90a\x1CaV[\x90\x91UPa\x0E\xEC\x90PV[a\x0E\xD3\x81a\x1D\xC7V[\x87`\0\x01`\0\x82\x82Ta\x0E\xE6\x91\x90a\x1A\xF6V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x0F\x19WPQ\x90V[`\0\x82` \x01QB\x11a\x0F,WBa\x0F2V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0FF\x91\x90a\x1A\xF6V[\x90P`\0\x84`@\x01Q\x13\x15a\x0FpW`@\x84\x01Qa\x0Fd\x90\x82a\x1D\xE3V[\x84Qa\t\x0B\x91\x90a\x1CaV[\x83`@\x01Qa\x0F~\x90a\x1D\xC7V[a\x0F\x88\x90\x82a\x1D\xE3V[\x84Qa\t\x0B\x91\x90a\x1A\xF6V[`\0a\t\x0B\x84\x84\x84a\x10uV[`\0a\x0F\xB6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x94V[\x90P[\x92\x91PPV[`\0a\x0F\xB6g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F\xD7\x86a\x10\xC2V[a\x0F\xE1\x91\x90a\x1D\xFAV[a\x0F\xEB\x91\x90a\x1D\x85V[a\x12\x9DV[`\0a\x0F\xB6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x94V[`\0a\r\xADa\x10\x14\x87\x87a\x0F\xA1V[a\x10 \x86\x81\x87\x87a\x0F\xF0V[\x90a\x0F\xF0V[`\0a\t\x0B\x84\x84\x84a\x10\x94V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x10j\x90a\x0F\x03V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\x8DW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xACW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x10\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03@V[`\0``a\x11\x0C\x84a\x14FV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x12\xB8WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x12\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03@V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x14\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03@V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x02W`\0\x80\xFD[PV[\x805a\x15\x10\x81a\x14\xEDV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15MWa\x15Ma\x15\x15V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15{Wa\x15{a\x15\x15V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\x9CWa\x15\x9Ca\x15\x15V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x15\xB7W`\0\x80\xFD[\x815` a\x15\xCCa\x15\xC7\x83a\x15\x83V[a\x15SV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x15\xEEW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16\x13W\x805a\x16\x06\x81a\x14\xEDV[\x83R\x91\x83\x01\x91\x83\x01a\x15\xF3V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x16/W`\0\x80\xFD[\x815` a\x16?a\x15\xC7\x83a\x15\x83V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x16aW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16\x13W\x805\x83R\x91\x83\x01\x91\x83\x01a\x16fV[`\0`\xE0\x82\x84\x03\x12\x15a\x16\x8FW`\0\x80\xFD[a\x16\x97a\x15+V[\x90Pa\x16\xA2\x82a\x15\x05V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xBEW`\0\x80\xFD[a\x16\xCA\x85\x83\x86\x01a\x15\xA6V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x16\xE3W`\0\x80\xFD[Pa\x16\xF0\x84\x82\x85\x01a\x16\x1EV[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x17\x0C`\x80\x83\x01a\x15\x05V[`\x80\x82\x01Ra\x17\x1D`\xA0\x83\x01a\x15\x05V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x17DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17[W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x17sW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17\x92W`\0\x80\xFD[\x855a\x17\x9D\x81a\x14\xEDV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xC0W`\0\x80\xFD[a\x17\xCC\x89\x83\x8A\x01a\x16}V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x17\xE2W`\0\x80\xFD[Pa\x17\xEF\x88\x82\x89\x01a\x172V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x18KW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x18/V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\x88W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18lV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0F\xB6` \x83\x01\x84a\x18bV[`\0` \x82\x84\x03\x12\x15a\x18\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x18\xECW`\0\x80\xFD[\x855a\x18\xF7\x81a\x14\xEDV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x1AW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x19.W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x17\xE2W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x19UW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19nWa\x19na\x15\x15V[a\x19\x81`\x1F\x82\x01`\x1F\x19\x16` \x01a\x15SV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19\x96W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19\xC9W`\0\x80\xFD[\x845a\x19\xD4\x81a\x14\xEDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\xF7W`\0\x80\xFD[a\x1A\x03\x88\x83\x89\x01a\x16}V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1A\x19W`\0\x80\xFD[Pa\x1A&\x87\x82\x88\x01a\x19DV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1AGW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A^W`\0\x80\xFD[a\x1Aj\x87\x83\x88\x01a\x16\x1EV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1A\x87W`\0\x80\xFD[Pa\x1A\x94\x86\x82\x87\x01a\x19DV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xB3W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1B!W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B7W`\0\x80\xFD[a\x1BC\x88\x82\x89\x01a\x16\x1EV[\x95PP` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015a\x1Bi\x81a\x14\xEDV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1B\x8EW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1B\xA8W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x17sW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1B\xD2W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1B\xF4Wa\x1B\xF4a\x15\x15V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa\x1C\x1F\x81a\x14\xEDV[``\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1CAW`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[\x805`\x04\x81\x10a\x15\x10W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1C\x95W`\0\x80\xFD[a\x0F\xB6\x82a\x1CtV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xC7W`\0\x80\xFD[a\x1C\xD0\x83a\x1CtV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\xF3W`\0\x80\xFD[a\x1C\xFC\x84a\x1CtV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D$W`\0\x80\xFD[a\x1D-\x83a\x1CtV[\x91P` \x83\x015a\x1D=\x81a\x14\xEDV[\x80\x91PP\x92P\x92\x90PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1DhWa\x1Dha\x1A\xE0V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1D\x94Wa\x1D\x94a\x1DoV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1D\xAEWa\x1D\xAEa\x1A\xE0V[P\x05\x90V[`\0\x82a\x1D\xC2Wa\x1D\xC2a\x1DoV[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a\x1D\xDCWa\x1D\xDCa\x1A\xE0V[P`\0\x03\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1E\x16Wa\x1E\x16a\x1A\xE0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V\xFE\xA2dipfsX\"\x12 \x8D\xA7\xA6\xA9\x86\x05\xF0\x1B\xD7\xB0\x06\x02\xA9\xAA\xEEx\xD8\xAF\x96\x9C\x84\x10n/\xA3\xD9\xC2\xC4\xAC\xCC\xF2\x1BdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEAN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x02\x10W\x80c\x8D\xDA\0=\x14a\x02#W\x80c\xAF\xBA\x13\xC4\x14a\x02DW\x80c\xD8\xB5\xED\x12\x14a\x02\x83W\x80c\xDC\x17\x83U\x14a\x02\x98W`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x08W\x80cO\x17\xD9\x13\x14a\x01\xB3W\x80cu\xE6D\x0F\x14a\x01\xC6W[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x17zV[a\x02\xABV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x18\0V[`@Q\x80\x91\x03\x90\xF3[a\0\xFB`@Q\x80`@\x01`@R\x80`\r\x81R` \x01l#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x18\xA8V[a\x01ka\x01\x166`\x04a\x18\xBBV[`\0` \x81\x81R\x91\x81R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R\x81T\x81R`\x01\x82\x01T\x93\x81\x01\x93\x90\x93R`\x02\x81\x01T\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x84Q\x81R` \x80\x86\x01Q\x90\x82\x01R\x84\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x93\x84\x01Q\x93\x81\x01\x93\x90\x93R`\x80\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01a\0\xC6V[a\0\xB6a\x01\xC16`\x04a\x18\xD4V[a\x04NV[a\x01\xD9a\x01\xD46`\x04a\x19\xB3V[a\x06\x92V[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x02\x1E6`\x04a\x17zV[a\x07nV[a\x026a\x0216`\x04a\x1A2V[a\x08\xBBV[`@Q\x90\x81R` \x01a\0\xC6V[a\x02k\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02\x96a\x02\x916`\x04a\x18\xD4V[a\t\x13V[\0[a\0\xFBa\x02\xA66`\x04a\x18\xBBV[a\n\xE3V[`\0\x80``\x81\x80\x80\x80a\x02\xC0\x88\x8A\x01\x8Aa\x1A\x9EV[\x92P\x92P\x92P\x80\x93Pa\x02\xDC\x84\x8Ba\x02\xD7\x8Ea\n\xE3V[a\x0C\x02V[\x94P\x84`\0\x81Q\x81\x10a\x02\xF1Wa\x02\xF1a\x1A\xCAV[` \x02` \x01\x01Q\x83\x11\x15a\x03IW\x82\x85`\0\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03@\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[`@Q\x80\x91\x03\x90\xFD[\x84`\x01\x81Q\x81\x10a\x03\\Wa\x03\\a\x1A\xCAV[` \x02` \x01\x01Q\x82\x11\x15a\x03\x7FW\x81\x85`\x01\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x84`\0\x81Q\x81\x10a\x03\x92Wa\x03\x92a\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x03\xB1Wa\x03\xB1a\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x03\xC5\x91\x90a\x1A\xF6V[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x03\xDDWa\x03\xDDa\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x03\xFCWa\x03\xFCa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x04\x10\x91\x90a\x1A\xF6V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x046\x91\x90a\x04-\x90\x87\x90a\x1A\xF6V[a\x021\x8Ea\n\xE3V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x9DW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xE8`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01`\0\x81RP\x90V[a\x04\xF4\x86\x88\x01\x88a\x1B\tV[`\x01`\x01`\xA0\x1B\x03\x16`@\x80\x87\x01\x91\x90\x91R``\x86\x01\x91\x90\x91R`\x80\x85\x01\x91\x90\x91R`\xC0\x84\x01\x91\x90\x91R`\xA0\x83\x01\x91\x90\x91Ra\x052\x90\x89\x01\x89a\x1BwV[\x90P`\x02\x14\x15\x80a\x05IWP\x80`\xA0\x01QQ`\x02\x14\x15[\x15a\x05gW`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q\x15\x80a\x05\x84WPg\r\xE0\xB6\xB3\xA7d\0\0\x81`\x80\x01Q\x10\x15[\x15a\x05\xA2W`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x80\x81\x01Q`\0\x8A\x81R` \x81\x90R`@\x80\x82 \x92\x83U``\x84\x01Q`\x04\x84\x01U\x83\x01Q`\x05\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U`\xA0\x82\x01Q\x80Qa\x06N\x92\x90a\x06\x01Wa\x06\x01a\x1A\xCAV[` \x02` \x01\x01Q\x82`\xA0\x01Q`\x01\x81Q\x81\x10a\x06 Wa\x06 a\x1A\xCAV[` \x02` \x01\x01Q\x83`\xC0\x01Qa\x066\x8Da\n\xE3V[\x80` \x01\x90Q\x81\x01\x90a\x06I\x91\x90a\x1B\xC0V[a\x0C\xAEV[` \x82\x01\x81\x90R`\0\x13\x80\x15\x90a\x06jWP`\x1E\x81` \x01Q\x13\x15[\x15\x15\x80\x82R` \x82\x01Q`\xA0\x83\x01Q`\xC0\x90\x93\x01Q\x91\x9C\x90\x9BP\x91\x99P\x97P\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x06\xA7\x8Ba\n\xE3V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x06\xBD\x91\x90a\x1C+V[\x92\x98P\x90\x96P\x94P\x92Pa\x06\xD5\x8A\x82\x88\x88\x88\x88a\r\x12V[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x06\xEEWa\x06\xEEa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x07\x02\x91\x90a\x1CaV[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x07\x1FWa\x07\x1Fa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x073\x91\x90a\x1A\xF6V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x07V\x91\x90a\x07P\x90\x85\x90a\x1CaV[\x83a\x08\xBBV[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x07\x83\x88\x8A\x01\x8Aa\x1A\x9EV[\x92P\x92P\x92P\x80\x93Pa\x07\x9F\x84\x8Ba\x07\x9A\x8Ea\n\xE3V[a\r\xB7V[\x94P\x82\x85`\0\x81Q\x81\x10a\x07\xB5Wa\x07\xB5a\x1A\xCAV[` \x02` \x01\x01Q\x11\x15a\x07\xD7W\x82\x85`\0\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x81\x85`\x01\x81Q\x81\x10a\x07\xEBWa\x07\xEBa\x1A\xCAV[` \x02` \x01\x01Q\x11\x15a\x08\rW\x81\x85`\x01\x81Q\x81\x10a\x03\x14Wa\x03\x14a\x1A\xCAV[\x84`\0\x81Q\x81\x10a\x08 Wa\x08 a\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\0\x81Q\x81\x10a\x08?Wa\x08?a\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x08S\x91\x90a\x1CaV[\x90RP\x84Q\x85\x90`\x01\x90\x81\x10a\x08kWa\x08ka\x1A\xCAV[` \x02` \x01\x01Q\x8A`@\x01Q`\x01\x81Q\x81\x10a\x08\x8AWa\x08\x8Aa\x1A\xCAV[` \x02` \x01\x01\x81\x81Qa\x08\x9E\x91\x90a\x1CaV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x046\x91\x90a\x04-\x90\x87\x90a\x1CaV[`\0a\t\x0B\x84`\0\x81Q\x81\x10a\x08\xD3Wa\x08\xD3a\x1A\xCAV[` \x02` \x01\x01Q\x85`\x01\x81Q\x81\x10a\x08\xEEWa\x08\xEEa\x1A\xCAV[` \x02` \x01\x01Q\x85\x85\x80` \x01\x90Q\x81\x01\x90a\x06I\x91\x90a\x1B\xC0V[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\t\\W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x05\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\t\x99W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\xA7\x82\x84\x01\x84a\x1C\x83V[\x90P`\x01\x81`\x03\x81\x11\x15a\t\xBDWa\t\xBDa\x1C\x9EV[\x03a\t\xE6Wa\t\xCE\x82\x84\x01\x84a\x1C\xB4V[`\0\x87\x81R` \x81\x90R`@\x90 `\x04\x01UPa\n\xDBV[`\x02\x81`\x03\x81\x11\x15a\t\xFAWa\t\xFAa\x1C\x9EV[\x03a\nhW`\0\x80a\n\x0E\x84\x86\x01\x86a\x1C\xDEV[\x92P\x92PP\x81`\0\x14\x80a\n*WPg\r\xE0\xB6\xB3\xA7d\0\0\x82\x10\x15[\x15a\nHW`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x87\x81R` \x81\x90R`@\x90 a\na\x90\x83\x83a\x0E=V[PPa\n\xDBV[`\x03\x81`\x03\x81\x11\x15a\n|Wa\n|a\x1C\x9EV[\x03a\n\xC2Wa\n\x8D\x82\x84\x01\x84a\x1D\x11V[`\0\x87\x81R` \x81\x90R`@\x90 `\x05\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UPa\n\xDBV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\x0B\x19`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R\x81T\x81R`\x01\x82\x01T\x92\x81\x01\x92\x90\x92R`\x02\x81\x01T\x92\x82\x01\x92\x90\x92R`\x03\x90\x91\x01T``\x82\x01Ra\x0Ba\x90a\x0F\x03V[\x80\x82Ra\x0Bv\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF6V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R\x80\x82R`@\x80\x82 `\x04\x81\x01T\x82\x86\x01R\x86\x83R\x91\x83R`\x05\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16``\x84\x01RQa\x0B\xEB\x91\x83\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x91\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0CO\x83`@\x01Q`\0\x81Q\x81\x10a\x0C<Wa\x0C<a\x1A\xCAV[` \x02` \x01\x01Q\x85\x85``\x01Qa\x0F\x94V[\x81`\0\x81Q\x81\x10a\x0CbWa\x0Cba\x1A\xCAV[` \x02` \x01\x01\x81\x81RPPa\x0C\x88\x83`@\x01Q`\x01\x81Q\x81\x10a\x0C<Wa\x0C<a\x1A\xCAV[\x81`\x01\x81Q\x81\x10a\x0C\x9BWa\x0C\x9Ba\x1A\xCAV[` \x02` \x01\x01\x81\x81RPP\x93\x92PPPV[\x80Q`\0\x90\x81\x90a\x0C\xC9\x90a\x0C\xC3\x88\x87a\x0F\xA1V[\x90a\x0F\xBFV[\x90P`\0a\x0C\xE8\x84` \x01Qa\x0C\xC3\x87\x89a\x0F\xA1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xFD\x83\x83a\x0F\xF0V[a\r\x07\x91\x90a\x1DHV[\x97\x96PPPPPPPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r)\x91\x90a\x1B\xC0V[\x90P\x85`\0\x03a\rrWa\rj\x84\x89`@\x01Q`\0\x81Q\x81\x10a\rNWa\rNa\x1A\xCAV[` \x02` \x01\x01Q\x8A``\x01Q\x84`\0\x01Q\x85`@\x01Qa\x10\x05V[\x91PPa\r\xADV[a\r\xA9\x84\x89`@\x01Q`\x01\x81Q\x81\x10a\r\x8DWa\r\x8Da\x1A\xCAV[` \x02` \x01\x01Q\x8A``\x01Q\x84` \x01Q\x85`@\x01Qa\x10\x05V[\x91PP[\x96\x95PPPPPPV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92` \x83\x01\x90\x806\x837\x01\x90PP\x90Pa\x0E\x04\x83`@\x01Q`\0\x81Q\x81\x10a\r\xF1Wa\r\xF1a\x1A\xCAV[` \x02` \x01\x01Q\x85\x85``\x01Qa\x10&V[\x81`\0\x81Q\x81\x10a\x0E\x17Wa\x0E\x17a\x1A\xCAV[` \x02` \x01\x01\x81\x81RPPa\x0C\x88\x83`@\x01Q`\x01\x81Q\x81\x10a\r\xF1Wa\r\xF1a\x1A\xCAV[B\x81\x11a\x0E]W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x0Ef\x83a\x103V[`\0a\x0ErB\x83a\x1A\xF6V[\x84T\x90\x91P`\0\x90a\x0E\x84\x90\x85a\x1DHV[\x90P`\0a\x0E\x92\x83\x83a\x1D\x85V[\x90P`\0a\x0E\xA0\x84\x84a\x1D\xB3V[\x90P`\0\x81\x13\x15a\x0E\xCAW\x80\x87`\0\x01`\0\x82\x82Ta\x0E\xBF\x91\x90a\x1CaV[\x90\x91UPa\x0E\xEC\x90PV[a\x0E\xD3\x81a\x1D\xC7V[\x87`\0\x01`\0\x82\x82Ta\x0E\xE6\x91\x90a\x1A\xF6V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x0F\x19WPQ\x90V[`\0\x82` \x01QB\x11a\x0F,WBa\x0F2V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0FF\x91\x90a\x1A\xF6V[\x90P`\0\x84`@\x01Q\x13\x15a\x0FpW`@\x84\x01Qa\x0Fd\x90\x82a\x1D\xE3V[\x84Qa\t\x0B\x91\x90a\x1CaV[\x83`@\x01Qa\x0F~\x90a\x1D\xC7V[a\x0F\x88\x90\x82a\x1D\xE3V[\x84Qa\t\x0B\x91\x90a\x1A\xF6V[`\0a\t\x0B\x84\x84\x84a\x10uV[`\0a\x0F\xB6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x94V[\x90P[\x92\x91PPV[`\0a\x0F\xB6g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0F\xD7\x86a\x10\xC2V[a\x0F\xE1\x91\x90a\x1D\xFAV[a\x0F\xEB\x91\x90a\x1D\x85V[a\x12\x9DV[`\0a\x0F\xB6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x94V[`\0a\r\xADa\x10\x14\x87\x87a\x0F\xA1V[a\x10 \x86\x81\x87\x87a\x0F\xF0V[\x90a\x0F\xF0V[`\0a\t\x0B\x84\x84\x84a\x10\x94V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x10j\x90a\x0F\x03V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\x8DW`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xACW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x10\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03@V[`\0``a\x11\x0C\x84a\x14FV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x12\xB8WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x12\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03@V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80\x82\x11a\x14\x83W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03@V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\x02W`\0\x80\xFD[PV[\x805a\x15\x10\x81a\x14\xEDV[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15MWa\x15Ma\x15\x15V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x15{Wa\x15{a\x15\x15V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x15\x9CWa\x15\x9Ca\x15\x15V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x15\xB7W`\0\x80\xFD[\x815` a\x15\xCCa\x15\xC7\x83a\x15\x83V[a\x15SV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x15\xEEW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16\x13W\x805a\x16\x06\x81a\x14\xEDV[\x83R\x91\x83\x01\x91\x83\x01a\x15\xF3V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x16/W`\0\x80\xFD[\x815` a\x16?a\x15\xC7\x83a\x15\x83V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x16aW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x16\x13W\x805\x83R\x91\x83\x01\x91\x83\x01a\x16fV[`\0`\xE0\x82\x84\x03\x12\x15a\x16\x8FW`\0\x80\xFD[a\x16\x97a\x15+V[\x90Pa\x16\xA2\x82a\x15\x05V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x16\xBEW`\0\x80\xFD[a\x16\xCA\x85\x83\x86\x01a\x15\xA6V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x16\xE3W`\0\x80\xFD[Pa\x16\xF0\x84\x82\x85\x01a\x16\x1EV[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x17\x0C`\x80\x83\x01a\x15\x05V[`\x80\x82\x01Ra\x17\x1D`\xA0\x83\x01a\x15\x05V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x17DW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x17[W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x17sW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17\x92W`\0\x80\xFD[\x855a\x17\x9D\x81a\x14\xEDV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xC0W`\0\x80\xFD[a\x17\xCC\x89\x83\x8A\x01a\x16}V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x17\xE2W`\0\x80\xFD[Pa\x17\xEF\x88\x82\x89\x01a\x172V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x18KW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x18/V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x18\x88W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18lV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0F\xB6` \x83\x01\x84a\x18bV[`\0` \x82\x84\x03\x12\x15a\x18\xCDW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x18\xECW`\0\x80\xFD[\x855a\x18\xF7\x81a\x14\xEDV[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\x1AW`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x19.W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x17\xE2W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x19UW`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x19nWa\x19na\x15\x15V[a\x19\x81`\x1F\x82\x01`\x1F\x19\x16` \x01a\x15SV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x19\x96W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x19\xC9W`\0\x80\xFD[\x845a\x19\xD4\x81a\x14\xEDV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x19\xF7W`\0\x80\xFD[a\x1A\x03\x88\x83\x89\x01a\x16}V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1A\x19W`\0\x80\xFD[Pa\x1A&\x87\x82\x88\x01a\x19DV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1AGW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A^W`\0\x80\xFD[a\x1Aj\x87\x83\x88\x01a\x16\x1EV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1A\x87W`\0\x80\xFD[Pa\x1A\x94\x86\x82\x87\x01a\x19DV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1A\xB3W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1B!W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B7W`\0\x80\xFD[a\x1BC\x88\x82\x89\x01a\x16\x1EV[\x95PP` \x86\x015\x93P`@\x86\x015\x92P``\x86\x015\x91P`\x80\x86\x015a\x1Bi\x81a\x14\xEDV[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1B\x8EW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1B\xA8W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x17sW`\0\x80\xFD[`\0`\x80\x82\x84\x03\x12\x15a\x1B\xD2W`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a\x1B\xF4Wa\x1B\xF4a\x15\x15V[\x80`@RP\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Qa\x1C\x1F\x81a\x14\xEDV[``\x82\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1CAW`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[\x805`\x04\x81\x10a\x15\x10W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1C\x95W`\0\x80\xFD[a\x0F\xB6\x82a\x1CtV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xC7W`\0\x80\xFD[a\x1C\xD0\x83a\x1CtV[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C\xF3W`\0\x80\xFD[a\x1C\xFC\x84a\x1CtV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1D$W`\0\x80\xFD[a\x1D-\x83a\x1CtV[\x91P` \x83\x015a\x1D=\x81a\x14\xEDV[\x80\x91PP\x92P\x92\x90PV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1DhWa\x1Dha\x1A\xE0V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1D\x94Wa\x1D\x94a\x1DoV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1D\xAEWa\x1D\xAEa\x1A\xE0V[P\x05\x90V[`\0\x82a\x1D\xC2Wa\x1D\xC2a\x1DoV[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a\x1D\xDCWa\x1D\xDCa\x1A\xE0V[P`\0\x03\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1E\x16Wa\x1E\x16a\x1A\xE0V[\x81\x81\x05\x83\x14\x82\x15\x17a\x0F\xB9Wa\x0F\xB9a\x1A\xE0V\xFE\xA2dipfsX\"\x12 \x8D\xA7\xA6\xA9\x86\x05\xF0\x1B\xD7\xB0\x06\x02\xA9\xAA\xEEx\xD8\xAF\x96\x9C\x84\x10n/\xA3\xD9\xC2\xC4\xAC\xCC\xF2\x1BdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEAN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct GeometricMean<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMean<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMean<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMean<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMean<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMean))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMean<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GEOMETRICMEAN_ABI.clone(),
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
                GEOMETRICMEAN_ABI.clone(),
                GEOMETRICMEAN_BYTECODE.clone().into(),
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GeometricMean<M>
    {
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
    /// Custom Error type `InvalidWeightX` with signature `InvalidWeightX()` and
    /// selector `0xe8a38a61`
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
    #[etherror(name = "InvalidWeightX", abi = "InvalidWeightX()")]
    pub struct InvalidWeightX;
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
    pub enum GeometricMeanErrors {
        DeltaError(DeltaError),
        InvalidReservesLength(InvalidReservesLength),
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWeightX(InvalidWeightX),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanErrors {
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
            if let Ok(decoded) = <InvalidWeightX as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidWeightX(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeometricMeanErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeltaError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidReservesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidWeightX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GeometricMeanErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DeltaError as ::ethers::contract::EthError>::selector() => true,
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
                _ if selector == <InvalidWeightX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeltaError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReservesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeightX(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GeometricMeanErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeltaError> for GeometricMeanErrors {
        fn from(value: DeltaError) -> Self {
            Self::DeltaError(value)
        }
    }
    impl ::core::convert::From<InvalidReservesLength> for GeometricMeanErrors {
        fn from(value: InvalidReservesLength) -> Self {
            Self::InvalidReservesLength(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for GeometricMeanErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for GeometricMeanErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for GeometricMeanErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWeightX> for GeometricMeanErrors {
        fn from(value: InvalidWeightX) -> Self {
            Self::InvalidWeightX(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for GeometricMeanErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
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
    pub enum GeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiDecode for GeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanCalls {
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
    impl ::core::fmt::Display for GeometricMeanCalls {
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
    impl ::core::convert::From<DfmmCall> for GeometricMeanCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for GeometricMeanCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for GeometricMeanCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for GeometricMeanCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for GeometricMeanCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<TradingFunctionCall> for GeometricMeanCalls {
        fn from(value: TradingFunctionCall) -> Self {
            Self::TradingFunction(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for GeometricMeanCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateCall> for GeometricMeanCalls {
        fn from(value: ValidateAllocateCall) -> Self {
            Self::ValidateAllocate(value)
        }
    }
    impl ::core::convert::From<ValidateDeallocateCall> for GeometricMeanCalls {
        fn from(value: ValidateDeallocateCall) -> Self {
            Self::ValidateDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for GeometricMeanCalls {
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
    pub struct InitReturn(
        pub bool,
        pub ::ethers::core::types::I256,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::U256,
    );
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
        pub w_x: DynamicParam,
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

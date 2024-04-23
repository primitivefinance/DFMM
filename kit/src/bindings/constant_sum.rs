pub use constant_sum::*;
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
pub mod constant_sum {
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
                                name: ::std::borrow::ToOwned::to_owned("price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidDeltaLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidDeltaLiquidity",),
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
    pub static CONSTANTSUM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x15\x048\x03\x80a\x15\x04\x839\x81\x01`@\x81\x90Ra\0/\x91a\0@V[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0pV[`\0` \x82\x84\x03\x12\x15a\0RW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0iW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x14ka\0\x99`\09`\0\x81\x81a\x01\xF8\x01R\x81\x81a\x03\xC5\x01Ra\x07\xB5\x01Ra\x14k`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x01\xBFW\x80c\x8D\xDA\0=\x14a\x01\xD2W\x80c\xAF\xBA\x13\xC4\x14a\x01\xF3W\x80c\xD8\xB5\xED\x12\x14a\x022W\x80c\xDC\x17\x83U\x14a\x02GW`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x06W\x80cO\x17\xD9\x13\x14a\x01bW\x80cu\xE6D\x0F\x14a\x01uW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x0E\x80V[a\x02ZV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x0F\x06V[`@Q\x80\x91\x03\x90\xF3[a\0\xF9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jConstantSum`\xA8\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x0F\xAEV[a\x01>a\x01\x146`\x04a\x0F\xC1V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\0\xC6V[a\0\xB6a\x01p6`\x04a\x0F\xDAV[a\x03\xB4V[a\x01\x88a\x01\x836`\x04a\x10\xB9V[a\x05jV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x01\xCD6`\x04a\x0E\x80V[a\x06FV[a\x01\xE5a\x01\xE06`\x04a\x118V[a\x07\x7FV[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\x1A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02Ea\x02@6`\x04a\x0F\xDAV[a\x07\xAAV[\0[a\0\xF9a\x02U6`\x04a\x0F\xC1V[a\t\xC8V[`\0\x80``\x81\x80\x80\x80a\x02o\x88\x8A\x01\x8Aa\x11\xA4V[\x92P\x92P\x92Pa\x02\x95\x83\x83`\0\x80\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x01Ta\nhV[\x93P\x83\x81\x11\x15a\x02\xB8W`@Qc\x1B\x15n\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x94P\x82\x85`\0\x81Q\x81\x10a\x02\xECWa\x02\xECa\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x81\x85`\x01\x81Q\x81\x10a\x03\x0CWa\x03\x0Ca\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x82\x8A`@\x01Q`\0\x81Q\x81\x10a\x030Wa\x030a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x03D\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q\x80Q\x83\x91\x90`\x01\x90\x81\x10a\x03bWa\x03ba\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x03v\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x03\x9C\x91\x90a\x03\x93\x90\x87\x90a\x11\xFCV[a\x01\xE0\x8Ea\t\xC8V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x03W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x040`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x04<\x86\x88\x01\x88a\x12\x0FV[\x80\x92P\x81\x94PPPa\x04\x87\x83`\0\x81Q\x81\x10a\x04ZWa\x04Za\x11\xD0V[` \x02` \x01\x01Q\x84`\x01\x81Q\x81\x10a\x04uWa\x04ua\x11\xD0V[` \x02` \x01\x01Q\x83`\0\x01Qa\nhV[\x91Pa\x04\x96`@\x89\x01\x89a\x12\x92V[\x90P`\x02\x14\x15\x80a\x04\xA9WP\x82Q`\x02\x14\x15[\x15a\x04\xC7W`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x8A\x81R` \x81\x81R`@\x91\x82\x90 \x92\x83U\x80\x84\x01\x80Q`\x01\x85\x01U\x82\x85\x01\x80Q`\x02\x90\x95\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x90U\x83Q\x86Q\x93\x81\x01\x93\x90\x93R\x90Q\x92\x82\x01\x92\x90\x92R\x90Q\x90\x91\x16``\x82\x01Ra\x05G\x90\x84\x90\x84\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x07\x7FV[\x93P`\0\x84\x12\x15\x80\x15a\x05[WP`\x1E\x84\x13\x15[\x94PP\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x05\x7F\x8Ba\t\xC8V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x05\x95\x91\x90a\x12\xDBV[\x92\x98P\x90\x96P\x94P\x92Pa\x05\xAD\x8A\x82\x88\x88\x88\x88a\n\x7FV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x05\xC6Wa\x05\xC6a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x05\xDA\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x05\xF7Wa\x05\xF7a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x06\x0B\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x06.\x91\x90a\x06(\x90\x85\x90a\x13\x11V[\x83a\x07\x7FV[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x06[\x88\x8A\x01\x8Aa\x11\xA4V[\x92P\x92P\x92Pa\x06\x81\x83\x83`\0\x80\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x01Ta\nhV[\x93P\x80\x84\x10\x15a\x06\xA4W`@Qc\x1B\x15n\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x94P\x82\x85`\0\x81Q\x81\x10a\x06\xD8Wa\x06\xD8a\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x81\x85`\x01\x81Q\x81\x10a\x06\xF8Wa\x06\xF8a\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x82\x8A`@\x01Q`\0\x81Q\x81\x10a\x07\x1CWa\x07\x1Ca\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x070\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q\x80Q\x83\x91\x90`\x01\x90\x81\x10a\x07NWa\x07Na\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x07b\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x03\x9C\x91\x90a\x03\x93\x90\x87\x90a\x13\x11V[`\0a\x07\xA0\x84\x84\x84\x80` \x01\x90Q\x81\x01\x90a\x07\x9A\x91\x90a\x13$V[Qa\n\xABV[\x90P[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xF3W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x080W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08>\x82\x84\x01\x84a\x13sV[\x90P`\x02\x81`\x03\x81\x11\x15a\x08TWa\x08Ta\x13\x90V[\x03a\x08\xACWa\x08\x98\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x18\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 Ua\t\xC0V[`\x01\x81`\x03\x81\x11\x15a\x08\xC0Wa\x08\xC0a\x13\x90V[\x03a\t\x1BWa\t\x04\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x18\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x01\x01Ua\t\xC0V[`\x03\x81`\x03\x81\x11\x15a\t/Wa\t/a\x13\x90V[\x03a\t\xA7Wa\ts\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B.\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\t\xC0V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\t\xF7`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x80\x83 \x80T\x85R`\x01\x81\x01T\x85\x84\x01\x90\x81R\x87\x85R\x93\x83R`\x02\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x83\x01\x90\x81R\x82Q\x86Q\x94\x81\x01\x94\x90\x94R\x93Q\x91\x83\x01\x91\x90\x91R\x91Q\x90\x91\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x82a\nu\x83\x86a\x0BDV[a\x07\xA0\x91\x90a\x13\x11V[`\0a\n\xA0\x83\x87\x80` \x01\x90Q\x81\x01\x90a\n\x99\x91\x90a\x13$V[\x87\x15a\x0BbV[\x97\x96PPPPPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3\x84\x86`\x01\x81Q\x81\x10a\n\xCDWa\n\xCDa\x11\xD0V[` \x02` \x01\x01Qa\x0B\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0B\x04a\n\xFD\x86\x88`\0\x81Q\x81\x10a\n\xCDWa\n\xCDa\x11\xD0V[\x85\x90a\x0BDV[a\x0B\x0E\x91\x90a\x13\x11V[a\x07\xA0\x91\x90a\x13\xA6V[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA3\x91\x90a\x13\xCDV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA3\x91\x90a\x13\xFBV[`\0a\x0BY\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xA3V[\x90P[\x92\x91PPV[`\0\x81\x15a\x0B\x80W` \x83\x01Qa\x0By\x90\x85a\x0BDV[\x90Pa\x07\xA3V[\x82Q` \x84\x01Qa\x0By\x91\x86\x90a\x0B\xA3V[`\0a\x0BY\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B\xBBW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xE6W`\0\x80\xFD[PV[\x805a\x0B\xF4\x81a\x0B\xD1V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C1Wa\x0C1a\x0B\xF9V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C1Wa\x0C1a\x0B\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x81Wa\x0C\x81a\x0B\xF9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xA2Wa\x0C\xA2a\x0B\xF9V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\xBDW`\0\x80\xFD[\x815` a\x0C\xD2a\x0C\xCD\x83a\x0C\x89V[a\x0CYV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0C\xF4W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\r\x19W\x805a\r\x0C\x81a\x0B\xD1V[\x83R\x91\x83\x01\x91\x83\x01a\x0C\xF9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\r5W`\0\x80\xFD[\x815` a\rEa\x0C\xCD\x83a\x0C\x89V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\rgW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\r\x19W\x805\x83R\x91\x83\x01\x91\x83\x01a\rlV[`\0`\xE0\x82\x84\x03\x12\x15a\r\x95W`\0\x80\xFD[a\r\x9Da\x0C\x0FV[\x90Pa\r\xA8\x82a\x0B\xE9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xC4W`\0\x80\xFD[a\r\xD0\x85\x83\x86\x01a\x0C\xACV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\r\xE9W`\0\x80\xFD[Pa\r\xF6\x84\x82\x85\x01a\r$V[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x0E\x12`\x80\x83\x01a\x0B\xE9V[`\x80\x82\x01Ra\x0E#`\xA0\x83\x01a\x0B\xE9V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0EJW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EaW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0EyW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0E\x98W`\0\x80\xFD[\x855a\x0E\xA3\x81a\x0B\xD1V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xC6W`\0\x80\xFD[a\x0E\xD2\x89\x83\x8A\x01a\r\x83V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x0E\xE8W`\0\x80\xFD[Pa\x0E\xF5\x88\x82\x89\x01a\x0E8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x0FQW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0F5V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0F\x8EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0FrV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0BY` \x83\x01\x84a\x0FhV[`\0` \x82\x84\x03\x12\x15a\x0F\xD3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0F\xF2W`\0\x80\xFD[\x855a\x0F\xFD\x81a\x0B\xD1V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10 W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x104W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x0E\xE8W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x10[W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10tWa\x10ta\x0B\xF9V[a\x10\x87`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0CYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10\x9CW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\xCFW`\0\x80\xFD[\x845a\x10\xDA\x81a\x0B\xD1V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\xFDW`\0\x80\xFD[a\x11\t\x88\x83\x89\x01a\r\x83V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x11\x1FW`\0\x80\xFD[Pa\x11,\x87\x82\x88\x01a\x10JV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11MW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11dW`\0\x80\xFD[a\x11p\x87\x83\x88\x01a\r$V[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x11\x8DW`\0\x80\xFD[Pa\x11\x9A\x86\x82\x87\x01a\x10JV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xB9W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\\Wa\x0B\\a\x11\xE6V[`\0\x80\x82\x84\x03`\x80\x81\x12\x15a\x12#W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x129W`\0\x80\xFD[a\x12E\x86\x82\x87\x01a\r$V[\x93PP```\x1F\x19\x82\x01\x12\x15a\x12ZW`\0\x80\xFD[Pa\x12ca\x0C7V[` \x84\x015\x81R`@\x84\x015` \x82\x01R``\x84\x015a\x12\x82\x81a\x0B\xD1V[`@\x82\x01R\x91\x94\x91\x93P\x90\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12\xA9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x12\xC3W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x0EyW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xF1W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0B\\Wa\x0B\\a\x11\xE6V[`\0``\x82\x84\x03\x12\x15a\x136W`\0\x80\xFD[a\x13>a\x0C7V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x13Z\x81a\x0B\xD1V[`@\x82\x01R\x93\x92PPPV[`\x04\x81\x10a\x0B\xE6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x85W`\0\x80\xFD[\x815a\x07\xA3\x81a\x13fV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13\xC6Wa\x13\xC6a\x11\xE6V[P\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xE0W`\0\x80\xFD[\x82Qa\x13\xEB\x81a\x13fV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x0EW`\0\x80\xFD[\x82Qa\x14\x19\x81a\x13fV[` \x84\x01Q\x90\x92Pa\x14*\x81a\x0B\xD1V[\x80\x91PP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 {0\xCC=\x0C\x03\xB2rD\x8A\xEF5\xBE\x13\x0E]\xFC\xCE\xD8\x15[_oT\x01zB$^T\xBCRdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUM_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x01\xBFW\x80c\x8D\xDA\0=\x14a\x01\xD2W\x80c\xAF\xBA\x13\xC4\x14a\x01\xF3W\x80c\xD8\xB5\xED\x12\x14a\x022W\x80c\xDC\x17\x83U\x14a\x02GW`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x06W\x80cO\x17\xD9\x13\x14a\x01bW\x80cu\xE6D\x0F\x14a\x01uW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x0E\x80V[a\x02ZV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x0F\x06V[`@Q\x80\x91\x03\x90\xF3[a\0\xF9`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jConstantSum`\xA8\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x0F\xAEV[a\x01>a\x01\x146`\x04a\x0F\xC1V[`\0` \x81\x90R\x90\x81R`@\x90 \x80T`\x01\x82\x01T`\x02\x90\x92\x01T\x90\x91\x90`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\0\xC6V[a\0\xB6a\x01p6`\x04a\x0F\xDAV[a\x03\xB4V[a\x01\x88a\x01\x836`\x04a\x10\xB9V[a\x05jV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x01\xCD6`\x04a\x0E\x80V[a\x06FV[a\x01\xE5a\x01\xE06`\x04a\x118V[a\x07\x7FV[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\x1A\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02Ea\x02@6`\x04a\x0F\xDAV[a\x07\xAAV[\0[a\0\xF9a\x02U6`\x04a\x0F\xC1V[a\t\xC8V[`\0\x80``\x81\x80\x80\x80a\x02o\x88\x8A\x01\x8Aa\x11\xA4V[\x92P\x92P\x92Pa\x02\x95\x83\x83`\0\x80\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x01Ta\nhV[\x93P\x83\x81\x11\x15a\x02\xB8W`@Qc\x1B\x15n\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x94P\x82\x85`\0\x81Q\x81\x10a\x02\xECWa\x02\xECa\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x81\x85`\x01\x81Q\x81\x10a\x03\x0CWa\x03\x0Ca\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x82\x8A`@\x01Q`\0\x81Q\x81\x10a\x030Wa\x030a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x03D\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q\x80Q\x83\x91\x90`\x01\x90\x81\x10a\x03bWa\x03ba\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x03v\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x03\x9C\x91\x90a\x03\x93\x90\x87\x90a\x11\xFCV[a\x01\xE0\x8Ea\t\xC8V[\x95P`\0\x86\x12\x15\x96PPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x04\x03W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x040`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x04<\x86\x88\x01\x88a\x12\x0FV[\x80\x92P\x81\x94PPPa\x04\x87\x83`\0\x81Q\x81\x10a\x04ZWa\x04Za\x11\xD0V[` \x02` \x01\x01Q\x84`\x01\x81Q\x81\x10a\x04uWa\x04ua\x11\xD0V[` \x02` \x01\x01Q\x83`\0\x01Qa\nhV[\x91Pa\x04\x96`@\x89\x01\x89a\x12\x92V[\x90P`\x02\x14\x15\x80a\x04\xA9WP\x82Q`\x02\x14\x15[\x15a\x04\xC7W`@Qcc\xFB\x1F/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80Q`\0\x8A\x81R` \x81\x81R`@\x91\x82\x90 \x92\x83U\x80\x84\x01\x80Q`\x01\x85\x01U\x82\x85\x01\x80Q`\x02\x90\x95\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x17\x90U\x83Q\x86Q\x93\x81\x01\x93\x90\x93R\x90Q\x92\x82\x01\x92\x90\x92R\x90Q\x90\x91\x16``\x82\x01Ra\x05G\x90\x84\x90\x84\x90`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@Ra\x07\x7FV[\x93P`\0\x84\x12\x15\x80\x15a\x05[WP`\x1E\x84\x13\x15[\x94PP\x95P\x95P\x95P\x95\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x05\x7F\x8Ba\t\xC8V[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x05\x95\x91\x90a\x12\xDBV[\x92\x98P\x90\x96P\x94P\x92Pa\x05\xAD\x8A\x82\x88\x88\x88\x88a\n\x7FV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x05\xC6Wa\x05\xC6a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x05\xDA\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x05\xF7Wa\x05\xF7a\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x06\x0B\x91\x90a\x11\xFCV[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x06.\x91\x90a\x06(\x90\x85\x90a\x13\x11V[\x83a\x07\x7FV[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80\x80a\x06[\x88\x8A\x01\x8Aa\x11\xA4V[\x92P\x92P\x92Pa\x06\x81\x83\x83`\0\x80\x8F\x81R` \x01\x90\x81R` \x01`\0 `\0\x01Ta\nhV[\x93P\x80\x84\x10\x15a\x06\xA4W`@Qc\x1B\x15n\xCD`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x94P\x82\x85`\0\x81Q\x81\x10a\x06\xD8Wa\x06\xD8a\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x81\x85`\x01\x81Q\x81\x10a\x06\xF8Wa\x06\xF8a\x11\xD0V[` \x02` \x01\x01\x81\x81RPP\x82\x8A`@\x01Q`\0\x81Q\x81\x10a\x07\x1CWa\x07\x1Ca\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x070\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q\x80Q\x83\x91\x90`\x01\x90\x81\x10a\x07NWa\x07Na\x11\xD0V[` \x02` \x01\x01\x81\x81Qa\x07b\x91\x90a\x13\x11V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x03\x9C\x91\x90a\x03\x93\x90\x87\x90a\x13\x11V[`\0a\x07\xA0\x84\x84\x84\x80` \x01\x90Q\x81\x01\x90a\x07\x9A\x91\x90a\x13$V[Qa\n\xABV[\x90P[\x93\x92PPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07\xF3W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x080W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x08>\x82\x84\x01\x84a\x13sV[\x90P`\x02\x81`\x03\x81\x11\x15a\x08TWa\x08Ta\x13\x90V[\x03a\x08\xACWa\x08\x98\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x18\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 Ua\t\xC0V[`\x01\x81`\x03\x81\x11\x15a\x08\xC0Wa\x08\xC0a\x13\x90V[\x03a\t\x1BWa\t\x04\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B\x18\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x01\x01Ua\t\xC0V[`\x03\x81`\x03\x81\x11\x15a\t/Wa\t/a\x13\x90V[\x03a\t\xA7Wa\ts\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x0B.\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\t\xC0V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\t\xF7`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x81R`@\x80\x83 \x80T\x85R`\x01\x81\x01T\x85\x84\x01\x90\x81R\x87\x85R\x93\x83R`\x02\x01T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x85\x83\x01\x90\x81R\x82Q\x86Q\x94\x81\x01\x94\x90\x94R\x93Q\x91\x83\x01\x91\x90\x91R\x91Q\x90\x91\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x82a\nu\x83\x86a\x0BDV[a\x07\xA0\x91\x90a\x13\x11V[`\0a\n\xA0\x83\x87\x80` \x01\x90Q\x81\x01\x90a\n\x99\x91\x90a\x13$V[\x87\x15a\x0BbV[\x97\x96PPPPPPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\n\xE3\x84\x86`\x01\x81Q\x81\x10a\n\xCDWa\n\xCDa\x11\xD0V[` \x02` \x01\x01Qa\x0B\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0B\x04a\n\xFD\x86\x88`\0\x81Q\x81\x10a\n\xCDWa\n\xCDa\x11\xD0V[\x85\x90a\x0BDV[a\x0B\x0E\x91\x90a\x13\x11V[a\x07\xA0\x91\x90a\x13\xA6V[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA3\x91\x90a\x13\xCDV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x07\xA3\x91\x90a\x13\xFBV[`\0a\x0BY\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0B\xA3V[\x90P[\x92\x91PPV[`\0\x81\x15a\x0B\x80W` \x83\x01Qa\x0By\x90\x85a\x0BDV[\x90Pa\x07\xA3V[\x82Q` \x84\x01Qa\x0By\x91\x86\x90a\x0B\xA3V[`\0a\x0BY\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0B\xBBW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xE6W`\0\x80\xFD[PV[\x805a\x0B\xF4\x81a\x0B\xD1V[\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C1Wa\x0C1a\x0B\xF9V[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C1Wa\x0C1a\x0B\xF9V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x0C\x81Wa\x0C\x81a\x0B\xF9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x0C\xA2Wa\x0C\xA2a\x0B\xF9V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\xBDW`\0\x80\xFD[\x815` a\x0C\xD2a\x0C\xCD\x83a\x0C\x89V[a\x0CYV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0C\xF4W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\r\x19W\x805a\r\x0C\x81a\x0B\xD1V[\x83R\x91\x83\x01\x91\x83\x01a\x0C\xF9V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\r5W`\0\x80\xFD[\x815` a\rEa\x0C\xCD\x83a\x0C\x89V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\rgW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\r\x19W\x805\x83R\x91\x83\x01\x91\x83\x01a\rlV[`\0`\xE0\x82\x84\x03\x12\x15a\r\x95W`\0\x80\xFD[a\r\x9Da\x0C\x0FV[\x90Pa\r\xA8\x82a\x0B\xE9V[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\r\xC4W`\0\x80\xFD[a\r\xD0\x85\x83\x86\x01a\x0C\xACV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\r\xE9W`\0\x80\xFD[Pa\r\xF6\x84\x82\x85\x01a\r$V[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x0E\x12`\x80\x83\x01a\x0B\xE9V[`\x80\x82\x01Ra\x0E#`\xA0\x83\x01a\x0B\xE9V[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0EJW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0EaW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0EyW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0E\x98W`\0\x80\xFD[\x855a\x0E\xA3\x81a\x0B\xD1V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x0E\xC6W`\0\x80\xFD[a\x0E\xD2\x89\x83\x8A\x01a\r\x83V[\x94P``\x88\x015\x91P\x80\x82\x11\x15a\x0E\xE8W`\0\x80\xFD[Pa\x0E\xF5\x88\x82\x89\x01a\x0E8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x0FQW\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x0F5V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0F\x8EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0FrV[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x0BY` \x83\x01\x84a\x0FhV[`\0` \x82\x84\x03\x12\x15a\x0F\xD3W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x0F\xF2W`\0\x80\xFD[\x855a\x0F\xFD\x81a\x0B\xD1V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10 W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x104W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x0E\xE8W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a\x10[W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x10tWa\x10ta\x0B\xF9V[a\x10\x87`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0CYV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x10\x9CW`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x10\xCFW`\0\x80\xFD[\x845a\x10\xDA\x81a\x0B\xD1V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x10\xFDW`\0\x80\xFD[a\x11\t\x88\x83\x89\x01a\r\x83V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x11\x1FW`\0\x80\xFD[Pa\x11,\x87\x82\x88\x01a\x10JV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11MW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x11dW`\0\x80\xFD[a\x11p\x87\x83\x88\x01a\r$V[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x11\x8DW`\0\x80\xFD[Pa\x11\x9A\x86\x82\x87\x01a\x10JV[\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x11\xB9W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0B\\Wa\x0B\\a\x11\xE6V[`\0\x80\x82\x84\x03`\x80\x81\x12\x15a\x12#W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x81\x11\x15a\x129W`\0\x80\xFD[a\x12E\x86\x82\x87\x01a\r$V[\x93PP```\x1F\x19\x82\x01\x12\x15a\x12ZW`\0\x80\xFD[Pa\x12ca\x0C7V[` \x84\x015\x81R`@\x84\x015` \x82\x01R``\x84\x015a\x12\x82\x81a\x0B\xD1V[`@\x82\x01R\x91\x94\x91\x93P\x90\x91PPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x12\xA9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x12\xC3W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x0EyW`\0\x80\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x12\xF1W`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[\x80\x82\x01\x80\x82\x11\x15a\x0B\\Wa\x0B\\a\x11\xE6V[`\0``\x82\x84\x03\x12\x15a\x136W`\0\x80\xFD[a\x13>a\x0C7V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x13Z\x81a\x0B\xD1V[`@\x82\x01R\x93\x92PPPV[`\x04\x81\x10a\x0B\xE6W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x13\x85W`\0\x80\xFD[\x815a\x07\xA3\x81a\x13fV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x13\xC6Wa\x13\xC6a\x11\xE6V[P\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x13\xE0W`\0\x80\xFD[\x82Qa\x13\xEB\x81a\x13fV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14\x0EW`\0\x80\xFD[\x82Qa\x14\x19\x81a\x13fV[` \x84\x01Q\x90\x92Pa\x14*\x81a\x0B\xD1V[\x80\x91PP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 {0\xCC=\x0C\x03\xB2rD\x8A\xEF5\xBE\x13\x0E]\xFC\xCE\xD8\x15[_oT\x01zB$^T\xBCRdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static CONSTANTSUM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ConstantSum<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConstantSum<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConstantSum<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConstantSum<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConstantSum<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConstantSum))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConstantSum<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CONSTANTSUM_ABI.clone(),
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
                CONSTANTSUM_ABI.clone(),
                CONSTANTSUM_BYTECODE.clone().into(),
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
                ::ethers::core::types::U256,
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ConstantSum<M> {
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
    /// Custom Error type `InvalidDeltaLiquidity` with signature
    /// `InvalidDeltaLiquidity()` and selector `0x1b156ecd`
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
    #[etherror(name = "InvalidDeltaLiquidity", abi = "InvalidDeltaLiquidity()")]
    pub struct InvalidDeltaLiquidity;
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
    pub enum ConstantSumErrors {
        DeltaError(DeltaError),
        InvalidDeltaLiquidity(InvalidDeltaLiquidity),
        InvalidReservesLength(InvalidReservesLength),
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumErrors {
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
                <InvalidDeltaLiquidity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidDeltaLiquidity(decoded));
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
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantSumErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeltaError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidDeltaLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReservesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ConstantSumErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DeltaError as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidDeltaLiquidity as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
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
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeltaError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidDeltaLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReservesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ConstantSumErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeltaError> for ConstantSumErrors {
        fn from(value: DeltaError) -> Self {
            Self::DeltaError(value)
        }
    }
    impl ::core::convert::From<InvalidDeltaLiquidity> for ConstantSumErrors {
        fn from(value: InvalidDeltaLiquidity) -> Self {
            Self::InvalidDeltaLiquidity(value)
        }
    }
    impl ::core::convert::From<InvalidReservesLength> for ConstantSumErrors {
        fn from(value: InvalidReservesLength) -> Self {
            Self::InvalidReservesLength(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for ConstantSumErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for ConstantSumErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for ConstantSumErrors {
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
    pub enum ConstantSumCalls {
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
    impl ::ethers::core::abi::AbiDecode for ConstantSumCalls {
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
    impl ::ethers::core::abi::AbiEncode for ConstantSumCalls {
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
    impl ::core::fmt::Display for ConstantSumCalls {
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
    impl ::core::convert::From<DfmmCall> for ConstantSumCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for ConstantSumCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for ConstantSumCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for ConstantSumCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for ConstantSumCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<TradingFunctionCall> for ConstantSumCalls {
        fn from(value: TradingFunctionCall) -> Self {
            Self::TradingFunction(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for ConstantSumCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateCall> for ConstantSumCalls {
        fn from(value: ValidateAllocateCall) -> Self {
            Self::ValidateAllocate(value)
        }
    }
    impl ::core::convert::From<ValidateDeallocateCall> for ConstantSumCalls {
        fn from(value: ValidateDeallocateCall) -> Self {
            Self::ValidateDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for ConstantSumCalls {
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
        pub price: ::ethers::core::types::U256,
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

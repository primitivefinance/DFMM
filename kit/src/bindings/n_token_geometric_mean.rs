pub use n_token_geometric_mean::*;
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
pub mod n_token_geometric_mean {
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
                                name: ::std::borrow::ToOwned::to_owned("tokenDeltas"),
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
                                name: ::std::borrow::ToOwned::to_owned("tokenDeltas"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidConfiguration"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidConfiguration",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reservesLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("weightsLength"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidTokenDeltas"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTokenDeltas"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidWeightUpdateLength"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidWeightUpdateLength",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("updateLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("weightsLength"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWeights"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidWeights"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("totalWeight"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
    pub static NTOKENGEOMETRICMEAN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0\"\xAB8\x03\x80b\0\"\xAB\x839\x81\x01`@\x81\x90Ra\x001\x91a\0BV[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0rV[`\0` \x82\x84\x03\x12\x15a\0TW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0kW`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\"\x0Fb\0\0\x9C`\09`\0\x81\x81a\x01\xF4\x01R\x81\x81a\x03\x04\x01Ra\x07\x16\x01Ra\"\x0F`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x01\xBBW\x80c\x8D\xDA\0=\x14a\x01\xCEW\x80c\xAF\xBA\x13\xC4\x14a\x01\xEFW\x80c\xD8\xB5\xED\x12\x14a\x02.W\x80c\xDC\x17\x83U\x14a\x02CW`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x0EW\x80cO\x17\xD9\x13\x14a\x01^W\x80cu\xE6D\x0F\x14a\x01qW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x17\xCEV[a\x02VV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x18\x86V[`@Q\x80\x91\x03\x90\xF3[a\x01\x01`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r'*7\xB5\xB2\xB7#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`i\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x19.V[a\x01Aa\x01\x1C6`\x04a\x19AV[`\0` \x81\x90R\x90\x81R`@\x90 `\x01\x81\x01T`\x02\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x82V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01a\0\xC6V[a\0\xB6a\x01l6`\x04a\x17\xCEV[a\x02\xF3V[a\x01\x84a\x01\x7F6`\x04a\x1B\xE6V[a\x05\x8CV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x01\xC96`\x04a\x17\xCEV[a\x06hV[a\x01\xE1a\x01\xDC6`\x04a\x1CeV[a\x06\xE3V[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02Aa\x02<6`\x04a\x17\xCEV[a\x07\x0BV[\0[a\x01\x01a\x02Q6`\x04a\x19AV[a\nNV[`\0\x80``\x81\x80\x80a\x02j\x87\x89\x01\x89a\x1C\xD1V[\x90\x92P\x90Pa\x02|`@\x8A\x01\x8Aa\x1D\x15V[\x90P\x82Q\x14a\x02\x9EW`@Qc+?q5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P\x81`\0\x80a\x02\xB7\x83\x85a\x02\xB2\x8Ea\x1DeV[a\x0B\xC5V[\x90\x96P\x86\x92P\x90Pa\x02\xDA\x81a\x02\xD1\x87``\x8F\x015a\x1D\x87V[a\x01\xDC\x8Fa\nNV[\x96P`\0\x87\x12\x15\x97PPPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03BW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x8D`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x03\x99\x86\x88\x01\x88a\x1D\x9AV[`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0\x84\x01\x81\x90R`\x80\x84\x01\x91\x90\x91R`\xA0\x83\x01\x82\x90RQ\x90Q\x14a\x03\xFDW`\xA0\x81\x01QQ`\xC0\x82\x01QQ`@Qc\x04\xC7\x18A`\xE4\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x82`\xC0\x01QQ\x81\x10\x15a\x04\xD0W\x82`\xC0\x01Q\x81\x81Q\x81\x10a\x04$Wa\x04$a\x1E%V[` \x02` \x01\x01Q\x82a\x047\x91\x90a\x1E;V[\x91P`\0\x80\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x80\x85`\xC0\x01Q\x84\x81Q\x81\x10a\x04oWa\x04oa\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R`\0\x82\x82\x01\x81\x90R`@\x80\x84\x01\x82\x90R``\x93\x84\x01\x82\x90R\x85T`\x01\x81\x81\x01\x88U\x96\x83R\x91\x83\x90 \x85Q`\x04\x90\x93\x02\x01\x91\x82U\x91\x84\x01Q\x81\x86\x01U\x90\x83\x01Q`\x02\x82\x01U\x91\x01Q`\x03\x90\x91\x01U\x01a\x04\x01V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x04\xFCW`@Qc>3\x1D\xC9`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[``\x82\x01Q`\0\x8B\x81R` \x81\x90R`@\x80\x82 `\x01\x81\x01\x93\x90\x93U\x84\x01Q`\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U`\xA0\x83\x01Q`\x80\x84\x01Qa\x05V\x91\x90a\x01\xDC\x8Ea\nNV[\x90P`\0\x80\x82\x12\x15\x80\x15a\x05kWP`\x1E\x82\x13\x15[`\xA0\x85\x01Q`\x80\x90\x95\x01Q\x90\x9E\x92\x9DP\x93\x9BP\x92\x99P\x97PPPPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x05\xA1\x8Ba\nNV[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a\x1ENV[\x92\x98P\x90\x96P\x94P\x92Pa\x05\xCF\x8A\x82\x88\x88\x88\x88a\r\x9FV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a\x1E%V[` \x02` \x01\x01\x81\x81Qa\x05\xFC\x91\x90a\x1E;V[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x06\x19Wa\x06\x19a\x1E%V[` \x02` \x01\x01\x81\x81Qa\x06-\x91\x90a\x1D\x87V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x06P\x91\x90a\x06J\x90\x85\x90a\x1E;V[\x83a\x06\xE3V[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80a\x06|\x87\x89\x01\x89a\x1C\xD1V[\x90\x92P\x90Pa\x06\x8E`@\x8A\x01\x8Aa\x1D\x15V[\x90P\x82Q\x14a\x06\xB0W`@Qc+?q5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P\x81`\0\x80a\x06\xC9\x83\x85a\x06\xC4\x8Ea\x1DeV[a\x0E\x13V[\x90\x96P\x86\x92P\x90Pa\x02\xDA\x81a\x02\xD1\x87``\x8F\x015a\x1E;V[`\0a\x07\x03\x84\x84\x84\x80` \x01\x90Q\x81\x01\x90a\x06\xFE\x91\x90a\x1E\xE3V[a\x0F\xBBV[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07TW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x07\x91W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x9F\x82\x84\x01\x84a\x1F\x93V[\x90P`\x01\x81`\x03\x81\x11\x15a\x07\xB5Wa\x07\xB5a\x1F\xB0V[\x03a\x08\x10Wa\x07\xF9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10Y\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x01\x01Ua\nFV[`\x02\x81`\x03\x81\x11\x15a\x08$Wa\x08$a\x1F\xB0V[\x03a\t\xA1W`\0\x80a\x08k\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10p\x92PPPV[`\0\x89\x81R` \x81\x90R`@\x90 T\x82Q\x92\x94P\x90\x92P\x14a\x08\xBBW\x81Q`\0\x88\x81R` \x81\x90R`@\x90\x81\x90 T\x90Qc\xAC\x18\x83\x95`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x03\xF4V[`\0\x80[\x83Q\x81\x10\x15a\x08\xF7W\x83\x81\x81Q\x81\x10a\x08\xDAWa\x08\xDAa\x1E%V[` \x02` \x01\x01Q\x82a\x08\xED\x91\x90a\x1E;V[\x91P`\x01\x01a\x08\xBFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\t#W`@Qc>3\x1D\xC9`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[`\0[\x83Q\x81\x10\x15a\t\x98Wa\t\x90\x84\x82\x81Q\x81\x10a\tDWa\tDa\x1E%V[` \x02` \x01\x01Q\x84`\0\x80\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x84\x81T\x81\x10a\tsWa\tsa\x1E%V[\x90`\0R` `\0 \x90`\x04\x02\x01a\x10\x93\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\t&V[PPPPa\nFV[`\x03\x81`\x03\x81\x11\x15a\t\xB5Wa\t\xB5a\x1F\xB0V[\x03a\n-Wa\t\xF9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11Y\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\nFV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\n}`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xA3Wa\n\xA3a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`\0[\x81QQ\x81\x10\x15a\x0BnW`\0\x84\x81R` \x81\x90R`@\x90 \x80Ta\x0BG\x91\x90\x83\x90\x81\x10a\x0B\x01Wa\x0B\x01a\x1E%V[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x11vV[\x82Q\x80Q\x83\x90\x81\x10a\x0B[Wa\x0B[a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xD2V[P`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01\x81\x01T\x85\x84\x01R\x86\x84R\x92\x82R`\x02\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x01R\x90Qa\x0B\xAE\x91\x83\x91\x01a\x1F\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x80\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xE5Wa\x0B\xE5a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x0EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C.Wa\x0C.a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CWW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83`@\x01QQ\x81\x10\x15a\r\x96W`\0\x84`@\x01Q\x82\x81Q\x81\x10a\x0C\x82Wa\x0C\x82a\x1E%V[` \x02` \x01\x01Q\x90Pa\x0C\x9B\x81\x88\x87``\x01Qa\x12\x07V[\x84\x83\x81Q\x81\x10a\x0C\xADWa\x0C\xADa\x1E%V[` \x02` \x01\x01\x81\x81RPP\x83\x82\x81Q\x81\x10a\x0C\xCBWa\x0C\xCBa\x1E%V[` \x02` \x01\x01Q\x86\x83\x81Q\x81\x10a\x0C\xE5Wa\x0C\xE5a\x1E%V[` \x02` \x01\x01Q\x11\x15a\rKW\x85\x82\x81Q\x81\x10a\r\x05Wa\r\x05a\x1E%V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a\r\x1FWa\r\x1Fa\x1E%V[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x83\x82\x81Q\x81\x10a\r]Wa\r]a\x1E%V[` \x02` \x01\x01Q\x81a\rp\x91\x90a\x1D\x87V[\x83\x83\x81Q\x81\x10a\r\x82Wa\r\x82a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C]V[P\x93P\x93\x91PPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r\xB6\x91\x90a\x1E\xE3V[\x90Pa\x0E\x07\x84\x89`@\x01Q\x88\x81Q\x81\x10a\r\xD2Wa\r\xD2a\x1E%V[` \x02` \x01\x01Q\x8A``\x01Q\x84`\0\x01Q\x8A\x81Q\x81\x10a\r\xF5Wa\r\xF5a\x1E%V[` \x02` \x01\x01Q\x85` \x01Qa\x12\x1DV[\x98\x97PPPPPPPPV[``\x80\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E3Wa\x0E3a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\\W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E|Wa\x0E|a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xA5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83`@\x01QQ\x81\x10\x15a\r\x96W`\0\x84`@\x01Q\x82\x81Q\x81\x10a\x0E\xD0Wa\x0E\xD0a\x1E%V[` \x02` \x01\x01Q\x90Pa\x0F\x06\x85`@\x01Q\x83\x81Q\x81\x10a\x0E\xF3Wa\x0E\xF3a\x1E%V[` \x02` \x01\x01Q\x88\x87``\x01Qa\x12HV[\x84\x83\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a\x1E%V[` \x02` \x01\x01\x81\x81RPP\x85\x82\x81Q\x81\x10a\x0F6Wa\x0F6a\x1E%V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a\x0FPWa\x0FPa\x1E%V[` \x02` \x01\x01Q\x11\x15a\x0FpW\x85\x82\x81Q\x81\x10a\r\x05Wa\r\x05a\x1E%V[\x83\x82\x81Q\x81\x10a\x0F\x82Wa\x0F\x82a\x1E%V[` \x02` \x01\x01Q\x81a\x0F\x95\x91\x90a\x1E;V[\x83\x83\x81Q\x81\x10a\x0F\xA7Wa\x0F\xA7a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\xABV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81[\x85Q\x81\x10\x15a\x10=W`\0a\x10&\x85`\0\x01Q\x83\x81Q\x81\x10a\x0F\xECWa\x0F\xECa\x1E%V[` \x02` \x01\x01Qa\x10 \x88\x8A\x86\x81Q\x81\x10a\x10\nWa\x10\na\x1E%V[` \x02` \x01\x01Qa\x12^\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x12|V[\x90Pa\x102\x83\x82a\x12\xADV[\x92PP`\x01\x01a\x0F\xC8V[Pa\x10Pg\r\xE0\xB6\xB3\xA7d\0\0\x82a 6V[\x95\x94PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x07\x03\x91\x90a ]V[```\0\x82\x80` \x01\x90Q\x81\x01\x90a\x10\x88\x91\x90a \x8BV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x10\xB3W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xBC\x83a\x12\xC2V[`\0a\x10\xC8B\x83a\x1D\x87V[\x84T\x90\x91P`\0\x90a\x10\xDA\x90\x85a 6V[\x90P`\0a\x10\xE8\x83\x83a \xFAV[\x90P`\0a\x10\xF6\x84\x84a!(V[\x90P`\0\x81\x13\x15a\x11 W\x80\x87`\0\x01`\0\x82\x82Ta\x11\x15\x91\x90a\x1E;V[\x90\x91UPa\x11B\x90PV[a\x11)\x81a!<V[\x87`\0\x01`\0\x82\x82Ta\x11<\x91\x90a\x1D\x87V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x11o\x91\x90a!XV[\x93\x92PPPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x11\x8CWPQ\x90V[`\0\x82` \x01QB\x11a\x11\x9FWBa\x11\xA5V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x11\xB9\x91\x90a\x1D\x87V[\x90P`\0\x84`@\x01Q\x13\x15a\x11\xE3W`@\x84\x01Qa\x11\xD7\x90\x82a!\x92V[\x84Qa\x07\x03\x91\x90a\x1E;V[\x83`@\x01Qa\x11\xF1\x90a!<V[a\x11\xFB\x90\x82a!\x92V[\x84Qa\x07\x03\x91\x90a\x1D\x87V[`\0a\x07\x03a\x12\x16\x84\x84a\x12^V[\x85\x90a\x13\x04V[`\0a\x12>a\x12,\x87\x87a\x12^V[a\x128\x86\x81\x87\x87a\x13\x04V[\x90a\x13\x04V[\x96\x95PPPPPPV[`\0a\x07\x03a\x12W\x84\x84a\x13\x19V[\x85\x90a\x12\xADV[`\0a\x12s\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13.V[\x90P[\x92\x91PPV[`\0a\x12sg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x12\x94\x86a\x13MV[a\x12\x9E\x91\x90a!\xA9V[a\x12\xA8\x91\x90a \xFAV[a\x15(V[`\0a\x12s\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xD1V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x12\xF9\x90a\x11vV[\x81UB`\x03\x90\x91\x01UV[`\0a\x12s\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13.V[`\0a\x12s\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xD1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\x13\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\xF4V[`\0``a\x13\x97\x84a\x16\xFFV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15CWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\xF4V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xE9W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x17<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\xF4V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xBBW`\0\x80\xFD[PV[\x805a\x17\xC9\x81a\x17\xA6V[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17\xE6W`\0\x80\xFD[\x855a\x17\xF1\x81a\x17\xA6V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x14W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x18(W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x18>W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x18RW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18aW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x18sW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x18\xD1W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x18\xB5V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\x0EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xF2V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12s` \x83\x01\x84a\x18\xE8V[`\0` \x82\x84\x03\x12\x15a\x19SW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\x92Wa\x19\x92a\x19ZV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\xC0Wa\x19\xC0a\x19ZV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xE1Wa\x19\xE1a\x19ZV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x19\xFCW`\0\x80\xFD[\x815` a\x1A\x11a\x1A\x0C\x83a\x19\xC8V[a\x19\x98V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A3W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x805a\x1AK\x81a\x17\xA6V[\x83R\x91\x83\x01\x91\x83\x01a\x1A8V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x1AtW`\0\x80\xFD[\x815` a\x1A\x84a\x1A\x0C\x83a\x19\xC8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A\xA6W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A\xABV[`\0`\xE0\x82\x84\x03\x12\x15a\x1A\xD4W`\0\x80\xFD[a\x1A\xDCa\x19pV[\x90Pa\x1A\xE7\x82a\x17\xBEV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1B\x03W`\0\x80\xFD[a\x1B\x0F\x85\x83\x86\x01a\x19\xEBV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x1B(W`\0\x80\xFD[Pa\x1B5\x84\x82\x85\x01a\x1AcV[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x1BQ`\x80\x83\x01a\x17\xBEV[`\x80\x82\x01Ra\x1Bb`\xA0\x83\x01a\x17\xBEV[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x1B\x88W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xA1Wa\x1B\xA1a\x19ZV[a\x1B\xB4`\x1F\x82\x01`\x1F\x19\x16` \x01a\x19\x98V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1B\xC9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1B\xFCW`\0\x80\xFD[\x845a\x1C\x07\x81a\x17\xA6V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C*W`\0\x80\xFD[a\x1C6\x88\x83\x89\x01a\x1A\xC2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1CLW`\0\x80\xFD[Pa\x1CY\x87\x82\x88\x01a\x1BwV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1CzW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C\x91W`\0\x80\xFD[a\x1C\x9D\x87\x83\x88\x01a\x1AcV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1C\xBAW`\0\x80\xFD[Pa\x1C\xC7\x86\x82\x87\x01a\x1BwV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xE4W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xFAW`\0\x80\xFD[a\x1D\x06\x85\x82\x86\x01a\x1AcV[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1D,W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DFW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1D^W`\0\x80\xFD[\x92P\x92\x90PV[`\0a\x12v6\x83a\x1A\xC2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12vWa\x12va\x1DqV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1D\xB2W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\xC9W`\0\x80\xFD[a\x1D\xD5\x89\x83\x8A\x01a\x1AcV[\x96P` \x88\x015\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x1D\xF2W`\0\x80\xFD[Pa\x1D\xFF\x88\x82\x89\x01a\x1AcV[\x93PP``\x86\x015\x91P`\x80\x86\x015a\x1E\x17\x81a\x17\xA6V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12vWa\x12va\x1DqV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1EdW`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1E\x95W`\0\x80\xFD[\x81Q` a\x1E\xA5a\x1A\x0C\x83a\x19\xC8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1E\xC7W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x1E\xCCV[`\0` \x82\x84\x03\x12\x15a\x1E\xF5W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x1F W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x1F;Wa\x1F;a\x19ZV[`@R\x82Q\x82\x81\x11\x15a\x1FMW`\0\x80\xFD[a\x1FY\x87\x82\x86\x01a\x1E\x84V[\x82RP` \x83\x01Q` \x82\x01R`@\x83\x01Q\x92Pa\x1Fv\x83a\x17\xA6V[`@\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\x04\x81\x10a\x17\xBBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1F\xA5W`\0\x80\xFD[\x815a\x11o\x81a\x1F\x86V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a \x0BW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1F\xEBV[P\x92\x86\x01Q`@\x86\x81\x01\x91\x90\x91R\x90\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x90\x94\x01\x93\x90\x93R\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a VWa Va\x1DqV[P\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a pW`\0\x80\xFD[\x82Qa {\x81a\x1F\x86V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a \xA0W`\0\x80\xFD[\x83Qa \xAB\x81a\x1F\x86V[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xC7W`\0\x80\xFD[a \xD3\x86\x82\x87\x01a\x1E\x84V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!\tWa!\ta \xE4V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a!#Wa!#a\x1DqV[P\x05\x90V[`\0\x82a!7Wa!7a \xE4V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a!QWa!Qa\x1DqV[P`\0\x03\x90V[`\0\x80`@\x83\x85\x03\x12\x15a!kW`\0\x80\xFD[\x82Qa!v\x81a\x1F\x86V[` \x84\x01Q\x90\x92Pa!\x87\x81a\x17\xA6V[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12vWa\x12va\x1DqV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a!\xC5Wa!\xC5a\x1DqV[\x81\x81\x05\x83\x14\x82\x15\x17a\x12vWa\x12va\x1DqV\xFE\xA2dipfsX\"\x12 \xF6\x83\x9E\xBF\xEC\x85\x07)\xCD\xDF|\xA6\x941\x83F<\xB2\xD1i\x13\xE0=e\xB2kV\xB7\xF6\xE66\xE5dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static NTOKENGEOMETRICMEAN_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9EW`\x005`\xE0\x1C\x80c|\x10\x12D\x11a\0fW\x80c|\x10\x12D\x14a\x01\xBBW\x80c\x8D\xDA\0=\x14a\x01\xCEW\x80c\xAF\xBA\x13\xC4\x14a\x01\xEFW\x80c\xD8\xB5\xED\x12\x14a\x02.W\x80c\xDC\x17\x83U\x14a\x02CW`\0\x80\xFD[\x80c\x04\r\x95\x1E\x14a\0\xA3W\x80c\x06\xFD\xDE\x03\x14a\0\xCFW\x80c\x1E\xDBq\xE5\x14a\x01\x0EW\x80cO\x17\xD9\x13\x14a\x01^W\x80cu\xE6D\x0F\x14a\x01qW[`\0\x80\xFD[a\0\xB6a\0\xB16`\x04a\x17\xCEV[a\x02VV[`@Qa\0\xC6\x94\x93\x92\x91\x90a\x18\x86V[`@Q\x80\x91\x03\x90\xF3[a\x01\x01`@Q\x80`@\x01`@R\x80`\x13\x81R` \x01r'*7\xB5\xB2\xB7#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`i\x1B\x81RP\x81V[`@Qa\0\xC6\x91\x90a\x19.V[a\x01Aa\x01\x1C6`\x04a\x19AV[`\0` \x81\x90R\x90\x81R`@\x90 `\x01\x81\x01T`\x02\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x82V[`@\x80Q\x92\x83R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16` \x83\x01R\x01a\0\xC6V[a\0\xB6a\x01l6`\x04a\x17\xCEV[a\x02\xF3V[a\x01\x84a\x01\x7F6`\x04a\x1B\xE6V[a\x05\x8CV[`@\x80Q\x97\x15\x15\x88R` \x88\x01\x96\x90\x96R\x94\x86\x01\x93\x90\x93R``\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01R`\xC0\x82\x01R`\xE0\x01a\0\xC6V[a\0\xB6a\x01\xC96`\x04a\x17\xCEV[a\x06hV[a\x01\xE1a\x01\xDC6`\x04a\x1CeV[a\x06\xE3V[`@Q\x90\x81R` \x01a\0\xC6V[a\x02\x16\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xC6V[a\x02Aa\x02<6`\x04a\x17\xCEV[a\x07\x0BV[\0[a\x01\x01a\x02Q6`\x04a\x19AV[a\nNV[`\0\x80``\x81\x80\x80a\x02j\x87\x89\x01\x89a\x1C\xD1V[\x90\x92P\x90Pa\x02|`@\x8A\x01\x8Aa\x1D\x15V[\x90P\x82Q\x14a\x02\x9EW`@Qc+?q5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P\x81`\0\x80a\x02\xB7\x83\x85a\x02\xB2\x8Ea\x1DeV[a\x0B\xC5V[\x90\x96P\x86\x92P\x90Pa\x02\xDA\x81a\x02\xD1\x87``\x8F\x015a\x1D\x87V[a\x01\xDC\x8Fa\nNV[\x96P`\0\x87\x12\x15\x97PPPPP\x95P\x95P\x95P\x95\x91PPV[`\0\x80``\x813`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x03BW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\x8D`@Q\x80`\xE0\x01`@R\x80`\0\x15\x15\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01``\x81R` \x01``\x81RP\x90V[a\x03\x99\x86\x88\x01\x88a\x1D\x9AV[`\x01`\x01`\xA0\x1B\x03\x16`@\x86\x01R``\x85\x01R`\xC0\x84\x01\x81\x90R`\x80\x84\x01\x91\x90\x91R`\xA0\x83\x01\x82\x90RQ\x90Q\x14a\x03\xFDW`\xA0\x81\x01QQ`\xC0\x82\x01QQ`@Qc\x04\xC7\x18A`\xE4\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x82`\xC0\x01QQ\x81\x10\x15a\x04\xD0W\x82`\xC0\x01Q\x81\x81Q\x81\x10a\x04$Wa\x04$a\x1E%V[` \x02` \x01\x01Q\x82a\x047\x91\x90a\x1E;V[\x91P`\0\x80\x8C\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x80\x85`\xC0\x01Q\x84\x81Q\x81\x10a\x04oWa\x04oa\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R`\0\x82\x82\x01\x81\x90R`@\x80\x84\x01\x82\x90R``\x93\x84\x01\x82\x90R\x85T`\x01\x81\x81\x01\x88U\x96\x83R\x91\x83\x90 \x85Q`\x04\x90\x93\x02\x01\x91\x82U\x91\x84\x01Q\x81\x86\x01U\x90\x83\x01Q`\x02\x82\x01U\x91\x01Q`\x03\x90\x91\x01U\x01a\x04\x01V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\x04\xFCW`@Qc>3\x1D\xC9`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[``\x82\x01Q`\0\x8B\x81R` \x81\x90R`@\x80\x82 `\x01\x81\x01\x93\x90\x93U\x84\x01Q`\x02\x90\x92\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x90\x91U`\xA0\x83\x01Q`\x80\x84\x01Qa\x05V\x91\x90a\x01\xDC\x8Ea\nNV[\x90P`\0\x80\x82\x12\x15\x80\x15a\x05kWP`\x1E\x82\x13\x15[`\xA0\x85\x01Q`\x80\x90\x95\x01Q\x90\x9E\x92\x9DP\x93\x9BP\x92\x99P\x97PPPPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80a\x05\xA1\x8Ba\nNV[\x90P\x88\x80` \x01\x90Q\x81\x01\x90a\x05\xB7\x91\x90a\x1ENV[\x92\x98P\x90\x96P\x94P\x92Pa\x05\xCF\x8A\x82\x88\x88\x88\x88a\r\x9FV[\x91P\x83\x8A`@\x01Q\x87\x81Q\x81\x10a\x05\xE8Wa\x05\xE8a\x1E%V[` \x02` \x01\x01\x81\x81Qa\x05\xFC\x91\x90a\x1E;V[\x90RP`@\x8A\x01Q\x80Q\x84\x91\x90\x87\x90\x81\x10a\x06\x19Wa\x06\x19a\x1E%V[` \x02` \x01\x01\x81\x81Qa\x06-\x91\x90a\x1D\x87V[\x90RP`@\x8A\x01Q``\x8B\x01Qa\x06P\x91\x90a\x06J\x90\x85\x90a\x1E;V[\x83a\x06\xE3V[\x96P`\0\x87\x12\x15\x97PP\x94\x99P\x94\x99\x92\x97P\x94P\x94PV[`\0\x80``\x81\x80\x80a\x06|\x87\x89\x01\x89a\x1C\xD1V[\x90\x92P\x90Pa\x06\x8E`@\x8A\x01\x8Aa\x1D\x15V[\x90P\x82Q\x14a\x06\xB0W`@Qc+?q5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x91P\x81`\0\x80a\x06\xC9\x83\x85a\x06\xC4\x8Ea\x1DeV[a\x0E\x13V[\x90\x96P\x86\x92P\x90Pa\x02\xDA\x81a\x02\xD1\x87``\x8F\x015a\x1E;V[`\0a\x07\x03\x84\x84\x84\x80` \x01\x90Q\x81\x01\x90a\x06\xFE\x91\x90a\x1E\xE3V[a\x0F\xBBV[\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x07TW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R` \x81\x90R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x07\x91W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x9F\x82\x84\x01\x84a\x1F\x93V[\x90P`\x01\x81`\x03\x81\x11\x15a\x07\xB5Wa\x07\xB5a\x1F\xB0V[\x03a\x08\x10Wa\x07\xF9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10Y\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x01\x01Ua\nFV[`\x02\x81`\x03\x81\x11\x15a\x08$Wa\x08$a\x1F\xB0V[\x03a\t\xA1W`\0\x80a\x08k\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x10p\x92PPPV[`\0\x89\x81R` \x81\x90R`@\x90 T\x82Q\x92\x94P\x90\x92P\x14a\x08\xBBW\x81Q`\0\x88\x81R` \x81\x90R`@\x90\x81\x90 T\x90Qc\xAC\x18\x83\x95`\xE0\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x01a\x03\xF4V[`\0\x80[\x83Q\x81\x10\x15a\x08\xF7W\x83\x81\x81Q\x81\x10a\x08\xDAWa\x08\xDAa\x1E%V[` \x02` \x01\x01Q\x82a\x08\xED\x91\x90a\x1E;V[\x91P`\x01\x01a\x08\xBFV[Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x14a\t#W`@Qc>3\x1D\xC9`\xE1\x1B\x81R`\x04\x81\x01\x82\x90R`$\x01a\x03\xF4V[`\0[\x83Q\x81\x10\x15a\t\x98Wa\t\x90\x84\x82\x81Q\x81\x10a\tDWa\tDa\x1E%V[` \x02` \x01\x01Q\x84`\0\x80\x8D\x81R` \x01\x90\x81R` \x01`\0 `\0\x01\x84\x81T\x81\x10a\tsWa\tsa\x1E%V[\x90`\0R` `\0 \x90`\x04\x02\x01a\x10\x93\x90\x92\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\x01\x01a\t&V[PPPPa\nFV[`\x03\x81`\x03\x81\x11\x15a\t\xB5Wa\t\xB5a\x1F\xB0V[\x03a\n-Wa\t\xF9\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x11Y\x92PPPV[`\0\x86\x81R` \x81\x90R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\nFV[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[``a\n}`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R` \x81\x90R`@\x90 T`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\xA3Wa\n\xA3a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xCCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R`\0[\x81QQ\x81\x10\x15a\x0BnW`\0\x84\x81R` \x81\x90R`@\x90 \x80Ta\x0BG\x91\x90\x83\x90\x81\x10a\x0B\x01Wa\x0B\x01a\x1E%V[\x90`\0R` `\0 \x90`\x04\x02\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x11vV[\x82Q\x80Q\x83\x90\x81\x10a\x0B[Wa\x0B[a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01R`\x01\x01a\n\xD2V[P`\0\x83\x81R` \x81\x81R`@\x80\x83 `\x01\x81\x01T\x85\x84\x01R\x86\x84R\x92\x82R`\x02\x90\x92\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83\x83\x01R\x90Qa\x0B\xAE\x91\x83\x91\x01a\x1F\xC6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[``\x80\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0B\xE5Wa\x0B\xE5a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x0EW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0C.Wa\x0C.a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CWW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83`@\x01QQ\x81\x10\x15a\r\x96W`\0\x84`@\x01Q\x82\x81Q\x81\x10a\x0C\x82Wa\x0C\x82a\x1E%V[` \x02` \x01\x01Q\x90Pa\x0C\x9B\x81\x88\x87``\x01Qa\x12\x07V[\x84\x83\x81Q\x81\x10a\x0C\xADWa\x0C\xADa\x1E%V[` \x02` \x01\x01\x81\x81RPP\x83\x82\x81Q\x81\x10a\x0C\xCBWa\x0C\xCBa\x1E%V[` \x02` \x01\x01Q\x86\x83\x81Q\x81\x10a\x0C\xE5Wa\x0C\xE5a\x1E%V[` \x02` \x01\x01Q\x11\x15a\rKW\x85\x82\x81Q\x81\x10a\r\x05Wa\r\x05a\x1E%V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a\r\x1FWa\r\x1Fa\x1E%V[` \x02` \x01\x01Q`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x01a\x03\xF4\x92\x91\x90\x91\x82R` \x82\x01R`@\x01\x90V[\x83\x82\x81Q\x81\x10a\r]Wa\r]a\x1E%V[` \x02` \x01\x01Q\x81a\rp\x91\x90a\x1D\x87V[\x83\x83\x81Q\x81\x10a\r\x82Wa\r\x82a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0C]V[P\x93P\x93\x91PPV[`\0\x80\x86\x80` \x01\x90Q\x81\x01\x90a\r\xB6\x91\x90a\x1E\xE3V[\x90Pa\x0E\x07\x84\x89`@\x01Q\x88\x81Q\x81\x10a\r\xD2Wa\r\xD2a\x1E%V[` \x02` \x01\x01Q\x8A``\x01Q\x84`\0\x01Q\x8A\x81Q\x81\x10a\r\xF5Wa\r\xF5a\x1E%V[` \x02` \x01\x01Q\x85` \x01Qa\x12\x1DV[\x98\x97PPPPPPPPV[``\x80\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E3Wa\x0E3a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\\W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x91P\x82`@\x01QQ`\x01`\x01`@\x1B\x03\x81\x11\x15a\x0E|Wa\x0E|a\x19ZV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0E\xA5W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83`@\x01QQ\x81\x10\x15a\r\x96W`\0\x84`@\x01Q\x82\x81Q\x81\x10a\x0E\xD0Wa\x0E\xD0a\x1E%V[` \x02` \x01\x01Q\x90Pa\x0F\x06\x85`@\x01Q\x83\x81Q\x81\x10a\x0E\xF3Wa\x0E\xF3a\x1E%V[` \x02` \x01\x01Q\x88\x87``\x01Qa\x12HV[\x84\x83\x81Q\x81\x10a\x0F\x18Wa\x0F\x18a\x1E%V[` \x02` \x01\x01\x81\x81RPP\x85\x82\x81Q\x81\x10a\x0F6Wa\x0F6a\x1E%V[` \x02` \x01\x01Q\x84\x83\x81Q\x81\x10a\x0FPWa\x0FPa\x1E%V[` \x02` \x01\x01Q\x11\x15a\x0FpW\x85\x82\x81Q\x81\x10a\r\x05Wa\r\x05a\x1E%V[\x83\x82\x81Q\x81\x10a\x0F\x82Wa\x0F\x82a\x1E%V[` \x02` \x01\x01Q\x81a\x0F\x95\x91\x90a\x1E;V[\x83\x83\x81Q\x81\x10a\x0F\xA7Wa\x0F\xA7a\x1E%V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\x0E\xABV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81[\x85Q\x81\x10\x15a\x10=W`\0a\x10&\x85`\0\x01Q\x83\x81Q\x81\x10a\x0F\xECWa\x0F\xECa\x1E%V[` \x02` \x01\x01Qa\x10 \x88\x8A\x86\x81Q\x81\x10a\x10\nWa\x10\na\x1E%V[` \x02` \x01\x01Qa\x12^\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x12|V[\x90Pa\x102\x83\x82a\x12\xADV[\x92PP`\x01\x01a\x0F\xC8V[Pa\x10Pg\r\xE0\xB6\xB3\xA7d\0\0\x82a 6V[\x95\x94PPPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x07\x03\x91\x90a ]V[```\0\x82\x80` \x01\x90Q\x81\x01\x90a\x10\x88\x91\x90a \x8BV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\x10\xB3W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xBC\x83a\x12\xC2V[`\0a\x10\xC8B\x83a\x1D\x87V[\x84T\x90\x91P`\0\x90a\x10\xDA\x90\x85a 6V[\x90P`\0a\x10\xE8\x83\x83a \xFAV[\x90P`\0a\x10\xF6\x84\x84a!(V[\x90P`\0\x81\x13\x15a\x11 W\x80\x87`\0\x01`\0\x82\x82Ta\x11\x15\x91\x90a\x1E;V[\x90\x91UPa\x11B\x90PV[a\x11)\x81a!<V[\x87`\0\x01`\0\x82\x82Ta\x11<\x91\x90a\x1D\x87V[\x90\x91UPP[P`\x01\x86\x01\x93\x90\x93UPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x11o\x91\x90a!XV[\x93\x92PPPV[`\0\x81` \x01Q\x82``\x01Q\x03a\x11\x8CWPQ\x90V[`\0\x82` \x01QB\x11a\x11\x9FWBa\x11\xA5V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x11\xB9\x91\x90a\x1D\x87V[\x90P`\0\x84`@\x01Q\x13\x15a\x11\xE3W`@\x84\x01Qa\x11\xD7\x90\x82a!\x92V[\x84Qa\x07\x03\x91\x90a\x1E;V[\x83`@\x01Qa\x11\xF1\x90a!<V[a\x11\xFB\x90\x82a!\x92V[\x84Qa\x07\x03\x91\x90a\x1D\x87V[`\0a\x07\x03a\x12\x16\x84\x84a\x12^V[\x85\x90a\x13\x04V[`\0a\x12>a\x12,\x87\x87a\x12^V[a\x128\x86\x81\x87\x87a\x13\x04V[\x90a\x13\x04V[\x96\x95PPPPPPV[`\0a\x07\x03a\x12W\x84\x84a\x13\x19V[\x85\x90a\x12\xADV[`\0a\x12s\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13.V[\x90P[\x92\x91PPV[`\0a\x12sg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x12\x94\x86a\x13MV[a\x12\x9E\x91\x90a!\xA9V[a\x12\xA8\x91\x90a \xFAV[a\x15(V[`\0a\x12s\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\xD1V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x12\xF9\x90a\x11vV[\x81UB`\x03\x90\x91\x01UV[`\0a\x12s\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13.V[`\0a\x12s\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\xD1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x13FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\x13\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\xF4V[`\0``a\x13\x97\x84a\x16\xFFV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15CWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x15\x8AW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x03\xF4V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xE9W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x17<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x03\xF4V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x17\xBBW`\0\x80\xFD[PV[\x805a\x17\xC9\x81a\x17\xA6V[\x91\x90PV[`\0\x80`\0\x80`\0`\x80\x86\x88\x03\x12\x15a\x17\xE6W`\0\x80\xFD[\x855a\x17\xF1\x81a\x17\xA6V[\x94P` \x86\x015\x93P`@\x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x18\x14W`\0\x80\xFD[\x90\x87\x01\x90`\xE0\x82\x8A\x03\x12\x15a\x18(W`\0\x80\xFD[\x90\x93P``\x87\x015\x90\x80\x82\x11\x15a\x18>W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x18RW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x18aW`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x18sW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0`\x80\x82\x01\x86\x15\x15\x83R` \x86` \x85\x01R`\x80`@\x85\x01R\x81\x86Q\x80\x84R`\xA0\x86\x01\x91P` \x88\x01\x93P`\0[\x81\x81\x10\x15a\x18\xD1W\x84Q\x83R\x93\x83\x01\x93\x91\x83\x01\x91`\x01\x01a\x18\xB5V[PP\x80\x93PPPP\x82``\x83\x01R\x95\x94PPPPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x19\x0EW` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x18\xF2V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x12s` \x83\x01\x84a\x18\xE8V[`\0` \x82\x84\x03\x12\x15a\x19SW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\x92Wa\x19\x92a\x19ZV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x19\xC0Wa\x19\xC0a\x19ZV[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x19\xE1Wa\x19\xE1a\x19ZV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x19\xFCW`\0\x80\xFD[\x815` a\x1A\x11a\x1A\x0C\x83a\x19\xC8V[a\x19\x98V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A3W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x805a\x1AK\x81a\x17\xA6V[\x83R\x91\x83\x01\x91\x83\x01a\x1A8V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x1AtW`\0\x80\xFD[\x815` a\x1A\x84a\x1A\x0C\x83a\x19\xC8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1A\xA6W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x805\x83R\x91\x83\x01\x91\x83\x01a\x1A\xABV[`\0`\xE0\x82\x84\x03\x12\x15a\x1A\xD4W`\0\x80\xFD[a\x1A\xDCa\x19pV[\x90Pa\x1A\xE7\x82a\x17\xBEV[\x81R` \x82\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1B\x03W`\0\x80\xFD[a\x1B\x0F\x85\x83\x86\x01a\x19\xEBV[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15a\x1B(W`\0\x80\xFD[Pa\x1B5\x84\x82\x85\x01a\x1AcV[`@\x83\x01RP``\x82\x015``\x82\x01Ra\x1BQ`\x80\x83\x01a\x17\xBEV[`\x80\x82\x01Ra\x1Bb`\xA0\x83\x01a\x17\xBEV[`\xA0\x82\x01R`\xC0\x82\x015`\xC0\x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a\x1B\x88W`\0\x80\xFD[\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1B\xA1Wa\x1B\xA1a\x19ZV[a\x1B\xB4`\x1F\x82\x01`\x1F\x19\x16` \x01a\x19\x98V[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a\x1B\xC9W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1B\xFCW`\0\x80\xFD[\x845a\x1C\x07\x81a\x17\xA6V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C*W`\0\x80\xFD[a\x1C6\x88\x83\x89\x01a\x1A\xC2V[\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x1CLW`\0\x80\xFD[Pa\x1CY\x87\x82\x88\x01a\x1BwV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1CzW`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1C\x91W`\0\x80\xFD[a\x1C\x9D\x87\x83\x88\x01a\x1AcV[\x94P` \x86\x015\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x1C\xBAW`\0\x80\xFD[Pa\x1C\xC7\x86\x82\x87\x01a\x1BwV[\x91PP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a\x1C\xE4W`\0\x80\xFD[\x825`\x01`\x01`@\x1B\x03\x81\x11\x15a\x1C\xFAW`\0\x80\xFD[a\x1D\x06\x85\x82\x86\x01a\x1AcV[\x95` \x94\x90\x94\x015\x94PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a\x1D,W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a\x1DFW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a\x1D^W`\0\x80\xFD[\x92P\x92\x90PV[`\0a\x12v6\x83a\x1A\xC2V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x12vWa\x12va\x1DqV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x1D\xB2W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1D\xC9W`\0\x80\xFD[a\x1D\xD5\x89\x83\x8A\x01a\x1AcV[\x96P` \x88\x015\x95P`@\x88\x015\x91P\x80\x82\x11\x15a\x1D\xF2W`\0\x80\xFD[Pa\x1D\xFF\x88\x82\x89\x01a\x1AcV[\x93PP``\x86\x015\x91P`\x80\x86\x015a\x1E\x17\x81a\x17\xA6V[\x80\x91PP\x92\x95P\x92\x95\x90\x93PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x12vWa\x12va\x1DqV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1EdW`\0\x80\xFD[PP\x82Q` \x84\x01Q`@\x85\x01Q``\x90\x95\x01Q\x91\x96\x90\x95P\x90\x92P\x90PV[`\0\x82`\x1F\x83\x01\x12a\x1E\x95W`\0\x80\xFD[\x81Q` a\x1E\xA5a\x1A\x0C\x83a\x19\xC8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x1E\xC7W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1AXW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x1E\xCCV[`\0` \x82\x84\x03\x12\x15a\x1E\xF5W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1F\x0CW`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x1F W`\0\x80\xFD[`@Q``\x81\x01\x81\x81\x10\x83\x82\x11\x17\x15a\x1F;Wa\x1F;a\x19ZV[`@R\x82Q\x82\x81\x11\x15a\x1FMW`\0\x80\xFD[a\x1FY\x87\x82\x86\x01a\x1E\x84V[\x82RP` \x83\x01Q` \x82\x01R`@\x83\x01Q\x92Pa\x1Fv\x83a\x17\xA6V[`@\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\x04\x81\x10a\x17\xBBW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1F\xA5W`\0\x80\xFD[\x815a\x11o\x81a\x1F\x86V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[` \x80\x82R\x82Q``\x83\x83\x01R\x80Q`\x80\x84\x01\x81\x90R`\0\x92\x91\x82\x01\x90\x83\x90`\xA0\x86\x01\x90[\x80\x83\x10\x15a \x0BW\x83Q\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1F\xEBV[P\x92\x86\x01Q`@\x86\x81\x01\x91\x90\x91R\x90\x95\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x90\x94\x01\x93\x90\x93R\x93\x92PPPV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a VWa Va\x1DqV[P\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a pW`\0\x80\xFD[\x82Qa {\x81a\x1F\x86V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a \xA0W`\0\x80\xFD[\x83Qa \xAB\x81a\x1F\x86V[` \x85\x01Q\x90\x93P`\x01`\x01`@\x1B\x03\x81\x11\x15a \xC7W`\0\x80\xFD[a \xD3\x86\x82\x87\x01a\x1E\x84V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a!\tWa!\ta \xE4V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a!#Wa!#a\x1DqV[P\x05\x90V[`\0\x82a!7Wa!7a \xE4V[P\x07\x90V[`\0`\x01`\xFF\x1B\x82\x01a!QWa!Qa\x1DqV[P`\0\x03\x90V[`\0\x80`@\x83\x85\x03\x12\x15a!kW`\0\x80\xFD[\x82Qa!v\x81a\x1F\x86V[` \x84\x01Q\x90\x92Pa!\x87\x81a\x17\xA6V[\x80\x91PP\x92P\x92\x90PV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x12vWa\x12va\x1DqV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a!\xC5Wa!\xC5a\x1DqV[\x81\x81\x05\x83\x14\x82\x15\x17a\x12vWa\x12va\x1DqV\xFE\xA2dipfsX\"\x12 \xF6\x83\x9E\xBF\xEC\x85\x07)\xCD\xDF|\xA6\x941\x83F<\xB2\xD1i\x13\xE0=e\xB2kV\xB7\xF6\xE66\xE5dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static NTOKENGEOMETRICMEAN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NTokenGeometricMean<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NTokenGeometricMean<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NTokenGeometricMean<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NTokenGeometricMean<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NTokenGeometricMean<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NTokenGeometricMean))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NTokenGeometricMean<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NTOKENGEOMETRICMEAN_ABI.clone(),
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
                NTOKENGEOMETRICMEAN_ABI.clone(),
                NTOKENGEOMETRICMEAN_BYTECODE.clone().into(),
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
            p2: Pool,
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
                .method_hash([79, 23, 217, 19], (p0, pool_id, p2, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::Address),
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
        for NTokenGeometricMean<M>
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
    /// Custom Error type `InvalidConfiguration` with signature
    /// `InvalidConfiguration(uint256,uint256)` and selector `0x4c718410`
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
    #[etherror(
        name = "InvalidConfiguration",
        abi = "InvalidConfiguration(uint256,uint256)"
    )]
    pub struct InvalidConfiguration {
        pub reserves_length: ::ethers::core::types::U256,
        pub weights_length: ::ethers::core::types::U256,
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
    /// Custom Error type `InvalidTokenDeltas` with signature
    /// `InvalidTokenDeltas()` and selector `0xacfdc4d4`
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
    #[etherror(name = "InvalidTokenDeltas", abi = "InvalidTokenDeltas()")]
    pub struct InvalidTokenDeltas;
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
    /// Custom Error type `InvalidWeightUpdateLength` with signature
    /// `InvalidWeightUpdateLength(uint256,uint256)` and selector `0xac188395`
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
    #[etherror(
        name = "InvalidWeightUpdateLength",
        abi = "InvalidWeightUpdateLength(uint256,uint256)"
    )]
    pub struct InvalidWeightUpdateLength {
        pub update_length: ::ethers::core::types::U256,
        pub weights_length: ::ethers::core::types::U256,
    }
    /// Custom Error type `InvalidWeights` with signature
    /// `InvalidWeights(uint256)` and selector `0x7c663b92`
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
    #[etherror(name = "InvalidWeights", abi = "InvalidWeights(uint256)")]
    pub struct InvalidWeights {
        pub total_weight: ::ethers::core::types::U256,
    }
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
    pub enum NTokenGeometricMeanErrors {
        DeltaError(DeltaError),
        InvalidConfiguration(InvalidConfiguration),
        InvalidReservesLength(InvalidReservesLength),
        InvalidSender(InvalidSender),
        InvalidTokenDeltas(InvalidTokenDeltas),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWeightUpdateLength(InvalidWeightUpdateLength),
        InvalidWeights(InvalidWeights),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for NTokenGeometricMeanErrors {
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
                <InvalidConfiguration as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidConfiguration(decoded));
            }
            if let Ok(decoded) =
                <InvalidReservesLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidReservesLength(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) =
                <InvalidTokenDeltas as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidTokenDeltas(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) =
                <InvalidWeightUpdateLength as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidWeightUpdateLength(decoded));
            }
            if let Ok(decoded) = <InvalidWeights as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidWeights(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for NTokenGeometricMeanErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DeltaError(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidConfiguration(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidReservesLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidTokenDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidWeightUpdateLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWeights(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for NTokenGeometricMeanErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <DeltaError as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidConfiguration as ::ethers::contract::EthError>::selector() =>
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
                    == <InvalidTokenDeltas as ::ethers::contract::EthError>::selector() =>
                {
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
                _ if selector
                    == <InvalidWeightUpdateLength as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidWeights as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for NTokenGeometricMeanErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DeltaError(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidConfiguration(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidReservesLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokenDeltas(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeightUpdateLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for NTokenGeometricMeanErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DeltaError> for NTokenGeometricMeanErrors {
        fn from(value: DeltaError) -> Self {
            Self::DeltaError(value)
        }
    }
    impl ::core::convert::From<InvalidConfiguration> for NTokenGeometricMeanErrors {
        fn from(value: InvalidConfiguration) -> Self {
            Self::InvalidConfiguration(value)
        }
    }
    impl ::core::convert::From<InvalidReservesLength> for NTokenGeometricMeanErrors {
        fn from(value: InvalidReservesLength) -> Self {
            Self::InvalidReservesLength(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for NTokenGeometricMeanErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidTokenDeltas> for NTokenGeometricMeanErrors {
        fn from(value: InvalidTokenDeltas) -> Self {
            Self::InvalidTokenDeltas(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for NTokenGeometricMeanErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for NTokenGeometricMeanErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWeightUpdateLength> for NTokenGeometricMeanErrors {
        fn from(value: InvalidWeightUpdateLength) -> Self {
            Self::InvalidWeightUpdateLength(value)
        }
    }
    impl ::core::convert::From<InvalidWeights> for NTokenGeometricMeanErrors {
        fn from(value: InvalidWeights) -> Self {
            Self::InvalidWeights(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for NTokenGeometricMeanErrors {
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
        pub p2: Pool,
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
    pub enum NTokenGeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiDecode for NTokenGeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiEncode for NTokenGeometricMeanCalls {
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
    impl ::core::fmt::Display for NTokenGeometricMeanCalls {
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
    impl ::core::convert::From<DfmmCall> for NTokenGeometricMeanCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for NTokenGeometricMeanCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for NTokenGeometricMeanCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for NTokenGeometricMeanCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for NTokenGeometricMeanCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<TradingFunctionCall> for NTokenGeometricMeanCalls {
        fn from(value: TradingFunctionCall) -> Self {
            Self::TradingFunction(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for NTokenGeometricMeanCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateCall> for NTokenGeometricMeanCalls {
        fn from(value: ValidateAllocateCall) -> Self {
            Self::ValidateAllocate(value)
        }
    }
    impl ::core::convert::From<ValidateDeallocateCall> for NTokenGeometricMeanCalls {
        fn from(value: ValidateDeallocateCall) -> Self {
            Self::ValidateDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for NTokenGeometricMeanCalls {
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
        pub token_deltas: ::std::vec::Vec<::ethers::core::types::U256>,
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
        pub token_deltas: ::std::vec::Vec<::ethers::core::types::U256>,
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

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
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeSwapConstant",),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                                name: ::std::borrow::ToOwned::to_owned("pool"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaY"),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaY"),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                                name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextRx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextL"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0E\xC98\x03\x80a\x0E\xC9\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x0E6\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c2\xE3\x830\x11a\0fW\x80c2\xE3\x830\x14a\x01\xACW\x80c9n>|\x14a\x01\xBFW\x80ce\xC9\xFF\xC2\x14a\x01\xD2W\x80c\xAF\xBA\x13\xC4\x14a\x02\x14W\x80c\xDC\x17\x83U\x14a\x02?W`\0\x80\xFD[\x80b.RK\x14a\0\xA2W\x80c\x02J\xA2\x06\x14a\0\xC8W\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\x18\x1C\xBA\xB4\x14a\x01<W\x80c\x1E\xDBq\xE5\x14a\x01QW[`\0\x80\xFD[a\0\xB5a\0\xB06`\x04a\t\xB8V[a\x02RV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\noV[a\x02\xA6V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xBFV[a\x01/`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jConstantSum`\xA8\x1B\x81RP\x81V[`@Qa\0\xBF\x91\x90a\x0B`V[a\x01Oa\x01J6`\x04a\noV[a\x03\x1EV[\0[a\x01\x88a\x01_6`\x04a\x0BsV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01T`\x02\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\0\xBFV[a\0\xDBa\x01\xBA6`\x04a\noV[a\x05\x1FV[a\0\xDBa\x01\xCD6`\x04a\noV[a\x05\xEEV[a\x01\xE5a\x01\xE06`\x04a\noV[a\x064V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\0\xBFV[`\0Ta\x02'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBFV[a\x01/a\x02M6`\x04a\x0BsV[a\x07\xC9V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x02l\x91\x90a\x0B\x8CV[\x92P\x92P\x92Pa\x02\x9A\x83\x83\x83a\x02\x81\x8Aa\x07\xC9V[\x80` \x01\x90Q\x81\x01\x90a\x02\x94\x91\x90a\x0B\xBAV[Qa\x08[V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80\x80a\x02\xB8\x86\x88\x01\x88a\x0B\xFCV[\x91\x94P\x92P\x90Pa\x02\xF5a\x02\xD0\x84``\x8B\x015a\x0C>V[a\x02\xDE\x84``\x8C\x015a\x0C>V[a\x02\xEC\x84`\xA0\x8D\x015a\x0C>V[a\x02\x81\x8Da\x07\xC9V[\x93P\x83a\x03\x02`\x14a\x0CQV[\x12\x80\x15a\x03\x0FWP`\x14\x84\x12[\x94P\x95P\x95P\x95P\x95P\x95\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03IW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x01` R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x03\x86W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x94\x82\x84\x01\x84a\x0CzV[\x90P`\x02\x81`\x03\x81\x11\x15a\x03\xAAWa\x03\xAAa\x0C\x97V[\x03a\x04\x02Wa\x03\xEE\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xA1\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 Ua\x05\x17V[`\x01\x81`\x03\x81\x11\x15a\x04\x16Wa\x04\x16a\x0C\x97V[\x03a\x04rWa\x04Z\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xBE\x92PPPV[`\0\x86\x81R`\x01` \x81\x90R`@\x90\x91 \x01Ua\x05\x17V[`\x03\x81`\x03\x81\x11\x15a\x04\x86Wa\x04\x86a\x0C\x97V[\x03a\x04\xFEWa\x04\xCA\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xDD\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x05\x17V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x80T\x81\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05SW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x80`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x05\x8C\x87\x89\x01\x89a\x0C\xADV[\x80Q`\0\x8F\x81R`\x01` \x81\x81R`@\x90\x92 \x92\x83U\x90\x83\x01Q\x91\x01U\x80Q\x93\x97P\x91\x95P\x93P\x91Pa\x05\xC4\x90\x85\x90\x85\x90\x85\x90a\x08[V[\x94P\x84a\x05\xD1`\x14a\x0CQV[\x12\x80\x15a\x05\xDEWP`\x14\x85\x12[\x95PP\x95P\x95P\x95P\x95P\x95\x90PV[`\0\x80\x80\x80\x80a\x06\0\x86\x88\x01\x88a\x0B\xFCV[\x91\x94P\x92P\x90Pa\x02\xF5a\x06\x18\x84``\x8B\x015a\r V[a\x06&\x84`\x80\x8C\x015a\r V[a\x02\xEC\x84`\xA0\x8D\x015a\r V[`\0\x80`\0\x80`\0\x80`\0a\x06H\x8Ba\x07\xC9V[\x80` \x01\x90Q\x81\x01\x90a\x06[\x91\x90a\x0B\xBAV[\x90Pa\x06i\x88\x8A\x01\x8Aa\x0B\xFCV[\x91\x95P\x93P\x91P`\0\x80\x80``\x8D\x015\x87\x11\x15a\x06\xBAWa\x06\x8E``\x8E\x015\x88a\r V[\x91Pa\x06\xA7\x84` \x01Q\x83a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB3\x81\x84a\x0C>V[\x92Pa\x07lV[\x8C`\x80\x015\x86\x11\x15a\x07\x07Wa\x06\xD4`\x80\x8E\x015\x87a\r V[\x91Pa\x06\xED\x84` \x01Q\x83a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q\x90\x91Pa\x06\xFD\x90\x82\x90a\t\tV[a\x06\xB3\x90\x84a\x0C>V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[a\x07z`\xA0\x8E\x015\x86a\r3V[\x97P\x82\x88\x12\x15a\x07\x8CWa\x07\x8Ca\rZV[a\x07\x9C\x87\x87\x87\x87`\0\x01Qa\x08[V[\x98P\x88a\x07\xA9`\x14a\x0CQV[\x12\x80\x15a\x07\xB6WP`\x14\x89\x12[\x99PPPPP\x95P\x95P\x95P\x95P\x95P\x95V[``a\x07\xF8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x86R\x93\x87\x90R\x82\x82R\x92\x90\x91\x01T\x83\x82\x01\x90\x81R\x82Q\x84Q\x92\x81\x01\x92\x90\x92RQ\x81\x83\x01R\x90\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x08za\x08s\x85\x85a\x08\xF4V[\x86\x90a\t\tV[a\x08\x84\x87\x86a\t\tV[a\x08\x8E\x91\x90a\rpV[a\x08\x98\x91\x90a\r3V[\x95\x94PPPPPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x08\xB7\x91\x90a\r\x98V[\x93\x92PPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD5\x91\x90a\r\x98V[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD5\x91\x90a\r\xC6V[`\0a\x08\xB7\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\t\x1AV[`\0a\x08\xB7\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\t2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\x81Wa\t\x81a\tHV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xB0Wa\t\xB0a\tHV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\t\xCBW`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xEBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\t\xFFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x11Wa\n\x11a\tHV[a\n#`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\t\x87V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\n9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nlW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x85\x87\x03a\x01@\x81\x12\x15a\n\x89W`\0\x80\xFD[\x865a\n\x94\x81a\nWV[\x95P` \x87\x015\x94P`\xE0`?\x19\x82\x01\x12\x15a\n\xAFW`\0\x80\xFD[P`@\x86\x01\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xD2W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\xE6W`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\xF5W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x0B\x07W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B@W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B$V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x08\xB7` \x83\x01\x84a\x0B\x1AV[`\0` \x82\x84\x03\x12\x15a\x0B\x85W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xA1W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[a\x0B\xD4a\t^V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0B\xF0\x81a\nWV[`@\x82\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x11W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xA0Wa\x02\xA0a\x0C(V[`\0`\x01`\xFF\x1B\x82\x01a\x0CfWa\x0Cfa\x0C(V[P`\0\x03\x90V[`\x04\x81\x10a\nlW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x8CW`\0\x80\xFD[\x815a\x08\xB7\x81a\x0CmV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a\x0C\xC4W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a\x0C\xE8W`\0\x80\xFD[Pa\x0C\xF1a\t^V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015a\r\x10\x81a\nWV[`@\x82\x01R\x93\x96\x92\x95P\x90\x93PPV[\x81\x81\x03\x81\x81\x11\x15a\x02\xA0Wa\x02\xA0a\x0C(V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\rSWa\rSa\x0C(V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\r\x90Wa\r\x90a\x0C(V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xABW`\0\x80\xFD[\x82Qa\r\xB6\x81a\x0CmV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xD9W`\0\x80\xFD[\x82Qa\r\xE4\x81a\x0CmV[` \x84\x01Q\x90\x92Pa\r\xF5\x81a\nWV[\x80\x91PP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \0q*\x83dZ\x07\x97?\xB3L\x16\xFF\xBF\xF9VyU\xBC\xCE\"+\xF48\xC1\x7F\x8E\xC9\x95\"!\xBBdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUM_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c2\xE3\x830\x11a\0fW\x80c2\xE3\x830\x14a\x01\xACW\x80c9n>|\x14a\x01\xBFW\x80ce\xC9\xFF\xC2\x14a\x01\xD2W\x80c\xAF\xBA\x13\xC4\x14a\x02\x14W\x80c\xDC\x17\x83U\x14a\x02?W`\0\x80\xFD[\x80b.RK\x14a\0\xA2W\x80c\x02J\xA2\x06\x14a\0\xC8W\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\x18\x1C\xBA\xB4\x14a\x01<W\x80c\x1E\xDBq\xE5\x14a\x01QW[`\0\x80\xFD[a\0\xB5a\0\xB06`\x04a\t\xB8V[a\x02RV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\noV[a\x02\xA6V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xBFV[a\x01/`@Q\x80`@\x01`@R\x80`\x0B\x81R` \x01jConstantSum`\xA8\x1B\x81RP\x81V[`@Qa\0\xBF\x91\x90a\x0B`V[a\x01Oa\x01J6`\x04a\noV[a\x03\x1EV[\0[a\x01\x88a\x01_6`\x04a\x0BsV[`\x01` \x81\x90R`\0\x91\x82R`@\x90\x91 \x80T\x91\x81\x01T`\x02\x90\x91\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x01a\0\xBFV[a\0\xDBa\x01\xBA6`\x04a\noV[a\x05\x1FV[a\0\xDBa\x01\xCD6`\x04a\noV[a\x05\xEEV[a\x01\xE5a\x01\xE06`\x04a\noV[a\x064V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\0\xBFV[`\0Ta\x02'\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBFV[a\x01/a\x02M6`\x04a\x0BsV[a\x07\xC9V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x02l\x91\x90a\x0B\x8CV[\x92P\x92P\x92Pa\x02\x9A\x83\x83\x83a\x02\x81\x8Aa\x07\xC9V[\x80` \x01\x90Q\x81\x01\x90a\x02\x94\x91\x90a\x0B\xBAV[Qa\x08[V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80\x80a\x02\xB8\x86\x88\x01\x88a\x0B\xFCV[\x91\x94P\x92P\x90Pa\x02\xF5a\x02\xD0\x84``\x8B\x015a\x0C>V[a\x02\xDE\x84``\x8C\x015a\x0C>V[a\x02\xEC\x84`\xA0\x8D\x015a\x0C>V[a\x02\x81\x8Da\x07\xC9V[\x93P\x83a\x03\x02`\x14a\x0CQV[\x12\x80\x15a\x03\x0FWP`\x14\x84\x12[\x94P\x95P\x95P\x95P\x95P\x95\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x03IW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x01` R`@\x90 `\x02\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x03\x86W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x03\x94\x82\x84\x01\x84a\x0CzV[\x90P`\x02\x81`\x03\x81\x11\x15a\x03\xAAWa\x03\xAAa\x0C\x97V[\x03a\x04\x02Wa\x03\xEE\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xA1\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 Ua\x05\x17V[`\x01\x81`\x03\x81\x11\x15a\x04\x16Wa\x04\x16a\x0C\x97V[\x03a\x04rWa\x04Z\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xBE\x92PPPV[`\0\x86\x81R`\x01` \x81\x90R`@\x90\x91 \x01Ua\x05\x17V[`\x03\x81`\x03\x81\x11\x15a\x04\x86Wa\x04\x86a\x0C\x97V[\x03a\x04\xFEWa\x04\xCA\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x08\xDD\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\x02\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x05\x17V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x80T\x81\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05SW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x80`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[a\x05\x8C\x87\x89\x01\x89a\x0C\xADV[\x80Q`\0\x8F\x81R`\x01` \x81\x81R`@\x90\x92 \x92\x83U\x90\x83\x01Q\x91\x01U\x80Q\x93\x97P\x91\x95P\x93P\x91Pa\x05\xC4\x90\x85\x90\x85\x90\x85\x90a\x08[V[\x94P\x84a\x05\xD1`\x14a\x0CQV[\x12\x80\x15a\x05\xDEWP`\x14\x85\x12[\x95PP\x95P\x95P\x95P\x95P\x95\x90PV[`\0\x80\x80\x80\x80a\x06\0\x86\x88\x01\x88a\x0B\xFCV[\x91\x94P\x92P\x90Pa\x02\xF5a\x06\x18\x84``\x8B\x015a\r V[a\x06&\x84`\x80\x8C\x015a\r V[a\x02\xEC\x84`\xA0\x8D\x015a\r V[`\0\x80`\0\x80`\0\x80`\0a\x06H\x8Ba\x07\xC9V[\x80` \x01\x90Q\x81\x01\x90a\x06[\x91\x90a\x0B\xBAV[\x90Pa\x06i\x88\x8A\x01\x8Aa\x0B\xFCV[\x91\x95P\x93P\x91P`\0\x80\x80``\x8D\x015\x87\x11\x15a\x06\xBAWa\x06\x8E``\x8E\x015\x88a\r V[\x91Pa\x06\xA7\x84` \x01Q\x83a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x06\xB3\x81\x84a\x0C>V[\x92Pa\x07lV[\x8C`\x80\x015\x86\x11\x15a\x07\x07Wa\x06\xD4`\x80\x8E\x015\x87a\r V[\x91Pa\x06\xED\x84` \x01Q\x83a\x08\xF4\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q\x90\x91Pa\x06\xFD\x90\x82\x90a\t\tV[a\x06\xB3\x90\x84a\x0C>V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01`@Q\x80\x91\x03\x90\xFD[a\x07z`\xA0\x8E\x015\x86a\r3V[\x97P\x82\x88\x12\x15a\x07\x8CWa\x07\x8Ca\rZV[a\x07\x9C\x87\x87\x87\x87`\0\x01Qa\x08[V[\x98P\x88a\x07\xA9`\x14a\x0CQV[\x12\x80\x15a\x07\xB6WP`\x14\x89\x12[\x99PPPPP\x95P\x95P\x95P\x95P\x95P\x95V[``a\x07\xF8`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0\x83\x81R`\x01` \x81\x81R`@\x80\x84 \x80T\x86R\x93\x87\x90R\x82\x82R\x92\x90\x91\x01T\x83\x82\x01\x90\x81R\x82Q\x84Q\x92\x81\x01\x92\x90\x92RQ\x81\x83\x01R\x90\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x82\x01R`\x80\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0a\x08za\x08s\x85\x85a\x08\xF4V[\x86\x90a\t\tV[a\x08\x84\x87\x86a\t\tV[a\x08\x8E\x91\x90a\rpV[a\x08\x98\x91\x90a\r3V[\x95\x94PPPPPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\x08\xB7\x91\x90a\r\x98V[\x93\x92PPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD5\x91\x90a\r\x98V[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\x08\xD5\x91\x90a\r\xC6V[`\0a\x08\xB7\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\t\x1AV[`\0a\x08\xB7\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\t2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\x81Wa\t\x81a\tHV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\t\xB0Wa\t\xB0a\tHV[`@R\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\t\xCBW`\0\x80\xFD[\x825\x91P` \x80\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\t\xEBW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\t\xFFW`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\x11Wa\n\x11a\tHV[a\n#`\x1F\x82\x01`\x1F\x19\x16\x85\x01a\t\x87V[\x91P\x80\x82R\x87\x84\x82\x85\x01\x01\x11\x15a\n9W`\0\x80\xFD[\x80\x84\x84\x01\x85\x84\x017`\0\x84\x82\x84\x01\x01RP\x80\x93PPPP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\nlW`\0\x80\xFD[PV[`\0\x80`\0\x80`\0\x85\x87\x03a\x01@\x81\x12\x15a\n\x89W`\0\x80\xFD[\x865a\n\x94\x81a\nWV[\x95P` \x87\x015\x94P`\xE0`?\x19\x82\x01\x12\x15a\n\xAFW`\0\x80\xFD[P`@\x86\x01\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xD2W`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\n\xE6W`\0\x80\xFD[\x815\x81\x81\x11\x15a\n\xF5W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x0B\x07W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x0B@W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x0B$V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\x08\xB7` \x83\x01\x84a\x0B\x1AV[`\0` \x82\x84\x03\x12\x15a\x0B\x85W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xA1W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0``\x82\x84\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[a\x0B\xD4a\t^V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0B\xF0\x81a\nWV[`@\x82\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\x11W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xA0Wa\x02\xA0a\x0C(V[`\0`\x01`\xFF\x1B\x82\x01a\x0CfWa\x0Cfa\x0C(V[P`\0\x03\x90V[`\x04\x81\x10a\nlW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0C\x8CW`\0\x80\xFD[\x815a\x08\xB7\x81a\x0CmV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15a\x0C\xC4W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P```_\x19\x82\x01\x12\x15a\x0C\xE8W`\0\x80\xFD[Pa\x0C\xF1a\t^V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015a\r\x10\x81a\nWV[`@\x82\x01R\x93\x96\x92\x95P\x90\x93PPV[\x81\x81\x03\x81\x81\x11\x15a\x02\xA0Wa\x02\xA0a\x0C(V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\rSWa\rSa\x0C(V[P\x92\x91PPV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\r\x90Wa\r\x90a\x0C(V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xABW`\0\x80\xFD[\x82Qa\r\xB6\x81a\x0CmV[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\r\xD9W`\0\x80\xFD[\x82Qa\r\xE4\x81a\x0CmV[` \x84\x01Q\x90\x92Pa\r\xF5\x81a\nWV[\x80\x91PP\x92P\x92\x90PV\xFE\xA2dipfsX\"\x12 \0q*\x83dZ\x07\x97?\xB3L\x16\xFF\xBF\xF9VyU\xBC\xCE\"+\xF48\xC1\x7F\x8E\xC9\x95\"!\xBBdsolcC\0\x08\x16\x003";
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
        /// Calls the contract's `computeSwapConstant` (0x002e524b) function
        pub fn compute_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([0, 46, 82, 75], (pool_id, data))
                .expect("method not found (this should never happen)")
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
        /// Calls the contract's `init` (0x32e38330) function
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
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([50, 227, 131, 48], (p0, pool_id, pool, data))
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
        /// Calls the contract's `update` (0x181cbab4) function
        pub fn update(
            &self,
            sender: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            pool: Pool,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 28, 186, 180], (sender, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateAllocate` (0x024aa206) function
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
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([2, 74, 162, 6], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateDeallocate` (0x396e3e7c) function
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
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([57, 110, 62, 124], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `validateSwap` (0x65c9ffc2) function
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
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([101, 201, 255, 194], (p0, pool_id, pool, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for ConstantSum<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    /// Container type for all input parameters for the `computeSwapConstant`
    /// function with signature `computeSwapConstant(uint256,bytes)` and
    /// selector `0x002e524b`
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
        name = "computeSwapConstant",
        abi = "computeSwapConstant(uint256,bytes)"
    )]
    pub struct ComputeSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    /// signature `init(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x32e38330`
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
        abi = "init(address,uint256,(address,address,address,uint256,uint256,uint256,address),bytes)"
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
    /// Container type for all input parameters for the `update` function with
    /// signature `update(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x181cbab4`
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
        abi = "update(address,uint256,(address,address,address,uint256,uint256,uint256,address),bytes)"
    )]
    pub struct UpdateCall {
        pub sender: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateAllocate`
    /// function with signature
    /// `validateAllocate(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x024aa206`
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
        abi = "validateAllocate(address,uint256,(address,address,address,uint256,uint256,uint256,address),bytes)"
    )]
    pub struct ValidateAllocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateDeallocate`
    /// function with signature
    /// `validateDeallocate(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x396e3e7c`
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
        abi = "validateDeallocate(address,uint256,(address,address,address,uint256,uint256,uint256,address),bytes)"
    )]
    pub struct ValidateDeallocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub pool: Pool,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateSwap` function
    /// with signature
    /// `validateSwap(address,uint256,(address,address,address,uint256,uint256,
    /// uint256,address),bytes)` and selector `0x65c9ffc2`
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
        abi = "validateSwap(address,uint256,(address,address,address,uint256,uint256,uint256,address),bytes)"
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
        ComputeSwapConstant(ComputeSwapConstantCall),
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Name(NameCall),
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
            if let Ok(decoded) =
                <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
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
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::ComputeSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateDeallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for ConstantSumCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
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
    /// Container type for all return fields from the `computeSwapConstant`
    /// function with signature `computeSwapConstant(uint256,bytes)` and
    /// selector `0x002e524b`
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
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
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
    /// signature `init(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x32e38330`
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
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
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
    /// Container type for all return fields from the `validateAllocate`
    /// function with signature
    /// `validateAllocate(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x024aa206`
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
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `validateDeallocate`
    /// function with signature
    /// `validateDeallocate(address,uint256,(address,address,address,uint256,
    /// uint256,uint256,address),bytes)` and selector `0x396e3e7c`
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
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `validateSwap` function
    /// with signature
    /// `validateSwap(address,uint256,(address,address,address,uint256,uint256,
    /// uint256,address),bytes)` and selector `0x65c9ffc2`
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
        pub liquidity_delta: ::ethers::core::types::I256,
        pub next_rx: ::ethers::core::types::U256,
        pub next_ry: ::ethers::core::types::U256,
        pub next_l: ::ethers::core::types::U256,
    }
}

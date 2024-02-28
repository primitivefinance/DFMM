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
                                name: ::std::borrow::ToOwned::to_owned("sigma"),
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
                                name: ::std::borrow::ToOwned::to_owned("tau"),
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
                                name: ::std::borrow::ToOwned::to_owned("strike"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qb\0!\x9A8\x03\x80b\0!\x9A\x839\x81\x01`@\x81\x90Ra\x001\x91a\0VV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x86V[`\0` \x82\x84\x03\x12\x15a\0hW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x7FW`\0\x80\xFD[\x93\x92PPPV[a!\x04\x80b\0\0\x96`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c2\xE3\x830\x11a\0fW\x80c2\xE3\x830\x14a\x02\x9CW\x80c9n>|\x14a\x02\xAFW\x80ce\xC9\xFF\xC2\x14a\x02\xC2W\x80c\xAF\xBA\x13\xC4\x14a\x03\x04W\x80c\xDC\x17\x83U\x14a\x03/W`\0\x80\xFD[\x80b.RK\x14a\0\xA2W\x80c\x02J\xA2\x06\x14a\0\xC8W\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\x18\x1C\xBA\xB4\x14a\x01:W\x80c\x1E\xDBq\xE5\x14a\x01OW[`\0\x80\xFD[a\0\xB5a\0\xB06`\x04a\x1B\x97V[a\x03BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\x1C\x0EV[a\x03\x95V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xBFV[a\x01-`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B\x81RP\x81V[`@Qa\0\xBF\x91\x90a\x1C\xF4V[a\x01Ma\x01H6`\x04a\x1C\x0EV[a\x04\xA0V[\0[a\x02\x10a\x01]6`\x04a\x1D\x07V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R\x85Q\x80\x86\x01\x87R`\x04\x84\x01T\x81R`\x05\x84\x01T\x81\x86\x01R`\x06\x84\x01T\x81\x88\x01R`\x07\x84\x01T\x81\x83\x01R\x86Q\x95\x86\x01\x87R`\x08\x84\x01T\x86R`\t\x84\x01T\x94\x86\x01\x94\x90\x94R`\n\x83\x01T\x95\x85\x01\x95\x90\x95R`\x0B\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0C\x81\x01T`\r\x90\x91\x01T\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x85V[`@\x80Q\x86Q\x81R` \x80\x88\x01Q\x81\x83\x01R\x87\x83\x01Q\x82\x84\x01R``\x97\x88\x01Q\x88\x83\x01R\x86Q`\x80\x83\x01R\x80\x87\x01Q`\xA0\x83\x01R\x86\x83\x01Q`\xC0\x83\x01R\x95\x87\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R\x94\x84\x01Qa\x01 \x86\x01R\x83\x01Qa\x01@\x85\x01R\x93\x90\x91\x01Qa\x01`\x83\x01Ra\x01\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16a\x01\xA0\x82\x01Ra\x01\xC0\x01a\0\xBFV[a\0\xDBa\x02\xAA6`\x04a\x1C\x0EV[a\x07\xACV[a\0\xDBa\x02\xBD6`\x04a\x1C\x0EV[a\x08\x02V[a\x02\xD5a\x02\xD06`\x04a\x1D V[a\x08\xD2V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\0\xBFV[`\0Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBFV[a\x01-a\x03=6`\x04a\x1D\x07V[a\n\x83V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\\\x91\x90a\x1D\x8DV[\x92P\x92P\x92Pa\x03\x89\x83\x83\x83a\x03q\x8Aa\n\x83V[\x80` \x01\x90Q\x81\x01\x90a\x03\x84\x91\x90a\x1D\xBBV[a\x0B\xF4V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80\x80\x80\x80\x80a\x03\xAA\x89\x8B\x01\x8Ba\x1E\x11V[\x92P\x92P\x92P\x80\x93Pa\x03\xC6\x84\x8C`\xA0\x015\x8D``\x015a\r\x06V[\x95Pa\x03\xDB\x86\x8C``\x015\x8D`\x80\x015a\r\x06V[\x94P\x82\x86\x11\x15a\x04\rW`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x87\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x81\x85\x11\x15a\x048W`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x01a\x04\x04V[\x8Ba\x04sa\x04J\x88``\x8F\x015a\x1ESV[\x87\x8E`\x80\x015a\x04Z\x91\x90a\x1ESV[\x87\x8F`\xA0\x015a\x04j\x91\x90a\x1ESV[a\x03q\x85a\n\x83V[\x97P\x87a\x04\x80`\x14a\x1EfV[\x12\x80\x15a\x04\x8DWP`\x14\x88\x12[\x98PPPPP\x95P\x95P\x95P\x95P\x95\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xCBW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x01` R`@\x90 `\r\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x05\x08W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\x16\x82\x84\x01\x84a\x1E\x8FV[\x90P`\x01\x81`\x05\x81\x11\x15a\x05,Wa\x05,a\x1E\xACV[\x03a\x05\x87Wa\x05p\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r$\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\x0C\x01Ua\x07\xA4V[`\x03\x81`\x05\x81\x11\x15a\x05\x9BWa\x05\x9Ba\x1E\xACV[\x03a\x06\x07W`\0\x80a\x05\xE2\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90\x83\x83a\r]V[PPa\x07\xA4V[`\x04\x81`\x05\x81\x11\x15a\x06\x1BWa\x06\x1Ba\x1E\xACV[\x03a\x06\x83W`\0\x80a\x06b\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90`\x04\x01\x83\x83a\r]V[`\x02\x81`\x05\x81\x11\x15a\x06\x97Wa\x06\x97a\x1E\xACV[\x03a\x06\xFFW`\0\x80a\x06\xDE\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90`\x08\x01\x83\x83a\r]V[`\x05\x81`\x05\x81\x11\x15a\x07\x13Wa\x07\x13a\x1E\xACV[\x03a\x07\x8BWa\x07W\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r\xC9\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\r\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x07\xA4V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x80T\x81\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xE0W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xEB\x89\x88\x88a\r\xE6V[P\x93\x9E\x92\x9DP\x90\x9BP\x99P\x90\x97P\x95PPPPPPV[`\0\x80\x80\x80\x80\x80\x80\x80a\x08\x17\x89\x8B\x01\x8Ba\x1E\x11V[\x92P\x92P\x92P\x80\x93Pa\x083\x84\x8C`\xA0\x015\x8D``\x015a\r\x06V[\x95Pa\x08H\x86\x8C``\x015\x8D`\x80\x015a\r\x06V[\x94P\x85\x83\x11\x15a\x08uW`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x87\x90R`D\x01a\x04\x04V[\x84\x82\x11\x15a\x08\xA0W`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x01a\x04\x04V[\x8Ba\x04sa\x08\xB2\x88``\x8F\x015a\x1E\xC2V[\x87\x8E`\x80\x015a\x08\xC2\x91\x90a\x1E\xC2V[\x87\x8F`\xA0\x015a\x04j\x91\x90a\x1E\xC2V[`\0\x80`\0\x80`\0\x80`\0a\x08\xE6\x8Aa\n\x83V[\x80` \x01\x90Q\x81\x01\x90a\x08\xF9\x91\x90a\x1D\xBBV[\x90P\x87\x80` \x01\x90Q\x81\x01\x90a\t\x0F\x91\x90a\x1D\x8DV[\x91\x95P\x93P\x91P`\0\x80\x80``\x8C\x015\x87\x11\x15a\t\x85Wa\t4``\x8D\x015\x88a\x1E\xC2V[\x91Pa\tM\x84``\x01Q\x83a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\tt\x8C``\x015a\tn\x8E`\xA0\x015\x84a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0E\xB2V[a\t~\x90\x84a\x1ESV[\x92Pa\n:V[\x8B`\x80\x015\x86\x11\x15a\t\xD9Wa\t\x9F`\x80\x8D\x015\x87a\x1E\xC2V[\x91Pa\t\xB8\x84``\x01Q\x83a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\tt\x8C`\x80\x015a\tn\x8E`\xA0\x015\x84a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x04\x04V[a\nH`\xA0\x8D\x015\x86a\x1E\xD5V[\x97Pa\nV\x87\x87\x87\x87a\x0B\xF4V[\x98P\x88a\nc`\x14a\x1EfV[\x12\x80\x15a\npWP`\x14\x89\x12[\x99PPPPP\x94\x99\x93\x98P\x94P\x94P\x94PV[``a\n\x8Da\x1A\x93V[a\n\xDE`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x0E\xC7V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R`\x08\x82\x01T\x81R`\t\x82\x01T\x93\x81\x01\x93\x90\x93R`\n\x81\x01T\x91\x83\x01\x91\x90\x91R`\x0B\x01T``\x82\x01Ra\x0B/\x90a\x0E\xC7V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R`\x04\x82\x01T\x81R`\x05\x82\x01T\x92\x81\x01\x92\x90\x92R`\x06\x81\x01T\x92\x82\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x82\x01Ra\x0B~\x90a\x0E\xC7V[`@\x82\x81\x01\x91\x82R`\0\x85\x81R`\x01` \x90\x81R\x90\x82\x90 `\x0C\x01T``\x80\x86\x01\x91\x82R\x83Q\x86Q\x81\x85\x01R\x92\x86\x01Q\x93\x83\x01\x93\x90\x93R\x92Q\x91\x81\x01\x91\x90\x91R\x90Q`\x80\x80\x83\x01\x91\x90\x91R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x82\x85\x10a\x0CEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x04V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0C[\x88\x87a\x0FXV[\x10a\x0CoW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0C\x84V[a\x0C\x81a\x0C|\x88\x87a\x0FXV[a\x0FmV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xA4\x87a\x0C\x9F\x87`\0\x01Q\x89a\x10\x13V[a\x0FXV[\x10a\x0C\xB7WP`\x01`\x01`\xFF\x1B\x03a\x0C\xCFV[a\x0C\xCCa\x0C|\x87a\x0C\x9F\x87`\0\x01Q\x89a\x10\x13V[\x90P[`\0a\x0C\xE3\x85` \x01Q\x86`@\x01Qa\x10(V[\x90P\x80a\x0C\xF0\x83\x85a\x1E\xFCV[a\x0C\xFA\x91\x90a\x1E\xFCV[\x98\x97PPPPPPPPV[`\0a\r\x1Ca\r\x15\x85\x85a\x0FXV[\x83\x90a\x10\x13V[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\r\x1C\x91\x90a\x1F$V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\rR\x91\x90a\x1FRV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\r}W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\x86\x83a\x10NV[`\0a\r\x92B\x83a\x1E\xC2V[\x84T\x90\x91P`\0\x90a\r\xA4\x90\x85a\x1E\xD5V[\x90P`\0a\r\xB2\x83\x83a\x1F\x89V[`\x01\x87\x01\x94\x90\x94UPPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\r\xDF\x91\x90a\x1F\xC5V[\x93\x92PPPV[`\0\x80`\0\x80`\0a\r\xF6a\x1A\x93V[a\x0E\x02\x87\x89\x01\x89a\x1F\xFFV[` \x81\x81\x01Q`\0\x8F\x81R`\x01\x90\x92R`@\x91\x82\x90 \x90\x81U\x90\x82\x01Q`\x04\x82\x01U\x81Q`\x08\x82\x01U``\x82\x01Q`\x0C\x82\x01U`\x80\x82\x01Q`\r\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x92\x96P\x90\x94P\x92P\x90Pa\x0Eu\x84\x84\x84a\x03q\x8Da\n\x83V[\x94P\x84a\x0E\x82`\x14a\x1EfV[\x12\x80\x15a\x0E\x8FWP`\x14\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x90V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x90V[`\0\x81` \x01Q\x82``\x01Q\x03a\x0E\xDDWPQ\x90V[`\0\x82` \x01QB\x11a\x0E\xF0WBa\x0E\xF6V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0F\n\x91\x90a\x1E\xC2V[\x90P`\0\x84`@\x01Q\x13\x15a\x0F4W`@\x84\x01Qa\x0F(\x90\x82a \x87V[\x84Qa\r\x1C\x91\x90a\x1ESV[\x83`@\x01Qa\x0FB\x90a\x1EfV[a\x0FL\x90\x82a \x87V[\x84Qa\r\x1C\x91\x90a\x1E\xC2V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xBEV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0F\x86WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x0F\xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0F\xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xDC\x83`\x02a \x9EV[\x90P`\0a\x0F\xE9\x82a\x10\xDDV[\x90P`\0a\x0F\xFFg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x13[V[\x90Pa\x10\n\x81a\x1EfV[\x95\x94PPPPPV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xBEV[`\0\x80a\x104\x83a\x13pV[a\x10B\x90c;\x9A\xCA\0a \x87V[\x90Pa\r\x1C\x84\x82a\x10\x13V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x10\x85\x90a\x0E\xC7V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xA8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xD6W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x10\xF4WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x11\x12W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x113W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x11[W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x11fW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x11\x8EWa\x11\x89\x83g\x1B\xC1mgN\xC8\0\0a\x1E\xD5V[a\x11\x90V[\x82[\x90P`\0a\x11\xA6\x82g\x1B\xC1mgN\xC8\0\0a\x14\x14V[\x90P\x80`\0\x03a\x11\xC9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xD4\x82a\x14)V[\x90P`\0c;\x9A\xCA\0a\x11\xFFa\x11\xFAa\x11\xF4g\x1B\xC1mgN\xC8\0\0a\x1EfV[\x85a\x13[V[a\x13pV[a\x12\t\x91\x90a \x9EV[\x90P`\0\x80a\x12 \x83g\x03\xC1f\\z\xAB \0a\x13[V[a\x122\x90g \x05\xFEO&\x8E\xA0\0a\x1E\xFCV[\x90P`\0a\x12b\x84a\x12K\x86f\x9F2u$b\xA0\0a\x13[V[a\x12]\x90g\r\xC5R\x7Fd, \0a\x1E\xFCV[a\x13[V[a\x12t\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xFCV[\x90Pa\x12\x98g\t\xD0(\xCCo _\xFF\x19\x85a\x12\x8E\x85\x85a\x14\x14V[a\x12]\x91\x90a\x1E\xD5V[\x92PPP`\0[`\x02\x81\x10\x15a\x133W`\0\x86a\x12\xB4\x84a\x16\x04V[a\x12\xBE\x91\x90a\x1E\xD5V[\x90P`\0a\x12\xCC\x84\x85a\x13[V[a\x12\xD5\x90a\x1EfV[\x90P`\0a\x12\xE2\x82a\x17\xE8V[\x90P`\0a\x12\xF0\x86\x85a\x13[V[a\x13\x02g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x13[V[a\x13\x0C\x91\x90a\x1E\xD5V[\x90Pa\x13\x18\x84\x82a\x14\x14V[a\x13\"\x90\x87a\x1E\xFCV[\x95P\x84`\x01\x01\x94PPPPPa\x12\x9FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x13PWa\x13K\x82a\x1EfV[a\x0C\xFAV[P\x96\x95PPPPPPV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x91V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x13\x89W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x13\xA5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x13\xBDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x13\xD3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x19\x91V[`\0\x80\x82\x13a\x14fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\x04V[`\0``a\x14s\x84a\x19\xB0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x16\x1DWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x164WP`\0\x91\x90PV[a\x16EgV\x98\xEE\xF0fp\0\0a\x1EfV[\x82\x13a\x16ZWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16e\x83a\x1AXV[\x90P`\0a\x16\x9Eg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x87\x84g\x1B\xC1mgN\xC8\0\0a\x0FXV[a\x16\x99\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xFCV[a\x14\x14V[\x90P`\0\x80\x82a\x16\xFA\x81a\x16\xE7\x81a\x16\xD5\x81a\x16\xC2\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x13[V[a\x12]\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1E\xFCV[a\x12]\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1E\xFCV[a\x12]\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1E\xFCV[a\x17\x0C\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1E\xFCV[\x91P\x83\x90Pa\x17t\x81a\x17b\x81a\x17P\x81a\x17>\x81a\x17+\x81\x8Ba\x13[V[a\x12]\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1E\xFCV[a\x12]\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1E\xFCV[a\x12]\x90g\x051\n\xA7\xD5!0\0a\x1E\xFCV[a\x12]\x90g\r\xE0\xCC=\x15a\0\0a\x1E\xFCV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17\x8A\x87\x88a\x13[V[a\x17\x96\x90`\0\x19a \x9EV[a\x17\xA0\x91\x90a\x1E\xD5V[a\x17\xAA\x91\x90a\x1E\xFCV[\x92PP`\0a\x17\xB8\x83a\x17\xE8V[\x90P`\0a\x17\xC6\x85\x83a\x13[V[\x90P`\0\x88\x12a\x17\xD6W\x80a\x0C\xFAV[a\x0C\xFA\x81g\x1B\xC1mgN\xC8\0\0a\x1E\xD5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04\x04V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x19\xA9W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x19\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\x04V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1A~W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1A\x8FWP\x19`\x01\x01\x90V[P\x90V[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1B\x04Wa\x1B\x04a\x1A\xCBV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x1B\x1BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B6Wa\x1B6a\x1A\xCBV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1B^Wa\x1B^a\x1A\xCBV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x1BwW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xAAW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xC8W`\0\x80\xFD[a\x1B\xD4\x85\x82\x86\x01a\x1B\nV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF3W`\0\x80\xFD[PV[`\0`\xE0\x82\x84\x03\x12\x15a\x1C\x08W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\0a\x01@\x86\x88\x03\x12\x15a\x1C'W`\0\x80\xFD[\x855a\x1C2\x81a\x1B\xDEV[\x94P` \x86\x015\x93Pa\x1CH\x87`@\x88\x01a\x1B\xF6V[\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1CfW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1CzW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\x89W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x1C\x9BW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1C\xD4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1C\xB8V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\r\xDF` \x83\x01\x84a\x1C\xAEV[`\0` \x82\x84\x03\x12\x15a\x1D\x19W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80a\x01@\x85\x87\x03\x12\x15a\x1D7W`\0\x80\xFD[\x845a\x1DB\x81a\x1B\xDEV[\x93P` \x85\x015\x92Pa\x1DX\x86`@\x87\x01a\x1B\xF6V[\x91Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DuW`\0\x80\xFD[a\x1D\x81\x87\x82\x88\x01a\x1B\nV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xA2W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\xA0\x82\x84\x03\x12\x15a\x1D\xCDW`\0\x80\xFD[a\x1D\xD5a\x1A\xE1V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Qa\x1E\x05\x81a\x1B\xDEV[`\x80\x82\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E&W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x8FWa\x03\x8Fa\x1E=V[`\0`\x01`\xFF\x1B\x82\x01a\x1E{Wa\x1E{a\x1E=V[P`\0\x03\x90V[`\x06\x81\x10a\x1B\xF3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1E\xA1W`\0\x80\xFD[\x815a\r\xDF\x81a\x1E\x82V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x8FWa\x03\x8Fa\x1E=V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1E\xF5Wa\x1E\xF5a\x1E=V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1F\x1CWa\x1F\x1Ca\x1E=V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F7W`\0\x80\xFD[\x82Qa\x1FB\x81a\x1E\x82V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1FgW`\0\x80\xFD[\x83Qa\x1Fr\x81a\x1E\x82V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82a\x1F\xA6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1F\xC0Wa\x1F\xC0a\x1E=V[P\x05\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xD8W`\0\x80\xFD[\x82Qa\x1F\xE3\x81a\x1E\x82V[` \x84\x01Q\x90\x92Pa\x1F\xF4\x81a\x1B\xDEV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a \x17W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\xA0`_\x19\x82\x01\x12\x15a ;W`\0\x80\xFD[Pa Da\x1A\xE1V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R`\xE0\x86\x015a w\x81a\x1B\xDEV[`\x80\x82\x01R\x93\x96\x92\x95P\x90\x93PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x8FWa\x03\x8Fa\x1E=V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a \xBAWa \xBAa\x1E=V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x8FWa\x03\x8Fa\x1E=V\xFE\xA2dipfsX\"\x12 \x92\xEB\x1D\x8F\r\xFD\xADC\x0B:\x8D\xF85\x05V\xFF|\x9B\xBD\x866\xF1\x1C\xEC\xDD\xEC<z\x0B\xCD\xBDAdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\x9DW`\x005`\xE0\x1C\x80c2\xE3\x830\x11a\0fW\x80c2\xE3\x830\x14a\x02\x9CW\x80c9n>|\x14a\x02\xAFW\x80ce\xC9\xFF\xC2\x14a\x02\xC2W\x80c\xAF\xBA\x13\xC4\x14a\x03\x04W\x80c\xDC\x17\x83U\x14a\x03/W`\0\x80\xFD[\x80b.RK\x14a\0\xA2W\x80c\x02J\xA2\x06\x14a\0\xC8W\x80c\x06\xFD\xDE\x03\x14a\x01\x05W\x80c\x18\x1C\xBA\xB4\x14a\x01:W\x80c\x1E\xDBq\xE5\x14a\x01OW[`\0\x80\xFD[a\0\xB5a\0\xB06`\x04a\x1B\x97V[a\x03BV[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\0\xDBa\0\xD66`\x04a\x1C\x0EV[a\x03\x95V[`@\x80Q\x95\x15\x15\x86R` \x86\x01\x94\x90\x94R\x92\x84\x01\x91\x90\x91R``\x83\x01R`\x80\x82\x01R`\xA0\x01a\0\xBFV[a\x01-`@Q\x80`@\x01`@R\x80`\t\x81R` \x01h\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B\x81RP\x81V[`@Qa\0\xBF\x91\x90a\x1C\xF4V[a\x01Ma\x01H6`\x04a\x1C\x0EV[a\x04\xA0V[\0[a\x02\x10a\x01]6`\x04a\x1D\x07V[`\x01` \x81\x81R`\0\x92\x83R`@\x92\x83\x90 \x83Q`\x80\x80\x82\x01\x86R\x82T\x82R\x93\x82\x01T\x81\x84\x01R`\x02\x82\x01T\x81\x86\x01R`\x03\x82\x01T``\x80\x83\x01\x91\x90\x91R\x85Q\x80\x86\x01\x87R`\x04\x84\x01T\x81R`\x05\x84\x01T\x81\x86\x01R`\x06\x84\x01T\x81\x88\x01R`\x07\x84\x01T\x81\x83\x01R\x86Q\x95\x86\x01\x87R`\x08\x84\x01T\x86R`\t\x84\x01T\x94\x86\x01\x94\x90\x94R`\n\x83\x01T\x95\x85\x01\x95\x90\x95R`\x0B\x82\x01T\x94\x84\x01\x94\x90\x94R`\x0C\x81\x01T`\r\x90\x91\x01T\x91\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x85V[`@\x80Q\x86Q\x81R` \x80\x88\x01Q\x81\x83\x01R\x87\x83\x01Q\x82\x84\x01R``\x97\x88\x01Q\x88\x83\x01R\x86Q`\x80\x83\x01R\x80\x87\x01Q`\xA0\x83\x01R\x86\x83\x01Q`\xC0\x83\x01R\x95\x87\x01Q`\xE0\x82\x01R\x84Qa\x01\0\x82\x01R\x94\x84\x01Qa\x01 \x86\x01R\x83\x01Qa\x01@\x85\x01R\x93\x90\x91\x01Qa\x01`\x83\x01Ra\x01\x80\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16a\x01\xA0\x82\x01Ra\x01\xC0\x01a\0\xBFV[a\0\xDBa\x02\xAA6`\x04a\x1C\x0EV[a\x07\xACV[a\0\xDBa\x02\xBD6`\x04a\x1C\x0EV[a\x08\x02V[a\x02\xD5a\x02\xD06`\x04a\x1D V[a\x08\xD2V[`@\x80Q\x96\x15\x15\x87R` \x87\x01\x95\x90\x95R\x93\x85\x01\x92\x90\x92R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R`\xC0\x01a\0\xBFV[`\0Ta\x03\x17\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xBFV[a\x01-a\x03=6`\x04a\x1D\x07V[a\n\x83V[`\0\x80`\0\x80\x84\x80` \x01\x90Q\x81\x01\x90a\x03\\\x91\x90a\x1D\x8DV[\x92P\x92P\x92Pa\x03\x89\x83\x83\x83a\x03q\x8Aa\n\x83V[\x80` \x01\x90Q\x81\x01\x90a\x03\x84\x91\x90a\x1D\xBBV[a\x0B\xF4V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80\x80\x80\x80\x80a\x03\xAA\x89\x8B\x01\x8Ba\x1E\x11V[\x92P\x92P\x92P\x80\x93Pa\x03\xC6\x84\x8C`\xA0\x015\x8D``\x015a\r\x06V[\x95Pa\x03\xDB\x86\x8C``\x015\x8D`\x80\x015a\r\x06V[\x94P\x82\x86\x11\x15a\x04\rW`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x87\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[\x81\x85\x11\x15a\x048W`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x01a\x04\x04V[\x8Ba\x04sa\x04J\x88``\x8F\x015a\x1ESV[\x87\x8E`\x80\x015a\x04Z\x91\x90a\x1ESV[\x87\x8F`\xA0\x015a\x04j\x91\x90a\x1ESV[a\x03q\x85a\n\x83V[\x97P\x87a\x04\x80`\x14a\x1EfV[\x12\x80\x15a\x04\x8DWP`\x14\x88\x12[\x98PPPPP\x95P\x95P\x95P\x95P\x95\x90PV[`\0T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xCBW`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x84\x81R`\x01` R`@\x90 `\r\x01T`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x91\x16\x14a\x05\x08W`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x05\x16\x82\x84\x01\x84a\x1E\x8FV[\x90P`\x01\x81`\x05\x81\x11\x15a\x05,Wa\x05,a\x1E\xACV[\x03a\x05\x87Wa\x05p\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r$\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\x0C\x01Ua\x07\xA4V[`\x03\x81`\x05\x81\x11\x15a\x05\x9BWa\x05\x9Ba\x1E\xACV[\x03a\x06\x07W`\0\x80a\x05\xE2\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90\x83\x83a\r]V[PPa\x07\xA4V[`\x04\x81`\x05\x81\x11\x15a\x06\x1BWa\x06\x1Ba\x1E\xACV[\x03a\x06\x83W`\0\x80a\x06b\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90`\x04\x01\x83\x83a\r]V[`\x02\x81`\x05\x81\x11\x15a\x06\x97Wa\x06\x97a\x1E\xACV[\x03a\x06\xFFW`\0\x80a\x06\xDE\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r;\x92PPPV[`\0\x89\x81R`\x01` R`@\x90 \x91\x93P\x91Pa\x06\0\x90`\x08\x01\x83\x83a\r]V[`\x05\x81`\x05\x81\x11\x15a\x07\x13Wa\x07\x13a\x1E\xACV[\x03a\x07\x8BWa\x07W\x83\x83\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\r\xC9\x92PPPV[`\0\x86\x81R`\x01` R`@\x90 `\r\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x07\xA4V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPV[`\0\x80T\x81\x90\x81\x90\x81\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x163\x14a\x07\xE0W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x07\xEB\x89\x88\x88a\r\xE6V[P\x93\x9E\x92\x9DP\x90\x9BP\x99P\x90\x97P\x95PPPPPPV[`\0\x80\x80\x80\x80\x80\x80\x80a\x08\x17\x89\x8B\x01\x8Ba\x1E\x11V[\x92P\x92P\x92P\x80\x93Pa\x083\x84\x8C`\xA0\x015\x8D``\x015a\r\x06V[\x95Pa\x08H\x86\x8C``\x015\x8D`\x80\x015a\r\x06V[\x94P\x85\x83\x11\x15a\x08uW`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`$\x81\x01\x87\x90R`D\x01a\x04\x04V[\x84\x82\x11\x15a\x08\xA0W`@Qcmh_\xA7`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x86\x90R`D\x01a\x04\x04V[\x8Ba\x04sa\x08\xB2\x88``\x8F\x015a\x1E\xC2V[\x87\x8E`\x80\x015a\x08\xC2\x91\x90a\x1E\xC2V[\x87\x8F`\xA0\x015a\x04j\x91\x90a\x1E\xC2V[`\0\x80`\0\x80`\0\x80`\0a\x08\xE6\x8Aa\n\x83V[\x80` \x01\x90Q\x81\x01\x90a\x08\xF9\x91\x90a\x1D\xBBV[\x90P\x87\x80` \x01\x90Q\x81\x01\x90a\t\x0F\x91\x90a\x1D\x8DV[\x91\x95P\x93P\x91P`\0\x80\x80``\x8C\x015\x87\x11\x15a\t\x85Wa\t4``\x8D\x015\x88a\x1E\xC2V[\x91Pa\tM\x84``\x01Q\x83a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\tt\x8C``\x015a\tn\x8E`\xA0\x015\x84a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0E\xB2V[a\t~\x90\x84a\x1ESV[\x92Pa\n:V[\x8B`\x80\x015\x86\x11\x15a\t\xD9Wa\t\x9F`\x80\x8D\x015\x87a\x1E\xC2V[\x91Pa\t\xB8\x84``\x01Q\x83a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\tt\x8C`\x80\x015a\tn\x8E`\xA0\x015\x84a\x0E\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x01a\x04\x04V[a\nH`\xA0\x8D\x015\x86a\x1E\xD5V[\x97Pa\nV\x87\x87\x87\x87a\x0B\xF4V[\x98P\x88a\nc`\x14a\x1EfV[\x12\x80\x15a\npWP`\x14\x89\x12[\x99PPPPP\x94\x99\x93\x98P\x94P\x94P\x94PV[``a\n\x8Da\x1A\x93V[a\n\xDE`\x01`\0\x85\x81R` \x01\x90\x81R` \x01`\0 `\0\x01`@Q\x80`\x80\x01`@R\x90\x81`\0\x82\x01T\x81R` \x01`\x01\x82\x01T\x81R` \x01`\x02\x82\x01T\x81R` \x01`\x03\x82\x01T\x81RPPa\x0E\xC7V[` \x80\x83\x01\x91\x90\x91R`\0\x84\x81R`\x01\x82R`@\x90\x81\x90 \x81Q`\x80\x81\x01\x83R`\x08\x82\x01T\x81R`\t\x82\x01T\x93\x81\x01\x93\x90\x93R`\n\x81\x01T\x91\x83\x01\x91\x90\x91R`\x0B\x01T``\x82\x01Ra\x0B/\x90a\x0E\xC7V[\x81R`\0\x83\x81R`\x01` \x90\x81R`@\x91\x82\x90 \x82Q`\x80\x81\x01\x84R`\x04\x82\x01T\x81R`\x05\x82\x01T\x92\x81\x01\x92\x90\x92R`\x06\x81\x01T\x92\x82\x01\x92\x90\x92R`\x07\x90\x91\x01T``\x82\x01Ra\x0B~\x90a\x0E\xC7V[`@\x82\x81\x01\x91\x82R`\0\x85\x81R`\x01` \x90\x81R\x90\x82\x90 `\x0C\x01T``\x80\x86\x01\x91\x82R\x83Q\x86Q\x81\x85\x01R\x92\x86\x01Q\x93\x83\x01\x93\x90\x93R\x92Q\x91\x81\x01\x91\x90\x91R\x90Q`\x80\x80\x83\x01\x91\x90\x91R\x82\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x91\x90PV[`\0\x82\x85\x10a\x0CEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x04\x04V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x0C[\x88\x87a\x0FXV[\x10a\x0CoW`\x01`\x01`\xFF\x1B\x03\x91Pa\x0C\x84V[a\x0C\x81a\x0C|\x88\x87a\x0FXV[a\x0FmV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xA4\x87a\x0C\x9F\x87`\0\x01Q\x89a\x10\x13V[a\x0FXV[\x10a\x0C\xB7WP`\x01`\x01`\xFF\x1B\x03a\x0C\xCFV[a\x0C\xCCa\x0C|\x87a\x0C\x9F\x87`\0\x01Q\x89a\x10\x13V[\x90P[`\0a\x0C\xE3\x85` \x01Q\x86`@\x01Qa\x10(V[\x90P\x80a\x0C\xF0\x83\x85a\x1E\xFCV[a\x0C\xFA\x91\x90a\x1E\xFCV[\x98\x97PPPPPPPPV[`\0a\r\x1Ca\r\x15\x85\x85a\x0FXV[\x83\x90a\x10\x13V[\x94\x93PPPPV[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\r\x1C\x91\x90a\x1F$V[`\0\x80\x82\x80` \x01\x90Q\x81\x01\x90a\rR\x91\x90a\x1FRV[\x90\x95\x90\x94P\x92PPPV[B\x81\x11a\r}W`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\r\x86\x83a\x10NV[`\0a\r\x92B\x83a\x1E\xC2V[\x84T\x90\x91P`\0\x90a\r\xA4\x90\x85a\x1E\xD5V[\x90P`\0a\r\xB2\x83\x83a\x1F\x89V[`\x01\x87\x01\x94\x90\x94UPPP`\x02\x90\x92\x01\x91\x90\x91UPV[`\0\x81\x80` \x01\x90Q\x81\x01\x90a\r\xDF\x91\x90a\x1F\xC5V[\x93\x92PPPV[`\0\x80`\0\x80`\0a\r\xF6a\x1A\x93V[a\x0E\x02\x87\x89\x01\x89a\x1F\xFFV[` \x81\x81\x01Q`\0\x8F\x81R`\x01\x90\x92R`@\x91\x82\x90 \x90\x81U\x90\x82\x01Q`\x04\x82\x01U\x81Q`\x08\x82\x01U``\x82\x01Q`\x0C\x82\x01U`\x80\x82\x01Q`\r\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x92\x96P\x90\x94P\x92P\x90Pa\x0Eu\x84\x84\x84a\x03q\x8Da\n\x83V[\x94P\x84a\x0E\x82`\x14a\x1EfV[\x12\x80\x15a\x0E\x8FWP`\x14\x85\x12[\x95P\x93\x97P\x93\x97\x91\x95P\x93PV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x90V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x90V[`\0\x81` \x01Q\x82``\x01Q\x03a\x0E\xDDWPQ\x90V[`\0\x82` \x01QB\x11a\x0E\xF0WBa\x0E\xF6V[\x82` \x01Q[\x90P`\0\x83``\x01Q\x82a\x0F\n\x91\x90a\x1E\xC2V[\x90P`\0\x84`@\x01Q\x13\x15a\x0F4W`@\x84\x01Qa\x0F(\x90\x82a \x87V[\x84Qa\r\x1C\x91\x90a\x1ESV[\x83`@\x01Qa\x0FB\x90a\x1EfV[a\x0FL\x90\x82a \x87V[\x84Qa\r\x1C\x91\x90a\x1E\xC2V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\xBEV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x0F\x86WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x0F\xAEW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x0F\xCFW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x0F\xDC\x83`\x02a \x9EV[\x90P`\0a\x0F\xE9\x82a\x10\xDDV[\x90P`\0a\x0F\xFFg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x13[V[\x90Pa\x10\n\x81a\x1EfV[\x95\x94PPPPPV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\xBEV[`\0\x80a\x104\x83a\x13pV[a\x10B\x90c;\x9A\xCA\0a \x87V[\x90Pa\r\x1C\x84\x82a\x10\x13V[`@\x80Q`\x80\x81\x01\x82R\x82T\x81R`\x01\x83\x01T` \x82\x01R`\x02\x83\x01T\x91\x81\x01\x91\x90\x91R`\x03\x82\x01T``\x82\x01Ra\x10\x85\x90a\x0E\xC7V[\x81UB`\x03\x90\x91\x01UV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xA8W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xD6W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x12\x80a\x10\xF4WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x11\x12W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x113W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x11[W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x11fW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x11\x8EWa\x11\x89\x83g\x1B\xC1mgN\xC8\0\0a\x1E\xD5V[a\x11\x90V[\x82[\x90P`\0a\x11\xA6\x82g\x1B\xC1mgN\xC8\0\0a\x14\x14V[\x90P\x80`\0\x03a\x11\xC9W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xD4\x82a\x14)V[\x90P`\0c;\x9A\xCA\0a\x11\xFFa\x11\xFAa\x11\xF4g\x1B\xC1mgN\xC8\0\0a\x1EfV[\x85a\x13[V[a\x13pV[a\x12\t\x91\x90a \x9EV[\x90P`\0\x80a\x12 \x83g\x03\xC1f\\z\xAB \0a\x13[V[a\x122\x90g \x05\xFEO&\x8E\xA0\0a\x1E\xFCV[\x90P`\0a\x12b\x84a\x12K\x86f\x9F2u$b\xA0\0a\x13[V[a\x12]\x90g\r\xC5R\x7Fd, \0a\x1E\xFCV[a\x13[V[a\x12t\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xFCV[\x90Pa\x12\x98g\t\xD0(\xCCo _\xFF\x19\x85a\x12\x8E\x85\x85a\x14\x14V[a\x12]\x91\x90a\x1E\xD5V[\x92PPP`\0[`\x02\x81\x10\x15a\x133W`\0\x86a\x12\xB4\x84a\x16\x04V[a\x12\xBE\x91\x90a\x1E\xD5V[\x90P`\0a\x12\xCC\x84\x85a\x13[V[a\x12\xD5\x90a\x1EfV[\x90P`\0a\x12\xE2\x82a\x17\xE8V[\x90P`\0a\x12\xF0\x86\x85a\x13[V[a\x13\x02g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x13[V[a\x13\x0C\x91\x90a\x1E\xD5V[\x90Pa\x13\x18\x84\x82a\x14\x14V[a\x13\"\x90\x87a\x1E\xFCV[\x95P\x84`\x01\x01\x94PPPPPa\x12\x9FV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x13PWa\x13K\x82a\x1EfV[a\x0C\xFAV[P\x96\x95PPPPPPV[`\0a\r\xDF\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19\x91V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x13\x89W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x13\xA5W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x13\xBDW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x13\xD3W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\r\xDF\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x19\x91V[`\0\x80\x82\x13a\x14fW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\x04V[`\0``a\x14s\x84a\x19\xB0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0\x81`\0\x03a\x16\x1DWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x164WP`\0\x91\x90PV[a\x16EgV\x98\xEE\xF0fp\0\0a\x1EfV[\x82\x13a\x16ZWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16e\x83a\x1AXV[\x90P`\0a\x16\x9Eg\r\xE0\xB6\xB3\xA7d\0\0a\x16\x87\x84g\x1B\xC1mgN\xC8\0\0a\x0FXV[a\x16\x99\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xFCV[a\x14\x14V[\x90P`\0\x80\x82a\x16\xFA\x81a\x16\xE7\x81a\x16\xD5\x81a\x16\xC2\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x13[V[a\x12]\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1E\xFCV[a\x12]\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1E\xFCV[a\x12]\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1E\xFCV[a\x17\x0C\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1E\xFCV[\x91P\x83\x90Pa\x17t\x81a\x17b\x81a\x17P\x81a\x17>\x81a\x17+\x81\x8Ba\x13[V[a\x12]\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1E\xFCV[a\x12]\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1E\xFCV[a\x12]\x90g\x051\n\xA7\xD5!0\0a\x1E\xFCV[a\x12]\x90g\r\xE0\xCC=\x15a\0\0a\x1E\xFCV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17\x8A\x87\x88a\x13[V[a\x17\x96\x90`\0\x19a \x9EV[a\x17\xA0\x91\x90a\x1E\xD5V[a\x17\xAA\x91\x90a\x1E\xFCV[\x92PP`\0a\x17\xB8\x83a\x17\xE8V[\x90P`\0a\x17\xC6\x85\x83a\x13[V[\x90P`\0\x88\x12a\x17\xD6W\x80a\x0C\xFAV[a\x0C\xFA\x81g\x1B\xC1mgN\xC8\0\0a\x1E\xD5V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18\x03WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04\x04V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x19\xA9W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\x19\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\x04V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0`\x01`\xFF\x1B\x82\x03a\x1A~W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1A\x8FWP\x19`\x01\x01\x90V[P\x90V[`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x1B\x04Wa\x1B\x04a\x1A\xCBV[`@R\x90V[`\0\x82`\x1F\x83\x01\x12a\x1B\x1BW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B6Wa\x1B6a\x1A\xCBV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\x1B^Wa\x1B^a\x1A\xCBV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\x1BwW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\xAAW`\0\x80\xFD[\x825\x91P` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1B\xC8W`\0\x80\xFD[a\x1B\xD4\x85\x82\x86\x01a\x1B\nV[\x91PP\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF3W`\0\x80\xFD[PV[`\0`\xE0\x82\x84\x03\x12\x15a\x1C\x08W`\0\x80\xFD[P\x91\x90PV[`\0\x80`\0\x80`\0a\x01@\x86\x88\x03\x12\x15a\x1C'W`\0\x80\xFD[\x855a\x1C2\x81a\x1B\xDEV[\x94P` \x86\x015\x93Pa\x1CH\x87`@\x88\x01a\x1B\xF6V[\x92Pa\x01 \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1CfW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a\x1CzW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x1C\x89W`\0\x80\xFD[\x89` \x82\x85\x01\x01\x11\x15a\x1C\x9BW`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96P` \x01\x94\x93\x92PPPV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x1C\xD4W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x1C\xB8V[P`\0` \x82\x86\x01\x01R` `\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x91PP\x92\x91PPV[` \x81R`\0a\r\xDF` \x83\x01\x84a\x1C\xAEV[`\0` \x82\x84\x03\x12\x15a\x1D\x19W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0\x80a\x01@\x85\x87\x03\x12\x15a\x1D7W`\0\x80\xFD[\x845a\x1DB\x81a\x1B\xDEV[\x93P` \x85\x015\x92Pa\x1DX\x86`@\x87\x01a\x1B\xF6V[\x91Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x1DuW`\0\x80\xFD[a\x1D\x81\x87\x82\x88\x01a\x1B\nV[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1D\xA2W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0`\xA0\x82\x84\x03\x12\x15a\x1D\xCDW`\0\x80\xFD[a\x1D\xD5a\x1A\xE1V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Q`@\x82\x01R``\x83\x01Q``\x82\x01R`\x80\x83\x01Qa\x1E\x05\x81a\x1B\xDEV[`\x80\x82\x01R\x93\x92PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1E&W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x8FWa\x03\x8Fa\x1E=V[`\0`\x01`\xFF\x1B\x82\x01a\x1E{Wa\x1E{a\x1E=V[P`\0\x03\x90V[`\x06\x81\x10a\x1B\xF3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x1E\xA1W`\0\x80\xFD[\x815a\r\xDF\x81a\x1E\x82V[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\x8FWa\x03\x8Fa\x1E=V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1E\xF5Wa\x1E\xF5a\x1E=V[P\x92\x91PPV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1F\x1CWa\x1F\x1Ca\x1E=V[PP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x1F7W`\0\x80\xFD[\x82Qa\x1FB\x81a\x1E\x82V[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1FgW`\0\x80\xFD[\x83Qa\x1Fr\x81a\x1E\x82V[` \x85\x01Q`@\x90\x95\x01Q\x90\x96\x94\x95P\x93\x92PPPV[`\0\x82a\x1F\xA6WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1F\xC0Wa\x1F\xC0a\x1E=V[P\x05\x90V[`\0\x80`@\x83\x85\x03\x12\x15a\x1F\xD8W`\0\x80\xFD[\x82Qa\x1F\xE3\x81a\x1E\x82V[` \x84\x01Q\x90\x92Pa\x1F\xF4\x81a\x1B\xDEV[\x80\x91PP\x92P\x92\x90PV[`\0\x80`\0\x80\x84\x86\x03a\x01\0\x81\x12\x15a \x17W`\0\x80\xFD[\x855\x94P` \x86\x015\x93P`@\x86\x015\x92P`\xA0`_\x19\x82\x01\x12\x15a ;W`\0\x80\xFD[Pa Da\x1A\xE1V[``\x86\x015\x81R`\x80\x86\x015` \x82\x01R`\xA0\x86\x015`@\x82\x01R`\xC0\x86\x015``\x82\x01R`\xE0\x86\x015a w\x81a\x1B\xDEV[`\x80\x82\x01R\x93\x96\x92\x95P\x90\x93PPV[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x03\x8FWa\x03\x8Fa\x1E=V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a \xBAWa \xBAa\x1E=V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x8FWa\x03\x8Fa\x1E=V\xFE\xA2dipfsX\"\x12 \x92\xEB\x1D\x8F\r\xFD\xADC\x0B:\x8D\xF85\x05V\xFF|\x9B\xBD\x866\xF1\x1C\xEC\xDD\xEC<z\x0B\xCD\xBDAdsolcC\0\x08\x16\x003";
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
                DynamicParam,
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
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
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
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
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
    pub enum LogNormalCalls {
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
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
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
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
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
    impl ::core::fmt::Display for LogNormalCalls {
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
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
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
        pub sigma: DynamicParam,
        pub tau: DynamicParam,
        pub strike: DynamicParam,
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

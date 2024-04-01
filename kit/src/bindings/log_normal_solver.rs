pub use log_normal_solver::*;
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
pub mod log_normal_solver {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("_strategy"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("BISECTION_EPSILON"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("MAX_BISECTION_ITERS",),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocateGivenDeltaX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenDeltaX",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![
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
                    ::std::borrow::ToOwned::to_owned("allocateGivenDeltaY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenDeltaY",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                        ],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
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
                    ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct LogNormalParams"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("S"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
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
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct LogNormalParams"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceGivenXL"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rx"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPriceGivenYL"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("ry"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("L"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareControllerUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareControllerUpdate",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("controller"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("swapFee"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareMeanUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareMeanUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetMean"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareWidthUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareWidthUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetWidth"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapXIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("strategy"),
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
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lower"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upper"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("lowerResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("upperResult"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("int256"),
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
    pub static LOGNORMALSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0-\xC58\x03\x80b\0-\xC5\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0ZV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\0\x8CV[`\0` \x82\x84\x03\x12\x15b\0\0mW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x85W`\0\x80\xFD[\x93\x92PPPV[a-)\x80b\0\0\x9C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xC3W\x80c\xDE^\xE1\xC3\x11a\0|W\x80c\xDE^\xE1\xC3\x14a\x03!W\x80c\xDE\xF1_\x92\x14a\x034W\x80c\xEA\xAE\x17\xBA\x14a\x01RW\x80c\xEE>\x8C\xFB\x14a\x03GW\x80c\xF3\r7\xF2\x14a\x03ZW\x80c\xF9\xC2\x82\x11\x14a\x03mW`\0\x80\xFD[\x80c\xA8\xC6.v\x14a\x02|W\x80c\xAFNC\x7F\x14a\x02\xA7W\x80c\xB0\x9D\x04\xE5\x14a\x02\xBAW\x80c\xCB\x1FU2\x14a\x02\xCDW\x80c\xCE\x15;\xF4\x14a\x02\xE0W\x80c\xDC\x17\x83U\x14a\x03\x01W`\0\x80\xFD[\x80cN\x81\x7F\xD9\x11a\x01\x15W\x80cN\x81\x7F\xD9\x14a\x01\xE5W\x80c^\xB4\x08\xFC\x14a\x01\xF8W\x80cb7V\x9F\x14a\x02\x0BW\x80cme\"\x99\x14a\x029W\x80c}\xDA\x1A#\x14a\x02AW\x80c\x7F\x17@\x9C\x14a\x02iW`\0\x80\xFD[\x80c\x0F\x85z\xB9\x14a\x01RW\x80c\x12\x06I\xC5\x14a\x01{W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9CW\x80c9(\xFF\x97\x14a\x01\xAFW\x80c;M\x100\x14a\x01\xD2W[`\0\x80\xFD[a\x01ea\x01`6`\x04a#&V[a\x03uV[`@Qa\x01r\x91\x90a#\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ea\x01\x896`\x04a#\xABV[a\x03\x8AV[`@Q\x90\x81R` \x01a\x01rV[a\x01\x8Ea\x01\xAA6`\x04a#\xDDV[a\x03\xD2V[a\x01\xC2a\x01\xBD6`\x04a$\x1AV[a\x03\xE7V[`@Qa\x01r\x94\x93\x92\x91\x90a$RV[a\x01\x8Ea\x01\xE06`\x04a$yV[a\t\rV[a\x01\x8Ea\x01\xF36`\x04a#\xDDV[a\tGV[a\x01\x8Ea\x02\x066`\x04a#\xABV[a\t\\V[a\x02\x1Ea\x02\x196`\x04a#&V[a\t\x96V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01rV[a\x01\x8E`\0\x81V[a\x02Ta\x02O6`\x04a#&V[a\n\tV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01rV[a\x02\x1Ea\x02w6`\x04a#&V[a\n\x9CV[`\0Ta\x02\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01rV[a\x01\x8Ea\x02\xB56`\x04a#\xABV[a\x0B\0V[a\x01ea\x02\xC86`\x04a$yV[a\x0B6V[a\x01ea\x02\xDB6`\x04a$\xA7V[a\x0BAV[a\x02\xF3a\x02\xEE6`\x04a$yV[a\x0BLV[`@Qa\x01r\x92\x91\x90a%\x07V[a\x03\x14a\x03\x0F6`\x04a$yV[a\x0CNV[`@Qa\x01r\x91\x90a%UV[a\x02Ta\x03/6`\x04a#&V[a\r\x06V[a\x01ea\x03B6`\x04a%\xF6V[a\rkV[a\x02\x1Ea\x03U6`\x04a#&V[a\rxV[a\x02\x1Ea\x03h6`\x04a#&V[a\r\xA9V[a\x01\x8E`x\x81V[``a\x03\x81\x83\x83a\r\xDAV[\x90P[\x92\x91PPV[`\0\x80a\x03\x96\x86a\x0CNV[\x90P`\0a\x03\xA5\x85\x85\x84a\x0E\tV[\x90P`\0a\x03\xB5\x87\x83\x88\x86a\x0EPV[\x90Pa\x03\xC4\x87\x87\x83\x85\x87a\x0E\xA9V[\x93PPPP[\x94\x93PPPPV[`\0a\x03\xCA\x83\x83a\x03\xE2\x87a\x0CNV[a\x0F\x82V[`\0\x80`\0``a\x04\x12`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80a\x04\x1E\x8Aa\x0BLV[\x91P\x91P`\0a\x04-\x8Ba\x0CNV[\x90Pa\x04S`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x04\x95\x8D\x86`\0\x81Q\x81\x10a\x04lWa\x04la&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x04\x87Wa\x04\x87a&lV[` \x02` \x01\x01Q\x87a\x0B\0V[\x90P\x8B\x15a\x06\x05Wa\x04\xDE\x8B\x86`\0\x81Q\x81\x10a\x04\xB4Wa\x04\xB4a&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x04\xCFWa\x04\xCFa&lV[` \x02` \x01\x01Q\x87\x87a\x0F\xF2V[` \x83\x01R\x84Q\x8B\x90\x86\x90`\0\x90a\x04\xF8Wa\x04\xF8a&lV[` \x02` \x01\x01Qa\x05\n\x91\x90a&\x98V[\x86R` \x82\x01Qa\x05\x1B\x90\x82a&\x98V[\x86`@\x01\x81\x81RPP`\0a\x059\x8E\x88`\0\x01Q\x89`@\x01Qa\x03\xD2V[\x90Pa\x05O\x8E\x88`\0\x01Q\x89`@\x01Q\x84a\x03\x8AV[` \x88\x01R\x85Q\x86\x90`\x01\x90\x81\x10a\x05iWa\x05ia&lV[` \x02` \x01\x01Q\x87` \x01Q\x10a\x05\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x86`\x01\x81Q\x81\x10a\x05\xEBWa\x05\xEBa&lV[` \x02` \x01\x01Qa\x05\xFD\x91\x90a&\xABV[\x83RPa\x07fV[a\x06F\x8B\x86`\0\x81Q\x81\x10a\x06\x1CWa\x06\x1Ca&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x067Wa\x067a&lV[` \x02` \x01\x01Q\x87\x87a\x10UV[` \x83\x01R\x84Q\x8B\x90\x86\x90`\x01\x90\x81\x10a\x06bWa\x06ba&lV[` \x02` \x01\x01Qa\x06t\x91\x90a&\x98V[` \x80\x88\x01\x91\x90\x91R\x82\x01Qa\x06\x8A\x90\x82a&\x98V[\x86`@\x01\x81\x81RPP`\0a\x06\xA8\x8E\x88` \x01Q\x89`@\x01Qa\tGV[\x90Pa\x06\xBE\x8E\x88` \x01Q\x89`@\x01Q\x84a\t\\V[\x87R\x85Q\x86\x90`\0\x90a\x06\xD3Wa\x06\xD3a&lV[` \x02` \x01\x01Q\x87`\0\x01Q\x10a\x078W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xCAV[\x86`\0\x01Q\x86`\0\x81Q\x81\x10a\x07PWa\x07Pa&lV[` \x02` \x01\x01Qa\x07b\x91\x90a&\xABV[\x83RP[Pa\x07\xC2`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`@\x81\x01\x85\x90R``\x80\x82\x01\x85\x90R\x8C\x15a\x08\x0FWP\x81Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8E\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x08CV[P\x81Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8E\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x8E\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x84\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x90\x94\x93\x92\x91\x90a&\xBEV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a'\xA7V[PPPPPP\x90P\x80\x85`\0\x01Qa\x08\xF2\x8B`\0\x01Q\x8C`@\x01Q\x8Aa\x0F\x82V[\x85\x9CP\x9CP\x9CP\x9CPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\t\x1B\x84a\x0BLV[\x91P\x91Pa\x03\xCA\x82`\0\x81Q\x81\x10a\t5Wa\t5a&lV[` \x02` \x01\x01Q\x82a\x03\xE2\x87a\x0CNV[`\0a\x03\xCA\x83\x83a\tW\x87a\x0CNV[a\x10\xA2V[`\0\x80a\th\x86a\x0CNV[\x90P`\0a\tw\x85\x85\x84a\x10\xFCV[\x90P`\0a\t\x87\x82\x88\x88\x86a\x0EPV[\x90Pa\x03\xC4\x87\x87\x83\x85\x87a\x11AV[`\0\x80`\0\x80`\0a\t\xA7\x87a\x0BLV[\x91P\x91P`\0\x80a\t\xD5`\0\x89\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[` \x02` \x01\x01Q\x86a\x12\x1BV[\x91P\x91P`\0a\t\xE6\x8A\x84\x84a\x03\xD2V[\x90P`\0a\t\xF6\x8B\x85\x85\x85a\x03\x8AV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80a\n\x18\x86a\x0BLV[\x91P\x91P`\0\x80a\n8`\x01\x88\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\nI\x89\x84\x84a\x03\xD2V[\x90P`\0a\nY\x8A\x85\x85\x85a\x03\x8AV[\x90P\x85`\x01\x81Q\x81\x10a\nnWa\nna&lV[` \x02` \x01\x01Q\x81a\n\x81\x91\x90a&\xABV[\x97Pa\n\x8D\x85\x84a&\xABV[\x96PPPPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\n\xAD\x87a\x0BLV[\x91P\x91P`\0\x80a\n\xCD`\x01\x89\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\n\xDE\x8A\x84\x84a\tGV[\x90P`\0a\n\xEE\x8B\x85\x85\x85a\t\\V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80a\x0B\x0C\x86a\x0CNV[\x90P`\0a\x0B\x1C\x86\x86\x86\x85a\x0EPV[\x90Pa\x0B+\x86\x86\x83\x87\x86a\x12vV[\x97\x96PPPPPPPV[``a\x03\x84\x82a\x13\x83V[``a\x03\x84\x82a\x13\xAFV[```\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC6\x91\x90a(\x0FV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xF3\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C8\x91\x90\x81\x01\x90a)\x1CV[\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP\x91P\x91V[a\x0C\x82`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xF3\x91\x90\x81\x01\x90a)\xF7V[\x80` \x01\x90Q\x81\x01\x90a\x03\x84\x91\x90a*\xD8V[`\0\x80`\0\x80a\r\x15\x86a\x0BLV[\x91P\x91P`\0\x80a\r5`\x01\x88\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\rF\x89\x84\x84a\tGV[\x90P`\0a\rV\x8A\x85\x85\x85a\t\\V[\x90P\x85`\0\x81Q\x81\x10a\nnWa\nna&lV[``a\x03\xCA\x84\x84\x84a\x13\xC5V[`\0\x80`\0\x80`\0a\r\x89\x87a\x0BLV[\x91P\x91P`\0\x80a\t\xD5`\x01\x89\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[`\0\x80`\0\x80`\0a\r\xBA\x87a\x0BLV[\x91P\x91P`\0\x80a\n\xCD`\0\x89\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[```\x02\x83\x83`@Q` \x01a\r\xF2\x93\x92\x91\x90a+\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0E\x16\x84\x84a\x14\x99V[\x90P`\0a\x0E+a\x0E&\x83a\x14\xE0V[a\x15IV[\x84Q\x90\x91Pa\x0EF\x90\x82\x90a\x0E@\x90\x89a\x15\x92V[\x90a\x15\x92V[\x96\x95PPPPPPV[`\0\x80a\x0Eea\x0E`\x87\x86a\x15\xA7V[a\x15\xBCV[\x90P`\0a\x0E\x8Da\x0E`a\x0E\x86\x86`\0\x01Q\x88a\x16Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x15\xA7V[` \x85\x01Q\x90\x91Pa\x0E\x9F\x82\x84a+5V[a\x0B+\x91\x90a+5V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0E\xE9W[`\0\x81\x12\x15a\x0E\xE4Wa\x0E\xCF\x83a\x03\xE9a\x03\xE8a\x16nV[\x92Pa\x0E\xDD\x89\x84\x8A\x88a\x0EPV[\x90Pa\x0E\xB7V[a\x0F\x16V[`\0\x81\x13\x15a\x0F\x16Wa\x0F\x01\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x91Pa\x0F\x0F\x89\x83\x8A\x88a\x0EPV[\x90Pa\x0E\xE9V[`\0\x80a\x0FP\x8B\x8B\x85\x8A`@Q` \x01a\x0F3\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x16\xBBa\x16\xE8V[P\x91P\x91Pa\x0Fa\x8B\x83\x8C\x8Aa\x0EPV[`\0\x03a\x0FpW\x81\x95Pa\x0FtV[\x80\x95P[PPPPP\x95\x94PPPPPV[`\0\x80a\x0F\x92\x83` \x01Qa\x18\x04V[\x90P`\0a\x0F\xB5a\x0F\xA3\x87\x87a\x15\xA7V[a\x0E`\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x90P`\0a\x0F\xE3\x83a\x0F\xD4\x87` \x01Q\x85a\x18\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\xDE\x91\x90a+\x7FV[a\x18UV[\x85Q\x90\x91Pa\x0B+\x90\x82a\x15\x92V[`\0\x80a\x10\x0C\x87\x84`@\x01Qa\x15\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\x1B\x87\x86\x86a\x0F\x82V[\x90Pa\x10I\x86a\x10+\x83\x8Aa\x16YV[a\x105\x91\x90a&\x98V[a\x10C\x84a\x0E@\x85\x8Aa\x15\x92V[\x90a\x15\xA7V[\x98\x97PPPPPPPPV[`\0\x80a\x10o\x87\x84`@\x01Qa\x15\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10~\x87\x86\x86a\x0F\x82V[\x90Pa\x10I\x86a\x10\x8E\x83\x8Aa\x16YV[a\x10\x98\x91\x90a&\x98V[a\x10C\x87\x85a\x15\x92V[`\0\x80a\x10\xB2\x83` \x01Qa\x18\x04V[\x90P`\0a\x10\xD3a\x0E`a\x0E\x86\x87\x87`\0\x01Qa\x16Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xE3\x83a\x10\xF2\x87` \x01Q\x85a\x18\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\xDE\x91\x90a+5V[`\0\x80a\x11\t\x84\x84a\x19\xFEV[\x90P`\0a\x11\x16\x82a\x14\xE0V[\x90P`\0a\x11#\x82a\x15IV[\x90Pa\x0B+a\x11:\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x88\x90a\x15\x92V[`\0\x82\x80\x85\x83\x81\x12\x15a\x11\x92W[`\0\x81\x12\x15a\x11\x8DWa\x11g\x83a\x03\xE9a\x03\xE8a\x16nV[\x92P\x87\x83\x11a\x11vW\x82a\x11xV[\x87[\x92Pa\x11\x86\x83\x8A\x8A\x88a\x0EPV[\x90Pa\x11OV[a\x11\xD0V[`\0\x81\x13\x15a\x11\xD0Wa\x11\xAA\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x91P\x87\x82\x11a\x11\xB9W\x81a\x11\xBBV[\x87[\x91Pa\x11\xC9\x82\x8A\x8A\x88a\x0EPV[\x90Pa\x11\x92V[`\0\x80a\x12\n\x8B\x8B\x85\x8A`@Q` \x01a\x11\xED\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x1A6a\x16\xE8V[P\x91P\x91Pa\x0Fa\x82\x8C\x8C\x8Aa\x0EPV[`\0\x80\x80a\x12*\x86\x85\x87a\x16\x9CV[\x90P\x86a\x12@Wa\x12;\x86\x86a&\xABV[a\x12JV[a\x12J\x86\x86a&\x98V[\x92P\x86a\x12`Wa\x12[\x81\x85a&\xABV[a\x12jV[a\x12j\x81\x85a&\x98V[\x91PP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\x0BW[`\0\x81\x12\x15a\x13\x06Wa\x12\x9C\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x85Q\x90\x92P`\0\x90a\x12\xAF\x90\x8A\x90a\x15\xA7V[\x8A\x11a\x12\xD3W\x85Qa\x12\xC2\x90\x8A\x90a\x15\xA7V[a\x12\xCE\x90a\x03\xE8a&\x98V[a\x12\xDFV[a\x12\xDF\x8Aa\x03\xE8a&\x98V[\x90P\x89\x83\x10a\x12\xEEW\x82a\x12\xF0V[\x80[\x92Pa\x12\xFE\x8A\x8A\x85\x89a\x0EPV[\x91PPa\x12\x84V[a\x138V[`\0\x81\x13\x15a\x138Wa\x13#\x83a\x03\xE9a\x03\xE8a\x16nV[\x92Pa\x131\x89\x89\x85\x88a\x0EPV[\x90Pa\x13\x0BV[`\0\x80a\x13r\x8B\x8B\x85\x8A`@Q` \x01a\x13U\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01`\x80a\x1Aca\x16\xE8V[\x92PP\x91Pa\x0Fa\x8B\x8B\x84\x8Aa\x0EPV[```\x01\x82`@Q` \x01a\x13\x99\x92\x91\x90a+\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x04\x82`@Q` \x01a\x13\x99\x92\x91\x90a+\xC1V[```\0a\x13\xD4\x85\x85\x85a\x1A\x90V[\x90P`\0a\x13\xE3\x82\x86\x86a\x0E\tV[\x90P`\0a\x13\xF3\x87\x83\x85\x88a\x0EPV[\x90Pa\x14\x02\x87\x83\x83\x86\x89a\x12vV[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x95P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x14<Wa\x14<a&lV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x01\x81Q\x81\x10a\x14\\Wa\x14\\a&lV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x87`@Q` \x01a\x14}\x93\x92\x91\x90a+\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0\x80a\x14\xAA\x84\x84`\0\x01Qa\x1A\xCBV[\x90P`\0a\x14\xBB\x84` \x01Qa\x18\x04V[\x90Pa\x14\xD7\x84` \x01Q\x82\x84a\x14\xD1\x91\x90a+\x7FV[\x90a\x1A\xDFV[\x95\x94PPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x14\xFEg\r\xE0\xB6\xB3\xA7d\0\0\x85a,\x0FV[a\x15\x08\x91\x90a,UV[\x90P`\0a\x15\x15\x82a,\x83V[\x90P`\0a\x15\"\x82a\x1B\x03V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x15?g\r\xE0\xB6\xB3\xA7d\0\0\x83a,\x0FV[a\x14\xD7\x91\x90a,UV[`\0\x80\x82\x12\x15a\x15\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05\xCAV[P\x90V[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16nV[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\x9CV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x15\xD5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x15\xFDW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x16\x1EW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16+\x83`\x02a,\x0FV[\x90P`\0a\x168\x82a\x1C\xECV[\x90P`\0a\x16Ng\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1FeV[\x90Pa\x14\xD7\x81a,\x83V[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\x9CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\x86W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xB4W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xD5\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x83\x86\x84\x84a\x0EPV[`\0\x80`\0\x86\x88\x11\x15a\x17\x18W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x05\xCAV[`\0a\x17(\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17:\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17H\x82\x84a,\x0FV[\x13\x15a\x17qW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05\xCAV[`\0a\x17}\x8B\x8Ba&\xABV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x17\x94\x87\x87a&\x98V[a\x17\x9E\x91\x90a,\xDFV[\x96P`\0a\x17\xB0\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xBE\x86\x83a,\x0FV[\x13a\x17\xCBW\x87\x96Pa\x17\xD2V[\x87\x95P\x80\x94P[a\x17\xDC\x8D\x8Da&\xABV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x17\xF0WP\x88\x81\x10[a\x17\x88WPPPP\x96P\x96P\x96\x93PPPPV[`\0a\x03\x84a\x18\x13\x83\x80a\x15\x92V[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x16YV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x18DW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18pWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xCAV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80a\x1A\x0F\x84\x84`\0\x01Qa\x1A\xCBV[\x90P`\0a\x1A \x84` \x01Qa\x18\x04V[\x90Pa\x14\xD7\x84` \x01Q\x82\x84a\x14\xD1\x91\x90a+5V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1AP\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x85\x84\x84\x84a\x0EPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1A}\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x83\x83\x87\x84a\x0EPV[`\0\x80a\x1A\x9D\x84\x84a\x19\xFEV[\x90P`\0a\x1A\xADa\x0E&\x83a\x14\xE0V[\x90Pa\x0EFa\x1A\xC4\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x87\x90a\x1FzV[`\0a\x03\x81a\x1A\xDA\x84\x84a\x1FzV[a\x1F\x8FV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1A\xFDW`\0\x80\xFD[\x05\x91\x90PV[`\0\x81`\0\x03a\x1B\x1CWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1B3WP`\0\x91\x90PV[a\x1BDgV\x98\xEE\xF0fp\0\0a,\x83V[\x82\x13a\x1BYWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1Bd\x83a!jV[\x90P`\0a\x1B\x9Dg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x86\x84g\x1B\xC1mgN\xC8\0\0a\x15\xA7V[a\x1B\x98\x90g\r\xE0\xB6\xB3\xA7d\0\0a+5V[a!\xA6V[\x90P`\0\x80\x82a\x1B\xFE\x81a\x1B\xEB\x81a\x1B\xD9\x81a\x1B\xC1\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1FeV[a\x1B\xD4\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a+5V[a\x1FeV[a\x1B\xD4\x90g\x14\xA8EL\x19\xE1\xAC\0a+5V[a\x1B\xD4\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a+5V[a\x1C\x10\x90g\x03\xDE\xBD\x08;\x8C|\0a+5V[\x91P\x83\x90Pa\x1Cx\x81a\x1Cf\x81a\x1CT\x81a\x1CB\x81a\x1C/\x81\x8Ba\x1FeV[a\x1B\xD4\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a+5V[a\x1B\xD4\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a+5V[a\x1B\xD4\x90g\x051\n\xA7\xD5!0\0a+5V[a\x1B\xD4\x90g\r\xE0\xCC=\x15a\0\0a+5V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1C\x8E\x87\x88a\x1FeV[a\x1C\x9A\x90`\0\x19a,\x0FV[a\x1C\xA4\x91\x90a+\x7FV[a\x1C\xAE\x91\x90a+5V[\x92PP`\0a\x1C\xBC\x83a\x18UV[\x90P`\0a\x1C\xCA\x85\x83a\x1FeV[\x90P`\0\x88\x12a\x1C\xDAW\x80a\x10IV[a\x10I\x81g\x1B\xC1mgN\xC8\0\0a+\x7FV[`\0\x80\x82\x12\x80a\x1D\x03WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1D!W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1DBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1DjW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1DuW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\x9DWa\x1D\x98\x83g\x1B\xC1mgN\xC8\0\0a+\x7FV[a\x1D\x9FV[\x82[\x90P`\0a\x1D\xB5\x82g\x1B\xC1mgN\xC8\0\0a!\xA6V[\x90P\x80`\0\x03a\x1D\xD8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D\xE3\x82a\x1F\x8FV[\x90P`\0c;\x9A\xCA\0a\x1E\x0Ea\x1E\ta\x1E\x03g\x1B\xC1mgN\xC8\0\0a,\x83V[\x85a\x1FeV[a!\xBBV[a\x1E\x18\x91\x90a,\x0FV[\x90P`\0\x80a\x1E/\x83g\x03\xC1f\\z\xAB \0a\x1FeV[a\x1EA\x90g \x05\xFEO&\x8E\xA0\0a+5V[\x90P`\0a\x1El\x84a\x1EZ\x86f\x9F2u$b\xA0\0a\x1FeV[a\x1B\xD4\x90g\r\xC5R\x7Fd, \0a+5V[a\x1E~\x90g\r\xE0\xB6\xB3\xA7d\0\0a+5V[\x90Pa\x1E\xA2g\t\xD0(\xCCo _\xFF\x19\x85a\x1E\x98\x85\x85a!\xA6V[a\x1B\xD4\x91\x90a+\x7FV[\x92PPP`\0[`\x02\x81\x10\x15a\x1F=W`\0\x86a\x1E\xBE\x84a\x1B\x03V[a\x1E\xC8\x91\x90a+\x7FV[\x90P`\0a\x1E\xD6\x84\x85a\x1FeV[a\x1E\xDF\x90a,\x83V[\x90P`\0a\x1E\xEC\x82a\x18UV[\x90P`\0a\x1E\xFA\x86\x85a\x1FeV[a\x1F\x0Cg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1FeV[a\x1F\x16\x91\x90a+\x7FV[\x90Pa\x1F\"\x84\x82a!\xA6V[a\x1F,\x90\x87a+5V[\x95P\x84`\x01\x01\x94PPPPPa\x1E\xA9V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1FZWa\x1FU\x82a,\x83V[a\x10IV[P\x96\x95PPPPPPV[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"_V[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16nV[`\0\x80\x82\x13a\x1F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xCAV[`\0``a\x1F\xD9\x84a\"~V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a!\x90W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x15\x8EWP\x19`\x01\x01\x90V[\x91\x90PV[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"_V[`\xB5\x81`\x01`\x88\x1B\x81\x10a!\xD4W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a!\xF0W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\"\x08W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\"\x1EW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"wW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\"\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xCAV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a#9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a#cW\x81\x81\x01Q\x83\x82\x01R` \x01a#KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x84\x81` \x86\x01` \x86\x01a#HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\x81` \x83\x01\x84a#lV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xC1W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xF2W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a$\x17W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$/W`\0\x80\xFD[\x835\x92P` \x84\x015a$A\x81a$\tV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0EF`\x80\x83\x01\x84a#lV[`\0` \x82\x84\x03\x12\x15a$\x8BW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\xB9W`\0\x80\xFD[\x815a$\xC4\x81a$\x92V[\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$\xFCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a$\xE0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a%\x1A`@\x83\x01\x85a$\xCBV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x03\x84\x82\x84a%)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x9CWa%\x9Ca%cV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x9CWa%\x9Ca%cV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xEEWa%\xEEa%cV[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a&\x0CW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a&)W`\0\x80\xFD[Pa&2a%yV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a&[\x81a$\x92V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x84Wa\x03\x84a&\x82V[\x81\x81\x03\x81\x81\x11\x15a\x03\x84Wa\x03\x84a&\x82V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a',W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a'\nV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa'J\x81\x86a$\xCBV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa'ua\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x0B+\x81\x85a#lV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a'\xC2W`\0\x80\xFD[\x87Qa'\xCD\x81a$\tV[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[\x80Qa!\xA1\x81a$\x92V[`\0` \x82\x84\x03\x12\x15a(!W`\0\x80\xFD[\x81Qa$\xC4\x81a$\x92V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(FWa(Fa%cV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a(aW`\0\x80\xFD[\x81Q` a(va(q\x83a(,V[a%\xC5V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a(\x98W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1FZW\x80Qa(\xB0\x81a$\x92V[\x83R\x91\x83\x01\x91\x83\x01a(\x9DV[`\0\x82`\x1F\x83\x01\x12a(\xCEW`\0\x80\xFD[\x81Q` a(\xDEa(q\x83a(,V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a)\0W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1FZW\x80Q\x83R\x91\x83\x01\x91\x83\x01a)\x05V[`\0` \x82\x84\x03\x12\x15a).W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)FW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a)ZW`\0\x80\xFD[a)ba%\xA2V[a)k\x83a(\x04V[\x81R` \x83\x01Q\x82\x81\x11\x15a)\x7FW`\0\x80\xFD[a)\x8B\x87\x82\x86\x01a(PV[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a)\xA3W`\0\x80\xFD[a)\xAF\x87\x82\x86\x01a(\xBDV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra)\xCB`\x80\x84\x01a(\x04V[`\x80\x82\x01Ra)\xDC`\xA0\x84\x01a(\x04V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*\tW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*!W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a*5W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a*GWa*Ga%cV[a*Z`\x1F\x82\x01`\x1F\x19\x16` \x01a%\xC5V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a*qW`\0\x80\xFD[a*\x82\x81` \x84\x01` \x86\x01a#HV[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a*\x9DW`\0\x80\xFD[a*\xA5a%yV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa*\xCD\x81a$\x92V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a*\xEAW`\0\x80\xFD[a\x03\x81\x83\x83a*\x8BV[`\x05\x81\x10a+\x12WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a+$\x82\x86a*\xF4V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+UWa+Ua&\x82V[PP\x92\x91PPV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x14\xD7``\x83\x01\x84a%)V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a+\x9FWa+\x9Fa&\x82V[P\x92\x91PPV[`@\x81\x01a+\xB4\x82\x85a*\xF4V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a+\xCF\x82\x85a*\xF4V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xC0\x81R`\0a+\xFA`\xC0\x83\x01\x86a$\xCBV[\x90P\x83` \x83\x01Ra\x03\xCA`@\x83\x01\x84a%)V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,+Wa,+a&\x82V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x84Wa\x03\x84a&\x82V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a,dWa,da,?V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a,~Wa,~a&\x82V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a,\x98Wa,\x98a&\x82V[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a,\xB5W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa,\xD4\x86``\x87\x01a*\x8BV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a,\xEEWa,\xEEa,?V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC5\x1A\xFAZ\x95\x84\x1C\xEA\xC6`\xB2\x9C<yH\x1C8\x1E_#\xB2^,\x91\xA1\x04\x92\xF0!tF\xA3dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xC3W\x80c\xDE^\xE1\xC3\x11a\0|W\x80c\xDE^\xE1\xC3\x14a\x03!W\x80c\xDE\xF1_\x92\x14a\x034W\x80c\xEA\xAE\x17\xBA\x14a\x01RW\x80c\xEE>\x8C\xFB\x14a\x03GW\x80c\xF3\r7\xF2\x14a\x03ZW\x80c\xF9\xC2\x82\x11\x14a\x03mW`\0\x80\xFD[\x80c\xA8\xC6.v\x14a\x02|W\x80c\xAFNC\x7F\x14a\x02\xA7W\x80c\xB0\x9D\x04\xE5\x14a\x02\xBAW\x80c\xCB\x1FU2\x14a\x02\xCDW\x80c\xCE\x15;\xF4\x14a\x02\xE0W\x80c\xDC\x17\x83U\x14a\x03\x01W`\0\x80\xFD[\x80cN\x81\x7F\xD9\x11a\x01\x15W\x80cN\x81\x7F\xD9\x14a\x01\xE5W\x80c^\xB4\x08\xFC\x14a\x01\xF8W\x80cb7V\x9F\x14a\x02\x0BW\x80cme\"\x99\x14a\x029W\x80c}\xDA\x1A#\x14a\x02AW\x80c\x7F\x17@\x9C\x14a\x02iW`\0\x80\xFD[\x80c\x0F\x85z\xB9\x14a\x01RW\x80c\x12\x06I\xC5\x14a\x01{W\x80c\x1E\x97\x8C\xB0\x14a\x01\x9CW\x80c9(\xFF\x97\x14a\x01\xAFW\x80c;M\x100\x14a\x01\xD2W[`\0\x80\xFD[a\x01ea\x01`6`\x04a#&V[a\x03uV[`@Qa\x01r\x91\x90a#\x98V[`@Q\x80\x91\x03\x90\xF3[a\x01\x8Ea\x01\x896`\x04a#\xABV[a\x03\x8AV[`@Q\x90\x81R` \x01a\x01rV[a\x01\x8Ea\x01\xAA6`\x04a#\xDDV[a\x03\xD2V[a\x01\xC2a\x01\xBD6`\x04a$\x1AV[a\x03\xE7V[`@Qa\x01r\x94\x93\x92\x91\x90a$RV[a\x01\x8Ea\x01\xE06`\x04a$yV[a\t\rV[a\x01\x8Ea\x01\xF36`\x04a#\xDDV[a\tGV[a\x01\x8Ea\x02\x066`\x04a#\xABV[a\t\\V[a\x02\x1Ea\x02\x196`\x04a#&V[a\t\x96V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01rV[a\x01\x8E`\0\x81V[a\x02Ta\x02O6`\x04a#&V[a\n\tV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01rV[a\x02\x1Ea\x02w6`\x04a#&V[a\n\x9CV[`\0Ta\x02\x8F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01rV[a\x01\x8Ea\x02\xB56`\x04a#\xABV[a\x0B\0V[a\x01ea\x02\xC86`\x04a$yV[a\x0B6V[a\x01ea\x02\xDB6`\x04a$\xA7V[a\x0BAV[a\x02\xF3a\x02\xEE6`\x04a$yV[a\x0BLV[`@Qa\x01r\x92\x91\x90a%\x07V[a\x03\x14a\x03\x0F6`\x04a$yV[a\x0CNV[`@Qa\x01r\x91\x90a%UV[a\x02Ta\x03/6`\x04a#&V[a\r\x06V[a\x01ea\x03B6`\x04a%\xF6V[a\rkV[a\x02\x1Ea\x03U6`\x04a#&V[a\rxV[a\x02\x1Ea\x03h6`\x04a#&V[a\r\xA9V[a\x01\x8E`x\x81V[``a\x03\x81\x83\x83a\r\xDAV[\x90P[\x92\x91PPV[`\0\x80a\x03\x96\x86a\x0CNV[\x90P`\0a\x03\xA5\x85\x85\x84a\x0E\tV[\x90P`\0a\x03\xB5\x87\x83\x88\x86a\x0EPV[\x90Pa\x03\xC4\x87\x87\x83\x85\x87a\x0E\xA9V[\x93PPPP[\x94\x93PPPPV[`\0a\x03\xCA\x83\x83a\x03\xE2\x87a\x0CNV[a\x0F\x82V[`\0\x80`\0``a\x04\x12`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80a\x04\x1E\x8Aa\x0BLV[\x91P\x91P`\0a\x04-\x8Ba\x0CNV[\x90Pa\x04S`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x04\x95\x8D\x86`\0\x81Q\x81\x10a\x04lWa\x04la&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x04\x87Wa\x04\x87a&lV[` \x02` \x01\x01Q\x87a\x0B\0V[\x90P\x8B\x15a\x06\x05Wa\x04\xDE\x8B\x86`\0\x81Q\x81\x10a\x04\xB4Wa\x04\xB4a&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x04\xCFWa\x04\xCFa&lV[` \x02` \x01\x01Q\x87\x87a\x0F\xF2V[` \x83\x01R\x84Q\x8B\x90\x86\x90`\0\x90a\x04\xF8Wa\x04\xF8a&lV[` \x02` \x01\x01Qa\x05\n\x91\x90a&\x98V[\x86R` \x82\x01Qa\x05\x1B\x90\x82a&\x98V[\x86`@\x01\x81\x81RPP`\0a\x059\x8E\x88`\0\x01Q\x89`@\x01Qa\x03\xD2V[\x90Pa\x05O\x8E\x88`\0\x01Q\x89`@\x01Q\x84a\x03\x8AV[` \x88\x01R\x85Q\x86\x90`\x01\x90\x81\x10a\x05iWa\x05ia&lV[` \x02` \x01\x01Q\x87` \x01Q\x10a\x05\xD3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x86`\x01\x81Q\x81\x10a\x05\xEBWa\x05\xEBa&lV[` \x02` \x01\x01Qa\x05\xFD\x91\x90a&\xABV[\x83RPa\x07fV[a\x06F\x8B\x86`\0\x81Q\x81\x10a\x06\x1CWa\x06\x1Ca&lV[` \x02` \x01\x01Q\x87`\x01\x81Q\x81\x10a\x067Wa\x067a&lV[` \x02` \x01\x01Q\x87\x87a\x10UV[` \x83\x01R\x84Q\x8B\x90\x86\x90`\x01\x90\x81\x10a\x06bWa\x06ba&lV[` \x02` \x01\x01Qa\x06t\x91\x90a&\x98V[` \x80\x88\x01\x91\x90\x91R\x82\x01Qa\x06\x8A\x90\x82a&\x98V[\x86`@\x01\x81\x81RPP`\0a\x06\xA8\x8E\x88` \x01Q\x89`@\x01Qa\tGV[\x90Pa\x06\xBE\x8E\x88` \x01Q\x89`@\x01Q\x84a\t\\V[\x87R\x85Q\x86\x90`\0\x90a\x06\xD3Wa\x06\xD3a&lV[` \x02` \x01\x01Q\x87`\0\x01Q\x10a\x078W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x05\xCAV[\x86`\0\x01Q\x86`\0\x81Q\x81\x10a\x07PWa\x07Pa&lV[` \x02` \x01\x01Qa\x07b\x91\x90a&\xABV[\x83RP[Pa\x07\xC2`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`@\x81\x01\x85\x90R``\x80\x82\x01\x85\x90R\x8C\x15a\x08\x0FWP\x81Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8E\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x08CV[P\x81Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8E\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x8E\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x84\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08\x90\x94\x93\x92\x91\x90a&\xBEV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xADW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xD1\x91\x90a'\xA7V[PPPPPP\x90P\x80\x85`\0\x01Qa\x08\xF2\x8B`\0\x01Q\x8C`@\x01Q\x8Aa\x0F\x82V[\x85\x9CP\x9CP\x9CP\x9CPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\t\x1B\x84a\x0BLV[\x91P\x91Pa\x03\xCA\x82`\0\x81Q\x81\x10a\t5Wa\t5a&lV[` \x02` \x01\x01Q\x82a\x03\xE2\x87a\x0CNV[`\0a\x03\xCA\x83\x83a\tW\x87a\x0CNV[a\x10\xA2V[`\0\x80a\th\x86a\x0CNV[\x90P`\0a\tw\x85\x85\x84a\x10\xFCV[\x90P`\0a\t\x87\x82\x88\x88\x86a\x0EPV[\x90Pa\x03\xC4\x87\x87\x83\x85\x87a\x11AV[`\0\x80`\0\x80`\0a\t\xA7\x87a\x0BLV[\x91P\x91P`\0\x80a\t\xD5`\0\x89\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[` \x02` \x01\x01Q\x86a\x12\x1BV[\x91P\x91P`\0a\t\xE6\x8A\x84\x84a\x03\xD2V[\x90P`\0a\t\xF6\x8B\x85\x85\x85a\x03\x8AV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80a\n\x18\x86a\x0BLV[\x91P\x91P`\0\x80a\n8`\x01\x88\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\nI\x89\x84\x84a\x03\xD2V[\x90P`\0a\nY\x8A\x85\x85\x85a\x03\x8AV[\x90P\x85`\x01\x81Q\x81\x10a\nnWa\nna&lV[` \x02` \x01\x01Q\x81a\n\x81\x91\x90a&\xABV[\x97Pa\n\x8D\x85\x84a&\xABV[\x96PPPPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\n\xAD\x87a\x0BLV[\x91P\x91P`\0\x80a\n\xCD`\x01\x89\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\n\xDE\x8A\x84\x84a\tGV[\x90P`\0a\n\xEE\x8B\x85\x85\x85a\t\\V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80a\x0B\x0C\x86a\x0CNV[\x90P`\0a\x0B\x1C\x86\x86\x86\x85a\x0EPV[\x90Pa\x0B+\x86\x86\x83\x87\x86a\x12vV[\x97\x96PPPPPPPV[``a\x03\x84\x82a\x13\x83V[``a\x03\x84\x82a\x13\xAFV[```\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B\xC6\x91\x90a(\x0FV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\xF3\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x10W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C8\x91\x90\x81\x01\x90a)\x1CV[\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP\x91P\x91V[a\x0C\x82`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\xCBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xF3\x91\x90\x81\x01\x90a)\xF7V[\x80` \x01\x90Q\x81\x01\x90a\x03\x84\x91\x90a*\xD8V[`\0\x80`\0\x80a\r\x15\x86a\x0BLV[\x91P\x91P`\0\x80a\r5`\x01\x88\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[\x91P\x91P`\0a\rF\x89\x84\x84a\tGV[\x90P`\0a\rV\x8A\x85\x85\x85a\t\\V[\x90P\x85`\0\x81Q\x81\x10a\nnWa\nna&lV[``a\x03\xCA\x84\x84\x84a\x13\xC5V[`\0\x80`\0\x80`\0a\r\x89\x87a\x0BLV[\x91P\x91P`\0\x80a\t\xD5`\x01\x89\x86`\0\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[`\0\x80`\0\x80`\0a\r\xBA\x87a\x0BLV[\x91P\x91P`\0\x80a\n\xCD`\0\x89\x86`\x01\x81Q\x81\x10a\t\xC7Wa\t\xC7a&lV[```\x02\x83\x83`@Q` \x01a\r\xF2\x93\x92\x91\x90a+\x16V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0E\x16\x84\x84a\x14\x99V[\x90P`\0a\x0E+a\x0E&\x83a\x14\xE0V[a\x15IV[\x84Q\x90\x91Pa\x0EF\x90\x82\x90a\x0E@\x90\x89a\x15\x92V[\x90a\x15\x92V[\x96\x95PPPPPPV[`\0\x80a\x0Eea\x0E`\x87\x86a\x15\xA7V[a\x15\xBCV[\x90P`\0a\x0E\x8Da\x0E`a\x0E\x86\x86`\0\x01Q\x88a\x16Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x15\xA7V[` \x85\x01Q\x90\x91Pa\x0E\x9F\x82\x84a+5V[a\x0B+\x91\x90a+5V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0E\xE9W[`\0\x81\x12\x15a\x0E\xE4Wa\x0E\xCF\x83a\x03\xE9a\x03\xE8a\x16nV[\x92Pa\x0E\xDD\x89\x84\x8A\x88a\x0EPV[\x90Pa\x0E\xB7V[a\x0F\x16V[`\0\x81\x13\x15a\x0F\x16Wa\x0F\x01\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x91Pa\x0F\x0F\x89\x83\x8A\x88a\x0EPV[\x90Pa\x0E\xE9V[`\0\x80a\x0FP\x8B\x8B\x85\x8A`@Q` \x01a\x0F3\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x16\xBBa\x16\xE8V[P\x91P\x91Pa\x0Fa\x8B\x83\x8C\x8Aa\x0EPV[`\0\x03a\x0FpW\x81\x95Pa\x0FtV[\x80\x95P[PPPPP\x95\x94PPPPPV[`\0\x80a\x0F\x92\x83` \x01Qa\x18\x04V[\x90P`\0a\x0F\xB5a\x0F\xA3\x87\x87a\x15\xA7V[a\x0E`\x90g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x90P`\0a\x0F\xE3\x83a\x0F\xD4\x87` \x01Q\x85a\x18\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\xDE\x91\x90a+\x7FV[a\x18UV[\x85Q\x90\x91Pa\x0B+\x90\x82a\x15\x92V[`\0\x80a\x10\x0C\x87\x84`@\x01Qa\x15\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\x1B\x87\x86\x86a\x0F\x82V[\x90Pa\x10I\x86a\x10+\x83\x8Aa\x16YV[a\x105\x91\x90a&\x98V[a\x10C\x84a\x0E@\x85\x8Aa\x15\x92V[\x90a\x15\xA7V[\x98\x97PPPPPPPPV[`\0\x80a\x10o\x87\x84`@\x01Qa\x15\x92\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10~\x87\x86\x86a\x0F\x82V[\x90Pa\x10I\x86a\x10\x8E\x83\x8Aa\x16YV[a\x10\x98\x91\x90a&\x98V[a\x10C\x87\x85a\x15\x92V[`\0\x80a\x10\xB2\x83` \x01Qa\x18\x04V[\x90P`\0a\x10\xD3a\x0E`a\x0E\x86\x87\x87`\0\x01Qa\x16Y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xE3\x83a\x10\xF2\x87` \x01Q\x85a\x18\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x0F\xDE\x91\x90a+5V[`\0\x80a\x11\t\x84\x84a\x19\xFEV[\x90P`\0a\x11\x16\x82a\x14\xE0V[\x90P`\0a\x11#\x82a\x15IV[\x90Pa\x0B+a\x11:\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x88\x90a\x15\x92V[`\0\x82\x80\x85\x83\x81\x12\x15a\x11\x92W[`\0\x81\x12\x15a\x11\x8DWa\x11g\x83a\x03\xE9a\x03\xE8a\x16nV[\x92P\x87\x83\x11a\x11vW\x82a\x11xV[\x87[\x92Pa\x11\x86\x83\x8A\x8A\x88a\x0EPV[\x90Pa\x11OV[a\x11\xD0V[`\0\x81\x13\x15a\x11\xD0Wa\x11\xAA\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x91P\x87\x82\x11a\x11\xB9W\x81a\x11\xBBV[\x87[\x91Pa\x11\xC9\x82\x8A\x8A\x88a\x0EPV[\x90Pa\x11\x92V[`\0\x80a\x12\n\x8B\x8B\x85\x8A`@Q` \x01a\x11\xED\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x1A6a\x16\xE8V[P\x91P\x91Pa\x0Fa\x82\x8C\x8C\x8Aa\x0EPV[`\0\x80\x80a\x12*\x86\x85\x87a\x16\x9CV[\x90P\x86a\x12@Wa\x12;\x86\x86a&\xABV[a\x12JV[a\x12J\x86\x86a&\x98V[\x92P\x86a\x12`Wa\x12[\x81\x85a&\xABV[a\x12jV[a\x12j\x81\x85a&\x98V[\x91PP\x94P\x94\x92PPPV[`\0\x82\x80\x85\x83\x81\x12\x15a\x13\x0BW[`\0\x81\x12\x15a\x13\x06Wa\x12\x9C\x82a\x03\xE7a\x03\xE8a\x16\x9CV[\x85Q\x90\x92P`\0\x90a\x12\xAF\x90\x8A\x90a\x15\xA7V[\x8A\x11a\x12\xD3W\x85Qa\x12\xC2\x90\x8A\x90a\x15\xA7V[a\x12\xCE\x90a\x03\xE8a&\x98V[a\x12\xDFV[a\x12\xDF\x8Aa\x03\xE8a&\x98V[\x90P\x89\x83\x10a\x12\xEEW\x82a\x12\xF0V[\x80[\x92Pa\x12\xFE\x8A\x8A\x85\x89a\x0EPV[\x91PPa\x12\x84V[a\x138V[`\0\x81\x13\x15a\x138Wa\x13#\x83a\x03\xE9a\x03\xE8a\x16nV[\x92Pa\x131\x89\x89\x85\x88a\x0EPV[\x90Pa\x13\x0BV[`\0\x80a\x13r\x8B\x8B\x85\x8A`@Q` \x01a\x13U\x94\x93\x92\x91\x90a+]V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01`\x80a\x1Aca\x16\xE8V[\x92PP\x91Pa\x0Fa\x8B\x8B\x84\x8Aa\x0EPV[```\x01\x82`@Q` \x01a\x13\x99\x92\x91\x90a+\xA6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x04\x82`@Q` \x01a\x13\x99\x92\x91\x90a+\xC1V[```\0a\x13\xD4\x85\x85\x85a\x1A\x90V[\x90P`\0a\x13\xE3\x82\x86\x86a\x0E\tV[\x90P`\0a\x13\xF3\x87\x83\x85\x88a\x0EPV[\x90Pa\x14\x02\x87\x83\x83\x86\x89a\x12vV[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x95P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x14<Wa\x14<a&lV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x01\x81Q\x81\x10a\x14\\Wa\x14\\a&lV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x87`@Q` \x01a\x14}\x93\x92\x91\x90a+\xE7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0\x80a\x14\xAA\x84\x84`\0\x01Qa\x1A\xCBV[\x90P`\0a\x14\xBB\x84` \x01Qa\x18\x04V[\x90Pa\x14\xD7\x84` \x01Q\x82\x84a\x14\xD1\x91\x90a+\x7FV[\x90a\x1A\xDFV[\x95\x94PPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x14\xFEg\r\xE0\xB6\xB3\xA7d\0\0\x85a,\x0FV[a\x15\x08\x91\x90a,UV[\x90P`\0a\x15\x15\x82a,\x83V[\x90P`\0a\x15\"\x82a\x1B\x03V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x15?g\r\xE0\xB6\xB3\xA7d\0\0\x83a,\x0FV[a\x14\xD7\x91\x90a,UV[`\0\x80\x82\x12\x15a\x15\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x05\xCAV[P\x90V[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16nV[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16\x9CV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x15\xD5WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x15\xFDW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x16\x1EW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x16+\x83`\x02a,\x0FV[\x90P`\0a\x168\x82a\x1C\xECV[\x90P`\0a\x16Ng\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1FeV[\x90Pa\x14\xD7\x81a,\x83V[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x16\x9CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\x86W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x16\xB4W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x16\xD5\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x83\x86\x84\x84a\x0EPV[`\0\x80`\0\x86\x88\x11\x15a\x17\x18W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x05\xCAV[`\0a\x17(\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17:\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17H\x82\x84a,\x0FV[\x13\x15a\x17qW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x05\xCAV[`\0a\x17}\x8B\x8Ba&\xABV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x17\x94\x87\x87a&\x98V[a\x17\x9E\x91\x90a,\xDFV[\x96P`\0a\x17\xB0\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x17\xBE\x86\x83a,\x0FV[\x13a\x17\xCBW\x87\x96Pa\x17\xD2V[\x87\x95P\x80\x94P[a\x17\xDC\x8D\x8Da&\xABV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x17\xF0WP\x88\x81\x10[a\x17\x88WPPPP\x96P\x96P\x96\x93PPPPV[`\0a\x03\x84a\x18\x13\x83\x80a\x15\x92V[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x16YV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x18DW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x18pWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x18\xB7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x05\xCAV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80a\x1A\x0F\x84\x84`\0\x01Qa\x1A\xCBV[\x90P`\0a\x1A \x84` \x01Qa\x18\x04V[\x90Pa\x14\xD7\x84` \x01Q\x82\x84a\x14\xD1\x91\x90a+5V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1AP\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x85\x84\x84\x84a\x0EPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1A}\x91\x90a,\x9FV[\x93PP\x92P\x92Pa\x0EF\x83\x83\x87\x84a\x0EPV[`\0\x80a\x1A\x9D\x84\x84a\x19\xFEV[\x90P`\0a\x1A\xADa\x0E&\x83a\x14\xE0V[\x90Pa\x0EFa\x1A\xC4\x82g\r\xE0\xB6\xB3\xA7d\0\0a&\xABV[\x87\x90a\x1FzV[`\0a\x03\x81a\x1A\xDA\x84\x84a\x1FzV[a\x1F\x8FV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1A\xFDW`\0\x80\xFD[\x05\x91\x90PV[`\0\x81`\0\x03a\x1B\x1CWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x1B3WP`\0\x91\x90PV[a\x1BDgV\x98\xEE\xF0fp\0\0a,\x83V[\x82\x13a\x1BYWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x1Bd\x83a!jV[\x90P`\0a\x1B\x9Dg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x86\x84g\x1B\xC1mgN\xC8\0\0a\x15\xA7V[a\x1B\x98\x90g\r\xE0\xB6\xB3\xA7d\0\0a+5V[a!\xA6V[\x90P`\0\x80\x82a\x1B\xFE\x81a\x1B\xEB\x81a\x1B\xD9\x81a\x1B\xC1\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1FeV[a\x1B\xD4\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a+5V[a\x1FeV[a\x1B\xD4\x90g\x14\xA8EL\x19\xE1\xAC\0a+5V[a\x1B\xD4\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a+5V[a\x1C\x10\x90g\x03\xDE\xBD\x08;\x8C|\0a+5V[\x91P\x83\x90Pa\x1Cx\x81a\x1Cf\x81a\x1CT\x81a\x1CB\x81a\x1C/\x81\x8Ba\x1FeV[a\x1B\xD4\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a+5V[a\x1B\xD4\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a+5V[a\x1B\xD4\x90g\x051\n\xA7\xD5!0\0a+5V[a\x1B\xD4\x90g\r\xE0\xCC=\x15a\0\0a+5V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1C\x8E\x87\x88a\x1FeV[a\x1C\x9A\x90`\0\x19a,\x0FV[a\x1C\xA4\x91\x90a+\x7FV[a\x1C\xAE\x91\x90a+5V[\x92PP`\0a\x1C\xBC\x83a\x18UV[\x90P`\0a\x1C\xCA\x85\x83a\x1FeV[\x90P`\0\x88\x12a\x1C\xDAW\x80a\x10IV[a\x10I\x81g\x1B\xC1mgN\xC8\0\0a+\x7FV[`\0\x80\x82\x12\x80a\x1D\x03WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1D!W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1DBW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1DjW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1DuW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1D\x9DWa\x1D\x98\x83g\x1B\xC1mgN\xC8\0\0a+\x7FV[a\x1D\x9FV[\x82[\x90P`\0a\x1D\xB5\x82g\x1B\xC1mgN\xC8\0\0a!\xA6V[\x90P\x80`\0\x03a\x1D\xD8W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1D\xE3\x82a\x1F\x8FV[\x90P`\0c;\x9A\xCA\0a\x1E\x0Ea\x1E\ta\x1E\x03g\x1B\xC1mgN\xC8\0\0a,\x83V[\x85a\x1FeV[a!\xBBV[a\x1E\x18\x91\x90a,\x0FV[\x90P`\0\x80a\x1E/\x83g\x03\xC1f\\z\xAB \0a\x1FeV[a\x1EA\x90g \x05\xFEO&\x8E\xA0\0a+5V[\x90P`\0a\x1El\x84a\x1EZ\x86f\x9F2u$b\xA0\0a\x1FeV[a\x1B\xD4\x90g\r\xC5R\x7Fd, \0a+5V[a\x1E~\x90g\r\xE0\xB6\xB3\xA7d\0\0a+5V[\x90Pa\x1E\xA2g\t\xD0(\xCCo _\xFF\x19\x85a\x1E\x98\x85\x85a!\xA6V[a\x1B\xD4\x91\x90a+\x7FV[\x92PPP`\0[`\x02\x81\x10\x15a\x1F=W`\0\x86a\x1E\xBE\x84a\x1B\x03V[a\x1E\xC8\x91\x90a+\x7FV[\x90P`\0a\x1E\xD6\x84\x85a\x1FeV[a\x1E\xDF\x90a,\x83V[\x90P`\0a\x1E\xEC\x82a\x18UV[\x90P`\0a\x1E\xFA\x86\x85a\x1FeV[a\x1F\x0Cg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1FeV[a\x1F\x16\x91\x90a+\x7FV[\x90Pa\x1F\"\x84\x82a!\xA6V[a\x1F,\x90\x87a+5V[\x95P\x84`\x01\x01\x94PPPPPa\x1E\xA9V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1FZWa\x1FU\x82a,\x83V[a\x10IV[P\x96\x95PPPPPPV[`\0a\x03\x81\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\"_V[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x16nV[`\0\x80\x82\x13a\x1F\xCCW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xCAV[`\0``a\x1F\xD9\x84a\"~V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a!\x90W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x15\x8EWP\x19`\x01\x01\x90V[\x91\x90PV[`\0a\x03\x81\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\"_V[`\xB5\x81`\x01`\x88\x1B\x81\x10a!\xD4W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a!\xF0W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\"\x08W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\"\x1EW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\"wW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a\"\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x05\xCAV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a#9W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a#cW\x81\x81\x01Q\x83\x82\x01R` \x01a#KV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra#\x84\x81` \x86\x01` \x86\x01a#HV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\x81` \x83\x01\x84a#lV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a#\xC1W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a#\xF2W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a$\x17W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a$/W`\0\x80\xFD[\x835\x92P` \x84\x015a$A\x81a$\tV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0EF`\x80\x83\x01\x84a#lV[`\0` \x82\x84\x03\x12\x15a$\x8BW`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a$\x17W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a$\xB9W`\0\x80\xFD[\x815a$\xC4\x81a$\x92V[\x93\x92PPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$\xFCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a$\xE0V[P\x94\x95\x94PPPPPV[`@\x81R`\0a%\x1A`@\x83\x01\x85a$\xCBV[\x90P\x82` \x83\x01R\x93\x92PPPV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x03\x84\x82\x84a%)V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x9CWa%\x9Ca%cV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\x9CWa%\x9Ca%cV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a%\xEEWa%\xEEa%cV[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a&\x0CW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a&)W`\0\x80\xFD[Pa&2a%yV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a&[\x81a$\x92V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03\x84Wa\x03\x84a&\x82V[\x81\x81\x03\x81\x81\x11\x15a\x03\x84Wa\x03\x84a&\x82V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a',W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a'\nV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa'J\x81\x86a$\xCBV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa'ua\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x0B+\x81\x85a#lV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a'\xC2W`\0\x80\xFD[\x87Qa'\xCD\x81a$\tV[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[\x80Qa!\xA1\x81a$\x92V[`\0` \x82\x84\x03\x12\x15a(!W`\0\x80\xFD[\x81Qa$\xC4\x81a$\x92V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a(FWa(Fa%cV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a(aW`\0\x80\xFD[\x81Q` a(va(q\x83a(,V[a%\xC5V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a(\x98W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1FZW\x80Qa(\xB0\x81a$\x92V[\x83R\x91\x83\x01\x91\x83\x01a(\x9DV[`\0\x82`\x1F\x83\x01\x12a(\xCEW`\0\x80\xFD[\x81Q` a(\xDEa(q\x83a(,V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a)\0W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1FZW\x80Q\x83R\x91\x83\x01\x91\x83\x01a)\x05V[`\0` \x82\x84\x03\x12\x15a).W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a)FW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a)ZW`\0\x80\xFD[a)ba%\xA2V[a)k\x83a(\x04V[\x81R` \x83\x01Q\x82\x81\x11\x15a)\x7FW`\0\x80\xFD[a)\x8B\x87\x82\x86\x01a(PV[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a)\xA3W`\0\x80\xFD[a)\xAF\x87\x82\x86\x01a(\xBDV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra)\xCB`\x80\x84\x01a(\x04V[`\x80\x82\x01Ra)\xDC`\xA0\x84\x01a(\x04V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a*\tW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a*!W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a*5W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a*GWa*Ga%cV[a*Z`\x1F\x82\x01`\x1F\x19\x16` \x01a%\xC5V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a*qW`\0\x80\xFD[a*\x82\x81` \x84\x01` \x86\x01a#HV[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a*\x9DW`\0\x80\xFD[a*\xA5a%yV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa*\xCD\x81a$\x92V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a*\xEAW`\0\x80\xFD[a\x03\x81\x83\x83a*\x8BV[`\x05\x81\x10a+\x12WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a+$\x82\x86a*\xF4V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a+UWa+Ua&\x82V[PP\x92\x91PPV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x14\xD7``\x83\x01\x84a%)V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a+\x9FWa+\x9Fa&\x82V[P\x92\x91PPV[`@\x81\x01a+\xB4\x82\x85a*\xF4V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a+\xCF\x82\x85a*\xF4V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xC0\x81R`\0a+\xFA`\xC0\x83\x01\x86a$\xCBV[\x90P\x83` \x83\x01Ra\x03\xCA`@\x83\x01\x84a%)V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a,+Wa,+a&\x82V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03\x84Wa\x03\x84a&\x82V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a,dWa,da,?V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a,~Wa,~a&\x82V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a,\x98Wa,\x98a&\x82V[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a,\xB5W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa,\xD4\x86``\x87\x01a*\x8BV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a,\xEEWa,\xEEa,?V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xC5\x1A\xFAZ\x95\x84\x1C\xEA\xC6`\xB2\x9C<yH\x1C8\x1E_#\xB2^,\x91\xA1\x04\x92\xF0!tF\xA3dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormalSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMALSOLVER_ABI.clone(),
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
                LOGNORMALSOLVER_ABI.clone(),
                LOGNORMALSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `BISECTION_EPSILON` (0x6d652299) function
        pub fn bisection_epsilon(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([109, 101, 34, 153], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `MAX_BISECTION_ITERS` (0xf9c28211) function
        pub fn max_bisection_iters(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([249, 194, 130, 17], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `allocateGivenDeltaX` (0x7dda1a23) function
        pub fn allocate_given_delta_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([125, 218, 26, 35], (pool_id, delta_x))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `allocateGivenDeltaY` (0xde5ee1c3) function
        pub fn allocate_given_delta_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([222, 94, 225, 195], (pool_id, delta_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `allocateGivenX` (0xee3e8cfb) function
        pub fn allocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([238, 62, 140, 251], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `allocateGivenY` (0x7f17409c) function
        pub fn allocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([127, 23, 64, 156], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `deallocateGivenX` (0x6237569f) function
        pub fn deallocate_given_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([98, 55, 86, 159], (pool_id, amount_x))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `deallocateGivenY` (0xf30d37f2) function
        pub fn deallocate_given_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([243, 13, 55, 242], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getInitialPoolData` (0xdef15f92) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([222, 241, 95, 146], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getNextLiquidity` (0xaf4e437f) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 78, 67, 127], (pool_id, rx, ry, l))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getNextReserveX` (0x5eb408fc) function
        pub fn get_next_reserve_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([94, 180, 8, 252], (pool_id, ry, l, s))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getNextReserveY` (0x120649c5) function
        pub fn get_next_reserve_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([18, 6, 73, 197], (pool_id, rx, l, s))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormalParams> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPriceGivenXL` (0x1e978cb0) function
        pub fn get_price_given_xl(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([30, 151, 140, 176], (pool_id, rx, l))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPriceGivenYL` (0x4e817fd9) function
        pub fn get_price_given_yl(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([78, 129, 127, 217], (pool_id, ry, l))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
        pub fn get_reserves_and_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([206, 21, 59, 244], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `internalPrice` (0x3b4d1030) function
        pub fn internal_price(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 77, 16, 48], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareControllerUpdate` (0xcb1f5532) function
        pub fn prepare_controller_update(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([203, 31, 85, 50], controller)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareFeeUpdate` (0xb09d04e5) function
        pub fn prepare_fee_update(
            &self,
            swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([176, 157, 4, 229], swap_fee)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareMeanUpdate` (0xeaae17ba) function
        pub fn prepare_mean_update(
            &self,
            target_mean: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([234, 174, 23, 186], (target_mean, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareWidthUpdate` (0x0f857ab9) function
        pub fn prepare_width_update(
            &self,
            target_width: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([15, 133, 122, 185], (target_width, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `simulateSwap` (0x3928ff97) function
        pub fn simulate_swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            swap_x_in: bool,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([57, 40, 255, 151], (pool_id, swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LogNormalSolver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `BisectionLib_InvalidBounds` with signature
    /// `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
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
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    /// Custom Error type `BisectionLib_RootOutsideBounds` with signature
    /// `BisectionLib_RootOutsideBounds(int256,int256)` and selector
    /// `0x1bc6f974`
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
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
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
    pub enum LogNormalSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        Infinity(Infinity),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverErrors {
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
                <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) =
                <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalSolverErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BisectionLib_InvalidBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BisectionLib_RootOutsideBounds as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for LogNormalSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalSolverErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalSolverErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalSolverErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalSolverErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    /// Container type for all input parameters for the `BISECTION_EPSILON`
    /// function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    #[ethcall(name = "BISECTION_EPSILON", abi = "BISECTION_EPSILON()")]
    pub struct BisectionEpsilonCall;
    /// Container type for all input parameters for the `MAX_BISECTION_ITERS`
    /// function with signature `MAX_BISECTION_ITERS()` and selector
    /// `0xf9c28211`
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
    #[ethcall(name = "MAX_BISECTION_ITERS", abi = "MAX_BISECTION_ITERS()")]
    pub struct MaxBisectionItersCall;
    /// Container type for all input parameters for the `allocateGivenDeltaX`
    /// function with signature `allocateGivenDeltaX(uint256,uint256)` and
    /// selector `0x7dda1a23`
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
        name = "allocateGivenDeltaX",
        abi = "allocateGivenDeltaX(uint256,uint256)"
    )]
    pub struct AllocateGivenDeltaXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `allocateGivenDeltaY`
    /// function with signature `allocateGivenDeltaY(uint256,uint256)` and
    /// selector `0xde5ee1c3`
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
        name = "allocateGivenDeltaY",
        abi = "allocateGivenDeltaY(uint256,uint256)"
    )]
    pub struct AllocateGivenDeltaYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `allocateGivenX`
    /// function with signature `allocateGivenX(uint256,uint256)` and selector
    /// `0xee3e8cfb`
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
    #[ethcall(name = "allocateGivenX", abi = "allocateGivenX(uint256,uint256)")]
    pub struct AllocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `allocateGivenY`
    /// function with signature `allocateGivenY(uint256,uint256)` and selector
    /// `0x7f17409c`
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
    #[ethcall(name = "allocateGivenY", abi = "allocateGivenY(uint256,uint256)")]
    pub struct AllocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `deallocateGivenX`
    /// function with signature `deallocateGivenX(uint256,uint256)` and selector
    /// `0x6237569f`
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
    #[ethcall(name = "deallocateGivenX", abi = "deallocateGivenX(uint256,uint256)")]
    pub struct DeallocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `deallocateGivenY`
    /// function with signature `deallocateGivenY(uint256,uint256)` and selector
    /// `0xf30d37f2`
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
    #[ethcall(name = "deallocateGivenY", abi = "deallocateGivenY(uint256,uint256)")]
    pub struct DeallocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))`
    /// and selector `0xdef15f92`
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
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: LogNormalParams,
    }
    /// Container type for all input parameters for the `getNextLiquidity`
    /// function with signature
    /// `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector
    /// `0xaf4e437f`
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
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getNextReserveX`
    /// function with signature
    /// `getNextReserveX(uint256,uint256,uint256,uint256)` and selector
    /// `0x5eb408fc`
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
        name = "getNextReserveX",
        abi = "getNextReserveX(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getNextReserveY`
    /// function with signature
    /// `getNextReserveY(uint256,uint256,uint256,uint256)` and selector
    /// `0x120649c5`
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
        name = "getNextReserveY",
        abi = "getNextReserveY(uint256,uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
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
    /// Container type for all input parameters for the `getPriceGivenXL`
    /// function with signature `getPriceGivenXL(uint256,uint256,uint256)` and
    /// selector `0x1e978cb0`
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
        name = "getPriceGivenXL",
        abi = "getPriceGivenXL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenXLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getPriceGivenYL`
    /// function with signature `getPriceGivenYL(uint256,uint256,uint256)` and
    /// selector `0x4e817fd9`
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
        name = "getPriceGivenYL",
        abi = "getPriceGivenYL(uint256,uint256,uint256)"
    )]
    pub struct GetPriceGivenYLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `getReservesAndLiquidity` function with signature
    /// `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `internalPrice` function
    /// with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256)")]
    pub struct InternalPriceCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `prepareControllerUpdate` function with signature
    /// `prepareControllerUpdate(address)` and selector `0xcb1f5532`
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
        name = "prepareControllerUpdate",
        abi = "prepareControllerUpdate(address)"
    )]
    pub struct PrepareControllerUpdateCall {
        pub controller: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `prepareFeeUpdate`
    /// function with signature `prepareFeeUpdate(uint256)` and selector
    /// `0xb09d04e5`
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
    #[ethcall(name = "prepareFeeUpdate", abi = "prepareFeeUpdate(uint256)")]
    pub struct PrepareFeeUpdateCall {
        pub swap_fee: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `prepareMeanUpdate`
    /// function with signature `prepareMeanUpdate(uint256,uint256)` and
    /// selector `0xeaae17ba`
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
    #[ethcall(name = "prepareMeanUpdate", abi = "prepareMeanUpdate(uint256,uint256)")]
    pub struct PrepareMeanUpdateCall {
        pub target_mean: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `prepareWidthUpdate`
    /// function with signature `prepareWidthUpdate(uint256,uint256)` and
    /// selector `0x0f857ab9`
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
        name = "prepareWidthUpdate",
        abi = "prepareWidthUpdate(uint256,uint256)"
    )]
    pub struct PrepareWidthUpdateCall {
        pub target_width: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `simulateSwap` function
    /// with signature `simulateSwap(uint256,bool,uint256)` and selector
    /// `0x3928ff97`
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
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(uint256,bool,uint256)")]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `strategy` function with
    /// signature `strategy()` and selector `0xa8c62e76`
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
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
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
    pub enum LogNormalSolverCalls {
        BisectionEpsilon(BisectionEpsilonCall),
        MaxBisectionIters(MaxBisectionItersCall),
        AllocateGivenDeltaX(AllocateGivenDeltaXCall),
        AllocateGivenDeltaY(AllocateGivenDeltaYCall),
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPoolParams(GetPoolParamsCall),
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareMeanUpdate(PrepareMeanUpdateCall),
        PrepareWidthUpdate(PrepareWidthUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <BisectionEpsilonCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::BisectionEpsilon(decoded));
            }
            if let Ok(decoded) =
                <MaxBisectionItersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::MaxBisectionIters(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenDeltaXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenDeltaX(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenDeltaYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenDeltaY(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenX(decoded));
            }
            if let Ok(decoded) =
                <AllocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllocateGivenY(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenX(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenY(decoded));
            }
            if let Ok(decoded) =
                <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) =
                <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) =
                <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) =
                <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) =
                <GetPriceGivenXLCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPriceGivenXL(decoded));
            }
            if let Ok(decoded) =
                <GetPriceGivenYLCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPriceGivenYL(decoded));
            }
            if let Ok(decoded) =
                <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) =
                <PrepareControllerUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareControllerUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareMeanUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareMeanUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareWidthUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareWidthUpdate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::BisectionEpsilon(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MaxBisectionIters(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllocateGivenDeltaX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenDeltaY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPriceGivenXL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPriceGivenYL(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareMeanUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareWidthUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionEpsilon(element) => ::core::fmt::Display::fmt(element, f),
                Self::MaxBisectionIters(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenDeltaX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenDeltaY(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareMeanUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWidthUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<BisectionEpsilonCall> for LogNormalSolverCalls {
        fn from(value: BisectionEpsilonCall) -> Self {
            Self::BisectionEpsilon(value)
        }
    }
    impl ::core::convert::From<MaxBisectionItersCall> for LogNormalSolverCalls {
        fn from(value: MaxBisectionItersCall) -> Self {
            Self::MaxBisectionIters(value)
        }
    }
    impl ::core::convert::From<AllocateGivenDeltaXCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenDeltaXCall) -> Self {
            Self::AllocateGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenDeltaYCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenDeltaYCall) -> Self {
            Self::AllocateGivenDeltaY(value)
        }
    }
    impl ::core::convert::From<AllocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for LogNormalSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for LogNormalSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for LogNormalSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalSolverCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenXLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenXLCall) -> Self {
            Self::GetPriceGivenXL(value)
        }
    }
    impl ::core::convert::From<GetPriceGivenYLCall> for LogNormalSolverCalls {
        fn from(value: GetPriceGivenYLCall) -> Self {
            Self::GetPriceGivenYL(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for LogNormalSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for LogNormalSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareMeanUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareMeanUpdateCall) -> Self {
            Self::PrepareMeanUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWidthUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareWidthUpdateCall) -> Self {
            Self::PrepareWidthUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for LogNormalSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for LogNormalSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    /// Container type for all return fields from the `BISECTION_EPSILON`
    /// function with signature `BISECTION_EPSILON()` and selector `0x6d652299`
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
    pub struct BisectionEpsilonReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `MAX_BISECTION_ITERS`
    /// function with signature `MAX_BISECTION_ITERS()` and selector
    /// `0xf9c28211`
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
    pub struct MaxBisectionItersReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `allocateGivenDeltaX`
    /// function with signature `allocateGivenDeltaX(uint256,uint256)` and
    /// selector `0x7dda1a23`
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
    pub struct AllocateGivenDeltaXReturn {
        pub delta_y: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `allocateGivenDeltaY`
    /// function with signature `allocateGivenDeltaY(uint256,uint256)` and
    /// selector `0xde5ee1c3`
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
    pub struct AllocateGivenDeltaYReturn {
        pub delta_x: ::ethers::core::types::U256,
        pub delta_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `allocateGivenX` function
    /// with signature `allocateGivenX(uint256,uint256)` and selector
    /// `0xee3e8cfb`
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
    pub struct AllocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `allocateGivenY` function
    /// with signature `allocateGivenY(uint256,uint256)` and selector
    /// `0x7f17409c`
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
    pub struct AllocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `deallocateGivenX`
    /// function with signature `deallocateGivenX(uint256,uint256)` and selector
    /// `0x6237569f`
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
    pub struct DeallocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `deallocateGivenY`
    /// function with signature `deallocateGivenY(uint256,uint256)` and selector
    /// `0xf30d37f2`
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
    pub struct DeallocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))`
    /// and selector `0xdef15f92`
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
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `getNextLiquidity`
    /// function with signature
    /// `getNextLiquidity(uint256,uint256,uint256,uint256)` and selector
    /// `0xaf4e437f`
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
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `getNextReserveX` function
    /// with signature `getNextReserveX(uint256,uint256,uint256,uint256)` and
    /// selector `0x5eb408fc`
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
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `getNextReserveY` function
    /// with signature `getNextReserveY(uint256,uint256,uint256,uint256)` and
    /// selector `0x120649c5`
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
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
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
    pub struct GetPoolParamsReturn(pub LogNormalParams);
    /// Container type for all return fields from the `getPriceGivenXL` function
    /// with signature `getPriceGivenXL(uint256,uint256,uint256)` and selector
    /// `0x1e978cb0`
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
    pub struct GetPriceGivenXLReturn {
        pub price: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `getPriceGivenYL` function
    /// with signature `getPriceGivenYL(uint256,uint256,uint256)` and selector
    /// `0x4e817fd9`
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
    pub struct GetPriceGivenYLReturn {
        pub price: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `getReservesAndLiquidity`
    /// function with signature `getReservesAndLiquidity(uint256)` and selector
    /// `0xce153bf4`
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `internalPrice` function
    /// with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `prepareControllerUpdate`
    /// function with signature `prepareControllerUpdate(address)` and selector
    /// `0xcb1f5532`
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
    pub struct PrepareControllerUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareFeeUpdate`
    /// function with signature `prepareFeeUpdate(uint256)` and selector
    /// `0xb09d04e5`
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
    pub struct PrepareFeeUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareMeanUpdate`
    /// function with signature `prepareMeanUpdate(uint256,uint256)` and
    /// selector `0xeaae17ba`
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
    pub struct PrepareMeanUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareWidthUpdate`
    /// function with signature `prepareWidthUpdate(uint256,uint256)` and
    /// selector `0x0f857ab9`
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
    pub struct PrepareWidthUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `simulateSwap` function
    /// with signature `simulateSwap(uint256,bool,uint256)` and selector
    /// `0x3928ff97`
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
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    /// Container type for all return fields from the `strategy` function with
    /// signature `strategy()` and selector `0xa8c62e76`
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
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
}

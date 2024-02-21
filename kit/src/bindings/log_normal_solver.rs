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
                    ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbLowerPrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeOptimalArbLowerPrice",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                name: ::std::borrow::ToOwned::to_owned("vUpper"),
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
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbRaisePrice"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeOptimalArbRaisePrice",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
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
                                name: ::std::borrow::ToOwned::to_owned("vUpper"),
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
                    ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
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
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct LogNormal.LogNormalParams",
                                ),
                            ),
                        },],
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
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct LogNormal.LogNormalParams",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareSigmaUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetSigma"),
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
                    ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareStrikeUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetStrike"),
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
                    ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareTauUpdate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetTau"),
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
    const __BYTECODE: &[u8] = b"`\x804b\0\0zW`\x1Fb\x007\xA08\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\x7FW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0zWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03b\0\0zW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa7\n\x90\x81b\0\0\x96\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xB7W\x80c\x12\x06I\xC5\x14a\x01\xB2W\x80c\x13N\xAD\x12\x14a\x01\xADW\x80c\x1E\x97\x8C\xB0\x14a\x01\xA8W\x80c0m\xB4k\x14a\x01\xA3W\x80c3\"f\xF3\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\x99W\x80c;&\x8D]\x14a\x01\x94W\x80c;M\x100\x14a\x01\x8FW\x80cN\x81\x7F\xD9\x14a\x01\x8AW\x80cO\xD6|X\x14a\x01\x85W\x80c^\xB4\x08\xFC\x14a\x01\x80W\x80cb7V\x9F\x14a\x01{W\x80cme\"\x99\x14a\x01vW\x80c\x7F\x17@\x9C\x14a\x01qW\x80c\x81\xB5\xFA\xC2\x14a\x01lW\x80c\x90.\xCA\xA2\x14a\x01gW\x80c\xA8\xC6.v\x14a\x01bW\x80c\xAFNC\x7F\x14a\x01]W\x80c\xB0\x9D\x04\xE5\x14a\x01XW\x80c\xCB\x1FU2\x14a\x01SW\x80c\xCE\x15;\xF4\x14a\x01NW\x80c\xE9G\x16\xD5\x14a\x01IW\x80c\xEE>\x8C\xFB\x14a\x01DW\x80c\xF3\r7\xF2\x14a\x01?Wc\xF9\xC2\x82\x11\x14a\x01:W`\0\x80\xFD[a\n\xFFV[a\n\xCFV[a\n\x9EV[a\ncV[a\n'V[a\t\xE2V[a\t\xAFV[a\t\x93V[a\tjV[a\tAV[a\t\x14V[a\x08rV[a\x08VV[a\x07\xE9V[a\x07\xCDV[a\x07\xA4V[a\x07\x88V[a\x07YV[a\x07\x1EV[a\x04\x8DV[a\x046V[a\x04\x07V[a\x03\xE2V[a\x03TV[a\x02\x8EV[a\x02\x18V[`\0[\x83\x81\x10a\x01\xCFWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xBFV[\x90` \x91a\x01\xF8\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xBCV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\x15\x92\x81\x81R\x01\x90a\x01\xDFV[\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xDFV[\x03\x90\xF3[`\0\x80\xFD[`\x80\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02kW` a\x02\xAAa\x02\xA16a\x02pV[\x92\x91\x90\x91a\x0BMV[`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[a\x02\xB2V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02kWV[4a\x02kW`\xE06`\x03\x19\x01\x12a\x02kW`\xA06`C\x19\x01\x12a\x02kWa\x02ga\x03\xBC`@Qa\x03\x83\x81a\x02\xC8V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x03\xAC\x81a\x03CV[`\x80\x82\x01R`$5`\x045a\x13\x9AV[`@Q\x91\x82\x91\x82a\x02\x04V[``\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90V[4a\x02kW` a\x02\xAAa\x04\x01a\x03\xF86a\x03\xC8V[\x91\x92\x90\x92a\x0F\x06V[\x91a\x15\x83V[4a\x02kW` a\x02\xAAa\x04\x1A6a\x03\xC8V[\x90a\x04-a\x04'\x84a\x0F\x06V[\x93a\x10\xD9V[\x92\x91\x90\x91a\x16pV[4a\x02kW` a\x02\xAAa\x04I6a\x03\xC8V[\x90a\x04Va\x04'\x84a\x0F\x06V[\x92\x90Pa\x19\xDAV[\x80\x15\x15\x03a\x02kWV[\x90\x92`\x80\x92a\x02\x15\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xDFV[4a\x02kW``6`\x03\x19\x01\x12a\x02kWa\x05\x03`$5a\x06\x1F`\x045a\x04\xB3\x83a\x04^V[`D5\x92a\x04\xBFa\x0CWV[\x93a\x04\xC8a\x0CWV[\x94a\x04\xD2\x84a\x10\xD9V[` \x84\x96\x93\x95\x92\x96\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x04\xF2\x87a\x0F\x06V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x10\tV[\x92\x15a\x06\x96W\x92\x82a\x05Ja\x05Q\x93a\x05Ca\x05>a\x056a\x05o\x98a\x051``a\x05\x97\x9D\x9C\x01Q\x86a%\xA2V[a%\xA2V[\x86Q\x90a%\xF8V[a\x0C\x9DV[\x93Qa\x0C\xB0V[\x8ARa\x0C\xB0V[a\x05c\x85\x89\x01\x91\x80\x83R\x89Q\x88a\x0CJV[\x90\x88Q\x90Q\x90\x87a\x0BMV[\x90a\x05\x8Ea\x05\x83` \x89\x01\x93\x80\x85Ra\x0C\x9DV[\x80\x84R\x82Q\x11a\r1V[Q\x90Q\x90a\r$V[\x94[\x84Q\x92`\xC0` \x87\x01Q\x84\x88\x01\x92a\x05\xDF\x84Q\x97a\x05\xD1\x88Q\x99\x8A\x95\x86\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x03!V[`\0Ta\x06\x02\x90a\x05\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\r\xBFV[\x03\x91Z\xFA\x94\x85\x15a\x06\x91W`\0\x95a\x06QW[P\x90a\x06F\x91a\x02g\x95\x96Q\x90Q\x90a\x15\x83V[\x90Q\x94\x85\x94\x85a\x04hV[a\x02g\x95P\x90a\x06|a\x06F\x93\x92`\xC0=`\xC0\x11a\x06\x8AW[a\x06t\x81\x83a\x03!V[\x81\x01\x90a\r\x88V[PPPPP\x95P\x90\x91a\x062V[P=a\x06jV[a\x0BAV[\x82a\x06\xDFa\x07\x18\x96a\x06\xD2a\x07\x04\x95a\x06\xCBa\x05>a\x06\xC3a\x07\x0F\x9Aa\x051``a\x06\xFC\x9B\x01Q\x86a%\xA2V[\x85Q\x90a%\xF8V[\x92Qa\x0C\xB0V[\x92` \x8D\x01\x93\x84Ra\x0C\xB0V[a\x06\xF1\x88\x8C\x01\x91\x80\x83R\x83Q\x8Ba\r\xE3V[\x91Q\x90Q\x90\x89a\r\xF0V[\x80\x89Ra\x0C\x9DV[\x80\x88R\x82Q\x11a\x0C\xBDV[Q\x85Q\x90a\r$V[\x94a\x05\x99V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW` a\x02\xAA`\x045a\x04\x01a\x07~\x82a\x10\xD9V[\x92\x91\x93\x90Pa\x0F\x06V[4a\x02kW` a\x02\xAAa\x07\x9Ea\x03\xF86a\x03\xC8V[\x91a\x1B\x9CV[4a\x02kW` a\x02\xAAa\x07\xB76a\x03\xC8V[\x90a\x07\xC4a\x04'\x84a\x0F\x06V[\x92\x91\x90\x91a\x1C\x16V[4a\x02kW` a\x02\xAAa\x07\xE06a\x02pV[\x92\x91\x90\x91a\r\xF0V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x088`\x045a\x02ga\x08\x1Aa\x08\x0F\x83a\x10\xD9V[\x91\x90P`$5a\x1FzV[\x93\x90\x92\x84\x84a\x082a\x08+\x84a\x0F\x06V[\x83\x83a\x15\x83V[\x92a\x0BMV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`\0\x81R\xF3[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\xC0a\x02ga\x08\xA2a\x08\x98\x84a\x10\xD9V[\x91P`$5a\x1F\xA7V[\x92\x90\x93\x83\x85a\x08\xBAa\x08\xB3\x84a\x0F\x06V[\x83\x83a\x1B\x9CV[\x92a\r\xF0V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW`\xA0a\t2`\x045a\x0F\x06V[a\t?`@Q\x80\x92a\x08\xDEV[\xF3[4a\x02kW` a\x02\xAAa\tT6a\x03\xC8V[\x90a\taa\x04'\x84a\x0F\x06V[\x92\x90\x91Pa\x1F\xCEV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02kW` a\x02\xAAa\t\xA66a\x02pV[\x92\x91\x90\x91a\x10\tV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`\x045a\n\x02\x81a\x03CV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02ga\nF`\x045a\x10\xD9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x088`\x045a\x02ga\x08\x1Aa\n\xC4\x83a\x10\xD9V[\x91\x90P`$5a\x1F\xA7V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\xC0a\x02ga\x08\xA2a\n\xF5\x84a\x10\xD9V[\x91P`$5a\x1FzV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`x\x81R\xF3[\x90\x81` \x91\x03\x12a\x02kWQ\x90V[`@\x90a\x02\x15\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x01\xDFV[`@Q=`\0\x82>=\x90\xFD[a\x0B\x85a\x0B\xEE\x94\x93\x92\x93a\x051\x84a\x0B~a\x0Bya\x0Bta\x0Bm\x88a\x0F\x06V[\x80\x96a#\nV[a#\x7FV[a#\xB2V[\x92Qa%\xA2V[\x91` `@Qa\x0B\xBC\x81a\x0B\xAE\x85\x88\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03!V[`\0Ta\x0B\xD3\x90a\x05\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B*V[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x0C\x15W[Pa\x0C\x0F\x90a\x0F\x06V[\x93a\x11\xF1V[a\x0C\x0F\x91\x93Pa\x0C<\x90` =` \x11a\x0CCW[a\x0C4\x81\x83a\x03!V[\x81\x01\x90a\x0B\x1BV[\x92\x90a\x0C\x05V[P=a\x0C*V[\x91a\x04\x01a\x02\x15\x93a\x0F\x06V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0C\xABWV[a\x0C\x87V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xABWV[\x15a\x0C\xC4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0C\xABWV[\x91\x90\x82\x03\x91\x82\x11a\x0C\xABWV[\x15a\r8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02kW\x81Qa\r\x9F\x81a\x04^V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\x15\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xDFV[\x91a\x07\x9Ea\x02\x15\x93a\x0F\x06V[\x92a\x0Bya\x0Bta\x0E\r\x92\x94\x93\x94a\x0E\x07\x87a\x0F\x06V[\x90a/DV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x93\x84\x03\x93\x84\x11a\x0C\xABWa\x0E.a\x0EW\x94\x83a%\xA2V[\x91` `@Qa\x0B\xBC\x81a\x0B\xAE\x85\x89\x89\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x0E~W[Pa\x0Ex\x90a\x0F\x06V[\x93a\x1E$V[a\x0Ex\x91\x93Pa\x0E\x9C\x90` =` \x11a\x0CCWa\x0C4\x81\x83a\x03!V[\x92\x90a\x0EnV[\x91\x90\x82`\xA0\x91\x03\x12a\x02kW`@Qa\x0E\xBB\x81a\x02\xC8V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x0E\xEE\x83a\x03CV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02kWa\x02\x15\x91a\x0E\xA3V[\x90`@Qa\x0F\x13\x81a\x02\xC8V[`\0\x90\x81\x81R\x81`\x80` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x06\x91W\x80\x92a\x0FzW[Pa\x02\x15\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0E\xF2V[\x90\x91P=\x80\x82\x86>a\x0F\x8C\x81\x86a\x03!V[\x84\x01\x90\x82\x85\x83\x03\x12a\x10\x02W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\x10\x05W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x10\x02W\x81Q\x95\x86\x11a\x02\xE4W`@Q\x92a\x0F\xD8`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x03!V[\x86\x84R\x84\x87\x84\x01\x01\x11a\x10\x02WPa\x02\x15\x93\x94a\x0F\xFA\x91\x84\x80\x85\x01\x91\x01a\x01\xBCV[\x90\x83\x92a\x0FfV[\x80\xFD[\x82\x80\xFD[a\x10]\x93\x91\x92` `@Qa\x107\x81a\x0B\xAE\x87\x86\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\x01\x80`\xA0\x1B\x03`\0T\x16`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B*V[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x10\x84W[Pa\x10~\x90a\x0F\x06V[\x93a!\xB4V[a\x10~\x91\x93Pa\x10\xA2\x90` =` \x11a\x0CCWa\x0C4\x81\x83a\x03!V[\x92\x90a\x10tV[\x90\x81` \x91\x03\x12a\x02kWQa\x02\x15\x81a\x03CV[\x90\x81``\x91\x03\x12a\x02kW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\x10\xF5a\x05\xF6a\x05\xF6`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x06\x91Wa\x11@\x93``\x92`\0\x91a\x11\x9DW[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06\x91W`\0\x80\x93`\0\x93a\x11fW[P\x92\x91\x90V[\x91\x93PPa\x11\x8C\x91P``=``\x11a\x11\x96W[a\x11\x84\x81\x83a\x03!V[\x81\x01\x90a\x10\xBEV[\x92\x90\x92\x918a\x11`V[P=a\x11zV[a\x11\xBF\x91P` =` \x11a\x11\xC5W[a\x11\xB7\x81\x83a\x03!V[\x81\x01\x90a\x10\xA9V[8a\x11\x1FV[P=a\x11\xADV[a\x11\xEF\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\x08\xDEV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x13gW[\x85\x85\x12a\x13HW\x90a\x0B\xAEa\x12$\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa\x12:\x81\x85a3\x7FV[\x92a\x12E\x81\x86a3\x7FV[\x88a\x12P\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\x12d\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\x12\x7FW[PPPPPPPPPP\x90V[\x15a\x12\xE0W[P\x86\x97\x98P\x81\x92a\x12\x9Fa\x12\x99\x8B\x89a\x0C\xB0V[`\x01\x1C\x90V[\x99a\x12\xAA\x8B\x88a3\x7FV[\x90\x84a\x12\xB6\x88\x84a\x15/V[\x13a\x12\xD4WPP\x89\x93[\x88a\x12\xCB\x89\x87a\r$V[\x92\x01\x94\x99a\x12mV[\x8B\x98P\x90\x95P\x93a\x12\xC0V[`\x14\x10\x80a\x12\xFBW[\x15a\x12\xF4W\x88a\x12\x85V[\x80\x80a\x12rV[P\x80\x83\x10a\x12\xE9V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x13T\x90a%\xCEV[\x91a\x13a\x84\x83\x85\x84a$\xA9V[\x93a\x12\x02V[\x85\x85\x13a\x13{W\x90a\x0B\xAEa\x12$\x92a\x12\x12V[\x93P\x94a\x13\x87\x90a#\xF5V[\x94a\x13\x94\x84\x83\x88\x84a$\xA9V[\x93a\x13gV[\x91a\x13\xABa\x0Bya\x0Bt\x83\x85a/DV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0C\xABWa\x14\x02\x82a\x13\xEEa\x13\xE3a\x0Bya\x0Bt\x84a\x13\xDDa\x14 \x9A\x8Ca%\xF8V[\x97a#\nV[a\x051\x85\x84Qa%\xA2V[\x92a\x13\xFB\x82\x82\x86\x8Aa$\xA9V[\x84\x88a!\xB4V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\x08\xDEV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\xABWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\xABWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\xABWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\xABWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\xABW`\0\x19\x83\x05\x03a\x0C\xABWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\xABW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x81\x15a\x15mW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\xABW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x15\xBE` \x83\x01\x93a\x15\xB8\x85Qa\x15\xB0a\x15\xA6`@\x88\x01\x92\x83Q\x90a(CV[\x97Q\x82Q\x90a(lV[\x90Q\x90a$\x14V[\x92a$5V[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x16<W`\0\x85\x13\x15a\x161W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0C\xABWa\x16%a\x16*\x92a\x16 a\x16\x12a\x0By\x94a\x16\ra\x02\x15\x99a(\x8BV[a\x15/V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x14\xA2V[a)\x17V[\x90Qa%\xA2V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x11\xEF\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\x08\xDEV[\x90\x92\x91\x82\x86Q` \x88\x01Q`@\x89\x01Q\x90``\x8A\x01Q\x92a\x16\x90\x83a*\xC8V[a\x16\x99\x90a\x18ZV[\x93a\x16\xA3\x90a\x14CV[\x90a\x16\xAD\x8Aa+\x86V[a\x16\xB6\x8Ba\x19\x81V[a\x16\xBF\x91a,8V[a\x16\xC8\x8Ba\x19\x81V[a\x16\xD1\x84a+\xB5V[a\x16\xDA\x91a\x14\xA2V[a\x16\xE4\x90\x88a,8V[a\x16\xED\x91a,\xB5V[a\x16\xF6\x90a,\xD3V[\x93a\x17\0\x84a/\xBBV[a\x17\t\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x17\x1D\x90a)\x17V[\x90a\x17'\x91a,8V[a\x170\x90a,gV[a\x179\x90a\x19\xC9V[\x84\x86a\x17D\x86a,\tV[\x90a\x17N\x91a,8V[\x90a\x17X\x91a,8V[a\x17a\x91a\x19\xADV[a\x17j\x90a)\x17V[\x82a\x17u\x8C\x84a,8V[\x90a\x17\x7F\x91a,8V[a\x17\x88\x91a,8V[a\x17\x91\x8Ba\x19\x81V[a\x17\x9A\x84a+\xB5V[a\x17\xA3\x91a\x14\xA2V[a\x17\xAC\x91a,\xB5V[\x95a\x17\xB6\x91a,8V[\x90a\x17\xC0\x90a\x19\x93V[a\x17\xC9\x91a,8V[\x92a\x17\xD3\x91a,8V[a\x17\xDC\x90a,\x8EV[\x90a\x17\xE6\x91a\x14\xA2V[a\x17\xEF\x90a.$V[a\x17\xF8\x91a,8V[a\x18\x01\x86a+\x86V[a\x18\n\x91a,\xB5V[\x90a\x18\x14\x84a\x19\xC9V[\x90a\x18\x1E\x91a\x19\xADV[\x90a\x18(\x91a\x19\xADV[`\0\x13a\x18OWa\x02\x15\x95a\x18J\x93a\x0B\xAE\x92`@Q\x96\x87\x95` \x87\x01a\x16FV[a&MV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0C\xABWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\xABWV[`\x01`\xFF\x1B\x81\x14a\x0C\xABW`\0\x03\x90V[\x93\x92\x90\x91\x92\x80Q\x91` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a\x19\xFC\x84a*\xC8V[a\x1A\x05\x90a\x18ZV[\x94a\x1A\x0F\x90a\x14CV[\x91\x82a\x1A\x1A\x88a+\x86V[a\x1A$\x89\x84a\x19\xADV[a\x1A-\x91a,8V[a\x1A7\x89\x84a\x19\xADV[a\x1AA\x83\x85a,8V[a\x1AJ\x91a\x14\xA2V[a\x1AT\x90\x8Ba,8V[a\x1A]\x91a,\xB5V[a\x1Af\x90a,\xD3V[\x95a\x1Ap\x86a/\xBBV[a\x1Ay\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1A\x8D\x90a)\x17V[\x90a\x1A\x97\x91a,8V[a\x1A\xA0\x90a,gV[a\x1A\xA9\x90a\x19\xC9V[\x86\x88a\x1A\xB4\x88a,\tV[\x90a\x1A\xBE\x91a,8V[\x90a\x1A\xC8\x91a,8V[a\x1A\xD1\x91a\x19\xADV[a\x1A\xDA\x90a)\x17V[\x81a\x1A\xE5\x8A\x86a,8V[\x90a\x1A\xEF\x91a,8V[a\x1A\xF8\x91a,8V[\x91a\x1B\x03\x89\x82a\x19\xADV[\x91a\x1B\r\x91a,8V[a\x1B\x16\x91a\x14\xA2V[a\x1B\x1F\x91a,\xB5V[\x96a\x1B)\x91a,8V[\x90a\x1B3\x90a\x19\x93V[a\x1B<\x91a,8V[\x92a\x1BF\x91a,8V[a\x1BO\x90a,\x8EV[\x90a\x1BY\x91a\x14\xA2V[a\x1Bb\x90a.$V[a\x1Bk\x91a,8V[\x90a\x1Bu\x90a+\x86V[a\x1B~\x91a,\xB5V[\x91a\x1B\x88\x90a\x19\xC9V[\x90a\x1B\x92\x91a\x19\xADV[\x90a\x02\x15\x91a\x19\xADV[\x91\x90\x91a\x1B\xDA` \x83\x01\x91a\x1B\xD4a\x1B\xCC\x84Qa\x15\xB0a\x1B\xC2`@\x89\x01\x92\x83Q\x90a(CV[\x96Q\x82Q\x90a(lV[\x95\x85Qa$\x14V[\x90a$5V[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x16<W`\0\x82\x13\x15a\x161Wa\x02\x15\x94a\x16*\x93a\x1C\x10a\x16%\x93a\x16\ra\x0By\x96a(\x8BV[\x05a\x19\xADV[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x1C4\x82a*\xC8V[a\x1C=\x90a\x18ZV[\x92a\x1CG\x90a\x14CV[\x93a\x1CQ\x86a+\x86V[a\x1CZ\x87a\x19\x81V[a\x1Cc\x91a,8V[a\x1Cm\x89\x83a,8V[a\x1Cv\x88a\x19\x81V[a\x1C\x7F\x91a,8V[a\x1C\x88\x90a\x19\xC9V[\x86a\x1C\x93\x8B\x85a,8V[a\x1C\x9C\x90a+\xDEV[\x90a\x1C\xA6\x91a,8V[a\x1C\xAF\x91a\x19\xADV[a\x1C\xB8\x91a,\xB5V[a\x1C\xC1\x90a\x19\xC9V[a\x1C\xCA\x90a,\xD3V[\x92a\x1C\xD4\x83a/\xBBV[a\x1C\xDD\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1C\xF1\x90a)\x17V[\x90a\x1C\xFB\x91a,8V[a\x1D\x04\x90a,gV[a\x1D\r\x90a\x19\xC9V[\x83\x85a\x1D\x18\x85a,\tV[\x90a\x1D\"\x91a,8V[\x90a\x1D,\x91a,8V[a\x1D5\x91a\x19\xADV[a\x1D>\x90a)\x17V[\x85a\x1DI\x88\x8Aa,8V[\x90a\x1DS\x91a,8V[a\x1D\\\x91a,8V[\x90a\x1Df\x87a\x19\x81V[a\x1Do\x87a+\xB5V[a\x1Dx\x91a\x14\xA2V[a\x1D\x81\x91a,8V[a\x1D\x8A\x91a,\xB5V[\x93a\x1D\x95\x87\x89a,8V[\x90a\x1D\x9F\x90a\x19\x93V[a\x1D\xA8\x91a,8V[\x92a\x1D\xB2\x91a,8V[a\x1D\xBB\x90a,\x8EV[\x90a\x1D\xC5\x91a\x14\xA2V[a\x1D\xCE\x90a.$V[a\x1D\xD7\x91a,8V[a\x1D\xE0\x83a+\x86V[a\x1D\xE9\x91a,\xB5V[\x90a\x1D\xF3\x90a\x19\x93V[\x90a\x1D\xFD\x91a\x19\xADV[`\0\x13a\x18OWa\x02\x15\x95a\x1E\x1F\x93a\x0B\xAE\x92`@Q\x96\x87\x95` \x87\x01a\x16FV[a'lV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1FGW[\x85\x85\x12a\x1F(W\x90a\x0B\xAEa\x1EV\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa\x1El\x81\x85a3\xA0V[\x92a\x1Ew\x81\x86a3\xA0V[\x88a\x1E\x82\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\x1E\x96\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\x1E\xB0WPPPPPPPPPP\x90V[\x15a\x1F\x0BW[P\x86\x97\x98P\x81\x92a\x1E\xCAa\x12\x99\x8B\x89a\x0C\xB0V[\x99a\x1E\xD5\x8B\x88a3\xA0V[\x90\x84a\x1E\xE1\x88\x84a\x15/V[\x13a\x1E\xFFWPP\x89\x93[\x88a\x1E\xF6\x89\x87a\r$V[\x92\x01\x94\x99a\x1E\x9FV[\x8B\x98P\x90\x95P\x93a\x1E\xEBV[`\x14\x10\x80a\x1F\x1FW[\x15a\x12\xF4W\x88a\x1E\xB6V[P\x80\x83\x10a\x1F\x14V[\x93P\x91a\x1F4\x90a%\xCEV[\x91a\x1FA\x84\x83\x83\x86a$\xA9V[\x93a\x1E5V[\x85\x85\x13a\x1F[W\x90a\x0B\xAEa\x1EV\x92a\x12\x12V[\x93P\x94a\x1Fg\x90a#\xF5V[\x94a\x1Ft\x84\x83\x83\x89a$\xA9V[\x93a\x1FGV[\x92\x91\x90a\x1F\x90a\x1F\x8A\x82\x84a%\xF8V[\x85a%\xA2V[\x93\x81\x03\x90\x81\x11a\x0C\xABW\x92\x81\x03\x90\x81\x11a\x0C\xABW\x90V[\x92\x91\x90a\x1F\xB7a\x1F\x8A\x82\x84a%\xF8V[\x93\x81\x01\x80\x91\x11a\x0C\xABW\x92\x81\x01\x80\x91\x11a\x0C\xABW\x90V[\x93\x90\x92\x91\x81Q` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94a\x1F\xEE\x85a*\xC8V[a\x1F\xF7\x90a\x18ZV[\x95a \x01\x90a\x14CV[\x92\x83a \x0C\x89a+\x86V[a \x16\x8A\x85a\x19\xADV[a \x1F\x91a,8V[a )\x85\x84a,8V[a 3\x8B\x86a\x19\xADV[a <\x91a,8V[a E\x90a\x19\xC9V[\x82\x85a Q\x88\x87a,8V[\x90a [\x91a,8V[\x90a e\x91a,8V[a n\x91a\x19\xADV[a w\x91a,\xB5V[a \x80\x90a\x19\xC9V[a \x89\x90a,\xD3V[\x96a \x93\x87a/\xBBV[a \x9C\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a \xB0\x90a)\x17V[\x90a \xBA\x91a,8V[a \xC3\x90a,gV[a \xCC\x90a\x19\xC9V[\x87\x89a \xD7\x89a,\tV[\x90a \xE1\x91a,8V[\x90a \xEB\x91a,8V[a \xF4\x91a\x19\xADV[a \xFD\x90a)\x17V[\x81a!\x08\x8B\x8Da,8V[\x90a!\x12\x91a,8V[a!\x1B\x91a,8V[\x92a!&\x8A\x82a\x19\xADV[\x91a!0\x91a,8V[a!9\x91a\x14\xA2V[a!B\x91a,8V[a!K\x91a,\xB5V[\x96a!U\x91a,8V[\x90a!_\x90a\x19\x93V[a!h\x91a,8V[\x92a!r\x91a,8V[a!{\x90a,\x8EV[\x90a!\x85\x91a\x14\xA2V[a!\x8E\x90a.$V[a!\x97\x91a,8V[\x90a!\xA1\x90a+\x86V[a!\xAA\x91a,\xB5V[\x90a\x1B\x92\x90a\x19\x93V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\xD7W[\x85\x85\x12a\"\xB8W\x90a\x0B\xAEa!\xE6\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa!\xFC\x81\x85a3\xC2V[\x92a\"\x07\x81\x86a3\xC2V[\x88a\"\x12\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\"&\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\"@WPPPPPPPPPP\x90V[\x15a\"\x9BW[P\x86\x97\x98P\x81\x92a\"Za\x12\x99\x8B\x89a\x0C\xB0V[\x99a\"e\x8B\x88a3\xC2V[\x90\x84a\"q\x88\x84a\x15/V[\x13a\"\x8FWPP\x89\x93[\x88a\"\x86\x89\x87a\r$V[\x92\x01\x94\x99a\"/V[\x8B\x98P\x90\x95P\x93a\"{V[`\x14\x10\x80a\"\xAFW[\x15a\x12\xF4W\x88a\"FV[P\x80\x83\x10a\"\xA4V[\x93P\x94a\"\xC4\x90a#\xF5V[\x94a\"\xD1\x84\x87\x84\x84a$\xA9V[\x93a!\xC5V[\x85\x85\x13a\"\xEBW\x90a\x0B\xAEa!\xE6\x92a\x12\x12V[\x93P\x91a\"\xF7\x90a%\xCEV[\x91a#\x04\x84\x84\x84\x84a$\xA9V[\x93a\"\xD7V[a#za#ua\x02\x15\x93a#oa#j\x82Qa#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#Ta#O`@` \x8B\x01Q\x9A\x01Q\x96a#I\x88\x8Ca(CV[\x9Da%\xF8V[a/\xBBV[\x97a/\xBBV[a\x14\xD8V[\x05a)\x17V[a$\x14V[a$WV[\x90a\x14\xA2V[a\x14\xBBV[a\x15RV[a#\xAEa#ua#\xA9g\x13\xA0K\xBD\xFD\xC9\xBE\x88a#\xA3g\x1B\xC1mgN\xC8\0\0\x95a\x14\xBBV[\x05a\x19\xC9V[a.$V[\x05\x90V[`\0\x81\x12a#\xBDW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a%]Wa\x02\x15\x93a%&\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a$\xD1\x83\x83a$5V[\x10a%JWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a$\xF9a$\xF3\x83\x85a$\x14V[\x85a$5V[\x10a%+WP`\x01`\x01`\xFF\x1B\x03\x92a% \x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a(CV[\x92a\x19\xADV[a\x19\xADV[a% \x92a\x1B\xD4a%?\x92a%D\x94a$\x14V[a(\x8BV[\x91a%\x10V[a%W\x91a%?\x91a$5V[\x94a$\xE3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02kW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02kW\x80Q\x92a\x02\x15` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x0E\xA3V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a'KWa&i\x81a3\xE2V[a&s\x85\x83a5!V[`\0a&\x7F\x82\x84a\x15/V[\x13a',WPa&\x90\x85\x96\x95a\r\x14V[`\x01\x94`\0\x91\x86\x80[a&\xAAW[PPPPPPPP\x90PV[\x15a'\x07W[P\x85\x96\x97\x98P\x80\x91a&\xC5a\x12\x99\x8B\x88a\x0C\xB0V[\x99a&\xD0\x8B\x87a5!V[\x90\x83a&\xDC\x87\x84a\x15/V[\x13a&\xFBWPP\x89\x92[\x87a&\xF1\x88\x86a\r$V[\x92\x01\x93\x99\x98a&\x99V[\x8B\x97P\x90\x94P\x92a&\xE6V[\x86\x10\x80a'!W[\x15a'\x1AW\x88a&\xB0V[\x80\x80a&\x9EV[Pa\x01\0\x82\x10a'\x0FV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a'KWa'\x88\x81a5CV[a'\x92\x85\x83a6\xB3V[`\0a'\x9E\x82\x84a\x15/V[\x13a',WPa'\xAF\x85\x96\x95a\r\x14V[`\x01\x94`\0\x91\x86\x80[a'\xC8WPPPPPPPP\x90PV[\x15a(%W[P\x85\x96\x97\x98P\x80\x91a'\xE3a\x12\x99\x8B\x88a\x0C\xB0V[\x99a'\xEE\x8B\x87a6\xB3V[\x90\x83a'\xFA\x87\x84a\x15/V[\x13a(\x19WPP\x89\x92[\x87a(\x0F\x88\x86a\r$V[\x92\x01\x93\x99\x98a'\xB8V[\x8B\x97P\x90\x94P\x92a(\x04V[\x86\x10\x80a(8W[\x15a'\x1AW\x88a'\xCEV[Pa\x01\0\x82\x10a(-V[\x90a(M\x90a*\xC8V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0C\xABWa\x02\x15\x91a$\x14V[a\x02\x15\x91a#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#j\x95a/\xBBV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a)\x11Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a(\xFFW\x80\x15a(\xEDW\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0C\xABWa(\xC9\x90a,\xD3V[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x02kWa\x02\x15\x91\x05a\x19\xC9V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a)\x11Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a*gWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x02kW\x82Q\x92` \x81\x01Q\x92a\x02\x15`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0E\xA3V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a+oW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a+bW[e\x01\0\0\0\0\0\x81\x10\x15a+UW[c\x01\0\0\0\x81\x10\x15a+HW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a+\x0CV[` \x1C\x91`\x10\x1B\x91a*\xFFV[`@\x1C\x91` \x1B\x91a*\xF0V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca*\xD8V[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x80\x82\x02\x91`\x01`\0\x19\x82\x10\x17\x91\x81\x84\x05\x14\x90\x15\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x02kW\x05\x90V[`\0\x81\x12\x80\x15a.\x13W[a.\x01W\x80\x15a(\xFFWg\x1B\xC1mgN\xC8\0\0\x81\x14a(\xEDWg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a-\xF2W\x90[a-\x14\x82a1\x89V[\x80\x15a(\xFFWa-}a-Aa-<a-7a-2a-\x82\x95a/\xBBV[a2JV[a*\xC8V[a\x14\xF5V[a\x16 a-Ua-P\x83a1\xB4V[a\x18sV[a-wa-ra-la-g\x86a1\xDFV[a\x18\x8BV[\x85a2\xC1V[a\x18\xA3V[\x90a2(V[a2rV[\x91`\0\x90[`\x02\x82\x10a-\xA2WPP\x15a-\x99W\x90V[a\x02\x15\x90a\x19\xC9V[\x90\x92a-\xEA\x81a-\xE4a-\xBA\x85a\x16 `\x01\x96a.$V[a-wa-\xDAa-\xD5a\x16%a-\xD0\x87\x80a2\xC1V[a\x19\xC9V[a2\x9AV[a#o\x83\x86a2\xC1V[\x90a\x19\xADV[\x93\x01\x90a-\x87V[a-\xFB\x90a\x14fV[\x90a-\x0BV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a,\xDEV[\x80\x15a/7WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a)\x11WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a/*W`\0a/\x1Aa.Y\x83a1\\V[a.\xE2a\x16%a.sa.na-r\x85a$~V[a2\tV[\x92a%&a/\x15a/\x10a/\ta/\x03a.\xFEa.\xF8a.\xF3a.\xEDa.\xE8\x8Da.\xE2a.\xDDa.\xD7a.\xD2a-la.\xCDa.\xC7a.\xC2a.\xBCa.\xB7\x8Aa2\xE2V[a\x18\xBBV[\x89a2\xC1V[a\x18\xD5V[\x87a2\xC1V[a\x18\xEDV[a\x19\x07V[\x83a2\xC1V[a\x19\x1FV[\x90a2\xC1V[a\x199V[\x8Ca2\xC1V[a\x19QV[\x8Aa2\xC1V[a\x19iV[\x88a2\xC1V[\x93\x80a2\xC1V[a\x15\x0EV[a\x14\x89V[\x91\x12\x15a\x02\x15Wa\x02\x15\x90a\x14fV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a#za#ua\x02\x15\x93a-\xE4a#j\x82Qa#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#Ta#O`@` \x8B\x01Q\x9A\x01Q\x96a#I\x88\x8Ca(CV[\x15a/\x8AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/\xE7`\0\x82\x13a/\x83V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a0\x03\x82a3\rV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a1wW`\0\x81\x12\x15a\x02\x15W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02kWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a3\x18\x81\x15\x15a/\x83V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a3\x96a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x91\x92\x90Pa$\xA9V[\x90a3\xB7a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x90P\x91\x90\x91a$\xA9V[\x90a3\xD9a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x92\x90Pa$\xA9V[\x80Q\x81\x01` \x01\x90` \x01\x90a3\xF7\x91a*\x9BV[\x80\x91\x93\x92PQ\x90` \x81\x01Q`@\x82\x01Q\x91``\x01Q\x92a4\x17\x83a*\xC8V[a4 \x90a\x18ZV[\x93a4*\x90a\x14CV[\x90a44\x86a+\x86V[a4=\x87a\x19\x81V[a4F\x91a,8V[a4O\x87a\x19\x81V[a4X\x84a+\xB5V[a4a\x91a\x14\xA2V[a4k\x90\x89a,8V[a4t\x91a,\xB5V[a4}\x90a,\xD3V[\x93a4\x87\x84a/\xBBV[a4\x90\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a4\xA4\x90a)\x17V[\x90a4\xAE\x91a,8V[a4\xB7\x90a,gV[a4\xC0\x90a\x19\xC9V[\x84\x86a4\xCB\x86a,\tV[\x90a4\xD5\x91a,8V[\x90a4\xDF\x91a,8V[a4\xE8\x91a\x19\xADV[a4\xF1\x90a)\x17V[\x82a4\xFC\x88\x84a,8V[\x90a5\x06\x91a,8V[a5\x0F\x91a,8V[a5\x18\x87a\x19\x81V[a\x1B\r\x84a+\xB5V[\x90a58a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a*\x9BV[\x94\x93\x90\x92\x91Pa\x19\xDAV[\x80Q\x81\x01` \x01\x90` \x01\x90a5X\x91a*\x9BV[\x80\x91\x92\x93PQ` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a5x\x84a*\xC8V[a5\x81\x90a\x18ZV[\x94a5\x8B\x90a\x14CV[\x91a5\x95\x87a+\x86V[a5\x9E\x88a\x19\x81V[a5\xA7\x91a,8V[a5\xB1\x83\x83a,8V[a5\xBA\x89a\x19\x81V[a5\xC3\x91a,8V[a5\xCC\x90a\x19\xC9V[\x84a5\xD7\x85\x85a,8V[a5\xE0\x90a+\xDEV[\x90a5\xEA\x91a,8V[a5\xF3\x91a\x19\xADV[a5\xFC\x91a,\xB5V[a6\x05\x90a\x19\xC9V[a6\x0E\x90a,\xD3V[\x94a6\x18\x85a/\xBBV[a6!\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a65\x90a)\x17V[\x90a6?\x91a,8V[a6H\x90a,gV[a6Q\x90a\x19\xC9V[\x85\x87a6\\\x87a,\tV[\x90a6f\x91a,8V[\x90a6p\x91a,8V[a6y\x91a\x19\xADV[a6\x82\x90a)\x17V[\x83a6\x8D\x89\x8Ba,8V[\x90a6\x97\x91a,8V[a6\xA0\x91a,8V[\x90a6\xAA\x88a\x19\x81V[a!0\x85a+\xB5V[\x90a6\xCAa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a*\x9BV[\x94\x93\x90\x92Pa\x1F\xCEV\xFE\xA2dipfsX\"\x12 \xF3\xE8u\xCD\x9CC\xC4S]\xA3\x80\x90c\x0Fd@7\xF9`6\xF7\xDA\xD5U\x19\xE2\xD7\xD7\x92\x04~KdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xB7W\x80c\x12\x06I\xC5\x14a\x01\xB2W\x80c\x13N\xAD\x12\x14a\x01\xADW\x80c\x1E\x97\x8C\xB0\x14a\x01\xA8W\x80c0m\xB4k\x14a\x01\xA3W\x80c3\"f\xF3\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\x99W\x80c;&\x8D]\x14a\x01\x94W\x80c;M\x100\x14a\x01\x8FW\x80cN\x81\x7F\xD9\x14a\x01\x8AW\x80cO\xD6|X\x14a\x01\x85W\x80c^\xB4\x08\xFC\x14a\x01\x80W\x80cb7V\x9F\x14a\x01{W\x80cme\"\x99\x14a\x01vW\x80c\x7F\x17@\x9C\x14a\x01qW\x80c\x81\xB5\xFA\xC2\x14a\x01lW\x80c\x90.\xCA\xA2\x14a\x01gW\x80c\xA8\xC6.v\x14a\x01bW\x80c\xAFNC\x7F\x14a\x01]W\x80c\xB0\x9D\x04\xE5\x14a\x01XW\x80c\xCB\x1FU2\x14a\x01SW\x80c\xCE\x15;\xF4\x14a\x01NW\x80c\xE9G\x16\xD5\x14a\x01IW\x80c\xEE>\x8C\xFB\x14a\x01DW\x80c\xF3\r7\xF2\x14a\x01?Wc\xF9\xC2\x82\x11\x14a\x01:W`\0\x80\xFD[a\n\xFFV[a\n\xCFV[a\n\x9EV[a\ncV[a\n'V[a\t\xE2V[a\t\xAFV[a\t\x93V[a\tjV[a\tAV[a\t\x14V[a\x08rV[a\x08VV[a\x07\xE9V[a\x07\xCDV[a\x07\xA4V[a\x07\x88V[a\x07YV[a\x07\x1EV[a\x04\x8DV[a\x046V[a\x04\x07V[a\x03\xE2V[a\x03TV[a\x02\x8EV[a\x02\x18V[`\0[\x83\x81\x10a\x01\xCFWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xBFV[\x90` \x91a\x01\xF8\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xBCV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\x15\x92\x81\x81R\x01\x90a\x01\xDFV[\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xDFV[\x03\x90\xF3[`\0\x80\xFD[`\x80\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02kW` a\x02\xAAa\x02\xA16a\x02pV[\x92\x91\x90\x91a\x0BMV[`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[a\x02\xB2V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02kWV[4a\x02kW`\xE06`\x03\x19\x01\x12a\x02kW`\xA06`C\x19\x01\x12a\x02kWa\x02ga\x03\xBC`@Qa\x03\x83\x81a\x02\xC8V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x03\xAC\x81a\x03CV[`\x80\x82\x01R`$5`\x045a\x13\x9AV[`@Q\x91\x82\x91\x82a\x02\x04V[``\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90V[4a\x02kW` a\x02\xAAa\x04\x01a\x03\xF86a\x03\xC8V[\x91\x92\x90\x92a\x0F\x06V[\x91a\x15\x83V[4a\x02kW` a\x02\xAAa\x04\x1A6a\x03\xC8V[\x90a\x04-a\x04'\x84a\x0F\x06V[\x93a\x10\xD9V[\x92\x91\x90\x91a\x16pV[4a\x02kW` a\x02\xAAa\x04I6a\x03\xC8V[\x90a\x04Va\x04'\x84a\x0F\x06V[\x92\x90Pa\x19\xDAV[\x80\x15\x15\x03a\x02kWV[\x90\x92`\x80\x92a\x02\x15\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xDFV[4a\x02kW``6`\x03\x19\x01\x12a\x02kWa\x05\x03`$5a\x06\x1F`\x045a\x04\xB3\x83a\x04^V[`D5\x92a\x04\xBFa\x0CWV[\x93a\x04\xC8a\x0CWV[\x94a\x04\xD2\x84a\x10\xD9V[` \x84\x96\x93\x95\x92\x96\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x04\xF2\x87a\x0F\x06V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x10\tV[\x92\x15a\x06\x96W\x92\x82a\x05Ja\x05Q\x93a\x05Ca\x05>a\x056a\x05o\x98a\x051``a\x05\x97\x9D\x9C\x01Q\x86a%\xA2V[a%\xA2V[\x86Q\x90a%\xF8V[a\x0C\x9DV[\x93Qa\x0C\xB0V[\x8ARa\x0C\xB0V[a\x05c\x85\x89\x01\x91\x80\x83R\x89Q\x88a\x0CJV[\x90\x88Q\x90Q\x90\x87a\x0BMV[\x90a\x05\x8Ea\x05\x83` \x89\x01\x93\x80\x85Ra\x0C\x9DV[\x80\x84R\x82Q\x11a\r1V[Q\x90Q\x90a\r$V[\x94[\x84Q\x92`\xC0` \x87\x01Q\x84\x88\x01\x92a\x05\xDF\x84Q\x97a\x05\xD1\x88Q\x99\x8A\x95\x86\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x03!V[`\0Ta\x06\x02\x90a\x05\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\r\xBFV[\x03\x91Z\xFA\x94\x85\x15a\x06\x91W`\0\x95a\x06QW[P\x90a\x06F\x91a\x02g\x95\x96Q\x90Q\x90a\x15\x83V[\x90Q\x94\x85\x94\x85a\x04hV[a\x02g\x95P\x90a\x06|a\x06F\x93\x92`\xC0=`\xC0\x11a\x06\x8AW[a\x06t\x81\x83a\x03!V[\x81\x01\x90a\r\x88V[PPPPP\x95P\x90\x91a\x062V[P=a\x06jV[a\x0BAV[\x82a\x06\xDFa\x07\x18\x96a\x06\xD2a\x07\x04\x95a\x06\xCBa\x05>a\x06\xC3a\x07\x0F\x9Aa\x051``a\x06\xFC\x9B\x01Q\x86a%\xA2V[\x85Q\x90a%\xF8V[\x92Qa\x0C\xB0V[\x92` \x8D\x01\x93\x84Ra\x0C\xB0V[a\x06\xF1\x88\x8C\x01\x91\x80\x83R\x83Q\x8Ba\r\xE3V[\x91Q\x90Q\x90\x89a\r\xF0V[\x80\x89Ra\x0C\x9DV[\x80\x88R\x82Q\x11a\x0C\xBDV[Q\x85Q\x90a\r$V[\x94a\x05\x99V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW` a\x02\xAA`\x045a\x04\x01a\x07~\x82a\x10\xD9V[\x92\x91\x93\x90Pa\x0F\x06V[4a\x02kW` a\x02\xAAa\x07\x9Ea\x03\xF86a\x03\xC8V[\x91a\x1B\x9CV[4a\x02kW` a\x02\xAAa\x07\xB76a\x03\xC8V[\x90a\x07\xC4a\x04'\x84a\x0F\x06V[\x92\x91\x90\x91a\x1C\x16V[4a\x02kW` a\x02\xAAa\x07\xE06a\x02pV[\x92\x91\x90\x91a\r\xF0V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x088`\x045a\x02ga\x08\x1Aa\x08\x0F\x83a\x10\xD9V[\x91\x90P`$5a\x1FzV[\x93\x90\x92\x84\x84a\x082a\x08+\x84a\x0F\x06V[\x83\x83a\x15\x83V[\x92a\x0BMV[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`\0\x81R\xF3[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\xC0a\x02ga\x08\xA2a\x08\x98\x84a\x10\xD9V[\x91P`$5a\x1F\xA7V[\x92\x90\x93\x83\x85a\x08\xBAa\x08\xB3\x84a\x0F\x06V[\x83\x83a\x1B\x9CV[\x92a\r\xF0V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW`\xA0a\t2`\x045a\x0F\x06V[a\t?`@Q\x80\x92a\x08\xDEV[\xF3[4a\x02kW` a\x02\xAAa\tT6a\x03\xC8V[\x90a\taa\x04'\x84a\x0F\x06V[\x92\x90\x91Pa\x1F\xCEV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02kW` a\x02\xAAa\t\xA66a\x02pV[\x92\x91\x90\x91a\x10\tV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`\x045a\n\x02\x81a\x03CV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02ga\nF`\x045a\x10\xD9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x088`\x045a\x02ga\x08\x1Aa\n\xC4\x83a\x10\xD9V[\x91\x90P`$5a\x1F\xA7V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\xC0a\x02ga\x08\xA2a\n\xF5\x84a\x10\xD9V[\x91P`$5a\x1FzV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`x\x81R\xF3[\x90\x81` \x91\x03\x12a\x02kWQ\x90V[`@\x90a\x02\x15\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x01\xDFV[`@Q=`\0\x82>=\x90\xFD[a\x0B\x85a\x0B\xEE\x94\x93\x92\x93a\x051\x84a\x0B~a\x0Bya\x0Bta\x0Bm\x88a\x0F\x06V[\x80\x96a#\nV[a#\x7FV[a#\xB2V[\x92Qa%\xA2V[\x91` `@Qa\x0B\xBC\x81a\x0B\xAE\x85\x88\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03!V[`\0Ta\x0B\xD3\x90a\x05\xF6\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B*V[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x0C\x15W[Pa\x0C\x0F\x90a\x0F\x06V[\x93a\x11\xF1V[a\x0C\x0F\x91\x93Pa\x0C<\x90` =` \x11a\x0CCW[a\x0C4\x81\x83a\x03!V[\x81\x01\x90a\x0B\x1BV[\x92\x90a\x0C\x05V[P=a\x0C*V[\x91a\x04\x01a\x02\x15\x93a\x0F\x06V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0C\xABWV[a\x0C\x87V[\x91\x90\x82\x01\x80\x92\x11a\x0C\xABWV[\x15a\x0C\xC4WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0C\xABWV[\x91\x90\x82\x03\x91\x82\x11a\x0C\xABWV[\x15a\r8WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02kW\x81Qa\r\x9F\x81a\x04^V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\x15\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xDFV[\x91a\x07\x9Ea\x02\x15\x93a\x0F\x06V[\x92a\x0Bya\x0Bta\x0E\r\x92\x94\x93\x94a\x0E\x07\x87a\x0F\x06V[\x90a/DV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x93\x84\x03\x93\x84\x11a\x0C\xABWa\x0E.a\x0EW\x94\x83a%\xA2V[\x91` `@Qa\x0B\xBC\x81a\x0B\xAE\x85\x89\x89\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x0E~W[Pa\x0Ex\x90a\x0F\x06V[\x93a\x1E$V[a\x0Ex\x91\x93Pa\x0E\x9C\x90` =` \x11a\x0CCWa\x0C4\x81\x83a\x03!V[\x92\x90a\x0EnV[\x91\x90\x82`\xA0\x91\x03\x12a\x02kW`@Qa\x0E\xBB\x81a\x02\xC8V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x0E\xEE\x83a\x03CV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02kWa\x02\x15\x91a\x0E\xA3V[\x90`@Qa\x0F\x13\x81a\x02\xC8V[`\0\x90\x81\x81R\x81`\x80` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x06\x91W\x80\x92a\x0FzW[Pa\x02\x15\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0E\xF2V[\x90\x91P=\x80\x82\x86>a\x0F\x8C\x81\x86a\x03!V[\x84\x01\x90\x82\x85\x83\x03\x12a\x10\x02W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\x10\x05W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x10\x02W\x81Q\x95\x86\x11a\x02\xE4W`@Q\x92a\x0F\xD8`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x03!V[\x86\x84R\x84\x87\x84\x01\x01\x11a\x10\x02WPa\x02\x15\x93\x94a\x0F\xFA\x91\x84\x80\x85\x01\x91\x01a\x01\xBCV[\x90\x83\x92a\x0FfV[\x80\xFD[\x82\x80\xFD[a\x10]\x93\x91\x92` `@Qa\x107\x81a\x0B\xAE\x87\x86\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\x01\x80`\xA0\x1B\x03`\0T\x16`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B*V[\x03\x91Z\xFA\x91\x82\x15a\x06\x91Wa\x02\x15\x95`\0\x93a\x10\x84W[Pa\x10~\x90a\x0F\x06V[\x93a!\xB4V[a\x10~\x91\x93Pa\x10\xA2\x90` =` \x11a\x0CCWa\x0C4\x81\x83a\x03!V[\x92\x90a\x10tV[\x90\x81` \x91\x03\x12a\x02kWQa\x02\x15\x81a\x03CV[\x90\x81``\x91\x03\x12a\x02kW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\x10\xF5a\x05\xF6a\x05\xF6`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x06\x91Wa\x11@\x93``\x92`\0\x91a\x11\x9DW[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06\x91W`\0\x80\x93`\0\x93a\x11fW[P\x92\x91\x90V[\x91\x93PPa\x11\x8C\x91P``=``\x11a\x11\x96W[a\x11\x84\x81\x83a\x03!V[\x81\x01\x90a\x10\xBEV[\x92\x90\x92\x918a\x11`V[P=a\x11zV[a\x11\xBF\x91P` =` \x11a\x11\xC5W[a\x11\xB7\x81\x83a\x03!V[\x81\x01\x90a\x10\xA9V[8a\x11\x1FV[P=a\x11\xADV[a\x11\xEF\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\x08\xDEV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x13gW[\x85\x85\x12a\x13HW\x90a\x0B\xAEa\x12$\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa\x12:\x81\x85a3\x7FV[\x92a\x12E\x81\x86a3\x7FV[\x88a\x12P\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\x12d\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\x12\x7FW[PPPPPPPPPP\x90V[\x15a\x12\xE0W[P\x86\x97\x98P\x81\x92a\x12\x9Fa\x12\x99\x8B\x89a\x0C\xB0V[`\x01\x1C\x90V[\x99a\x12\xAA\x8B\x88a3\x7FV[\x90\x84a\x12\xB6\x88\x84a\x15/V[\x13a\x12\xD4WPP\x89\x93[\x88a\x12\xCB\x89\x87a\r$V[\x92\x01\x94\x99a\x12mV[\x8B\x98P\x90\x95P\x93a\x12\xC0V[`\x14\x10\x80a\x12\xFBW[\x15a\x12\xF4W\x88a\x12\x85V[\x80\x80a\x12rV[P\x80\x83\x10a\x12\xE9V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x13T\x90a%\xCEV[\x91a\x13a\x84\x83\x85\x84a$\xA9V[\x93a\x12\x02V[\x85\x85\x13a\x13{W\x90a\x0B\xAEa\x12$\x92a\x12\x12V[\x93P\x94a\x13\x87\x90a#\xF5V[\x94a\x13\x94\x84\x83\x88\x84a$\xA9V[\x93a\x13gV[\x91a\x13\xABa\x0Bya\x0Bt\x83\x85a/DV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0C\xABWa\x14\x02\x82a\x13\xEEa\x13\xE3a\x0Bya\x0Bt\x84a\x13\xDDa\x14 \x9A\x8Ca%\xF8V[\x97a#\nV[a\x051\x85\x84Qa%\xA2V[\x92a\x13\xFB\x82\x82\x86\x8Aa$\xA9V[\x84\x88a!\xB4V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\x08\xDEV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\xABWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\xABWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\xABWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\xABWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\xABW`\0\x19\x83\x05\x03a\x0C\xABWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\xABW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\xABWV[\x81\x15a\x15mW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\xABW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x15\xBE` \x83\x01\x93a\x15\xB8\x85Qa\x15\xB0a\x15\xA6`@\x88\x01\x92\x83Q\x90a(CV[\x97Q\x82Q\x90a(lV[\x90Q\x90a$\x14V[\x92a$5V[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x16<W`\0\x85\x13\x15a\x161W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0C\xABWa\x16%a\x16*\x92a\x16 a\x16\x12a\x0By\x94a\x16\ra\x02\x15\x99a(\x8BV[a\x15/V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x14\xA2V[a)\x17V[\x90Qa%\xA2V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x11\xEF\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\x08\xDEV[\x90\x92\x91\x82\x86Q` \x88\x01Q`@\x89\x01Q\x90``\x8A\x01Q\x92a\x16\x90\x83a*\xC8V[a\x16\x99\x90a\x18ZV[\x93a\x16\xA3\x90a\x14CV[\x90a\x16\xAD\x8Aa+\x86V[a\x16\xB6\x8Ba\x19\x81V[a\x16\xBF\x91a,8V[a\x16\xC8\x8Ba\x19\x81V[a\x16\xD1\x84a+\xB5V[a\x16\xDA\x91a\x14\xA2V[a\x16\xE4\x90\x88a,8V[a\x16\xED\x91a,\xB5V[a\x16\xF6\x90a,\xD3V[\x93a\x17\0\x84a/\xBBV[a\x17\t\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x17\x1D\x90a)\x17V[\x90a\x17'\x91a,8V[a\x170\x90a,gV[a\x179\x90a\x19\xC9V[\x84\x86a\x17D\x86a,\tV[\x90a\x17N\x91a,8V[\x90a\x17X\x91a,8V[a\x17a\x91a\x19\xADV[a\x17j\x90a)\x17V[\x82a\x17u\x8C\x84a,8V[\x90a\x17\x7F\x91a,8V[a\x17\x88\x91a,8V[a\x17\x91\x8Ba\x19\x81V[a\x17\x9A\x84a+\xB5V[a\x17\xA3\x91a\x14\xA2V[a\x17\xAC\x91a,\xB5V[\x95a\x17\xB6\x91a,8V[\x90a\x17\xC0\x90a\x19\x93V[a\x17\xC9\x91a,8V[\x92a\x17\xD3\x91a,8V[a\x17\xDC\x90a,\x8EV[\x90a\x17\xE6\x91a\x14\xA2V[a\x17\xEF\x90a.$V[a\x17\xF8\x91a,8V[a\x18\x01\x86a+\x86V[a\x18\n\x91a,\xB5V[\x90a\x18\x14\x84a\x19\xC9V[\x90a\x18\x1E\x91a\x19\xADV[\x90a\x18(\x91a\x19\xADV[`\0\x13a\x18OWa\x02\x15\x95a\x18J\x93a\x0B\xAE\x92`@Q\x96\x87\x95` \x87\x01a\x16FV[a&MV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0C\xABWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0C\xABWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\xABWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\xABWV[`\x01`\xFF\x1B\x81\x14a\x0C\xABW`\0\x03\x90V[\x93\x92\x90\x91\x92\x80Q\x91` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a\x19\xFC\x84a*\xC8V[a\x1A\x05\x90a\x18ZV[\x94a\x1A\x0F\x90a\x14CV[\x91\x82a\x1A\x1A\x88a+\x86V[a\x1A$\x89\x84a\x19\xADV[a\x1A-\x91a,8V[a\x1A7\x89\x84a\x19\xADV[a\x1AA\x83\x85a,8V[a\x1AJ\x91a\x14\xA2V[a\x1AT\x90\x8Ba,8V[a\x1A]\x91a,\xB5V[a\x1Af\x90a,\xD3V[\x95a\x1Ap\x86a/\xBBV[a\x1Ay\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1A\x8D\x90a)\x17V[\x90a\x1A\x97\x91a,8V[a\x1A\xA0\x90a,gV[a\x1A\xA9\x90a\x19\xC9V[\x86\x88a\x1A\xB4\x88a,\tV[\x90a\x1A\xBE\x91a,8V[\x90a\x1A\xC8\x91a,8V[a\x1A\xD1\x91a\x19\xADV[a\x1A\xDA\x90a)\x17V[\x81a\x1A\xE5\x8A\x86a,8V[\x90a\x1A\xEF\x91a,8V[a\x1A\xF8\x91a,8V[\x91a\x1B\x03\x89\x82a\x19\xADV[\x91a\x1B\r\x91a,8V[a\x1B\x16\x91a\x14\xA2V[a\x1B\x1F\x91a,\xB5V[\x96a\x1B)\x91a,8V[\x90a\x1B3\x90a\x19\x93V[a\x1B<\x91a,8V[\x92a\x1BF\x91a,8V[a\x1BO\x90a,\x8EV[\x90a\x1BY\x91a\x14\xA2V[a\x1Bb\x90a.$V[a\x1Bk\x91a,8V[\x90a\x1Bu\x90a+\x86V[a\x1B~\x91a,\xB5V[\x91a\x1B\x88\x90a\x19\xC9V[\x90a\x1B\x92\x91a\x19\xADV[\x90a\x02\x15\x91a\x19\xADV[\x91\x90\x91a\x1B\xDA` \x83\x01\x91a\x1B\xD4a\x1B\xCC\x84Qa\x15\xB0a\x1B\xC2`@\x89\x01\x92\x83Q\x90a(CV[\x96Q\x82Q\x90a(lV[\x95\x85Qa$\x14V[\x90a$5V[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x16<W`\0\x82\x13\x15a\x161Wa\x02\x15\x94a\x16*\x93a\x1C\x10a\x16%\x93a\x16\ra\x0By\x96a(\x8BV[\x05a\x19\xADV[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x1C4\x82a*\xC8V[a\x1C=\x90a\x18ZV[\x92a\x1CG\x90a\x14CV[\x93a\x1CQ\x86a+\x86V[a\x1CZ\x87a\x19\x81V[a\x1Cc\x91a,8V[a\x1Cm\x89\x83a,8V[a\x1Cv\x88a\x19\x81V[a\x1C\x7F\x91a,8V[a\x1C\x88\x90a\x19\xC9V[\x86a\x1C\x93\x8B\x85a,8V[a\x1C\x9C\x90a+\xDEV[\x90a\x1C\xA6\x91a,8V[a\x1C\xAF\x91a\x19\xADV[a\x1C\xB8\x91a,\xB5V[a\x1C\xC1\x90a\x19\xC9V[a\x1C\xCA\x90a,\xD3V[\x92a\x1C\xD4\x83a/\xBBV[a\x1C\xDD\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a\x1C\xF1\x90a)\x17V[\x90a\x1C\xFB\x91a,8V[a\x1D\x04\x90a,gV[a\x1D\r\x90a\x19\xC9V[\x83\x85a\x1D\x18\x85a,\tV[\x90a\x1D\"\x91a,8V[\x90a\x1D,\x91a,8V[a\x1D5\x91a\x19\xADV[a\x1D>\x90a)\x17V[\x85a\x1DI\x88\x8Aa,8V[\x90a\x1DS\x91a,8V[a\x1D\\\x91a,8V[\x90a\x1Df\x87a\x19\x81V[a\x1Do\x87a+\xB5V[a\x1Dx\x91a\x14\xA2V[a\x1D\x81\x91a,8V[a\x1D\x8A\x91a,\xB5V[\x93a\x1D\x95\x87\x89a,8V[\x90a\x1D\x9F\x90a\x19\x93V[a\x1D\xA8\x91a,8V[\x92a\x1D\xB2\x91a,8V[a\x1D\xBB\x90a,\x8EV[\x90a\x1D\xC5\x91a\x14\xA2V[a\x1D\xCE\x90a.$V[a\x1D\xD7\x91a,8V[a\x1D\xE0\x83a+\x86V[a\x1D\xE9\x91a,\xB5V[\x90a\x1D\xF3\x90a\x19\x93V[\x90a\x1D\xFD\x91a\x19\xADV[`\0\x13a\x18OWa\x02\x15\x95a\x1E\x1F\x93a\x0B\xAE\x92`@Q\x96\x87\x95` \x87\x01a\x16FV[a'lV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1FGW[\x85\x85\x12a\x1F(W\x90a\x0B\xAEa\x1EV\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa\x1El\x81\x85a3\xA0V[\x92a\x1Ew\x81\x86a3\xA0V[\x88a\x1E\x82\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\x1E\x96\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\x1E\xB0WPPPPPPPPPP\x90V[\x15a\x1F\x0BW[P\x86\x97\x98P\x81\x92a\x1E\xCAa\x12\x99\x8B\x89a\x0C\xB0V[\x99a\x1E\xD5\x8B\x88a3\xA0V[\x90\x84a\x1E\xE1\x88\x84a\x15/V[\x13a\x1E\xFFWPP\x89\x93[\x88a\x1E\xF6\x89\x87a\r$V[\x92\x01\x94\x99a\x1E\x9FV[\x8B\x98P\x90\x95P\x93a\x1E\xEBV[`\x14\x10\x80a\x1F\x1FW[\x15a\x12\xF4W\x88a\x1E\xB6V[P\x80\x83\x10a\x1F\x14V[\x93P\x91a\x1F4\x90a%\xCEV[\x91a\x1FA\x84\x83\x83\x86a$\xA9V[\x93a\x1E5V[\x85\x85\x13a\x1F[W\x90a\x0B\xAEa\x1EV\x92a\x12\x12V[\x93P\x94a\x1Fg\x90a#\xF5V[\x94a\x1Ft\x84\x83\x83\x89a$\xA9V[\x93a\x1FGV[\x92\x91\x90a\x1F\x90a\x1F\x8A\x82\x84a%\xF8V[\x85a%\xA2V[\x93\x81\x03\x90\x81\x11a\x0C\xABW\x92\x81\x03\x90\x81\x11a\x0C\xABW\x90V[\x92\x91\x90a\x1F\xB7a\x1F\x8A\x82\x84a%\xF8V[\x93\x81\x01\x80\x91\x11a\x0C\xABW\x92\x81\x01\x80\x91\x11a\x0C\xABW\x90V[\x93\x90\x92\x91\x81Q` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94a\x1F\xEE\x85a*\xC8V[a\x1F\xF7\x90a\x18ZV[\x95a \x01\x90a\x14CV[\x92\x83a \x0C\x89a+\x86V[a \x16\x8A\x85a\x19\xADV[a \x1F\x91a,8V[a )\x85\x84a,8V[a 3\x8B\x86a\x19\xADV[a <\x91a,8V[a E\x90a\x19\xC9V[\x82\x85a Q\x88\x87a,8V[\x90a [\x91a,8V[\x90a e\x91a,8V[a n\x91a\x19\xADV[a w\x91a,\xB5V[a \x80\x90a\x19\xC9V[a \x89\x90a,\xD3V[\x96a \x93\x87a/\xBBV[a \x9C\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a \xB0\x90a)\x17V[\x90a \xBA\x91a,8V[a \xC3\x90a,gV[a \xCC\x90a\x19\xC9V[\x87\x89a \xD7\x89a,\tV[\x90a \xE1\x91a,8V[\x90a \xEB\x91a,8V[a \xF4\x91a\x19\xADV[a \xFD\x90a)\x17V[\x81a!\x08\x8B\x8Da,8V[\x90a!\x12\x91a,8V[a!\x1B\x91a,8V[\x92a!&\x8A\x82a\x19\xADV[\x91a!0\x91a,8V[a!9\x91a\x14\xA2V[a!B\x91a,8V[a!K\x91a,\xB5V[\x96a!U\x91a,8V[\x90a!_\x90a\x19\x93V[a!h\x91a,8V[\x92a!r\x91a,8V[a!{\x90a,\x8EV[\x90a!\x85\x91a\x14\xA2V[a!\x8E\x90a.$V[a!\x97\x91a,8V[\x90a!\xA1\x90a+\x86V[a!\xAA\x91a,\xB5V[\x90a\x1B\x92\x90a\x19\x93V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\xD7W[\x85\x85\x12a\"\xB8W\x90a\x0B\xAEa!\xE6\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xCCV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13%Wa!\xFC\x81\x85a3\xC2V[\x92a\"\x07\x81\x86a3\xC2V[\x88a\"\x12\x82\x87a\x15/V[\x13a\x13\x04WP\x90a\"&\x91\x97\x96\x92\x97a\r$V[`\x01\x95\x91\x82\x91\x87\x80[a\"@WPPPPPPPPPP\x90V[\x15a\"\x9BW[P\x86\x97\x98P\x81\x92a\"Za\x12\x99\x8B\x89a\x0C\xB0V[\x99a\"e\x8B\x88a3\xC2V[\x90\x84a\"q\x88\x84a\x15/V[\x13a\"\x8FWPP\x89\x93[\x88a\"\x86\x89\x87a\r$V[\x92\x01\x94\x99a\"/V[\x8B\x98P\x90\x95P\x93a\"{V[`\x14\x10\x80a\"\xAFW[\x15a\x12\xF4W\x88a\"FV[P\x80\x83\x10a\"\xA4V[\x93P\x94a\"\xC4\x90a#\xF5V[\x94a\"\xD1\x84\x87\x84\x84a$\xA9V[\x93a!\xC5V[\x85\x85\x13a\"\xEBW\x90a\x0B\xAEa!\xE6\x92a\x12\x12V[\x93P\x91a\"\xF7\x90a%\xCEV[\x91a#\x04\x84\x84\x84\x84a$\xA9V[\x93a\"\xD7V[a#za#ua\x02\x15\x93a#oa#j\x82Qa#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#Ta#O`@` \x8B\x01Q\x9A\x01Q\x96a#I\x88\x8Ca(CV[\x9Da%\xF8V[a/\xBBV[\x97a/\xBBV[a\x14\xD8V[\x05a)\x17V[a$\x14V[a$WV[\x90a\x14\xA2V[a\x14\xBBV[a\x15RV[a#\xAEa#ua#\xA9g\x13\xA0K\xBD\xFD\xC9\xBE\x88a#\xA3g\x1B\xC1mgN\xC8\0\0\x95a\x14\xBBV[\x05a\x19\xC9V[a.$V[\x05\x90V[`\0\x81\x12a#\xBDW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a%]Wa\x02\x15\x93a%&\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a$\xD1\x83\x83a$5V[\x10a%JWP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a$\xF9a$\xF3\x83\x85a$\x14V[\x85a$5V[\x10a%+WP`\x01`\x01`\xFF\x1B\x03\x92a% \x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a(CV[\x92a\x19\xADV[a\x19\xADV[a% \x92a\x1B\xD4a%?\x92a%D\x94a$\x14V[a(\x8BV[\x91a%\x10V[a%W\x91a%?\x91a$5V[\x94a$\xE3V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02kW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02kW\x80Q\x92a\x02\x15` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x0E\xA3V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a'KWa&i\x81a3\xE2V[a&s\x85\x83a5!V[`\0a&\x7F\x82\x84a\x15/V[\x13a',WPa&\x90\x85\x96\x95a\r\x14V[`\x01\x94`\0\x91\x86\x80[a&\xAAW[PPPPPPPP\x90PV[\x15a'\x07W[P\x85\x96\x97\x98P\x80\x91a&\xC5a\x12\x99\x8B\x88a\x0C\xB0V[\x99a&\xD0\x8B\x87a5!V[\x90\x83a&\xDC\x87\x84a\x15/V[\x13a&\xFBWPP\x89\x92[\x87a&\xF1\x88\x86a\r$V[\x92\x01\x93\x99\x98a&\x99V[\x8B\x97P\x90\x94P\x92a&\xE6V[\x86\x10\x80a'!W[\x15a'\x1AW\x88a&\xB0V[\x80\x80a&\x9EV[Pa\x01\0\x82\x10a'\x0FV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a'KWa'\x88\x81a5CV[a'\x92\x85\x83a6\xB3V[`\0a'\x9E\x82\x84a\x15/V[\x13a',WPa'\xAF\x85\x96\x95a\r\x14V[`\x01\x94`\0\x91\x86\x80[a'\xC8WPPPPPPPP\x90PV[\x15a(%W[P\x85\x96\x97\x98P\x80\x91a'\xE3a\x12\x99\x8B\x88a\x0C\xB0V[\x99a'\xEE\x8B\x87a6\xB3V[\x90\x83a'\xFA\x87\x84a\x15/V[\x13a(\x19WPP\x89\x92[\x87a(\x0F\x88\x86a\r$V[\x92\x01\x93\x99\x98a'\xB8V[\x8B\x97P\x90\x94P\x92a(\x04V[\x86\x10\x80a(8W[\x15a'\x1AW\x88a'\xCEV[Pa\x01\0\x82\x10a(-V[\x90a(M\x90a*\xC8V[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0C\xABWa\x02\x15\x91a$\x14V[a\x02\x15\x91a#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#j\x95a/\xBBV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a)\x11Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a(\xFFW\x80\x15a(\xEDW\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0C\xABWa(\xC9\x90a,\xD3V[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x02kWa\x02\x15\x91\x05a\x19\xC9V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a)\x11Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a*gWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x02kW\x82Q\x92` \x81\x01Q\x92a\x02\x15`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0E\xA3V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a+oW[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a+bW[e\x01\0\0\0\0\0\x81\x10\x15a+UW[c\x01\0\0\0\x81\x10\x15a+HW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a+\x0CV[` \x1C\x91`\x10\x1B\x91a*\xFFV[`@\x1C\x91` \x1B\x91a*\xF0V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca*\xD8V[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x80\x82\x02\x91`\x01`\0\x19\x82\x10\x17\x91\x81\x84\x05\x14\x90\x15\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x02kW\x05\x90V[`\0\x81\x12\x80\x15a.\x13W[a.\x01W\x80\x15a(\xFFWg\x1B\xC1mgN\xC8\0\0\x81\x14a(\xEDWg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a-\xF2W\x90[a-\x14\x82a1\x89V[\x80\x15a(\xFFWa-}a-Aa-<a-7a-2a-\x82\x95a/\xBBV[a2JV[a*\xC8V[a\x14\xF5V[a\x16 a-Ua-P\x83a1\xB4V[a\x18sV[a-wa-ra-la-g\x86a1\xDFV[a\x18\x8BV[\x85a2\xC1V[a\x18\xA3V[\x90a2(V[a2rV[\x91`\0\x90[`\x02\x82\x10a-\xA2WPP\x15a-\x99W\x90V[a\x02\x15\x90a\x19\xC9V[\x90\x92a-\xEA\x81a-\xE4a-\xBA\x85a\x16 `\x01\x96a.$V[a-wa-\xDAa-\xD5a\x16%a-\xD0\x87\x80a2\xC1V[a\x19\xC9V[a2\x9AV[a#o\x83\x86a2\xC1V[\x90a\x19\xADV[\x93\x01\x90a-\x87V[a-\xFB\x90a\x14fV[\x90a-\x0BV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a,\xDEV[\x80\x15a/7WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a)\x11WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a/*W`\0a/\x1Aa.Y\x83a1\\V[a.\xE2a\x16%a.sa.na-r\x85a$~V[a2\tV[\x92a%&a/\x15a/\x10a/\ta/\x03a.\xFEa.\xF8a.\xF3a.\xEDa.\xE8\x8Da.\xE2a.\xDDa.\xD7a.\xD2a-la.\xCDa.\xC7a.\xC2a.\xBCa.\xB7\x8Aa2\xE2V[a\x18\xBBV[\x89a2\xC1V[a\x18\xD5V[\x87a2\xC1V[a\x18\xEDV[a\x19\x07V[\x83a2\xC1V[a\x19\x1FV[\x90a2\xC1V[a\x199V[\x8Ca2\xC1V[a\x19QV[\x8Aa2\xC1V[a\x19iV[\x88a2\xC1V[\x93\x80a2\xC1V[a\x15\x0EV[a\x14\x89V[\x91\x12\x15a\x02\x15Wa\x02\x15\x90a\x14fV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a#za#ua\x02\x15\x93a-\xE4a#j\x82Qa#eg\r\xE0\xB6\xB3\xA7d\0\0a#_a#Za#Ta#O`@` \x8B\x01Q\x9A\x01Q\x96a#I\x88\x8Ca(CV[\x15a/\x8AWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/\xE7`\0\x82\x13a/\x83V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a0\x03\x82a3\rV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a1wW`\0\x81\x12\x15a\x02\x15W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02kWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a3\x18\x81\x15\x15a/\x83V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a3\x96a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x91\x92\x90Pa$\xA9V[\x90a3\xB7a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x90P\x91\x90\x91a$\xA9V[\x90a3\xD9a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a&(V[\x93\x92\x90Pa$\xA9V[\x80Q\x81\x01` \x01\x90` \x01\x90a3\xF7\x91a*\x9BV[\x80\x91\x93\x92PQ\x90` \x81\x01Q`@\x82\x01Q\x91``\x01Q\x92a4\x17\x83a*\xC8V[a4 \x90a\x18ZV[\x93a4*\x90a\x14CV[\x90a44\x86a+\x86V[a4=\x87a\x19\x81V[a4F\x91a,8V[a4O\x87a\x19\x81V[a4X\x84a+\xB5V[a4a\x91a\x14\xA2V[a4k\x90\x89a,8V[a4t\x91a,\xB5V[a4}\x90a,\xD3V[\x93a4\x87\x84a/\xBBV[a4\x90\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a4\xA4\x90a)\x17V[\x90a4\xAE\x91a,8V[a4\xB7\x90a,gV[a4\xC0\x90a\x19\xC9V[\x84\x86a4\xCB\x86a,\tV[\x90a4\xD5\x91a,8V[\x90a4\xDF\x91a,8V[a4\xE8\x91a\x19\xADV[a4\xF1\x90a)\x17V[\x82a4\xFC\x88\x84a,8V[\x90a5\x06\x91a,8V[a5\x0F\x91a,8V[a5\x18\x87a\x19\x81V[a\x1B\r\x84a+\xB5V[\x90a58a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a*\x9BV[\x94\x93\x90\x92\x91Pa\x19\xDAV[\x80Q\x81\x01` \x01\x90` \x01\x90a5X\x91a*\x9BV[\x80\x91\x92\x93PQ` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a5x\x84a*\xC8V[a5\x81\x90a\x18ZV[\x94a5\x8B\x90a\x14CV[\x91a5\x95\x87a+\x86V[a5\x9E\x88a\x19\x81V[a5\xA7\x91a,8V[a5\xB1\x83\x83a,8V[a5\xBA\x89a\x19\x81V[a5\xC3\x91a,8V[a5\xCC\x90a\x19\xC9V[\x84a5\xD7\x85\x85a,8V[a5\xE0\x90a+\xDEV[\x90a5\xEA\x91a,8V[a5\xF3\x91a\x19\xADV[a5\xFC\x91a,\xB5V[a6\x05\x90a\x19\xC9V[a6\x0E\x90a,\xD3V[\x94a6\x18\x85a/\xBBV[a6!\x90a\x14\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05a65\x90a)\x17V[\x90a6?\x91a,8V[a6H\x90a,gV[a6Q\x90a\x19\xC9V[\x85\x87a6\\\x87a,\tV[\x90a6f\x91a,8V[\x90a6p\x91a,8V[a6y\x91a\x19\xADV[a6\x82\x90a)\x17V[\x83a6\x8D\x89\x8Ba,8V[\x90a6\x97\x91a,8V[a6\xA0\x91a,8V[\x90a6\xAA\x88a\x19\x81V[a!0\x85a+\xB5V[\x90a6\xCAa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a*\x9BV[\x94\x93\x90\x92Pa\x1F\xCEV\xFE\xA2dipfsX\"\x12 \xF3\xE8u\xCD\x9CC\xC4S]\xA3\x80\x90c\x0Fd@7\xF9`6\xF7\xDA\xD5U\x19\xE2\xD7\xD7\x92\x04~KdsolcC\0\x08\x16\x003";
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
        /// Calls the contract's `calculateDiffLower` (0x332266f3) function
        pub fn calculate_diff_lower(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([51, 34, 102, 243], (pool_id, s, v))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `calculateDiffRaise` (0x902ecaa2) function
        pub fn calculate_diff_raise(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([144, 46, 202, 162], (pool_id, s, v))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `computeOptimalArbLowerPrice` (0x306db46b)
        /// function
        pub fn compute_optimal_arb_lower_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v_upper: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([48, 109, 180, 107], (pool_id, s, v_upper))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `computeOptimalArbRaisePrice` (0x4fd67c58)
        /// function
        pub fn compute_optimal_arb_raise_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            v_upper: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([79, 214, 124, 88], (pool_id, s, v_upper))
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
        /// Calls the contract's `fetchPoolParams` (0x81b5fac2) function
        pub fn fetch_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, LogNormalParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getInitialPoolData` (0x134ead12) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: LogNormalParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([19, 78, 173, 18], (rx, s, params))
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
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
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
        /// Calls the contract's `prepareSigmaUpdate` (0xe94716d5) function
        pub fn prepare_sigma_update(
            &self,
            target_sigma: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([233, 71, 22, 213], (target_sigma, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareStrikeUpdate` (0x0420580a) function
        pub fn prepare_strike_update(
            &self,
            target_strike: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([4, 32, 88, 10], (target_strike, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareTauUpdate` (0x3b268d5d) function
        pub fn prepare_tau_update(
            &self,
            target_tau: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([59, 38, 141, 93], (target_tau, target_timestamp))
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
    /// Container type for all input parameters for the `calculateDiffLower`
    /// function with signature `calculateDiffLower(uint256,uint256,uint256)`
    /// and selector `0x332266f3`
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
        name = "calculateDiffLower",
        abi = "calculateDiffLower(uint256,uint256,uint256)"
    )]
    pub struct CalculateDiffLowerCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `calculateDiffRaise`
    /// function with signature `calculateDiffRaise(uint256,uint256,uint256)`
    /// and selector `0x902ecaa2`
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
        name = "calculateDiffRaise",
        abi = "calculateDiffRaise(uint256,uint256,uint256)"
    )]
    pub struct CalculateDiffRaiseCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `computeOptimalArbLowerPrice` function with signature
    /// `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector
    /// `0x306db46b`
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
        name = "computeOptimalArbLowerPrice",
        abi = "computeOptimalArbLowerPrice(uint256,uint256,uint256)"
    )]
    pub struct ComputeOptimalArbLowerPriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v_upper: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `computeOptimalArbRaisePrice` function with signature
    /// `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector
    /// `0x4fd67c58`
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
        name = "computeOptimalArbRaisePrice",
        abi = "computeOptimalArbRaisePrice(uint256,uint256,uint256)"
    )]
    pub struct ComputeOptimalArbRaisePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub v_upper: ::ethers::core::types::U256,
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
    /// Container type for all input parameters for the `fetchPoolParams`
    /// function with signature `fetchPoolParams(uint256)` and selector
    /// `0x81b5fac2`
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
    #[ethcall(name = "fetchPoolParams", abi = "fetchPoolParams(uint256)")]
    pub struct FetchPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,
    /// address))` and selector `0x134ead12`
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
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,address))"
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
    /// Container type for all input parameters for the `prepareSigmaUpdate`
    /// function with signature `prepareSigmaUpdate(uint256,uint256)` and
    /// selector `0xe94716d5`
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
        name = "prepareSigmaUpdate",
        abi = "prepareSigmaUpdate(uint256,uint256)"
    )]
    pub struct PrepareSigmaUpdateCall {
        pub target_sigma: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `prepareStrikeUpdate`
    /// function with signature `prepareStrikeUpdate(uint256,uint256)` and
    /// selector `0x0420580a`
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
        name = "prepareStrikeUpdate",
        abi = "prepareStrikeUpdate(uint256,uint256)"
    )]
    pub struct PrepareStrikeUpdateCall {
        pub target_strike: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `prepareTauUpdate`
    /// function with signature `prepareTauUpdate(uint256,uint256)` and selector
    /// `0x3b268d5d`
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
    #[ethcall(name = "prepareTauUpdate", abi = "prepareTauUpdate(uint256,uint256)")]
    pub struct PrepareTauUpdateCall {
        pub target_tau: ::ethers::core::types::U256,
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
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareSigmaUpdate(PrepareSigmaUpdateCall),
        PrepareStrikeUpdate(PrepareStrikeUpdateCall),
        PrepareTauUpdate(PrepareTauUpdateCall),
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
                <CalculateDiffLowerCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CalculateDiffLower(decoded));
            }
            if let Ok(decoded) =
                <CalculateDiffRaiseCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CalculateDiffRaise(decoded));
            }
            if let Ok(decoded) =
                <ComputeOptimalArbLowerPriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeOptimalArbLowerPrice(decoded));
            }
            if let Ok(decoded) =
                <ComputeOptimalArbRaisePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeOptimalArbRaisePrice(decoded));
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
                <FetchPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::FetchPoolParams(decoded));
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
                <PrepareSigmaUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareSigmaUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareStrikeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareStrikeUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareTauUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareTauUpdate(decoded));
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
                Self::AllocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CalculateDiffLower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDiffRaise(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FetchPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::PrepareSigmaUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareStrikeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareTauUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffLower(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffRaise(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbLowerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbRaisePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareSigmaUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareStrikeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareTauUpdate(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<CalculateDiffLowerCall> for LogNormalSolverCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for LogNormalSolverCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall> for LogNormalSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall> for LogNormalSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
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
    impl ::core::convert::From<FetchPoolParamsCall> for LogNormalSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
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
    impl ::core::convert::From<PrepareSigmaUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareSigmaUpdateCall) -> Self {
            Self::PrepareSigmaUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareStrikeUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareStrikeUpdateCall) -> Self {
            Self::PrepareStrikeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareTauUpdateCall> for LogNormalSolverCalls {
        fn from(value: PrepareTauUpdateCall) -> Self {
            Self::PrepareTauUpdate(value)
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
    /// Container type for all return fields from the `calculateDiffLower`
    /// function with signature `calculateDiffLower(uint256,uint256,uint256)`
    /// and selector `0x332266f3`
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
    pub struct CalculateDiffLowerReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the `calculateDiffRaise`
    /// function with signature `calculateDiffRaise(uint256,uint256,uint256)`
    /// and selector `0x902ecaa2`
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
    pub struct CalculateDiffRaiseReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the
    /// `computeOptimalArbLowerPrice` function with signature
    /// `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector
    /// `0x306db46b`
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
    pub struct ComputeOptimalArbLowerPriceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the
    /// `computeOptimalArbRaisePrice` function with signature
    /// `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector
    /// `0x4fd67c58`
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
    pub struct ComputeOptimalArbRaisePriceReturn(pub ::ethers::core::types::U256);
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
    /// Container type for all return fields from the `fetchPoolParams` function
    /// with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
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
    pub struct FetchPoolParamsReturn(pub LogNormalParams);
    /// Container type for all return fields from the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,uint256,
    /// address))` and selector `0x134ead12`
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
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
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
    /// Container type for all return fields from the `prepareSigmaUpdate`
    /// function with signature `prepareSigmaUpdate(uint256,uint256)` and
    /// selector `0xe94716d5`
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
    pub struct PrepareSigmaUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareStrikeUpdate`
    /// function with signature `prepareStrikeUpdate(uint256,uint256)` and
    /// selector `0x0420580a`
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
    pub struct PrepareStrikeUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareTauUpdate`
    /// function with signature `prepareTauUpdate(uint256,uint256)` and selector
    /// `0x3b268d5d`
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
    pub struct PrepareTauUpdateReturn(pub ::ethers::core::types::Bytes);
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
    /// `LogNormalParams(uint256,uint256,uint256,uint256,address)`
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
        pub strike: ::ethers::core::types::U256,
        pub sigma: ::ethers::core::types::U256,
        pub tau: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}

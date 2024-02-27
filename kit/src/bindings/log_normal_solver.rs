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
    const __BYTECODE: &[u8] = b"`\x804b\0\0zW`\x1Fb\x006\x8D8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17b\0\0\x7FW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12b\0\0zWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03b\0\0zW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa5\xF7\x90\x81b\0\0\x96\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xB7W\x80c\x12\x06I\xC5\x14a\x01\xB2W\x80c\x13N\xAD\x12\x14a\x01\xADW\x80c\x1E\x97\x8C\xB0\x14a\x01\xA8W\x80c0m\xB4k\x14a\x01\xA3W\x80c3\"f\xF3\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\x99W\x80c;&\x8D]\x14a\x01\x94W\x80c;M\x100\x14a\x01\x8FW\x80cN\x81\x7F\xD9\x14a\x01\x8AW\x80cO\xD6|X\x14a\x01\x85W\x80c^\xB4\x08\xFC\x14a\x01\x80W\x80cb7V\x9F\x14a\x01{W\x80cme\"\x99\x14a\x01vW\x80c\x7F\x17@\x9C\x14a\x01qW\x80c\x81\xB5\xFA\xC2\x14a\x01lW\x80c\x90.\xCA\xA2\x14a\x01gW\x80c\xA8\xC6.v\x14a\x01bW\x80c\xAFNC\x7F\x14a\x01]W\x80c\xB0\x9D\x04\xE5\x14a\x01XW\x80c\xCB\x1FU2\x14a\x01SW\x80c\xCE\x15;\xF4\x14a\x01NW\x80c\xE9G\x16\xD5\x14a\x01IW\x80c\xEE>\x8C\xFB\x14a\x01DW\x80c\xF3\r7\xF2\x14a\x01?Wc\xF9\xC2\x82\x11\x14a\x01:W`\0\x80\xFD[a\n\xDDV[a\n\xADV[a\n|V[a\nAV[a\n\x05V[a\t\xC0V[a\t\x8DV[a\tqV[a\tHV[a\t\x1FV[a\x08\xF2V[a\x08PV[a\x084V[a\x07\xC7V[a\x07\xABV[a\x07\x82V[a\x07fV[a\x077V[a\x06\xFCV[a\x04\x8DV[a\x046V[a\x04\x07V[a\x03\xE2V[a\x03TV[a\x02\x8EV[a\x02\x18V[`\0[\x83\x81\x10a\x01\xCFWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xBFV[\x90` \x91a\x01\xF8\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xBCV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\x15\x92\x81\x81R\x01\x90a\x01\xDFV[\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xDFV[\x03\x90\xF3[`\0\x80\xFD[`\x80\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02kW` a\x02\xAAa\x02\xA16a\x02pV[\x92\x91\x90\x91a\x0B+V[`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[a\x02\xB2V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02kWV[4a\x02kW`\xE06`\x03\x19\x01\x12a\x02kW`\xA06`C\x19\x01\x12a\x02kWa\x02ga\x03\xBC`@Qa\x03\x83\x81a\x02\xC8V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x03\xAC\x81a\x03CV[`\x80\x82\x01R`$5`\x045a\x13}V[`@Q\x91\x82\x91\x82a\x02\x04V[``\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90V[4a\x02kW` a\x02\xAAa\x04\x01a\x03\xF86a\x03\xC8V[\x91\x92\x90\x92a\x0E\xE9V[\x91a\x15fV[4a\x02kW` a\x02\xAAa\x04\x1A6a\x03\xC8V[\x90a\x04-a\x04'\x84a\x0E\xE9V[\x93a\x10\xBCV[\x92\x91\x90\x91a\x16SV[4a\x02kW` a\x02\xAAa\x04I6a\x03\xC8V[\x90a\x04Va\x04'\x84a\x0E\xE9V[\x92\x90Pa\x19\xA9V[\x80\x15\x15\x03a\x02kWV[\x90\x92`\x80\x92a\x02\x15\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xDFV[4a\x02kW``6`\x03\x19\x01\x12a\x02kWa\x05\x03`$5a\x06\x03`\x045a\x04\xB3\x83a\x04^V[`D5\x92a\x04\xBFa\x0C:V[\x93a\x04\xC8a\x0C:V[\x94a\x04\xD2\x84a\x10\xBCV[` \x84\x96\x93\x95\x92\x96\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x04\xF2\x87a\x0E\xE9V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x0F\xECV[\x92\x15a\x06zW\x92a\x05S\x92a\x05.a\x055\x93a\x05'``a\x05{\x99\x98\x01Q\x82a$\xD4V[\x93Qa\x0C\x93V[\x8ARa\x0C\x93V[a\x05G\x85\x89\x01\x91\x80\x83R\x89Q\x88a\x0C-V[\x90\x88Q\x90Q\x90\x87a\x0B+V[\x90a\x05ra\x05g` \x89\x01\x93\x80\x85Ra\x0C\x80V[\x80\x84R\x82Q\x11a\r\x14V[Q\x90Q\x90a\r\x07V[\x94[\x84Q\x92`\xC0` \x87\x01Q\x84\x88\x01\x92a\x05\xC3\x84Q\x97a\x05\xB5\x88Q\x99\x8A\x95\x86\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x03!V[`\0Ta\x05\xE6\x90a\x05\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\r\xA2V[\x03\x91Z\xFA\x94\x85\x15a\x06uW`\0\x95a\x065W[P\x90a\x06*\x91a\x02g\x95\x96Q\x90Q\x90a\x15fV[\x90Q\x94\x85\x94\x85a\x04hV[a\x02g\x95P\x90a\x06`a\x06*\x93\x92`\xC0=`\xC0\x11a\x06nW[a\x06X\x81\x83a\x03!V[\x81\x01\x90a\rkV[PPPPP\x95P\x90\x91a\x06\x16V[P=a\x06NV[a\x0B\x1FV[a\x06\xED\x92a\x06\xBDa\x06\xF6\x96a\x06\xB0a\x06\xE2\x95a\x06\xA9\x86a\x06\xA1``a\x06\xDA\x99\x01Q\x84a$\xD4V[\x90Q\x90a%*V[\x92Qa\x0C\x93V[\x92` \x8D\x01\x93\x84Ra\x0C\x93V[a\x06\xCF\x88\x8C\x01\x91\x80\x83R\x83Q\x8Ba\r\xC6V[\x91Q\x90Q\x90\x89a\r\xD3V[\x80\x89Ra\x0C\x80V[\x80\x88R\x82Q\x11a\x0C\xA0V[Q\x85Q\x90a\r\x07V[\x94a\x05}V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW` a\x02\xAA`\x045a\x04\x01a\x07\\\x82a\x10\xBCV[\x92\x91\x93\x90Pa\x0E\xE9V[4a\x02kW` a\x02\xAAa\x07|a\x03\xF86a\x03\xC8V[\x91a\x1B;V[4a\x02kW` a\x02\xAAa\x07\x956a\x03\xC8V[\x90a\x07\xA2a\x04'\x84a\x0E\xE9V[\x92\x91\x90\x91a\x1B\xB5V[4a\x02kW` a\x02\xAAa\x07\xBE6a\x02pV[\x92\x91\x90\x91a\r\xD3V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x08\x16`\x045a\x02ga\x07\xF8a\x07\xED\x83a\x10\xBCV[\x91\x90P`$5a\x1E\xE1V[\x93\x90\x92\x84\x84a\x08\x10a\x08\t\x84a\x0E\xE9V[\x83\x83a\x15fV[\x92a\x0B+V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`\0\x81R\xF3[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\x9Ea\x02ga\x08\x80a\x08v\x84a\x10\xBCV[\x91P`$5a\x1F\x0EV[\x92\x90\x93\x83\x85a\x08\x98a\x08\x91\x84a\x0E\xE9V[\x83\x83a\x1B;V[\x92a\r\xD3V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW`\xA0a\t\x10`\x045a\x0E\xE9V[a\t\x1D`@Q\x80\x92a\x08\xBCV[\xF3[4a\x02kW` a\x02\xAAa\t26a\x03\xC8V[\x90a\t?a\x04'\x84a\x0E\xE9V[\x92\x90\x91Pa\x1F5V[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02kW` a\x02\xAAa\t\x846a\x02pV[\x92\x91\x90\x91a\x0F\xECV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`\x045a\t\xE0\x81a\x03CV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02ga\n$`\x045a\x10\xBCV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x08\x16`\x045a\x02ga\x07\xF8a\n\xA2\x83a\x10\xBCV[\x91\x90P`$5a\x1F\x0EV[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\x9Ea\x02ga\x08\x80a\n\xD3\x84a\x10\xBCV[\x91P`$5a\x1E\xE1V[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`x\x81R\xF3[\x90\x81` \x91\x03\x12a\x02kWQ\x90V[`@\x90a\x02\x15\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x01\xDFV[`@Q=`\0\x82>=\x90\xFD[a\x0Bha\x0B\xD1\x94\x93\x92\x93a\x0Bc\x84a\x0B\\a\x0BWa\x0BRa\x0BK\x88a\x0E\xE9V[\x80\x96a\"<V[a\"\xB1V[a\"\xE4V[\x92Qa$\xD4V[a$\xD4V[\x91` `@Qa\x0B\x9F\x81a\x0B\x91\x85\x88\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03!V[`\0Ta\x0B\xB6\x90a\x05\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\x08V[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x0B\xF8W[Pa\x0B\xF2\x90a\x0E\xE9V[\x93a\x11\xD4V[a\x0B\xF2\x91\x93Pa\x0C\x1F\x90` =` \x11a\x0C&W[a\x0C\x17\x81\x83a\x03!V[\x81\x01\x90a\n\xF9V[\x92\x90a\x0B\xE8V[P=a\x0C\rV[\x91a\x04\x01a\x02\x15\x93a\x0E\xE9V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0C\x8EWV[a\x0CjV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x8EWV[\x15a\x0C\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0C\x8EWV[\x91\x90\x82\x03\x91\x82\x11a\x0C\x8EWV[\x15a\r\x1BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02kW\x81Qa\r\x82\x81a\x04^V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\x15\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xDFV[\x91a\x07|a\x02\x15\x93a\x0E\xE9V[\x92a\x0BWa\x0BRa\r\xF0\x92\x94\x93\x94a\r\xEA\x87a\x0E\xE9V[\x90a.zV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x93\x84\x03\x93\x84\x11a\x0C\x8EWa\x0E\x11a\x0E:\x94\x83a$\xD4V[\x91` `@Qa\x0B\x9F\x81a\x0B\x91\x85\x89\x89\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x0EaW[Pa\x0E[\x90a\x0E\xE9V[\x93a\x1D\x8BV[a\x0E[\x91\x93Pa\x0E\x7F\x90` =` \x11a\x0C&Wa\x0C\x17\x81\x83a\x03!V[\x92\x90a\x0EQV[\x91\x90\x82`\xA0\x91\x03\x12a\x02kW`@Qa\x0E\x9E\x81a\x02\xC8V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x0E\xD1\x83a\x03CV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02kWa\x02\x15\x91a\x0E\x86V[\x90`@Qa\x0E\xF6\x81a\x02\xC8V[`\0\x90\x81\x81R\x81`\x80` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x06uW\x80\x92a\x0F]W[Pa\x02\x15\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0E\xD5V[\x90\x91P=\x80\x82\x86>a\x0Fo\x81\x86a\x03!V[\x84\x01\x90\x82\x85\x83\x03\x12a\x0F\xE5W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\x0F\xE8W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x0F\xE5W\x81Q\x95\x86\x11a\x02\xE4W`@Q\x92a\x0F\xBB`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x03!V[\x86\x84R\x84\x87\x84\x01\x01\x11a\x0F\xE5WPa\x02\x15\x93\x94a\x0F\xDD\x91\x84\x80\x85\x01\x91\x01a\x01\xBCV[\x90\x83\x92a\x0FIV[\x80\xFD[\x82\x80\xFD[a\x10@\x93\x91\x92` `@Qa\x10\x1A\x81a\x0B\x91\x87\x86\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\x01\x80`\xA0\x1B\x03`\0T\x16`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\x08V[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x10gW[Pa\x10a\x90a\x0E\xE9V[\x93a \xE6V[a\x10a\x91\x93Pa\x10\x85\x90` =` \x11a\x0C&Wa\x0C\x17\x81\x83a\x03!V[\x92\x90a\x10WV[\x90\x81` \x91\x03\x12a\x02kWQa\x02\x15\x81a\x03CV[\x90\x81``\x91\x03\x12a\x02kW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\x10\xD8a\x05\xDAa\x05\xDA`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x06uWa\x11#\x93``\x92`\0\x91a\x11\x80W[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06uW`\0\x80\x93`\0\x93a\x11IW[P\x92\x91\x90V[\x91\x93PPa\x11o\x91P``=``\x11a\x11yW[a\x11g\x81\x83a\x03!V[\x81\x01\x90a\x10\xA1V[\x92\x90\x92\x918a\x11CV[P=a\x11]V[a\x11\xA2\x91P` =` \x11a\x11\xA8W[a\x11\x9A\x81\x83a\x03!V[\x81\x01\x90a\x10\x8CV[8a\x11\x02V[P=a\x11\x90V[a\x11\xD2\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\x08\xBCV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x13JW[\x85\x85\x12a\x13+W\x90a\x0B\x91a\x12\x07\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa\x12\x1D\x81\x85a2\xB5V[\x92a\x12(\x81\x86a2\xB5V[\x88a\x123\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a\x12G\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a\x12bW[PPPPPPPPPP\x90V[\x15a\x12\xC3W[P\x86\x97\x98P\x81\x92a\x12\x82a\x12|\x8B\x89a\x0C\x93V[`\x01\x1C\x90V[\x99a\x12\x8D\x8B\x88a2\xB5V[\x90\x84a\x12\x99\x88\x84a\x15\x12V[\x13a\x12\xB7WPP\x89\x93[\x88a\x12\xAE\x89\x87a\r\x07V[\x92\x01\x94\x99a\x12PV[\x8B\x98P\x90\x95P\x93a\x12\xA3V[`\x14\x10\x80a\x12\xDEW[\x15a\x12\xD7W\x88a\x12hV[\x80\x80a\x12UV[P\x80\x83\x10a\x12\xCCV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x137\x90a%\0V[\x91a\x13D\x84\x83\x85\x84a#\xDBV[\x93a\x11\xE5V[\x85\x85\x13a\x13^W\x90a\x0B\x91a\x12\x07\x92a\x11\xF5V[\x93P\x94a\x13j\x90a#'V[\x94a\x13w\x84\x83\x88\x84a#\xDBV[\x93a\x13JV[\x91a\x13\x8Ea\x0BWa\x0BR\x83\x85a.zV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0C\x8EWa\x13\xE5\x82a\x13\xD1a\x13\xC6a\x0BWa\x0BR\x84a\x13\xC0a\x14\x03\x9A\x8Ca%*V[\x97a\"<V[a\x0Bc\x85\x84Qa$\xD4V[\x92a\x13\xDE\x82\x82\x86\x8Aa#\xDBV[\x84\x88a \xE6V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\x08\xBCV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x8EWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x8EWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x8EWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x8EWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x8EW`\0\x19\x83\x05\x03a\x0C\x8EWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x8EW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x81\x15a\x15PW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\x8EW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x15\xA1` \x83\x01\x93a\x15\x9B\x85Qa\x15\x93a\x15\x89`@\x88\x01\x92\x83Q\x90a'uV[\x97Q\x82Q\x90a'\x9EV[\x90Q\x90a#FV[\x92a#gV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x16\x1FW`\0\x85\x13\x15a\x16\x14W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0C\x8EWa\x16\x08a\x16\r\x92a\x16\x03a\x15\xF5a\x0BW\x94a\x15\xF0a\x02\x15\x99a'\xBDV[a\x15\x12V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x14\x85V[a(IV[\x90Qa$\xD4V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x11\xD2\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\x08\xBCV[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x16q\x82a)\xFAV[a\x16z\x90a\x18\x0BV[\x92a\x16\x84\x90a\x14&V[a\x16\x8D\x89a\x192V[a\x16\x96\x90a*\xB8V[a\x16\x9F\x89a\x19DV[a\x16\xA8\x83a+\x16V[a\x16\xB1\x91a\x14\x85V[a\x16\xBA\x91a+\xEBV[a\x16\xC3\x90a,\tV[\x92a\x16\xCE\x83\x80a+nV[\x90a\x16\xD8\x91a+nV[a\x16\xE1\x90a+\x9DV[a\x16\xEA\x90a\x19\x98V[\x83\x85a\x16\xF5\x85a+?V[\x90a\x16\xFF\x91a+nV[\x90a\x17\t\x91a+nV[a\x17\x12\x91a\x19|V[a\x17\x1B\x90a(IV[a\x17$\x82a\x19bV[a\x17.\x90\x8Ba+nV[a\x178\x90\x8Aa\x19|V[a\x17B\x90\x87a+nV[a\x17K\x91a+nV[a\x17T\x89a\x19DV[a\x17]\x83a+\x16V[a\x17f\x91a\x14\x85V[a\x17o\x91a+\xEBV[\x94a\x17y\x90a*\xE7V[\x90a\x17\x83\x90a\x19bV[a\x17\x8C\x91a+nV[\x92a\x17\x96\x91a+nV[a\x17\x9F\x90a+\xC4V[\x90a\x17\xA9\x91a\x14\x85V[a\x17\xB2\x90a-ZV[a\x17\xBB\x91a+nV[\x90a\x17\xC5\x84a\x19\x98V[\x90a\x17\xCF\x91a\x19|V[\x90a\x17\xD9\x91a\x19|V[`\0\x13a\x18\0Wa\x02\x15\x95a\x17\xFB\x93a\x0B\x91\x92`@Q\x96\x87\x95` \x87\x01a\x16)V[a%\x7FV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90a\x03\xE8\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x8EWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x8EWV[`\x01`\xFF\x1B\x81\x14a\x0C\x8EW`\0\x03\x90V[\x93\x90\x91\x92\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x81a\x19\xCB\x86a)\xFAV[a\x19\xD4\x90a\x18\x0BV[\x96a\x19\xDE\x90a\x14&V[\x93\x82a\x19\xEB\x86\x94\x84a\x19|V[a\x19\xF4\x90a*\xB8V[a\x19\xFE\x84\x84a\x19|V[a\x1A\x08\x86\x86a+nV[a\x1A\x11\x91a\x14\x85V[a\x1A\x1A\x91a+\xEBV[a\x1A#\x90a,\tV[\x97a\x1A.\x88\x80a+nV[\x90a\x1A8\x91a+nV[a\x1AA\x90a+\x9DV[a\x1AJ\x90a\x19\x98V[\x88\x8Aa\x1AU\x8Aa+?V[\x90a\x1A_\x91a+nV[\x90a\x1Ai\x91a+nV[a\x1Ar\x91a\x19|V[a\x1A{\x90a(IV[\x90a\x1A\x85\x85a\x19bV[a\x1A\x8E\x91a+nV[a\x1A\x98\x90\x83a\x19|V[a\x1A\xA2\x90\x8Ba+nV[a\x1A\xAB\x91a+nV[\x93a\x1A\xB5\x91a\x19|V[\x91a\x1A\xBF\x91a+nV[a\x1A\xC8\x91a\x14\x85V[a\x1A\xD1\x91a+\xEBV[\x94a\x1A\xDB\x90a*\xE7V[\x90a\x1A\xE5\x90a\x19bV[a\x1A\xEE\x91a+nV[\x92a\x1A\xF8\x91a+nV[a\x1B\x01\x90a+\xC4V[\x90a\x1B\x0B\x91a\x14\x85V[a\x1B\x14\x90a-ZV[a\x1B\x1D\x91a+nV[\x91a\x1B'\x90a\x19\x98V[\x90a\x1B1\x91a\x19|V[\x90a\x02\x15\x91a\x19|V[\x91\x90\x91a\x1By` \x83\x01\x91a\x1Bsa\x1Bk\x84Qa\x15\x93a\x1Ba`@\x89\x01\x92\x83Q\x90a'uV[\x96Q\x82Q\x90a'\x9EV[\x95\x85Qa#FV[\x90a#gV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x16\x1FW`\0\x82\x13\x15a\x16\x14Wa\x02\x15\x94a\x16\r\x93a\x1B\xAFa\x16\x08\x93a\x15\xF0a\x0BW\x96a'\xBDV[\x05a\x19|V[\x90\x92\x91\x85Q` \x87\x01Q\x90`@\x88\x01Q``\x89\x01Q\x90a\x1B\xD4\x81a)\xFAV[a\x1B\xDD\x90a\x18\x0BV[\x91a\x1B\xE7\x90a\x14&V[\x93a\x1B\xF1\x86a\x192V[a\x1B\xFA\x90a*\xB8V[a\x1C\x04\x89\x86a+nV[a\x1C\r\x90a\x19DV[a\x1C\x16\x87a+\x16V[a\x1C\x1F\x91a\x14\x85V[a\x1C(\x91a+\xEBV[a\x1C1\x90a,\tV[\x91a\x1C<\x82\x80a+nV[\x90a\x1CF\x91a+nV[a\x1CO\x90a+\x9DV[a\x1CX\x90a\x19\x98V[\x82\x84a\x1Cc\x84a+?V[\x90a\x1Cm\x91a+nV[\x90a\x1Cw\x91a+nV[a\x1C\x80\x91a\x19|V[a\x1C\x89\x90a(IV[a\x1C\x93\x89\x86a+nV[a\x1C\x9C\x87a\x19bV[a\x1C\xA6\x90\x89a+nV[a\x1C\xAF\x91a\x19|V[a\x1C\xB9\x90\x89a+nV[a\x1C\xC2\x91a+nV[a\x1C\xCC\x89\x86a+nV[a\x1C\xD5\x90a\x19DV[a\x1C\xDE\x87a+\x16V[a\x1C\xE7\x91a\x14\x85V[a\x1C\xF1\x90\x86a+nV[a\x1C\xFA\x91a+\xEBV[\x94a\x1D\x04\x90a\x19bV[a\x1D\x0E\x90\x88a+nV[\x92a\x1D\x18\x91a+nV[a\x1D!\x90a+\xC4V[\x90a\x1D+\x91a\x14\x85V[a\x1D4\x90a-ZV[a\x1D=\x91a+nV[\x90a\x1DG\x90a*\xB8V[a\x1DP\x91a+\xEBV[\x90a\x1DZ\x90a\x19bV[\x90a\x1Dd\x91a\x19|V[`\0\x13a\x18\0Wa\x02\x15\x95a\x1D\x86\x93a\x0B\x91\x92`@Q\x96\x87\x95` \x87\x01a\x16)V[a&\x9EV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1E\xAEW[\x85\x85\x12a\x1E\x8FW\x90a\x0B\x91a\x1D\xBD\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa\x1D\xD3\x81\x85a2\xD6V[\x92a\x1D\xDE\x81\x86a2\xD6V[\x88a\x1D\xE9\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a\x1D\xFD\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a\x1E\x17WPPPPPPPPPP\x90V[\x15a\x1ErW[P\x86\x97\x98P\x81\x92a\x1E1a\x12|\x8B\x89a\x0C\x93V[\x99a\x1E<\x8B\x88a2\xD6V[\x90\x84a\x1EH\x88\x84a\x15\x12V[\x13a\x1EfWPP\x89\x93[\x88a\x1E]\x89\x87a\r\x07V[\x92\x01\x94\x99a\x1E\x06V[\x8B\x98P\x90\x95P\x93a\x1ERV[`\x14\x10\x80a\x1E\x86W[\x15a\x12\xD7W\x88a\x1E\x1DV[P\x80\x83\x10a\x1E{V[\x93P\x91a\x1E\x9B\x90a%\0V[\x91a\x1E\xA8\x84\x83\x83\x86a#\xDBV[\x93a\x1D\x9CV[\x85\x85\x13a\x1E\xC2W\x90a\x0B\x91a\x1D\xBD\x92a\x11\xF5V[\x93P\x94a\x1E\xCE\x90a#'V[\x94a\x1E\xDB\x84\x83\x83\x89a#\xDBV[\x93a\x1E\xAEV[\x92\x91\x90a\x1E\xF7a\x1E\xF1\x82\x84a%*V[\x85a$\xD4V[\x93\x81\x03\x90\x81\x11a\x0C\x8EW\x92\x81\x03\x90\x81\x11a\x0C\x8EW\x90V[\x92\x91\x90a\x1F\x1Ea\x1E\xF1\x82\x84a%*V[\x93\x81\x01\x80\x91\x11a\x0C\x8EW\x92\x81\x01\x80\x91\x11a\x0C\x8EW\x90V[\x92\x93\x90\x91\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x87a\x1FW\x86a)\xFAV[a\x1F`\x90a\x18\x0BV[\x96a\x1Fj\x90a\x14&V[\x98\x82a\x1Fw\x8B\x94\x83a\x19|V[a\x1F\x80\x90a*\xB8V[\x82a\x1F\x8B\x87\x8Da+nV[\x90a\x1F\x95\x91a\x19|V[a\x1F\x9F\x86\x85a+nV[a\x1F\xA8\x91a\x14\x85V[a\x1F\xB1\x91a+\xEBV[a\x1F\xBA\x90a,\tV[\x97a\x1F\xC5\x88\x80a+nV[\x90a\x1F\xCF\x91a+nV[a\x1F\xD8\x90a+\x9DV[a\x1F\xE1\x90a\x19\x98V[\x88\x8Aa\x1F\xEC\x8Aa+?V[\x90a\x1F\xF6\x91a+nV[\x90a \0\x91a+nV[a \t\x91a\x19|V[a \x12\x90a(IV[\x90a \x1D\x86\x8Ca+nV[\x90a '\x86a\x19bV[a 0\x91a+nV[a 9\x91a\x19|V[a C\x90\x87a+nV[a L\x91a+nV[\x93a W\x90\x8Aa+nV[\x90a a\x91a\x19|V[\x91a k\x91a+nV[a t\x91a\x14\x85V[a ~\x90\x87a+nV[a \x87\x91a+\xEBV[\x95a \x91\x90a\x19bV[a \x9A\x91a+nV[\x92a \xA4\x91a+nV[a \xAD\x90a+\xC4V[\x90a \xB7\x91a\x14\x85V[a \xC0\x90a-ZV[a \xC9\x91a+nV[\x90a \xD3\x90a*\xB8V[a \xDC\x91a+\xEBV[\x90a\x1B1\x90a\x19bV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\tW[\x85\x85\x12a!\xEAW\x90a\x0B\x91a!\x18\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa!.\x81\x85a2\xF8V[\x92a!9\x81\x86a2\xF8V[\x88a!D\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a!X\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a!rWPPPPPPPPPP\x90V[\x15a!\xCDW[P\x86\x97\x98P\x81\x92a!\x8Ca\x12|\x8B\x89a\x0C\x93V[\x99a!\x97\x8B\x88a2\xF8V[\x90\x84a!\xA3\x88\x84a\x15\x12V[\x13a!\xC1WPP\x89\x93[\x88a!\xB8\x89\x87a\r\x07V[\x92\x01\x94\x99a!aV[\x8B\x98P\x90\x95P\x93a!\xADV[`\x14\x10\x80a!\xE1W[\x15a\x12\xD7W\x88a!xV[P\x80\x83\x10a!\xD6V[\x93P\x94a!\xF6\x90a#'V[\x94a\"\x03\x84\x87\x84\x84a#\xDBV[\x93a \xF7V[\x85\x85\x13a\"\x1DW\x90a\x0B\x91a!\x18\x92a\x11\xF5V[\x93P\x91a\")\x90a%\0V[\x91a\"6\x84\x84\x84\x84a#\xDBV[\x93a\"\tV[a\"\xACa\"\xA7a\x02\x15\x93a\"\xA1a\"\x9C\x82Qa\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x86a\"\x81`@` \x8B\x01Q\x9A\x01Q\x96a\"{\x88\x8Ca'uV[\x9Da%*V[a.\xF1V[\x97a.\xF1V[a\x14\xBBV[\x05a(IV[a#FV[a#\x89V[\x90a\x14\x85V[a\x14\x9EV[a\x155V[a\"\xE0a\"\xA7a\"\xDBg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\"\xD5g\x1B\xC1mgN\xC8\0\0\x95a\x14\x9EV[\x05a\x19\x98V[a-ZV[\x05\x90V[`\0\x81\x12a\"\xEFW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a$\x8FWa\x02\x15\x93a$X\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a$\x03\x83\x83a#gV[\x10a$|WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a$+a$%\x83\x85a#FV[\x85a#gV[\x10a$]WP`\x01`\x01`\xFF\x1B\x03\x92a$R\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a'uV[\x92a\x19|V[a\x19|V[a$R\x92a\x1Bsa$q\x92a$v\x94a#FV[a'\xBDV[\x91a$BV[a$\x89\x91a$q\x91a#gV[\x94a$\x15V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02kW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02kW\x80Q\x92a\x02\x15` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x0E\x86V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a&}Wa%\x9B\x81a3\x18V[a%\xA5\x85\x83a4;V[`\0a%\xB1\x82\x84a\x15\x12V[\x13a&^WPa%\xC2\x85\x96\x95a\x0C\xF7V[`\x01\x94`\0\x91\x86\x80[a%\xDCW[PPPPPPPP\x90PV[\x15a&9W[P\x85\x96\x97\x98P\x80\x91a%\xF7a\x12|\x8B\x88a\x0C\x93V[\x99a&\x02\x8B\x87a4;V[\x90\x83a&\x0E\x87\x84a\x15\x12V[\x13a&-WPP\x89\x92[\x87a&#\x88\x86a\r\x07V[\x92\x01\x93\x99\x98a%\xCBV[\x8B\x97P\x90\x94P\x92a&\x18V[\x86\x10\x80a&SW[\x15a&LW\x88a%\xE2V[\x80\x80a%\xD0V[Pa\x01\0\x82\x10a&AV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a&}Wa&\xBA\x81a4]V[a&\xC4\x85\x83a5\xA0V[`\0a&\xD0\x82\x84a\x15\x12V[\x13a&^WPa&\xE1\x85\x96\x95a\x0C\xF7V[`\x01\x94`\0\x91\x86\x80[a&\xFAWPPPPPPPP\x90PV[\x15a'WW[P\x85\x96\x97\x98P\x80\x91a'\x15a\x12|\x8B\x88a\x0C\x93V[\x99a' \x8B\x87a5\xA0V[\x90\x83a',\x87\x84a\x15\x12V[\x13a'KWPP\x89\x92[\x87a'A\x88\x86a\r\x07V[\x92\x01\x93\x99\x98a&\xEAV[\x8B\x97P\x90\x94P\x92a'6V[\x86\x10\x80a'jW[\x15a&LW\x88a'\0V[Pa\x01\0\x82\x10a'_V[\x90a'\x7F\x90a)\xFAV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0C\x8EWa\x02\x15\x91a#FV[a\x02\x15\x91a\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x9C\x95a.\xF1V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a(CWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a(1W\x80\x15a(\x1FW\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0C\x8EWa'\xFB\x90a,\tV[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x02kWa\x02\x15\x91\x05a\x19\x98V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a(CWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a)\x99We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x02kW\x82Q\x92` \x81\x01Q\x92a\x02\x15`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0E\x86V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a*\xA1W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a*\x94W[e\x01\0\0\0\0\0\x81\x10\x15a*\x87W[c\x01\0\0\0\x81\x10\x15a*zW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a*>V[` \x1C\x91`\x10\x1B\x91a*1V[`@\x1C\x91` \x1B\x91a*\"V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca*\nV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x02kW\x05\x90V[`\0\x81\x12\x80\x15a-IW[a-7W\x80\x15a(1Wg\x1B\xC1mgN\xC8\0\0\x81\x14a(\x1FWg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a-(W\x90[a,J\x82a0\xBFV[\x80\x15a(1Wa,\xB3a,wa,ra,ma,ha,\xB8\x95a.\xF1V[a1\x80V[a)\xFAV[a\x14\xD8V[a\x16\x03a,\x8Ba,\x86\x83a0\xEAV[a\x18$V[a,\xADa,\xA8a,\xA2a,\x9D\x86a1\x15V[a\x18<V[\x85a1\xF7V[a\x18TV[\x90a1^V[a1\xA8V[\x91`\0\x90[`\x02\x82\x10a,\xD8WPP\x15a,\xCFW\x90V[a\x02\x15\x90a\x19\x98V[\x90\x92a- \x81a-\x1Aa,\xF0\x85a\x16\x03`\x01\x96a-ZV[a,\xADa-\x10a-\x0Ba\x16\x08a-\x06\x87\x80a1\xF7V[a\x19\x98V[a1\xD0V[a\"\xA1\x83\x86a1\xF7V[\x90a\x19|V[\x93\x01\x90a,\xBDV[a-1\x90a\x14IV[\x90a,AV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a,\x14V[\x80\x15a.mWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a(CWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a.`W`\0a.Pa-\x8F\x83a0\x92V[a.\x18a\x16\x08a-\xA9a-\xA4a,\xA8\x85a#\xB0V[a1?V[\x92a$Xa.Ka.Fa.?a.9a.4a..a.)a.#a.\x1E\x8Da.\x18a.\x13a.\ra.\x08a,\xA2a.\x03a-\xFDa-\xF8a-\xF2a-\xED\x8Aa2\x18V[a\x18lV[\x89a1\xF7V[a\x18\x86V[\x87a1\xF7V[a\x18\x9EV[a\x18\xB8V[\x83a1\xF7V[a\x18\xD0V[\x90a1\xF7V[a\x18\xEAV[\x8Ca1\xF7V[a\x19\x02V[\x8Aa1\xF7V[a\x19\x1AV[\x88a1\xF7V[\x93\x80a1\xF7V[a\x14\xF1V[a\x14lV[\x91\x12\x15a\x02\x15Wa\x02\x15\x90a\x14IV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a\"\xACa\"\xA7a\x02\x15\x93a-\x1Aa\"\x9C\x82Qa\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x86a\"\x81`@` \x8B\x01Q\x9A\x01Q\x96a\"{\x88\x8Ca'uV[\x15a.\xC0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/\x1D`\0\x82\x13a.\xB9V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a/9\x82a2CV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a0\xADW`\0\x81\x12\x15a\x02\x15W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02kWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a2N\x81\x15\x15a.\xB9V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a2\xCCa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x91\x92\x90Pa#\xDBV[\x90a2\xEDa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x90P\x91\x90\x91a#\xDBV[\x90a3\x0Fa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x92\x90Pa#\xDBV[\x80Q\x81\x01` \x01\x90` \x01\x90a3-\x91a)\xCDV[\x80\x91\x92PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a3M\x84a)\xFAV[a3V\x90a\x18\x0BV[\x94a3`\x90a\x14&V[\x91a3j\x81a\x192V[a3s\x90a*\xB8V[a3|\x83a\x19DV[a3\x85\x85a+\x16V[a3\x8E\x91a\x14\x85V[a3\x97\x91a+\xEBV[a3\xA0\x90a,\tV[\x94a3\xAB\x85\x80a+nV[\x90a3\xB5\x91a+nV[a3\xBE\x90a+\x9DV[a3\xC7\x90a\x19\x98V[\x85\x87a3\xD2\x87a+?V[\x90a3\xDC\x91a+nV[\x90a3\xE6\x91a+nV[a3\xEF\x91a\x19|V[a3\xF8\x90a(IV[\x90a4\x02\x84a\x19bV[a4\x0B\x91a+nV[a4\x15\x90\x83a\x19|V[a4\x1F\x90\x88a+nV[a4(\x91a+nV[\x90a42\x90a\x19DV[a\x1A\xBF\x83a+\x16V[\x90a4Ra\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a)\xCDV[\x94\x93\x90\x92\x91Pa\x19\xA9V[\x80Q\x81\x01` \x01\x90` \x01\x90a4r\x91a)\xCDV[\x80\x91\x92\x94\x93PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a4\x94\x84a)\xFAV[a4\x9D\x90a\x18\x0BV[\x94a4\xA7\x90a\x14&V[\x96a4\xB1\x81a\x192V[a4\xBA\x90a*\xB8V[a4\xC4\x83\x89a+nV[a4\xCD\x90a\x19DV[a4\xD6\x8Aa+\x16V[a4\xDF\x91a\x14\x85V[a4\xE8\x91a+\xEBV[a4\xF1\x90a,\tV[\x94a4\xFC\x85\x80a+nV[\x90a5\x06\x91a+nV[a5\x0F\x90a+\x9DV[a5\x18\x90a\x19\x98V[\x85\x87a5#\x87a+?V[\x90a5-\x91a+nV[\x90a57\x91a+nV[a5@\x91a\x19|V[a5I\x90a(IV[\x90a5T\x83\x89a+nV[\x90a5^\x8Aa\x19bV[a5g\x91a+nV[a5p\x91a\x19|V[a5z\x90\x84a+nV[a5\x83\x91a+nV[\x90a5\x8E\x90\x87a+nV[a5\x97\x90a\x19DV[a k\x88a+\x16V[\x90a5\xB7a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a)\xCDV[\x94\x93\x90\x92Pa\x1F5V\xFE\xA2dipfsX\"\x12 \x9F\x9B9@s<\xB4i(\xE4}\x8B\x14\xB0\xB3?Y\x1D\x84\x8D\x1C\xB1\x18\xADTc\xCBw]\xAE.\xD4dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x04 X\n\x14a\x01\xB7W\x80c\x12\x06I\xC5\x14a\x01\xB2W\x80c\x13N\xAD\x12\x14a\x01\xADW\x80c\x1E\x97\x8C\xB0\x14a\x01\xA8W\x80c0m\xB4k\x14a\x01\xA3W\x80c3\"f\xF3\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\x99W\x80c;&\x8D]\x14a\x01\x94W\x80c;M\x100\x14a\x01\x8FW\x80cN\x81\x7F\xD9\x14a\x01\x8AW\x80cO\xD6|X\x14a\x01\x85W\x80c^\xB4\x08\xFC\x14a\x01\x80W\x80cb7V\x9F\x14a\x01{W\x80cme\"\x99\x14a\x01vW\x80c\x7F\x17@\x9C\x14a\x01qW\x80c\x81\xB5\xFA\xC2\x14a\x01lW\x80c\x90.\xCA\xA2\x14a\x01gW\x80c\xA8\xC6.v\x14a\x01bW\x80c\xAFNC\x7F\x14a\x01]W\x80c\xB0\x9D\x04\xE5\x14a\x01XW\x80c\xCB\x1FU2\x14a\x01SW\x80c\xCE\x15;\xF4\x14a\x01NW\x80c\xE9G\x16\xD5\x14a\x01IW\x80c\xEE>\x8C\xFB\x14a\x01DW\x80c\xF3\r7\xF2\x14a\x01?Wc\xF9\xC2\x82\x11\x14a\x01:W`\0\x80\xFD[a\n\xDDV[a\n\xADV[a\n|V[a\nAV[a\n\x05V[a\t\xC0V[a\t\x8DV[a\tqV[a\tHV[a\t\x1FV[a\x08\xF2V[a\x08PV[a\x084V[a\x07\xC7V[a\x07\xABV[a\x07\x82V[a\x07fV[a\x077V[a\x06\xFCV[a\x04\x8DV[a\x046V[a\x04\x07V[a\x03\xE2V[a\x03TV[a\x02\x8EV[a\x02\x18V[`\0[\x83\x81\x10a\x01\xCFWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xBFV[\x90` \x91a\x01\xF8\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xBCV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x02\x15\x92\x81\x81R\x01\x90a\x01\xDFV[\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xDFV[\x03\x90\xF3[`\0\x80\xFD[`\x80\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90`d5\x90V[4a\x02kW` a\x02\xAAa\x02\xA16a\x02pV[\x92\x91\x90\x91a\x0B+V[`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[a\x02\xB2V[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x02kWV[4a\x02kW`\xE06`\x03\x19\x01\x12a\x02kW`\xA06`C\x19\x01\x12a\x02kWa\x02ga\x03\xBC`@Qa\x03\x83\x81a\x02\xC8V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45``\x82\x01R`\xC45a\x03\xAC\x81a\x03CV[`\x80\x82\x01R`$5`\x045a\x13}V[`@Q\x91\x82\x91\x82a\x02\x04V[``\x90`\x03\x19\x01\x12a\x02kW`\x045\x90`$5\x90`D5\x90V[4a\x02kW` a\x02\xAAa\x04\x01a\x03\xF86a\x03\xC8V[\x91\x92\x90\x92a\x0E\xE9V[\x91a\x15fV[4a\x02kW` a\x02\xAAa\x04\x1A6a\x03\xC8V[\x90a\x04-a\x04'\x84a\x0E\xE9V[\x93a\x10\xBCV[\x92\x91\x90\x91a\x16SV[4a\x02kW` a\x02\xAAa\x04I6a\x03\xC8V[\x90a\x04Va\x04'\x84a\x0E\xE9V[\x92\x90Pa\x19\xA9V[\x80\x15\x15\x03a\x02kWV[\x90\x92`\x80\x92a\x02\x15\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xDFV[4a\x02kW``6`\x03\x19\x01\x12a\x02kWa\x05\x03`$5a\x06\x03`\x045a\x04\xB3\x83a\x04^V[`D5\x92a\x04\xBFa\x0C:V[\x93a\x04\xC8a\x0C:V[\x94a\x04\xD2\x84a\x10\xBCV[` \x84\x96\x93\x95\x92\x96\x01\x94`@\x96\x87\x86\x01\x92\x83R\x86R\x84Ra\x04\xF2\x87a\x0E\xE9V[\x99\x8A\x91\x85Q\x90\x87Q\x90Q\x91\x8Aa\x0F\xECV[\x92\x15a\x06zW\x92a\x05S\x92a\x05.a\x055\x93a\x05'``a\x05{\x99\x98\x01Q\x82a$\xD4V[\x93Qa\x0C\x93V[\x8ARa\x0C\x93V[a\x05G\x85\x89\x01\x91\x80\x83R\x89Q\x88a\x0C-V[\x90\x88Q\x90Q\x90\x87a\x0B+V[\x90a\x05ra\x05g` \x89\x01\x93\x80\x85Ra\x0C\x80V[\x80\x84R\x82Q\x11a\r\x14V[Q\x90Q\x90a\r\x07V[\x94[\x84Q\x92`\xC0` \x87\x01Q\x84\x88\x01\x92a\x05\xC3\x84Q\x97a\x05\xB5\x88Q\x99\x8A\x95\x86\x93` \x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x03!V[`\0Ta\x05\xE6\x90a\x05\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\r\xA2V[\x03\x91Z\xFA\x94\x85\x15a\x06uW`\0\x95a\x065W[P\x90a\x06*\x91a\x02g\x95\x96Q\x90Q\x90a\x15fV[\x90Q\x94\x85\x94\x85a\x04hV[a\x02g\x95P\x90a\x06`a\x06*\x93\x92`\xC0=`\xC0\x11a\x06nW[a\x06X\x81\x83a\x03!V[\x81\x01\x90a\rkV[PPPPP\x95P\x90\x91a\x06\x16V[P=a\x06NV[a\x0B\x1FV[a\x06\xED\x92a\x06\xBDa\x06\xF6\x96a\x06\xB0a\x06\xE2\x95a\x06\xA9\x86a\x06\xA1``a\x06\xDA\x99\x01Q\x84a$\xD4V[\x90Q\x90a%*V[\x92Qa\x0C\x93V[\x92` \x8D\x01\x93\x84Ra\x0C\x93V[a\x06\xCF\x88\x8C\x01\x91\x80\x83R\x83Q\x8Ba\r\xC6V[\x91Q\x90Q\x90\x89a\r\xD3V[\x80\x89Ra\x0C\x80V[\x80\x88R\x82Q\x11a\x0C\xA0V[Q\x85Q\x90a\r\x07V[\x94a\x05}V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x04` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW` a\x02\xAA`\x045a\x04\x01a\x07\\\x82a\x10\xBCV[\x92\x91\x93\x90Pa\x0E\xE9V[4a\x02kW` a\x02\xAAa\x07|a\x03\xF86a\x03\xC8V[\x91a\x1B;V[4a\x02kW` a\x02\xAAa\x07\x956a\x03\xC8V[\x90a\x07\xA2a\x04'\x84a\x0E\xE9V[\x92\x91\x90\x91a\x1B\xB5V[4a\x02kW` a\x02\xAAa\x07\xBE6a\x02pV[\x92\x91\x90\x91a\r\xD3V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x08\x16`\x045a\x02ga\x07\xF8a\x07\xED\x83a\x10\xBCV[\x91\x90P`$5a\x1E\xE1V[\x93\x90\x92\x84\x84a\x08\x10a\x08\t\x84a\x0E\xE9V[\x83\x83a\x15fV[\x92a\x0B+V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`\0\x81R\xF3[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\x9Ea\x02ga\x08\x80a\x08v\x84a\x10\xBCV[\x91P`$5a\x1F\x0EV[\x92\x90\x93\x83\x85a\x08\x98a\x08\x91\x84a\x0E\xE9V[\x83\x83a\x1B;V[\x92a\r\xD3V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kW`\xA0a\t\x10`\x045a\x0E\xE9V[a\t\x1D`@Q\x80\x92a\x08\xBCV[\xF3[4a\x02kW` a\x02\xAAa\t26a\x03\xC8V[\x90a\t?a\x04'\x84a\x0E\xE9V[\x92\x90\x91Pa\x1F5V[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x02kW` a\x02\xAAa\t\x846a\x02pV[\x92\x91\x90\x91a\x0F\xECV[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02g`\x045a\t\xE0\x81a\x03CV[`@\x80Q`\x05` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02S\x81a\x03\x05V[4a\x02kW` 6`\x03\x19\x01\x12a\x02kWa\x02ga\n$`\x045a\x10\xBCV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x02g`@Q`\x03` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02S\x81a\x02\xE9V[4a\x02kW`@6`\x03\x19\x01\x12a\x02kWa\x08\x16`\x045a\x02ga\x07\xF8a\n\xA2\x83a\x10\xBCV[\x91\x90P`$5a\x1F\x0EV[4a\x02kW`@6`\x03\x19\x01\x12a\x02kW`\x045a\x08\x9Ea\x02ga\x08\x80a\n\xD3\x84a\x10\xBCV[\x91P`$5a\x1E\xE1V[4a\x02kW`\x006`\x03\x19\x01\x12a\x02kW` `@Q`x\x81R\xF3[\x90\x81` \x91\x03\x12a\x02kWQ\x90V[`@\x90a\x02\x15\x93\x92\x81R\x81` \x82\x01R\x01\x90a\x01\xDFV[`@Q=`\0\x82>=\x90\xFD[a\x0Bha\x0B\xD1\x94\x93\x92\x93a\x0Bc\x84a\x0B\\a\x0BWa\x0BRa\x0BK\x88a\x0E\xE9V[\x80\x96a\"<V[a\"\xB1V[a\"\xE4V[\x92Qa$\xD4V[a$\xD4V[\x91` `@Qa\x0B\x9F\x81a\x0B\x91\x85\x88\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x03!V[`\0Ta\x0B\xB6\x90a\x05\xDA\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\x08V[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x0B\xF8W[Pa\x0B\xF2\x90a\x0E\xE9V[\x93a\x11\xD4V[a\x0B\xF2\x91\x93Pa\x0C\x1F\x90` =` \x11a\x0C&W[a\x0C\x17\x81\x83a\x03!V[\x81\x01\x90a\n\xF9V[\x92\x90a\x0B\xE8V[P=a\x0C\rV[\x91a\x04\x01a\x02\x15\x93a\x0E\xE9V[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\x0C\x8EWV[a\x0CjV[\x91\x90\x82\x01\x80\x92\x11a\x0C\x8EWV[\x15a\x0C\xA7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x0C\x8EWV[\x91\x90\x82\x03\x91\x82\x11a\x0C\x8EWV[\x15a\r\x1BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x02kW\x81Qa\r\x82\x81a\x04^V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x02\x15\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xDFV[\x91a\x07|a\x02\x15\x93a\x0E\xE9V[\x92a\x0BWa\x0BRa\r\xF0\x92\x94\x93\x94a\r\xEA\x87a\x0E\xE9V[\x90a.zV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x93\x84\x03\x93\x84\x11a\x0C\x8EWa\x0E\x11a\x0E:\x94\x83a$\xD4V[\x91` `@Qa\x0B\x9F\x81a\x0B\x91\x85\x89\x89\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x0EaW[Pa\x0E[\x90a\x0E\xE9V[\x93a\x1D\x8BV[a\x0E[\x91\x93Pa\x0E\x7F\x90` =` \x11a\x0C&Wa\x0C\x17\x81\x83a\x03!V[\x92\x90a\x0EQV[\x91\x90\x82`\xA0\x91\x03\x12a\x02kW`@Qa\x0E\x9E\x81a\x02\xC8V[`\x80\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R``\x81\x01Q``\x85\x01R\x01Q\x91a\x0E\xD1\x83a\x03CV[\x01RV[\x90`\xA0\x82\x82\x03\x12a\x02kWa\x02\x15\x91a\x0E\x86V[\x90`@Qa\x0E\xF6\x81a\x02\xC8V[`\0\x90\x81\x81R\x81`\x80` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x06uW\x80\x92a\x0F]W[Pa\x02\x15\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0E\xD5V[\x90\x91P=\x80\x82\x86>a\x0Fo\x81\x86a\x03!V[\x84\x01\x90\x82\x85\x83\x03\x12a\x0F\xE5W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\x0F\xE8W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x0F\xE5W\x81Q\x95\x86\x11a\x02\xE4W`@Q\x92a\x0F\xBB`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x03!V[\x86\x84R\x84\x87\x84\x01\x01\x11a\x0F\xE5WPa\x02\x15\x93\x94a\x0F\xDD\x91\x84\x80\x85\x01\x91\x01a\x01\xBCV[\x90\x83\x92a\x0FIV[\x80\xFD[\x82\x80\xFD[a\x10@\x93\x91\x92` `@Qa\x10\x1A\x81a\x0B\x91\x87\x86\x8A\x87\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[`\x01\x80`\xA0\x1B\x03`\0T\x16`@Q\x80\x80\x99\x81\x94b.RK`\xE0\x1B\x83R\x88`\x04\x84\x01a\x0B\x08V[\x03\x91Z\xFA\x91\x82\x15a\x06uWa\x02\x15\x95`\0\x93a\x10gW[Pa\x10a\x90a\x0E\xE9V[\x93a \xE6V[a\x10a\x91\x93Pa\x10\x85\x90` =` \x11a\x0C&Wa\x0C\x17\x81\x83a\x03!V[\x92\x90a\x10WV[\x90\x81` \x91\x03\x12a\x02kWQa\x02\x15\x81a\x03CV[\x90\x81``\x91\x03\x12a\x02kW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\x10\xD8a\x05\xDAa\x05\xDA`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x06uWa\x11#\x93``\x92`\0\x91a\x11\x80W[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x06uW`\0\x80\x93`\0\x93a\x11IW[P\x92\x91\x90V[\x91\x93PPa\x11o\x91P``=``\x11a\x11yW[a\x11g\x81\x83a\x03!V[\x81\x01\x90a\x10\xA1V[\x92\x90\x92\x918a\x11CV[P=a\x11]V[a\x11\xA2\x91P` =` \x11a\x11\xA8W[a\x11\x9A\x81\x83a\x03!V[\x81\x01\x90a\x10\x8CV[8a\x11\x02V[P=a\x11\x90V[a\x11\xD2\x93``\x92\x96\x95\x93a\x01\0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90a\x08\xBCV[V[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x13JW[\x85\x85\x12a\x13+W\x90a\x0B\x91a\x12\x07\x92[`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa\x12\x1D\x81\x85a2\xB5V[\x92a\x12(\x81\x86a2\xB5V[\x88a\x123\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a\x12G\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a\x12bW[PPPPPPPPPP\x90V[\x15a\x12\xC3W[P\x86\x97\x98P\x81\x92a\x12\x82a\x12|\x8B\x89a\x0C\x93V[`\x01\x1C\x90V[\x99a\x12\x8D\x8B\x88a2\xB5V[\x90\x84a\x12\x99\x88\x84a\x15\x12V[\x13a\x12\xB7WPP\x89\x93[\x88a\x12\xAE\x89\x87a\r\x07V[\x92\x01\x94\x99a\x12PV[\x8B\x98P\x90\x95P\x93a\x12\xA3V[`\x14\x10\x80a\x12\xDEW[\x15a\x12\xD7W\x88a\x12hV[\x80\x80a\x12UV[P\x80\x83\x10a\x12\xCCV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x86\x90R`$\x81\x01\x91\x90\x91R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`$\x81\x01\x92\x90\x92RP`D\x90\xFD[\x93P\x91a\x137\x90a%\0V[\x91a\x13D\x84\x83\x85\x84a#\xDBV[\x93a\x11\xE5V[\x85\x85\x13a\x13^W\x90a\x0B\x91a\x12\x07\x92a\x11\xF5V[\x93P\x94a\x13j\x90a#'V[\x94a\x13w\x84\x83\x88\x84a#\xDBV[\x93a\x13JV[\x91a\x13\x8Ea\x0BWa\x0BR\x83\x85a.zV[\x91g\r\xE0\xB6\xB3\xA7d\0\0\x92\x83\x03\x92\x83\x11a\x0C\x8EWa\x13\xE5\x82a\x13\xD1a\x13\xC6a\x0BWa\x0BR\x84a\x13\xC0a\x14\x03\x9A\x8Ca%*V[\x97a\"<V[a\x0Bc\x85\x84Qa$\xD4V[\x92a\x13\xDE\x82\x82\x86\x8Aa#\xDBV[\x84\x88a \xE6V[\x90`@Q\x94` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01\x90a\x08\xBCV[a\x01\0\x81Ra\x01 \x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x02\xE4W`@R\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x8EWV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\x0C\x8EWV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\x0C\x8EWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\x0C\x8EWV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90g\x1B\xC1mgN\xC8\0\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\x0C\x8EW`\0\x19\x83\x05\x03a\x0C\x8EWV[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\x0C\x8EW\x81\x84\x05\x14\x90\x15\x17\x15a\x0C\x8EWV[\x81\x15a\x15PW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\x0C\x8EW\x05\x90V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[\x91\x90\x91a\x15\xA1` \x83\x01\x93a\x15\x9B\x85Qa\x15\x93a\x15\x89`@\x88\x01\x92\x83Q\x90a'uV[\x97Q\x82Q\x90a'\x9EV[\x90Q\x90a#FV[\x92a#gV[\x92g\r\xE0\xB6\xB3\xA7d\0\0\x80\x85\x12\x15a\x16\x1FW`\0\x85\x13\x15a\x16\x14W`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\x0C\x8EWa\x16\x08a\x16\r\x92a\x16\x03a\x15\xF5a\x0BW\x94a\x15\xF0a\x02\x15\x99a'\xBDV[a\x15\x12V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x14\x85V[a(IV[\x90Qa$\xD4V[PPPPP`\0\x19\x90V[PPPPP`\0\x90V[\x90\x95\x94\x92\x93a\x11\xD2\x94`\x80\x93a\x01 \x84\x01\x98\x84R` \x84\x01R`@\x83\x01R``\x82\x01R\x01\x90a\x08\xBCV[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Q``\x89\x01Q\x91a\x16q\x82a)\xFAV[a\x16z\x90a\x18\x0BV[\x92a\x16\x84\x90a\x14&V[a\x16\x8D\x89a\x192V[a\x16\x96\x90a*\xB8V[a\x16\x9F\x89a\x19DV[a\x16\xA8\x83a+\x16V[a\x16\xB1\x91a\x14\x85V[a\x16\xBA\x91a+\xEBV[a\x16\xC3\x90a,\tV[\x92a\x16\xCE\x83\x80a+nV[\x90a\x16\xD8\x91a+nV[a\x16\xE1\x90a+\x9DV[a\x16\xEA\x90a\x19\x98V[\x83\x85a\x16\xF5\x85a+?V[\x90a\x16\xFF\x91a+nV[\x90a\x17\t\x91a+nV[a\x17\x12\x91a\x19|V[a\x17\x1B\x90a(IV[a\x17$\x82a\x19bV[a\x17.\x90\x8Ba+nV[a\x178\x90\x8Aa\x19|V[a\x17B\x90\x87a+nV[a\x17K\x91a+nV[a\x17T\x89a\x19DV[a\x17]\x83a+\x16V[a\x17f\x91a\x14\x85V[a\x17o\x91a+\xEBV[\x94a\x17y\x90a*\xE7V[\x90a\x17\x83\x90a\x19bV[a\x17\x8C\x91a+nV[\x92a\x17\x96\x91a+nV[a\x17\x9F\x90a+\xC4V[\x90a\x17\xA9\x91a\x14\x85V[a\x17\xB2\x90a-ZV[a\x17\xBB\x91a+nV[\x90a\x17\xC5\x84a\x19\x98V[\x90a\x17\xCF\x91a\x19|V[\x90a\x17\xD9\x91a\x19|V[`\0\x13a\x18\0Wa\x02\x15\x95a\x17\xFB\x93a\x0B\x91\x92`@Q\x96\x87\x95` \x87\x01a\x16)V[a%\x7FV[PPPPPP`\0\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x15a\x0C\x8EWV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\x0C\x8EWV[\x90a\x03\xE8\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x8EWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\x0C\x8EWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\x0C\x8EWV[`\x01`\xFF\x1B\x81\x14a\x0C\x8EW`\0\x03\x90V[\x93\x90\x91\x92\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x81a\x19\xCB\x86a)\xFAV[a\x19\xD4\x90a\x18\x0BV[\x96a\x19\xDE\x90a\x14&V[\x93\x82a\x19\xEB\x86\x94\x84a\x19|V[a\x19\xF4\x90a*\xB8V[a\x19\xFE\x84\x84a\x19|V[a\x1A\x08\x86\x86a+nV[a\x1A\x11\x91a\x14\x85V[a\x1A\x1A\x91a+\xEBV[a\x1A#\x90a,\tV[\x97a\x1A.\x88\x80a+nV[\x90a\x1A8\x91a+nV[a\x1AA\x90a+\x9DV[a\x1AJ\x90a\x19\x98V[\x88\x8Aa\x1AU\x8Aa+?V[\x90a\x1A_\x91a+nV[\x90a\x1Ai\x91a+nV[a\x1Ar\x91a\x19|V[a\x1A{\x90a(IV[\x90a\x1A\x85\x85a\x19bV[a\x1A\x8E\x91a+nV[a\x1A\x98\x90\x83a\x19|V[a\x1A\xA2\x90\x8Ba+nV[a\x1A\xAB\x91a+nV[\x93a\x1A\xB5\x91a\x19|V[\x91a\x1A\xBF\x91a+nV[a\x1A\xC8\x91a\x14\x85V[a\x1A\xD1\x91a+\xEBV[\x94a\x1A\xDB\x90a*\xE7V[\x90a\x1A\xE5\x90a\x19bV[a\x1A\xEE\x91a+nV[\x92a\x1A\xF8\x91a+nV[a\x1B\x01\x90a+\xC4V[\x90a\x1B\x0B\x91a\x14\x85V[a\x1B\x14\x90a-ZV[a\x1B\x1D\x91a+nV[\x91a\x1B'\x90a\x19\x98V[\x90a\x1B1\x91a\x19|V[\x90a\x02\x15\x91a\x19|V[\x91\x90\x91a\x1By` \x83\x01\x91a\x1Bsa\x1Bk\x84Qa\x15\x93a\x1Ba`@\x89\x01\x92\x83Q\x90a'uV[\x96Q\x82Q\x90a'\x9EV[\x95\x85Qa#FV[\x90a#gV[g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x82\x12\x15a\x16\x1FW`\0\x82\x13\x15a\x16\x14Wa\x02\x15\x94a\x16\r\x93a\x1B\xAFa\x16\x08\x93a\x15\xF0a\x0BW\x96a'\xBDV[\x05a\x19|V[\x90\x92\x91\x85Q` \x87\x01Q\x90`@\x88\x01Q``\x89\x01Q\x90a\x1B\xD4\x81a)\xFAV[a\x1B\xDD\x90a\x18\x0BV[\x91a\x1B\xE7\x90a\x14&V[\x93a\x1B\xF1\x86a\x192V[a\x1B\xFA\x90a*\xB8V[a\x1C\x04\x89\x86a+nV[a\x1C\r\x90a\x19DV[a\x1C\x16\x87a+\x16V[a\x1C\x1F\x91a\x14\x85V[a\x1C(\x91a+\xEBV[a\x1C1\x90a,\tV[\x91a\x1C<\x82\x80a+nV[\x90a\x1CF\x91a+nV[a\x1CO\x90a+\x9DV[a\x1CX\x90a\x19\x98V[\x82\x84a\x1Cc\x84a+?V[\x90a\x1Cm\x91a+nV[\x90a\x1Cw\x91a+nV[a\x1C\x80\x91a\x19|V[a\x1C\x89\x90a(IV[a\x1C\x93\x89\x86a+nV[a\x1C\x9C\x87a\x19bV[a\x1C\xA6\x90\x89a+nV[a\x1C\xAF\x91a\x19|V[a\x1C\xB9\x90\x89a+nV[a\x1C\xC2\x91a+nV[a\x1C\xCC\x89\x86a+nV[a\x1C\xD5\x90a\x19DV[a\x1C\xDE\x87a+\x16V[a\x1C\xE7\x91a\x14\x85V[a\x1C\xF1\x90\x86a+nV[a\x1C\xFA\x91a+\xEBV[\x94a\x1D\x04\x90a\x19bV[a\x1D\x0E\x90\x88a+nV[\x92a\x1D\x18\x91a+nV[a\x1D!\x90a+\xC4V[\x90a\x1D+\x91a\x14\x85V[a\x1D4\x90a-ZV[a\x1D=\x91a+nV[\x90a\x1DG\x90a*\xB8V[a\x1DP\x91a+\xEBV[\x90a\x1DZ\x90a\x19bV[\x90a\x1Dd\x91a\x19|V[`\0\x13a\x18\0Wa\x02\x15\x95a\x1D\x86\x93a\x0B\x91\x92`@Q\x96\x87\x95` \x87\x01a\x16)V[a&\x9EV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\x1E\xAEW[\x85\x85\x12a\x1E\x8FW\x90a\x0B\x91a\x1D\xBD\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa\x1D\xD3\x81\x85a2\xD6V[\x92a\x1D\xDE\x81\x86a2\xD6V[\x88a\x1D\xE9\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a\x1D\xFD\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a\x1E\x17WPPPPPPPPPP\x90V[\x15a\x1ErW[P\x86\x97\x98P\x81\x92a\x1E1a\x12|\x8B\x89a\x0C\x93V[\x99a\x1E<\x8B\x88a2\xD6V[\x90\x84a\x1EH\x88\x84a\x15\x12V[\x13a\x1EfWPP\x89\x93[\x88a\x1E]\x89\x87a\r\x07V[\x92\x01\x94\x99a\x1E\x06V[\x8B\x98P\x90\x95P\x93a\x1ERV[`\x14\x10\x80a\x1E\x86W[\x15a\x12\xD7W\x88a\x1E\x1DV[P\x80\x83\x10a\x1E{V[\x93P\x91a\x1E\x9B\x90a%\0V[\x91a\x1E\xA8\x84\x83\x83\x86a#\xDBV[\x93a\x1D\x9CV[\x85\x85\x13a\x1E\xC2W\x90a\x0B\x91a\x1D\xBD\x92a\x11\xF5V[\x93P\x94a\x1E\xCE\x90a#'V[\x94a\x1E\xDB\x84\x83\x83\x89a#\xDBV[\x93a\x1E\xAEV[\x92\x91\x90a\x1E\xF7a\x1E\xF1\x82\x84a%*V[\x85a$\xD4V[\x93\x81\x03\x90\x81\x11a\x0C\x8EW\x92\x81\x03\x90\x81\x11a\x0C\x8EW\x90V[\x92\x91\x90a\x1F\x1Ea\x1E\xF1\x82\x84a%*V[\x93\x81\x01\x80\x91\x11a\x0C\x8EW\x92\x81\x01\x80\x91\x11a\x0C\x8EW\x90V[\x92\x93\x90\x91\x81Q\x93` \x83\x01Q\x91`@\x84\x01Q\x93``\x01Q\x94\x87a\x1FW\x86a)\xFAV[a\x1F`\x90a\x18\x0BV[\x96a\x1Fj\x90a\x14&V[\x98\x82a\x1Fw\x8B\x94\x83a\x19|V[a\x1F\x80\x90a*\xB8V[\x82a\x1F\x8B\x87\x8Da+nV[\x90a\x1F\x95\x91a\x19|V[a\x1F\x9F\x86\x85a+nV[a\x1F\xA8\x91a\x14\x85V[a\x1F\xB1\x91a+\xEBV[a\x1F\xBA\x90a,\tV[\x97a\x1F\xC5\x88\x80a+nV[\x90a\x1F\xCF\x91a+nV[a\x1F\xD8\x90a+\x9DV[a\x1F\xE1\x90a\x19\x98V[\x88\x8Aa\x1F\xEC\x8Aa+?V[\x90a\x1F\xF6\x91a+nV[\x90a \0\x91a+nV[a \t\x91a\x19|V[a \x12\x90a(IV[\x90a \x1D\x86\x8Ca+nV[\x90a '\x86a\x19bV[a 0\x91a+nV[a 9\x91a\x19|V[a C\x90\x87a+nV[a L\x91a+nV[\x93a W\x90\x8Aa+nV[\x90a a\x91a\x19|V[\x91a k\x91a+nV[a t\x91a\x14\x85V[a ~\x90\x87a+nV[a \x87\x91a+\xEBV[\x95a \x91\x90a\x19bV[a \x9A\x91a+nV[\x92a \xA4\x91a+nV[a \xAD\x90a+\xC4V[\x90a \xB7\x91a\x14\x85V[a \xC0\x90a-ZV[a \xC9\x91a+nV[\x90a \xD3\x90a*\xB8V[a \xDC\x91a+\xEBV[\x90a\x1B1\x90a\x19bV[\x92\x93`\0\x93\x85\x92\x91\x85\x85\x12\x15a\"\tW[\x85\x85\x12a!\xEAW\x90a\x0B\x91a!\x18\x92`@\x96`@Q\x95\x86\x94` \x86\x01a\x11\xAFV[\x81\x85\x92\x85\x96\x82\x81\x11a\x13\x08Wa!.\x81\x85a2\xF8V[\x92a!9\x81\x86a2\xF8V[\x88a!D\x82\x87a\x15\x12V[\x13a\x12\xE7WP\x90a!X\x91\x97\x96\x92\x97a\r\x07V[`\x01\x95\x91\x82\x91\x87\x80[a!rWPPPPPPPPPP\x90V[\x15a!\xCDW[P\x86\x97\x98P\x81\x92a!\x8Ca\x12|\x8B\x89a\x0C\x93V[\x99a!\x97\x8B\x88a2\xF8V[\x90\x84a!\xA3\x88\x84a\x15\x12V[\x13a!\xC1WPP\x89\x93[\x88a!\xB8\x89\x87a\r\x07V[\x92\x01\x94\x99a!aV[\x8B\x98P\x90\x95P\x93a!\xADV[`\x14\x10\x80a!\xE1W[\x15a\x12\xD7W\x88a!xV[P\x80\x83\x10a!\xD6V[\x93P\x94a!\xF6\x90a#'V[\x94a\"\x03\x84\x87\x84\x84a#\xDBV[\x93a \xF7V[\x85\x85\x13a\"\x1DW\x90a\x0B\x91a!\x18\x92a\x11\xF5V[\x93P\x91a\")\x90a%\0V[\x91a\"6\x84\x84\x84\x84a#\xDBV[\x93a\"\tV[a\"\xACa\"\xA7a\x02\x15\x93a\"\xA1a\"\x9C\x82Qa\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x86a\"\x81`@` \x8B\x01Q\x9A\x01Q\x96a\"{\x88\x8Ca'uV[\x9Da%*V[a.\xF1V[\x97a.\xF1V[a\x14\xBBV[\x05a(IV[a#FV[a#\x89V[\x90a\x14\x85V[a\x14\x9EV[a\x155V[a\"\xE0a\"\xA7a\"\xDBg\x13\xA0K\xBD\xFD\xC9\xBE\x88a\"\xD5g\x1B\xC1mgN\xC8\0\0\x95a\x14\x9EV[\x05a\x19\x98V[a-ZV[\x05\x90V[`\0\x81\x12a\"\xEFW\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x90\xFD[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWa\x03\xE8\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x04\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[\x90\x92\x82\x82\x10\x15a$\x8FWa\x02\x15\x93a$X\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a$\x03\x83\x83a#gV[\x10a$|WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a$+a$%\x83\x85a#FV[\x85a#gV[\x10a$]WP`\x01`\x01`\xFF\x1B\x03\x92a$R\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a'uV[\x92a\x19|V[a\x19|V[a$R\x92a\x1Bsa$q\x92a$v\x94a#FV[a'\xBDV[\x91a$BV[a$\x89\x91a$q\x91a#gV[\x94a$\x15V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x02kW`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x02kW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[a\x01\0\x81\x83\x03\x12a\x02kW\x80Q\x92a\x02\x15` \x83\x01Q\x93```@\x85\x01Q\x94\x01a\x0E\x86V[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a&}Wa%\x9B\x81a3\x18V[a%\xA5\x85\x83a4;V[`\0a%\xB1\x82\x84a\x15\x12V[\x13a&^WPa%\xC2\x85\x96\x95a\x0C\xF7V[`\x01\x94`\0\x91\x86\x80[a%\xDCW[PPPPPPPP\x90PV[\x15a&9W[P\x85\x96\x97\x98P\x80\x91a%\xF7a\x12|\x8B\x88a\x0C\x93V[\x99a&\x02\x8B\x87a4;V[\x90\x83a&\x0E\x87\x84a\x15\x12V[\x13a&-WPP\x89\x92[\x87a&#\x88\x86a\r\x07V[\x92\x01\x93\x99\x98a%\xCBV[\x8B\x97P\x90\x94P\x92a&\x18V[\x86\x10\x80a&SW[\x15a&LW\x88a%\xE2V[\x80\x80a%\xD0V[Pa\x01\0\x82\x10a&AV[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a&}Wa&\xBA\x81a4]V[a&\xC4\x85\x83a5\xA0V[`\0a&\xD0\x82\x84a\x15\x12V[\x13a&^WPa&\xE1\x85\x96\x95a\x0C\xF7V[`\x01\x94`\0\x91\x86\x80[a&\xFAWPPPPPPPP\x90PV[\x15a'WW[P\x85\x96\x97\x98P\x80\x91a'\x15a\x12|\x8B\x88a\x0C\x93V[\x99a' \x8B\x87a5\xA0V[\x90\x83a',\x87\x84a\x15\x12V[\x13a'KWPP\x89\x92[\x87a'A\x88\x86a\r\x07V[\x92\x01\x93\x99\x98a&\xEAV[\x8B\x97P\x90\x94P\x92a'6V[\x86\x10\x80a'jW[\x15a&LW\x88a'\0V[Pa\x01\0\x82\x10a'_V[\x90a'\x7F\x90a)\xFAV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\x0C\x8EWa\x02\x15\x91a#FV[a\x02\x15\x91a\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x9C\x95a.\xF1V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a(CWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x12\x15a(1W\x80\x15a(\x1FW\x80`\x01\x1B\x90\x81\x05`\x02\x03a\x0C\x8EWa'\xFB\x90a,\tV[\x90g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x91\x80\x83\x02\x92\x83\x05\x14`\x01\x16\x15a\x02kWa\x02\x15\x91\x05a\x19\x98V[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[P`\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a(CWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a)\x99We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[\x91\x90a\x01 \x83\x82\x03\x12a\x02kW\x82Q\x92` \x81\x01Q\x92a\x02\x15`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0E\x86V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a*\xA1W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a*\x94W[e\x01\0\0\0\0\0\x81\x10\x15a*\x87W[c\x01\0\0\0\x81\x10\x15a*zW[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a*>V[` \x1C\x91`\x10\x1B\x91a*1V[`@\x1C\x91` \x1B\x91a*\"V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca*\nV[g\x1B\xC1mgN\xC8\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x06\xF0[Y\xD3\xB2\0\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x13\xA0K\xBD\xE7\x8C\xC4\0\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\x13\xA0K\xBD\xE7\x8C\xC4\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x02kW\x05\x90V[`\0\x81\x12\x80\x15a-IW[a-7W\x80\x15a(1Wg\x1B\xC1mgN\xC8\0\0\x81\x14a(\x1FWg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12\x90\x81\x15a-(W\x90[a,J\x82a0\xBFV[\x80\x15a(1Wa,\xB3a,wa,ra,ma,ha,\xB8\x95a.\xF1V[a1\x80V[a)\xFAV[a\x14\xD8V[a\x16\x03a,\x8Ba,\x86\x83a0\xEAV[a\x18$V[a,\xADa,\xA8a,\xA2a,\x9D\x86a1\x15V[a\x18<V[\x85a1\xF7V[a\x18TV[\x90a1^V[a1\xA8V[\x91`\0\x90[`\x02\x82\x10a,\xD8WPP\x15a,\xCFW\x90V[a\x02\x15\x90a\x19\x98V[\x90\x92a- \x81a-\x1Aa,\xF0\x85a\x16\x03`\x01\x96a-ZV[a,\xADa-\x10a-\x0Ba\x16\x08a-\x06\x87\x80a1\xF7V[a\x19\x98V[a1\xD0V[a\"\xA1\x83\x86a1\xF7V[\x90a\x19|V[\x93\x01\x90a,\xBDV[a-1\x90a\x14IV[\x90a,AV[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x81\x13a,\x14V[\x80\x15a.mWgV\x98\xEE\xF0fp\0\0\x81\x12\x15a(CWgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a.`W`\0a.Pa-\x8F\x83a0\x92V[a.\x18a\x16\x08a-\xA9a-\xA4a,\xA8\x85a#\xB0V[a1?V[\x92a$Xa.Ka.Fa.?a.9a.4a..a.)a.#a.\x1E\x8Da.\x18a.\x13a.\ra.\x08a,\xA2a.\x03a-\xFDa-\xF8a-\xF2a-\xED\x8Aa2\x18V[a\x18lV[\x89a1\xF7V[a\x18\x86V[\x87a1\xF7V[a\x18\x9EV[a\x18\xB8V[\x83a1\xF7V[a\x18\xD0V[\x90a1\xF7V[a\x18\xEAV[\x8Ca1\xF7V[a\x19\x02V[\x8Aa1\xF7V[a\x19\x1AV[\x88a1\xF7V[\x93\x80a1\xF7V[a\x14\xF1V[a\x14lV[\x91\x12\x15a\x02\x15Wa\x02\x15\x90a\x14IV[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[a\"\xACa\"\xA7a\x02\x15\x93a-\x1Aa\"\x9C\x82Qa\"\x97g\r\xE0\xB6\xB3\xA7d\0\0a\"\x91a\"\x8Ca\"\x86a\"\x81`@` \x8B\x01Q\x9A\x01Q\x96a\"{\x88\x8Ca'uV[\x15a.\xC0WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a/\x1D`\0\x82\x13a.\xB9V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a/9\x82a2CV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[`\x01`\xFF\x1B\x81\x14a0\xADW`\0\x81\x12\x15a\x02\x15W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x02kWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x02kW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x02kWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a2N\x81\x15\x15a.\xB9V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x90a2\xCCa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x91\x92\x90Pa#\xDBV[\x90a2\xEDa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x90P\x91\x90\x91a#\xDBV[\x90a3\x0Fa\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a%ZV[\x93\x92\x90Pa#\xDBV[\x80Q\x81\x01` \x01\x90` \x01\x90a3-\x91a)\xCDV[\x80\x91\x92PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a3M\x84a)\xFAV[a3V\x90a\x18\x0BV[\x94a3`\x90a\x14&V[\x91a3j\x81a\x192V[a3s\x90a*\xB8V[a3|\x83a\x19DV[a3\x85\x85a+\x16V[a3\x8E\x91a\x14\x85V[a3\x97\x91a+\xEBV[a3\xA0\x90a,\tV[\x94a3\xAB\x85\x80a+nV[\x90a3\xB5\x91a+nV[a3\xBE\x90a+\x9DV[a3\xC7\x90a\x19\x98V[\x85\x87a3\xD2\x87a+?V[\x90a3\xDC\x91a+nV[\x90a3\xE6\x91a+nV[a3\xEF\x91a\x19|V[a3\xF8\x90a(IV[\x90a4\x02\x84a\x19bV[a4\x0B\x91a+nV[a4\x15\x90\x83a\x19|V[a4\x1F\x90\x88a+nV[a4(\x91a+nV[\x90a42\x90a\x19DV[a\x1A\xBF\x83a+\x16V[\x90a4Ra\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a)\xCDV[\x94\x93\x90\x92\x91Pa\x19\xA9V[\x80Q\x81\x01` \x01\x90` \x01\x90a4r\x91a)\xCDV[\x80\x91\x92\x94\x93PQ\x92` \x82\x01Q\x90`@\x83\x01Q\x92``\x01Q\x93a4\x94\x84a)\xFAV[a4\x9D\x90a\x18\x0BV[\x94a4\xA7\x90a\x14&V[\x96a4\xB1\x81a\x192V[a4\xBA\x90a*\xB8V[a4\xC4\x83\x89a+nV[a4\xCD\x90a\x19DV[a4\xD6\x8Aa+\x16V[a4\xDF\x91a\x14\x85V[a4\xE8\x91a+\xEBV[a4\xF1\x90a,\tV[\x94a4\xFC\x85\x80a+nV[\x90a5\x06\x91a+nV[a5\x0F\x90a+\x9DV[a5\x18\x90a\x19\x98V[\x85\x87a5#\x87a+?V[\x90a5-\x91a+nV[\x90a57\x91a+nV[a5@\x91a\x19|V[a5I\x90a(IV[\x90a5T\x83\x89a+nV[\x90a5^\x8Aa\x19bV[a5g\x91a+nV[a5p\x91a\x19|V[a5z\x90\x84a+nV[a5\x83\x91a+nV[\x90a5\x8E\x90\x87a+nV[a5\x97\x90a\x19DV[a k\x88a+\x16V[\x90a5\xB7a\x02\x15\x92` \x80\x82Q\x83\x01\x01\x91\x01a)\xCDV[\x94\x93\x90\x92Pa\x1F5V\xFE\xA2dipfsX\"\x12 \x9F\x9B9@s<\xB4i(\xE4}\x8B\x14\xB0\xB3?Y\x1D\x84\x8D\x1C\xB1\x18\xADTc\xCBw]\xAE.\xD4dsolcC\0\x08\x16\x003";
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

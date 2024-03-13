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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\x005\xC38\x03\x80b\x005\xC3\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0ZV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\0\x8CV[`\0` \x82\x84\x03\x12\x15b\0\0mW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x85W`\0\x80\xFD[\x93\x92PPPV[a5'\x80b\0\0\x9C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c}\xDA\x1A#\x11a\0\xF9W\x80c\xCB\x1FU2\x11a\0\x97W\x80c\xE9G\x16\xD5\x11a\0qW\x80c\xE9G\x16\xD5\x14a\x04\x0FW\x80c\xEE>\x8C\xFB\x14a\x04\"W\x80c\xF3\r7\xF2\x14a\x045W\x80c\xF9\xC2\x82\x11\x14a\x04HW`\0\x80\xFD[\x80c\xCB\x1FU2\x14a\x03\xD6W\x80c\xCE\x15;\xF4\x14a\x03\xE9W\x80c\xDE^\xE1\xC3\x14a\x03\xFCW`\0\x80\xFD[\x80c\x90.\xCA\xA2\x11a\0\xD3W\x80c\x90.\xCA\xA2\x14a\x03rW\x80c\xA8\xC6.v\x14a\x03\x85W\x80c\xAFNC\x7F\x14a\x03\xB0W\x80c\xB0\x9D\x04\xE5\x14a\x03\xC3W`\0\x80\xFD[\x80c}\xDA\x1A#\x14a\x03\x17W\x80c\x7F\x17@\x9C\x14a\x03?W\x80c\x81\xB5\xFA\xC2\x14a\x03RW`\0\x80\xFD[\x80c;&\x8D]\x11a\x01fW\x80cO\xD6|X\x11a\x01@W\x80cO\xD6|X\x14a\x02\xBBW\x80c^\xB4\x08\xFC\x14a\x02\xCEW\x80cb7V\x9F\x14a\x02\xE1W\x80cme\"\x99\x14a\x03\x0FW`\0\x80\xFD[\x80c;&\x8D]\x14a\x02\x82W\x80c;M\x100\x14a\x02\x95W\x80cN\x81\x7F\xD9\x14a\x02\xA8W`\0\x80\xFD[\x80c\x1E\x97\x8C\xB0\x11a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x02&W\x80c0m\xB4k\x14a\x029W\x80c3\"f\xF3\x14a\x02LW\x80c9(\xFF\x97\x14a\x02_W`\0\x80\xFD[\x80c\x04 X\n\x14a\x01\xC9W\x80c\x12\x06I\xC5\x14a\x01\xF2W\x80c\x13N\xAD\x12\x14a\x02\x13W[`\0\x80\xFD[a\x01\xDCa\x01\xD76`\x04a-\x9DV[a\x04PV[`@Qa\x01\xE9\x91\x90a.\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x02\x05a\x02\x006`\x04a.\"V[a\x04eV[`@Q\x90\x81R` \x01a\x01\xE9V[a\x01\xDCa\x02!6`\x04a.\xABV[a\x05<V[a\x02\x05a\x0246`\x04a/+V[a\x05SV[a\x02\x05a\x02G6`\x04a/+V[a\x05hV[a\x02\x05a\x02Z6`\x04a/+V[a\x05\x9FV[a\x02ra\x02m6`\x04a/eV[a\x05\xCBV[`@Qa\x01\xE9\x94\x93\x92\x91\x90a/\x9DV[a\x01\xDCa\x02\x906`\x04a-\x9DV[a\t\xA0V[a\x02\x05a\x02\xA36`\x04a/\xC4V[a\t\xACV[a\x02\x05a\x02\xB66`\x04a/+V[a\t\xD5V[a\x02\x05a\x02\xC96`\x04a/+V[a\t\xEAV[a\x02\x05a\x02\xDC6`\x04a.\"V[a\n\x16V[a\x02\xF4a\x02\xEF6`\x04a-\x9DV[a\n\xE1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE9V[a\x02\x05`\0\x81V[a\x03*a\x03%6`\x04a-\x9DV[a\x0B;V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE9V[a\x02\xF4a\x03M6`\x04a-\x9DV[a\x0B\xABV[a\x03ea\x03`6`\x04a/\xC4V[a\x0C\x04V[`@Qa\x01\xE9\x91\x90a/\xDDV[a\x02\x05a\x03\x806`\x04a/+V[a\x0C\xC3V[`\0Ta\x03\x98\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE9V[a\x02\x05a\x03\xBE6`\x04a.\"V[a\x0C\xEFV[a\x01\xDCa\x03\xD16`\x04a/\xC4V[a\r\xA4V[a\x01\xDCa\x03\xE46`\x04a0\x1BV[a\r\xAFV[a\x02\xF4a\x03\xF76`\x04a/\xC4V[a\r\xBAV[a\x03*a\x04\n6`\x04a-\x9DV[a\x0E\xB0V[a\x01\xDCa\x04\x1D6`\x04a-\x9DV[a\x0F\x04V[a\x02\xF4a\x0406`\x04a-\x9DV[a\x0F\x10V[a\x02\xF4a\x04C6`\x04a-\x9DV[a\x0F6V[a\x02\x05`x\x81V[``a\x04\\\x83\x83a\x0F\\V[\x90P[\x92\x91PPV[`\0\x80a\x04{\x84\x84a\x04v\x89a\x0C\x04V[a\x0F\x8BV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x04\xD8\x90\x8B\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x19\x91\x90a0QV[\x90Pa\x050\x87\x87\x83\x86a\x05+\x8Da\x0C\x04V[a\x0F\xCDV[\x98\x97PPPPPPPPV[``a\x05I\x84\x84\x84a\x10~V[\x90P[\x93\x92PPPV[`\0a\x05I\x83\x83a\x05c\x87a\x0C\x04V[a\x10\xEFV[`\0\x80a\x05t\x85a\x0C\x04V[\x90P`\0\x80a\x05\x82\x87a\r\xBAV[\x92PP\x91Pa\x05\x94\x86\x83\x83\x88\x87a\x11\xF8V[\x97\x96PPPPPPPV[`\0\x80a\x05\xAB\x85a\x0C\x04V[\x90P`\0\x80a\x05\xB9\x87a\r\xBAV[\x92PP\x91Pa\x05\x94\x86\x83\x83\x88\x87a\x12dV[`\0\x80`\0``a\x05\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06\x1A`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06#\x89a\r\xBAV[`@\x85\x01R` \x84\x01R\x82R`\0a\x06:\x8Aa\x0C\x04V[\x90P`\0\x80a\x06W\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0C\xEFV[\x90P\x8A\x15a\x07dW`\0a\x06x\x84``\x01Q\x8Ca\x12\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91Pa\x06\x88\x90\x8C\x90a0\x80V[\x85Ra\x06\x94\x81\x83a0\x80V[\x85`@\x01\x81\x81RPP`\0a\x06\xB2\x8E\x87`\0\x01Q\x88`@\x01Qa\x05SV[\x90Pa\x06\xC8\x8E\x87`\0\x01Q\x88`@\x01Q\x84a\x04eV[` \x87\x01\x81\x81R`\x01\x91a\x06\xDD\x90\x83\x90a0\x80V[\x90RP` \x80\x88\x01Q\x90\x87\x01Q\x10a\x07GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x85` \x01Q\x87` \x01Qa\x07[\x91\x90a0\x93V[\x93PPPa\x08]V[\x82Q``\x84\x01Q`\0\x91a\x07\x83\x91a\x07}\x90\x8E\x90a\x12\xE0V[\x90a\x12\xF5V[\x90P\x8A\x86` \x01Qa\x07\x95\x91\x90a0\x80V[` \x86\x01Ra\x07\xA4\x81\x83a0\x80V[\x85`@\x01\x81\x81RPP`\0a\x07\xC2\x8E\x87` \x01Q\x88`@\x01Qa\t\xD5V[\x90Pa\x07\xD8\x8E\x87` \x01Q\x88`@\x01Q\x84a\n\x16V[\x80\x87R`\x01\x90\x87\x90a\x07\xEB\x90\x83\x90a0\x80V[\x90RP\x86Q\x86Q\x10a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07>V[\x85Q\x87Qa\x08X\x91\x90a0\x93V[\x93PPP[P`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x84Q``\x82\x01R` \x80\x86\x01Q`\x80\x83\x01R`@\x80\x87\x01Q`\xA0\x84\x01R\x85Q\x86\x83\x01Q\x87\x83\x01Q\x92Q`\0\x94a\x08\xDF\x94\x91\x01\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc2\xE4\xFF\xE1`\xE1\x1B\x84R\x91\x93P\x8F\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xC9\xFF\xC2\x90a\t)\x900\x90\x86\x90\x89\x90\x89\x90`\x04\x01a0\xA6V[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tj\x91\x90a1\x1DV[PPPPP\x90P\x80\x85a\t\x86\x89`\0\x01Q\x8A`@\x01Q\x8Aa\x10\xEFV[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x93P\x93P\x93P\x93V[``a\x04\\\x83\x83a\x13\nV[`\0\x80`\0a\t\xBA\x84a\r\xBAV[\x92PP\x91Pa\t\xCD\x82\x82a\x05c\x87a\x0C\x04V[\x94\x93PPPPV[`\0a\x05I\x83\x83a\t\xE5\x87a\x0C\x04V[a\x13\"V[`\0\x80a\t\xF6\x85a\x0C\x04V[\x90P`\0\x80a\n\x04\x87a\r\xBAV[\x92P\x92PPa\x05\x94\x86\x83\x83\x88\x87a\x13\xF0V[`\0\x80a\n,\x84\x84a\n'\x89a\x0C\x04V[a\x14SV[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n\x89\x90\x8B\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCA\x91\x90a0QV[\x90Pa\x050\x87\x87\x83\x86a\n\xDC\x8Da\x0C\x04V[a\x14\x98V[`\0\x80`\0\x80`\0a\n\xF2\x87a\r\xBAV[\x92PP\x91P`\0\x80a\x0B\x07`\0\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0B\x18\x8A\x84\x84a\x05SV[\x90P`\0a\x0B(\x8B\x85\x85\x85a\x04eV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0BL\x87a\r\xBAV[\x92P\x92P\x92P`\0\x80a\x0Bb`\x01\x89\x87\x86a\x15<V[\x91P\x91P`\0a\x0Bs\x8A\x84\x84a\x05SV[\x90P`\0a\x0B\x83\x8B\x85\x85\x85a\x04eV[\x90Pa\x0B\x8F\x86\x82a0\x93V[\x98Pa\x0B\x9B\x85\x84a0\x93V[\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\x0B\xBC\x87a\r\xBAV[\x92P\x92PP`\0\x80a\x0B\xD1`\x01\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0B\xE2\x8A\x84\x84a\t\xD5V[\x90P`\0a\x0B\xF2\x8B\x85\x85\x85a\n\x16V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x0C?`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xB0\x91\x90\x81\x01\x90a1pV[\x80` \x01\x90Q\x81\x01\x90a\x04_\x91\x90a2iV[`\0\x80a\x0C\xCF\x85a\x0C\x04V[\x90P`\0\x80a\x0C\xDD\x87a\r\xBAV[\x92P\x92PPa\x05\x94\x86\x83\x83\x88\x87a\x15\xA5V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\rL\x90\x8A\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90a0QV[\x90Pa\x05\x94\x86\x86\x83\x87a\r\x9F\x8Ca\x0C\x04V[a\x16\x01V[``a\x04_\x82a\x16\xA5V[``a\x04_\x82a\x16\xD1V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E5\x91\x90a2\x85V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Eb\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA3\x91\x90a2\xA2V[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0a\x0E\xC1\x87a\r\xBAV[\x92P\x92P\x92P`\0\x80a\x0E\xD7`\x01\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0E\xE8\x8A\x84\x84a\t\xD5V[\x90P`\0a\x0E\xF8\x8B\x85\x85\x85a\n\x16V[\x90Pa\x0B\x8F\x87\x82a0\x93V[``a\x04\\\x83\x83a\x16\xE7V[`\0\x80`\0\x80`\0a\x0F!\x87a\r\xBAV[\x92PP\x91P`\0\x80a\x0B\x07`\x01\x89\x86\x86a\x15<V[`\0\x80`\0\x80`\0a\x0FG\x87a\r\xBAV[\x92P\x92PP`\0\x80a\x0B\xD1`\0\x89\x86\x86a\x15<V[```\x02\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0F\x98\x84\x84a\x16\xFFV[\x90P`\0a\x0F\xA5\x82a\x17`V[\x90P`\0a\x0F\xB2\x82a\x17\xC9V[\x85Q\x90\x91Pa\x05\x94\x90\x82\x90a\x0F\xC7\x90\x8Aa\x12\xE0V[\x90a\x12\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x10\rW[`\0\x81\x12\x15a\x10\x08Wa\x0F\xF3\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x10\x01\x89\x84\x8A\x88a\x18@V[\x90Pa\x0F\xDBV[a\x10:V[`\0\x81\x13\x15a\x10:Wa\x10%\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x103\x89\x83\x8A\x88a\x18@V[\x90Pa\x10\rV[a\x10q\x89\x89\x83\x88`@Q` \x01a\x10T\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x19`a\x19\x97V[\x99\x98PPPPPPPPPV[```\0a\x10\x8D\x85\x85\x85a\x1A\xA8V[\x90P`\0a\x10\x9C\x82\x86\x86a\x0F\x8BV[\x90P`\0a\x10\xAC\x87\x83\x85\x88a\x18@V[\x90Pa\x10\xBB\x87\x83\x83\x86\x89a\x16\x01V[\x92P\x86\x82\x84\x87`@Q` \x01a\x10\xD4\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x11\x04\x83` \x01Q\x84`@\x01Qa\x1A\xEDV[\x90P`\0a\x11\x1A\x84` \x01Q\x85`@\x01Qa\x1B\x13V[\x90P`\0a\x115\x85`@\x01Q\x83a\x1BB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11C\x88\x88a\x1BWV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11aW`\0\x94PPPPPa\x05LV[`\0\x81\x13a\x11wW`\0\x19\x94PPPPPa\x05LV[`\0a\x11\x93a\x11\x8E\x83g\r\xE0\xB6\xB3\xA7d\0\0a3eV[a\x1BlV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xAB\x88\x85a3\x8CV[a\x11\xB5\x91\x90a3\xD2V[a\x11\xBF\x91\x90a3eV[\x90P`\0a\x11\xCC\x82a\x1C\tV[\x90P`\0a\x11\xD9\x82a\x17\xC9V[\x8AQ\x90\x91Pa\x11\xE8\x90\x82a\x12\xE0V[\x9C\x9BPPPPPPPPPPPPV[`\0\x82a\x03\xE8\x82a\x12\x0C\x89\x89\x89\x85\x89a\x12dV[\x90P`\0\x81\x12\x15a\x12#W`\0\x93PPPPa\x12[V[a\x10q\x89\x89\x89\x88`@Q` \x01a\x12=\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1D\xB2a\x19\x97V[\x95\x94PPPPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x12\x7F\x91\x90a3eV[\x90P`\0a\x12\x91\x88\x88\x88\x85\x89\x89a\x1D\xE3V[\x90P`\0a\x12\x9E\x82a\x1F>V[\x90P`\0a\x12\xAB\x83a sV[\x90P\x80\x82\x84a\x01\0\x01Qa\x12\xBE\x90a4\0V[a\x12\xC8\x91\x90a4\x1CV[a\x12\xD2\x91\x90a4\x1CV[\x9A\x99PPPPPPPPPPV[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x18\x12V[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x18\x12V[```\x04\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[`\0\x80a\x137\x83` \x01Q\x84`@\x01Qa\x1A\xEDV[\x90P`\0a\x13M\x84` \x01Q\x85`@\x01Qa\x1B\x13V[\x90P`\0a\x13h\x85`@\x01Q\x83a\x1BB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x13\x85\x90a\x13~\x90\x89a\x1BBV[\x89\x90a\x1BWV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13\xA3W`\0\x94PPPPPa\x05LV[`\0\x81\x13a\x13\xB9W`\0\x19\x94PPPPPa\x05LV[`\0a\x13\xC4\x82a\x1BlV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xDC\x88\x85a3\x8CV[a\x13\xE6\x91\x90a3\xD2V[a\x11\xBF\x91\x90a4\x1CV[`\0\x82a\x03\xE8\x82a\x14\x04\x89\x89\x89\x85\x89a\x15\xA5V[\x90P`\0\x81\x12\x15a\x14\x1BW`\0\x93PPPPa\x12[V[a\x10q\x89\x89\x89\x88`@Q` \x01a\x145\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a \xFFa\x19\x97V[`\0\x80a\x14`\x84\x84a!0V[\x90P`\0a\x14m\x82a\x17`V[\x90P`\0a\x14z\x82a\x17\xC9V[\x90Pa\x05\x94a\x14\x91\x82g\r\xE0\xB6\xB3\xA7d\0\0a0\x93V[\x88\x90a\x12\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x14\xD8W[`\0\x81\x12\x15a\x14\xD3Wa\x14\xBE\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x14\xCC\x83\x8A\x8A\x88a\x18@V[\x90Pa\x14\xA6V[a\x15\x05V[`\0\x81\x13\x15a\x15\x05Wa\x14\xF0\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x14\xFE\x82\x8A\x8A\x88a\x18@V[\x90Pa\x14\xD8V[a\x10q\x89\x89\x83\x88`@Q` \x01a\x15\x1F\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a!ua\x19\x97V[`\0\x80\x80a\x15J\x84\x86a\x12\xF5V[\x90P`\0a\x15X\x87\x83a\x12\xE0V[\x90P\x87a\x15nWa\x15i\x87\x87a0\x93V[a\x15xV[a\x15x\x87\x87a0\x80V[\x93P\x87a\x15\x8EWa\x15\x89\x81\x86a0\x93V[a\x15\x98V[a\x15\x98\x81\x86a0\x80V[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xC0\x91\x90a3eV[\x90P`\0a\x15\xD2\x88\x88\x88\x85\x89\x89a!\xA2V[\x90P`\0a\x15\xDF\x82a\"\xFAV[\x90P`\0a\x15\xEC\x83a$(V[\x90P\x80\x82a\x12\xBEg\r\xE0\xB6\xB3\xA7d\0\0a4\0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x16AW[`\0\x81\x12\x15a\x16<Wa\x16'\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x165\x89\x89\x84\x88a\x18@V[\x90Pa\x16\x0FV[a\x16nV[`\0\x81\x13\x15a\x16nWa\x16Y\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x16g\x89\x89\x85\x88a\x18@V[\x90Pa\x16AV[a\x10q\x89\x89\x83\x88`@Q` \x01a\x16\x88\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a$\xD4a\x19\x97V[```\x01\x82`@Q` \x01a\x16\xBB\x92\x91\x90a4DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x16\xBB\x92\x91\x90a4_V[```\x03\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x17\x1B\x83\x83a\x1A\xEDV[\x90P`\0a\x17)\x88\x86a%\x01V[\x90P`\0a\x177\x85\x85a\x1B\x13V[\x90P\x82a\x17D\x82\x84a3eV[a\x17V\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\x8CV[a\x10q\x91\x90a3\xD2V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x17~g\r\xE0\xB6\xB3\xA7d\0\0\x85a3\x8CV[a\x17\x88\x91\x90a3\xD2V[\x90P`\0a\x17\x95\x82a4\0V[\x90P`\0a\x17\xA2\x82a%\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x17\xBFg\r\xE0\xB6\xB3\xA7d\0\0\x83a3\x8CV[a\x12[\x91\x90a3\xD2V[`\0\x80\x82\x12\x15a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x07>V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x18*W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x18\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07>V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA7\x88\x87a\x1BWV[\x10a\x18\xBBW`\x01`\x01`\xFF\x1B\x03\x91Pa\x18\xCBV[a\x18\xC8a\x11\x8E\x88\x87a\x1BWV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xEB\x87a\x18\xE6\x87`\0\x01Q\x89a\x1BBV[a\x1BWV[\x10a\x18\xFEWP`\x01`\x01`\xFF\x1B\x03a\x19\x16V[a\x19\x13a\x11\x8E\x87a\x18\xE6\x87`\0\x01Q\x89a\x1BBV[\x90P[`\0a\x19*\x85` \x01Q\x86`@\x01Qa\x1A\xEDV[\x90P\x80a\x197\x83\x85a4\x1CV[a\x050\x91\x90a4\x1CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x19YW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x19z\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x83\x86\x84\x84a\x18@V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x19\xC4W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07>V[`\0a\x19\xD4\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xE6\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xF4\x82\x84a3\x8CV[\x13\x15a\x1A\x1DW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07>V[`\0a\x1A)\x89\x89a0\x93V[\x90P`\0[`\x02a\x1A:\x8A\x8Ca0\x80V[a\x1AD\x91\x90a4\xC6V[\x94P`\0a\x1AV\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1Ad\x86\x83a3\x8CV[\x13a\x1AqW\x85\x99Pa\x1AxV[\x85\x9AP\x80\x94P[a\x1A\x82\x8B\x8Ba0\x93V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1A\x96WP\x86\x81\x10[a\x1A.WPPPP\x96\x95PPPPPPV[`\0\x80a\x1A\xB5\x84\x84a!0V[\x90P`\0a\x1A\xC2\x82a\x17`V[\x90P`\0a\x1A\xCF\x82a\x17\xC9V[\x90Pa\x05\x94a\x1A\xE6\x82g\r\xE0\xB6\xB3\xA7d\0\0a0\x93V[\x88\x90a\x12\xF5V[`\0\x80a\x1A\xF9\x83a&\xFEV[a\x1B\x07\x90c;\x9A\xCA\0a4\xDAV[\x90Pa\t\xCD\x84\x82a\x1BBV[`\0\x80a\x1B2\x83a\x1B,\x86g\x1B\xC1mgN\xC8\0\0a'\xA2V[\x90a\x1BBV[\x90Pa\t\xCDg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19AV[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x19AV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1B\x85WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1B\xADW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1B\xCEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\xDB\x83`\x02a3\x8CV[\x90P`\0a\x1B\xE8\x82a'\xCEV[\x90P`\0a\x1B\xFEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a*GV[\x90Pa\x12[\x81a4\0V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1C$WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1CkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07>V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1D\xCE\x91\x90a4\x85V[\x93P\x93P\x93P\x93Pa\x05\x94\x84\x84\x84\x89\x85a\x12dV[a\x1E@`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x1E^a\x1EO\x88\x86a4\x1CV[g\x1B\xC1mgN\xC8\0\0\x90a*\\V[\x90P`\0a\x1El\x85\x87a*\\V[a\x1Ev\x86\x89a4\x1CV[a\x1E\x80\x91\x90a3eV[\x90P`\0a\x1E\x96a\x1E\x91\x84\x84a*\x8FV[a'\xCEV[\x90P`\0a\x1E\xABg\x1B\xC1mgN\xC8\0\0a&\xFEV[a\x1E\xB9\x90c;\x9A\xCA\0a4\xDAV[\x90P`\0a\x1E\xCA\x87`@\x01Qa&\xFEV[a\x1E\xD8\x90c;\x9A\xCA\0a4\xDAV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a\x1F|g\x1B\xC1mgN\xC8\0\0a\x1Fv\x85``\x01Qa\x1Fp\x87`@\x01Q\x88`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a*\\V[\x90a*\x8FV[a\x1F\x85\x90a4\0V[\x90P`\0a\x1F\xB6\x84`\0\x01Qa\x1Fp\x86a\x01 \x01Qa\x1Fp\x88`@\x01Q\x89a\x01@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1F\xCCa\x1F\xC7\x83\x85a4\x1CV[a\x1C\tV[\x90P`\0a \x1Ca \x02\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xED\x90a4\0V[a\x1F\xF7\x91\x90a4\x1CV[`\xA0\x89\x01Q\x90a*\\V[\x87`\xC0\x01Qa \x11\x91\x90a4\x1CV[` \x88\x01Q\x90a*\\V[\x90P`\0a *\x83\x83a*\\V[\x90P`\0a I\x88`\x80\x01Q\x89`\xE0\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa ]\x91\x90a4\x1CV[a g\x91\x90a3eV[\x90Pa\x050\x82\x82a*\x8FV[`\0\x80a \xB2\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a \x90\x90a4\0V[a \x9A\x91\x90a4\x1CV[` \x85\x01Qa\x1Fp\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a*\\V[\x90P`\0a \xDB\x84a\x01@\x01Qa\x1Fv\x86a\x01 \x01Q\x87`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\t\xCDa \xF8\x85`\0\x01Q\x83a \xF3\x91\x90a3eV[a%\x15V[\x83\x90a*\\V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a!\x1B\x91\x90a4\x85V[\x93P\x93P\x93P\x93Pa\x05\x94\x84\x84\x84\x89\x85a\x15\xA5V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a!L\x83\x83a\x1A\xEDV[\x90P`\0a!Z\x88\x86a%\x01V[\x90P`\0a!h\x85\x85a\x1B\x13V[\x90P\x82a\x17D\x82\x84a4\x1CV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a!\x8F\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x85\x84\x84\x84a\x18@V[a!\xFF`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\"\x0Ea\x1EO\x88\x86a4\x1CV[\x90P`\0a\"\x1C\x85\x87a*\\V[\x84Q\x86\x90a\"*\x90\x8Aa*\\V[a\"4\x91\x90a4\x1CV[a\">\x91\x90a3eV[\x90P`\0a\"Oa\x1E\x91\x84\x84a*\x8FV[\x90P`\0a\"dg\x1B\xC1mgN\xC8\0\0a&\xFEV[a\"r\x90c;\x9A\xCA\0a4\xDAV[\x90P`\0a\"\x83\x87`@\x01Qa&\xFEV[a\"\x91\x90c;\x9A\xCA\0a4\xDAV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a#,g\x1B\xC1mgN\xC8\0\0a\x1Fv\x85``\x01Qa\x1Fp\x87`@\x01Q\x88`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a#5\x90a4\0V[\x90P`\0a#f\x84`\0\x01Qa\x1Fp\x86a\x01@\x01Qa\x1Fp\x88`@\x01Q\x89a\x01 \x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#wa\x1F\xC7\x83\x85a4\x1CV[\x90P`\0a#\xC1a#\x98\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xED\x90a4\0V[`\xC0\x88\x01Q` \x89\x01Qa#\xAB\x91a*\\V[a#\xB5\x91\x90a4\x1CV[a\x01\0\x88\x01Q\x90a*\\V[\x90P`\0a#\xCF\x83\x83a*\\V[\x90P`\0a ga#\xF1\x89`\x80\x01Q\x8A`\xE0\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa$\t\x91a*\\V[a$\x13\x91\x90a4\x1CV[a$\x1D\x91\x90a3eV[` \x8A\x01Q\x90a*\\V[`\0\x80a$[\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a$E\x90a4\0V[a$O\x91\x90a4\x1CV[a\x01\0\x85\x01Q\x90a*\\V[\x90P`\0a$\x84\x84a\x01 \x01Qa\x1Fv\x86a\x01@\x01Q\x87`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xA5a$\x9E\x86`\0\x01Q\x84a \xF3\x91\x90a3eV[\x84\x90a*\\V[\x90P`\0a$\xC8\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x19\x8D\x82\x82a*\x8FV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a$\xEE\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x83\x83\x87\x84a\x18@V[`\0a\x04\\a%\x10\x84\x84a\x12\xF5V[a*\xB3V[`\0\x81`\0\x03a%.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a%EWP`\0\x91\x90PV[a%VgV\x98\xEE\xF0fp\0\0a4\0V[\x82\x13a%kWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a%v\x83a,\x8EV[\x90P`\0a%\xAFg\r\xE0\xB6\xB3\xA7d\0\0a%\x98\x84g\x1B\xC1mgN\xC8\0\0a\x1BWV[a%\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a4\x1CV[a,\xC5V[\x90P`\0\x80\x82a&\x10\x81a%\xFD\x81a%\xEB\x81a%\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a*GV[a%\xE6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a4\x1CV[a*GV[a%\xE6\x90g\x14\xA8EL\x19\xE1\xAC\0a4\x1CV[a%\xE6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a4\x1CV[a&\"\x90g\x03\xDE\xBD\x08;\x8C|\0a4\x1CV[\x91P\x83\x90Pa&\x8A\x81a&x\x81a&f\x81a&T\x81a&A\x81\x8Ba*GV[a%\xE6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a4\x1CV[a%\xE6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a4\x1CV[a%\xE6\x90g\x051\n\xA7\xD5!0\0a4\x1CV[a%\xE6\x90g\r\xE0\xCC=\x15a\0\0a4\x1CV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&\xA0\x87\x88a*GV[a&\xAC\x90`\0\x19a3\x8CV[a&\xB6\x91\x90a3eV[a&\xC0\x91\x90a4\x1CV[\x92PP`\0a&\xCE\x83a\x1C\tV[\x90P`\0a&\xDC\x85\x83a*GV[\x90P`\0\x88\x12a&\xECW\x80a\x050V[a\x050\x81g\x1B\xC1mgN\xC8\0\0a3eV[`\xB5\x81`\x01`\x88\x1B\x81\x10a'\x17W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a'3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a'KW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a'aW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\\g\r\xE0\xB6\xB3\xA7d\0\0\x83a'\xBA\x86a*\xB3V[a'\xC4\x91\x90a3\x8CV[a\x1F\xC7\x91\x90a3\xD2V[`\0\x80\x82\x12\x80a'\xE5WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a(\x03W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a($W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a(LW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a(WW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a(\x7FWa(z\x83g\x1B\xC1mgN\xC8\0\0a3eV[a(\x81V[\x82[\x90P`\0a(\x97\x82g\x1B\xC1mgN\xC8\0\0a,\xC5V[\x90P\x80`\0\x03a(\xBAW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a(\xC5\x82a*\xB3V[\x90P`\0c;\x9A\xCA\0a(\xF0a(\xEBa(\xE5g\x1B\xC1mgN\xC8\0\0a4\0V[\x85a*GV[a&\xFEV[a(\xFA\x91\x90a3\x8CV[\x90P`\0\x80a)\x11\x83g\x03\xC1f\\z\xAB \0a*GV[a)#\x90g \x05\xFEO&\x8E\xA0\0a4\x1CV[\x90P`\0a)N\x84a)<\x86f\x9F2u$b\xA0\0a*GV[a%\xE6\x90g\r\xC5R\x7Fd, \0a4\x1CV[a)`\x90g\r\xE0\xB6\xB3\xA7d\0\0a4\x1CV[\x90Pa)\x84g\t\xD0(\xCCo _\xFF\x19\x85a)z\x85\x85a,\xC5V[a%\xE6\x91\x90a3eV[\x92PPP`\0[`\x02\x81\x10\x15a*\x1FW`\0\x86a)\xA0\x84a%\x15V[a)\xAA\x91\x90a3eV[\x90P`\0a)\xB8\x84\x85a*GV[a)\xC1\x90a4\0V[\x90P`\0a)\xCE\x82a\x1C\tV[\x90P`\0a)\xDC\x86\x85a*GV[a)\xEEg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a*GV[a)\xF8\x91\x90a3eV[\x90Pa*\x04\x84\x82a,\xC5V[a*\x0E\x90\x87a4\x1CV[\x95P\x84`\x01\x01\x94PPPPPa)\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a*<Wa*7\x82a4\0V[a\x050V[P\x96\x95PPPPPPV[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\xD6V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a*~W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a*\xADW`\0\x80\xFD[\x05\x91\x90PV[`\0\x80\x82\x13a*\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07>V[`\0``a*\xFD\x84a,\xF5V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a,\xB4W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x18\x0EWP\x19`\x01\x01\x90V[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a,\xEEW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a-2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07>V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a-\xB0W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a-\xDAW\x81\x81\x01Q\x83\x82\x01R` \x01a-\xC2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra-\xFB\x81` \x86\x01` \x86\x01a-\xBFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\\` \x83\x01\x84a-\xE3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.8W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.\x8DWa.\x8Da.TV[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\xA8W`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a.\xC1W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a.\xDEW`\0\x80\xFD[Pa.\xE7a.jV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a/\x1A\x81a.\x93V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a/@W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a.\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a/zW`\0\x80\xFD[\x835\x92P` \x84\x015a/\x8C\x81a/WV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x19\x8D`\x80\x83\x01\x84a-\xE3V[`\0` \x82\x84\x03\x12\x15a/\xD6W`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04_V[`\0` \x82\x84\x03\x12\x15a0-W`\0\x80\xFD[\x815a\x05L\x81a.\x93V[\x82\x81R`@` \x82\x01R`\0a\x05I`@\x83\x01\x84a-\xE3V[`\0` \x82\x84\x03\x12\x15a0cW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04_Wa\x04_a0jV[\x81\x81\x03\x81\x81\x11\x15a\x04_Wa\x04_a0jV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x05\x94\x81\x84\x01\x85a-\xE3V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a16W`\0\x80\xFD[\x86Qa1A\x81a/WV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a1\x82W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\x9AW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a1\xAEW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a1\xC0Wa1\xC0a.TV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a1\xE8Wa1\xE8a.TV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a2\x01W`\0\x80\xFD[a\x05\x94\x83` \x83\x01` \x88\x01a-\xBFV[`\0`\xA0\x82\x84\x03\x12\x15a2$W`\0\x80\xFD[a2,a.jV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa2^\x81a.\x93V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a2{W`\0\x80\xFD[a\x04\\\x83\x83a2\x12V[`\0` \x82\x84\x03\x12\x15a2\x97W`\0\x80\xFD[\x81Qa\x05L\x81a.\x93V[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xB7W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a2\xEEWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a3\0\x82\x86a2\xD0V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x12[``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a3\x85Wa3\x85a0jV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a3\xA8Wa3\xA8a0jV[\x81\x81\x05\x83\x14\x82\x15\x17a\x04_Wa\x04_a0jV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3\xE1Wa3\xE1a3\xBCV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a3\xFBWa3\xFBa0jV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a4\x15Wa4\x15a0jV[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a4<Wa4<a0jV[PP\x92\x91PPV[`@\x81\x01a4R\x82\x85a2\xD0V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a4m\x82\x85a2\xD0V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a4\x9CW`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa4\xBB\x86``\x87\x01a2\x12V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a4\xD5Wa4\xD5a3\xBCV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04_Wa\x04_a0jV\xFE\xA2dipfsX\"\x12 Q\xCE\x08\xDF\x0FR\xB73\x04\xE9\xEB\xFD\xDF9\x80.\x19[\x99\xC5\x1B0y\x16\xD3\x94\xA2JN\xD9\t\xEBdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xC4W`\x005`\xE0\x1C\x80c}\xDA\x1A#\x11a\0\xF9W\x80c\xCB\x1FU2\x11a\0\x97W\x80c\xE9G\x16\xD5\x11a\0qW\x80c\xE9G\x16\xD5\x14a\x04\x0FW\x80c\xEE>\x8C\xFB\x14a\x04\"W\x80c\xF3\r7\xF2\x14a\x045W\x80c\xF9\xC2\x82\x11\x14a\x04HW`\0\x80\xFD[\x80c\xCB\x1FU2\x14a\x03\xD6W\x80c\xCE\x15;\xF4\x14a\x03\xE9W\x80c\xDE^\xE1\xC3\x14a\x03\xFCW`\0\x80\xFD[\x80c\x90.\xCA\xA2\x11a\0\xD3W\x80c\x90.\xCA\xA2\x14a\x03rW\x80c\xA8\xC6.v\x14a\x03\x85W\x80c\xAFNC\x7F\x14a\x03\xB0W\x80c\xB0\x9D\x04\xE5\x14a\x03\xC3W`\0\x80\xFD[\x80c}\xDA\x1A#\x14a\x03\x17W\x80c\x7F\x17@\x9C\x14a\x03?W\x80c\x81\xB5\xFA\xC2\x14a\x03RW`\0\x80\xFD[\x80c;&\x8D]\x11a\x01fW\x80cO\xD6|X\x11a\x01@W\x80cO\xD6|X\x14a\x02\xBBW\x80c^\xB4\x08\xFC\x14a\x02\xCEW\x80cb7V\x9F\x14a\x02\xE1W\x80cme\"\x99\x14a\x03\x0FW`\0\x80\xFD[\x80c;&\x8D]\x14a\x02\x82W\x80c;M\x100\x14a\x02\x95W\x80cN\x81\x7F\xD9\x14a\x02\xA8W`\0\x80\xFD[\x80c\x1E\x97\x8C\xB0\x11a\x01\xA2W\x80c\x1E\x97\x8C\xB0\x14a\x02&W\x80c0m\xB4k\x14a\x029W\x80c3\"f\xF3\x14a\x02LW\x80c9(\xFF\x97\x14a\x02_W`\0\x80\xFD[\x80c\x04 X\n\x14a\x01\xC9W\x80c\x12\x06I\xC5\x14a\x01\xF2W\x80c\x13N\xAD\x12\x14a\x02\x13W[`\0\x80\xFD[a\x01\xDCa\x01\xD76`\x04a-\x9DV[a\x04PV[`@Qa\x01\xE9\x91\x90a.\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x02\x05a\x02\x006`\x04a.\"V[a\x04eV[`@Q\x90\x81R` \x01a\x01\xE9V[a\x01\xDCa\x02!6`\x04a.\xABV[a\x05<V[a\x02\x05a\x0246`\x04a/+V[a\x05SV[a\x02\x05a\x02G6`\x04a/+V[a\x05hV[a\x02\x05a\x02Z6`\x04a/+V[a\x05\x9FV[a\x02ra\x02m6`\x04a/eV[a\x05\xCBV[`@Qa\x01\xE9\x94\x93\x92\x91\x90a/\x9DV[a\x01\xDCa\x02\x906`\x04a-\x9DV[a\t\xA0V[a\x02\x05a\x02\xA36`\x04a/\xC4V[a\t\xACV[a\x02\x05a\x02\xB66`\x04a/+V[a\t\xD5V[a\x02\x05a\x02\xC96`\x04a/+V[a\t\xEAV[a\x02\x05a\x02\xDC6`\x04a.\"V[a\n\x16V[a\x02\xF4a\x02\xEF6`\x04a-\x9DV[a\n\xE1V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xE9V[a\x02\x05`\0\x81V[a\x03*a\x03%6`\x04a-\x9DV[a\x0B;V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xE9V[a\x02\xF4a\x03M6`\x04a-\x9DV[a\x0B\xABV[a\x03ea\x03`6`\x04a/\xC4V[a\x0C\x04V[`@Qa\x01\xE9\x91\x90a/\xDDV[a\x02\x05a\x03\x806`\x04a/+V[a\x0C\xC3V[`\0Ta\x03\x98\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xE9V[a\x02\x05a\x03\xBE6`\x04a.\"V[a\x0C\xEFV[a\x01\xDCa\x03\xD16`\x04a/\xC4V[a\r\xA4V[a\x01\xDCa\x03\xE46`\x04a0\x1BV[a\r\xAFV[a\x02\xF4a\x03\xF76`\x04a/\xC4V[a\r\xBAV[a\x03*a\x04\n6`\x04a-\x9DV[a\x0E\xB0V[a\x01\xDCa\x04\x1D6`\x04a-\x9DV[a\x0F\x04V[a\x02\xF4a\x0406`\x04a-\x9DV[a\x0F\x10V[a\x02\xF4a\x04C6`\x04a-\x9DV[a\x0F6V[a\x02\x05`x\x81V[``a\x04\\\x83\x83a\x0F\\V[\x90P[\x92\x91PPV[`\0\x80a\x04{\x84\x84a\x04v\x89a\x0C\x04V[a\x0F\x8BV[`@\x80Q` \x81\x01\x88\x90R\x80\x82\x01\x83\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\x04\xD8\x90\x8B\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\xF5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\x19\x91\x90a0QV[\x90Pa\x050\x87\x87\x83\x86a\x05+\x8Da\x0C\x04V[a\x0F\xCDV[\x98\x97PPPPPPPPV[``a\x05I\x84\x84\x84a\x10~V[\x90P[\x93\x92PPPV[`\0a\x05I\x83\x83a\x05c\x87a\x0C\x04V[a\x10\xEFV[`\0\x80a\x05t\x85a\x0C\x04V[\x90P`\0\x80a\x05\x82\x87a\r\xBAV[\x92PP\x91Pa\x05\x94\x86\x83\x83\x88\x87a\x11\xF8V[\x97\x96PPPPPPPV[`\0\x80a\x05\xAB\x85a\x0C\x04V[\x90P`\0\x80a\x05\xB9\x87a\r\xBAV[\x92PP\x91Pa\x05\x94\x86\x83\x83\x88\x87a\x12dV[`\0\x80`\0``a\x05\xF6`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06\x1A`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x06#\x89a\r\xBAV[`@\x85\x01R` \x84\x01R\x82R`\0a\x06:\x8Aa\x0C\x04V[\x90P`\0\x80a\x06W\x8C\x86`\0\x01Q\x87` \x01Q\x88`@\x01Qa\x0C\xEFV[\x90P\x8A\x15a\x07dW`\0a\x06x\x84``\x01Q\x8Ca\x12\xE0\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86Q\x90\x91Pa\x06\x88\x90\x8C\x90a0\x80V[\x85Ra\x06\x94\x81\x83a0\x80V[\x85`@\x01\x81\x81RPP`\0a\x06\xB2\x8E\x87`\0\x01Q\x88`@\x01Qa\x05SV[\x90Pa\x06\xC8\x8E\x87`\0\x01Q\x88`@\x01Q\x84a\x04eV[` \x87\x01\x81\x81R`\x01\x91a\x06\xDD\x90\x83\x90a0\x80V[\x90RP` \x80\x88\x01Q\x90\x87\x01Q\x10a\x07GW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x85` \x01Q\x87` \x01Qa\x07[\x91\x90a0\x93V[\x93PPPa\x08]V[\x82Q``\x84\x01Q`\0\x91a\x07\x83\x91a\x07}\x90\x8E\x90a\x12\xE0V[\x90a\x12\xF5V[\x90P\x8A\x86` \x01Qa\x07\x95\x91\x90a0\x80V[` \x86\x01Ra\x07\xA4\x81\x83a0\x80V[\x85`@\x01\x81\x81RPP`\0a\x07\xC2\x8E\x87` \x01Q\x88`@\x01Qa\t\xD5V[\x90Pa\x07\xD8\x8E\x87` \x01Q\x88`@\x01Q\x84a\n\x16V[\x80\x87R`\x01\x90\x87\x90a\x07\xEB\x90\x83\x90a0\x80V[\x90RP\x86Q\x86Q\x10a\x08JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x07>V[\x85Q\x87Qa\x08X\x91\x90a0\x93V[\x93PPP[P`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x84Q``\x82\x01R` \x80\x86\x01Q`\x80\x83\x01R`@\x80\x87\x01Q`\xA0\x84\x01R\x85Q\x86\x83\x01Q\x87\x83\x01Q\x92Q`\0\x94a\x08\xDF\x94\x91\x01\x92\x83R` \x83\x01\x91\x90\x91R`@\x82\x01R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc2\xE4\xFF\xE1`\xE1\x1B\x84R\x91\x93P\x8F\x92\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90ce\xC9\xFF\xC2\x90a\t)\x900\x90\x86\x90\x89\x90\x89\x90`\x04\x01a0\xA6V[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\tFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tj\x91\x90a1\x1DV[PPPPP\x90P\x80\x85a\t\x86\x89`\0\x01Q\x8A`@\x01Q\x8Aa\x10\xEFV[\x85\x9BP\x9BP\x9BP\x9BPPPPPPPPP\x93P\x93P\x93P\x93V[``a\x04\\\x83\x83a\x13\nV[`\0\x80`\0a\t\xBA\x84a\r\xBAV[\x92PP\x91Pa\t\xCD\x82\x82a\x05c\x87a\x0C\x04V[\x94\x93PPPPV[`\0a\x05I\x83\x83a\t\xE5\x87a\x0C\x04V[a\x13\"V[`\0\x80a\t\xF6\x85a\x0C\x04V[\x90P`\0\x80a\n\x04\x87a\r\xBAV[\x92P\x92PPa\x05\x94\x86\x83\x83\x88\x87a\x13\xF0V[`\0\x80a\n,\x84\x84a\n'\x89a\x0C\x04V[a\x14SV[`@\x80Q` \x81\x01\x83\x90R\x80\x82\x01\x88\x90R``\x80\x82\x01\x88\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x93\x94P\x92\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\n\x89\x90\x8B\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\xCA\x91\x90a0QV[\x90Pa\x050\x87\x87\x83\x86a\n\xDC\x8Da\x0C\x04V[a\x14\x98V[`\0\x80`\0\x80`\0a\n\xF2\x87a\r\xBAV[\x92PP\x91P`\0\x80a\x0B\x07`\0\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0B\x18\x8A\x84\x84a\x05SV[\x90P`\0a\x0B(\x8B\x85\x85\x85a\x04eV[\x93\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[`\0\x80`\0\x80`\0a\x0BL\x87a\r\xBAV[\x92P\x92P\x92P`\0\x80a\x0Bb`\x01\x89\x87\x86a\x15<V[\x91P\x91P`\0a\x0Bs\x8A\x84\x84a\x05SV[\x90P`\0a\x0B\x83\x8B\x85\x85\x85a\x04eV[\x90Pa\x0B\x8F\x86\x82a0\x93V[\x98Pa\x0B\x9B\x85\x84a0\x93V[\x97PPPPPPPP\x92P\x92\x90PV[`\0\x80`\0\x80`\0a\x0B\xBC\x87a\r\xBAV[\x92P\x92PP`\0\x80a\x0B\xD1`\x01\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0B\xE2\x8A\x84\x84a\t\xD5V[\x90P`\0a\x0B\xF2\x8B\x85\x85\x85a\n\x16V[\x9B\x93\x9AP\x91\x98P\x91\x96PPPPPPPV[a\x0C?`@Q\x80`\xA0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0C\xB0\x91\x90\x81\x01\x90a1pV[\x80` \x01\x90Q\x81\x01\x90a\x04_\x91\x90a2iV[`\0\x80a\x0C\xCF\x85a\x0C\x04V[\x90P`\0\x80a\x0C\xDD\x87a\r\xBAV[\x92P\x92PPa\x05\x94\x86\x83\x83\x88\x87a\x15\xA5V[`@\x80Q` \x81\x01\x85\x90R\x80\x82\x01\x84\x90R``\x80\x82\x01\x84\x90R\x82Q\x80\x83\x03\x90\x91\x01\x81R`\x80\x82\x01\x92\x83\x90R`\0\x80Tb.RK`\xE0\x1B\x90\x94R\x92\x90\x91\x83\x91`\x01`\x01`\xA0\x1B\x03\x16\x90b.RK\x90a\rL\x90\x8A\x90\x86\x90`\x84\x01a08V[` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\riW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x8D\x91\x90a0QV[\x90Pa\x05\x94\x86\x86\x83\x87a\r\x9F\x8Ca\x0C\x04V[a\x16\x01V[``a\x04_\x82a\x16\xA5V[``a\x04_\x82a\x16\xD1V[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x11W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E5\x91\x90a2\x85V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0Eb\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E\x7FW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0E\xA3\x91\x90a2\xA2V[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80`\0a\x0E\xC1\x87a\r\xBAV[\x92P\x92P\x92P`\0\x80a\x0E\xD7`\x01\x89\x86\x86a\x15<V[\x91P\x91P`\0a\x0E\xE8\x8A\x84\x84a\t\xD5V[\x90P`\0a\x0E\xF8\x8B\x85\x85\x85a\n\x16V[\x90Pa\x0B\x8F\x87\x82a0\x93V[``a\x04\\\x83\x83a\x16\xE7V[`\0\x80`\0\x80`\0a\x0F!\x87a\r\xBAV[\x92PP\x91P`\0\x80a\x0B\x07`\x01\x89\x86\x86a\x15<V[`\0\x80`\0\x80`\0a\x0FG\x87a\r\xBAV[\x92P\x92PP`\0\x80a\x0B\xD1`\0\x89\x86\x86a\x15<V[```\x02\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0F\x98\x84\x84a\x16\xFFV[\x90P`\0a\x0F\xA5\x82a\x17`V[\x90P`\0a\x0F\xB2\x82a\x17\xC9V[\x85Q\x90\x91Pa\x05\x94\x90\x82\x90a\x0F\xC7\x90\x8Aa\x12\xE0V[\x90a\x12\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x10\rW[`\0\x81\x12\x15a\x10\x08Wa\x0F\xF3\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x10\x01\x89\x84\x8A\x88a\x18@V[\x90Pa\x0F\xDBV[a\x10:V[`\0\x81\x13\x15a\x10:Wa\x10%\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x103\x89\x83\x8A\x88a\x18@V[\x90Pa\x10\rV[a\x10q\x89\x89\x83\x88`@Q` \x01a\x10T\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a\x19`a\x19\x97V[\x99\x98PPPPPPPPPV[```\0a\x10\x8D\x85\x85\x85a\x1A\xA8V[\x90P`\0a\x10\x9C\x82\x86\x86a\x0F\x8BV[\x90P`\0a\x10\xAC\x87\x83\x85\x88a\x18@V[\x90Pa\x10\xBB\x87\x83\x83\x86\x89a\x16\x01V[\x92P\x86\x82\x84\x87`@Q` \x01a\x10\xD4\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0\x80a\x11\x04\x83` \x01Q\x84`@\x01Qa\x1A\xEDV[\x90P`\0a\x11\x1A\x84` \x01Q\x85`@\x01Qa\x1B\x13V[\x90P`\0a\x115\x85`@\x01Q\x83a\x1BB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11C\x88\x88a\x1BWV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x11aW`\0\x94PPPPPa\x05LV[`\0\x81\x13a\x11wW`\0\x19\x94PPPPPa\x05LV[`\0a\x11\x93a\x11\x8E\x83g\r\xE0\xB6\xB3\xA7d\0\0a3eV[a\x1BlV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xAB\x88\x85a3\x8CV[a\x11\xB5\x91\x90a3\xD2V[a\x11\xBF\x91\x90a3eV[\x90P`\0a\x11\xCC\x82a\x1C\tV[\x90P`\0a\x11\xD9\x82a\x17\xC9V[\x8AQ\x90\x91Pa\x11\xE8\x90\x82a\x12\xE0V[\x9C\x9BPPPPPPPPPPPPV[`\0\x82a\x03\xE8\x82a\x12\x0C\x89\x89\x89\x85\x89a\x12dV[\x90P`\0\x81\x12\x15a\x12#W`\0\x93PPPPa\x12[V[a\x10q\x89\x89\x89\x88`@Q` \x01a\x12=\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1D\xB2a\x19\x97V[\x95\x94PPPPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x12\x7F\x91\x90a3eV[\x90P`\0a\x12\x91\x88\x88\x88\x85\x89\x89a\x1D\xE3V[\x90P`\0a\x12\x9E\x82a\x1F>V[\x90P`\0a\x12\xAB\x83a sV[\x90P\x80\x82\x84a\x01\0\x01Qa\x12\xBE\x90a4\0V[a\x12\xC8\x91\x90a4\x1CV[a\x12\xD2\x91\x90a4\x1CV[\x9A\x99PPPPPPPPPPV[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x18\x12V[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x18\x12V[```\x04\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[`\0\x80a\x137\x83` \x01Q\x84`@\x01Qa\x1A\xEDV[\x90P`\0a\x13M\x84` \x01Q\x85`@\x01Qa\x1B\x13V[\x90P`\0a\x13h\x85`@\x01Q\x83a\x1BB\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90\x91P`\0\x90a\x13\x85\x90a\x13~\x90\x89a\x1BBV[\x89\x90a\x1BWV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0\x81\x12a\x13\xA3W`\0\x94PPPPPa\x05LV[`\0\x81\x13a\x13\xB9W`\0\x19\x94PPPPPa\x05LV[`\0a\x13\xC4\x82a\x1BlV[\x90P`\0\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xDC\x88\x85a3\x8CV[a\x13\xE6\x91\x90a3\xD2V[a\x11\xBF\x91\x90a4\x1CV[`\0\x82a\x03\xE8\x82a\x14\x04\x89\x89\x89\x85\x89a\x15\xA5V[\x90P`\0\x81\x12\x15a\x14\x1BW`\0\x93PPPPa\x12[V[a\x10q\x89\x89\x89\x88`@Q` \x01a\x145\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a \xFFa\x19\x97V[`\0\x80a\x14`\x84\x84a!0V[\x90P`\0a\x14m\x82a\x17`V[\x90P`\0a\x14z\x82a\x17\xC9V[\x90Pa\x05\x94a\x14\x91\x82g\r\xE0\xB6\xB3\xA7d\0\0a0\x93V[\x88\x90a\x12\xE0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x14\xD8W[`\0\x81\x12\x15a\x14\xD3Wa\x14\xBE\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x14\xCC\x83\x8A\x8A\x88a\x18@V[\x90Pa\x14\xA6V[a\x15\x05V[`\0\x81\x13\x15a\x15\x05Wa\x14\xF0\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x14\xFE\x82\x8A\x8A\x88a\x18@V[\x90Pa\x14\xD8V[a\x10q\x89\x89\x83\x88`@Q` \x01a\x15\x1F\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a!ua\x19\x97V[`\0\x80\x80a\x15J\x84\x86a\x12\xF5V[\x90P`\0a\x15X\x87\x83a\x12\xE0V[\x90P\x87a\x15nWa\x15i\x87\x87a0\x93V[a\x15xV[a\x15x\x87\x87a0\x80V[\x93P\x87a\x15\x8EWa\x15\x89\x81\x86a0\x93V[a\x15\x98V[a\x15\x98\x81\x86a0\x80V[\x92PPP\x94P\x94\x92PPPV[`\0\x80\x82``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x15\xC0\x91\x90a3eV[\x90P`\0a\x15\xD2\x88\x88\x88\x85\x89\x89a!\xA2V[\x90P`\0a\x15\xDF\x82a\"\xFAV[\x90P`\0a\x15\xEC\x83a$(V[\x90P\x80\x82a\x12\xBEg\r\xE0\xB6\xB3\xA7d\0\0a4\0V[`\0\x82\x80\x85\x83\x81\x12\x15a\x16AW[`\0\x81\x12\x15a\x16<Wa\x16'\x82a\x03\xE7a\x03\xE8a\x19AV[\x91Pa\x165\x89\x89\x84\x88a\x18@V[\x90Pa\x16\x0FV[a\x16nV[`\0\x81\x13\x15a\x16nWa\x16Y\x83a\x03\xE9a\x03\xE8a\x18\x12V[\x92Pa\x16g\x89\x89\x85\x88a\x18@V[\x90Pa\x16AV[a\x10q\x89\x89\x83\x88`@Q` \x01a\x16\x88\x94\x93\x92\x91\x90a3\x11V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x14`@a$\xD4a\x19\x97V[```\x01\x82`@Q` \x01a\x16\xBB\x92\x91\x90a4DV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x05\x82`@Q` \x01a\x16\xBB\x92\x91\x90a4_V[```\x03\x83\x83`@Q` \x01a\x0Ft\x93\x92\x91\x90a2\xF2V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a\x17\x1B\x83\x83a\x1A\xEDV[\x90P`\0a\x17)\x88\x86a%\x01V[\x90P`\0a\x177\x85\x85a\x1B\x13V[\x90P\x82a\x17D\x82\x84a3eV[a\x17V\x90g\r\xE0\xB6\xB3\xA7d\0\0a3\x8CV[a\x10q\x91\x90a3\xD2V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x17~g\r\xE0\xB6\xB3\xA7d\0\0\x85a3\x8CV[a\x17\x88\x91\x90a3\xD2V[\x90P`\0a\x17\x95\x82a4\0V[\x90P`\0a\x17\xA2\x82a%\x15V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x17\xBFg\r\xE0\xB6\xB3\xA7d\0\0\x83a3\x8CV[a\x12[\x91\x90a3\xD2V[`\0\x80\x82\x12\x15a\x18\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x07>V[P\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x18*W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x82\x85\x10a\x18\x91W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x01a\x07>V[`\0\x80g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xA7\x88\x87a\x1BWV[\x10a\x18\xBBW`\x01`\x01`\xFF\x1B\x03\x91Pa\x18\xCBV[a\x18\xC8a\x11\x8E\x88\x87a\x1BWV[\x91P[g\r\xE0\xB6\xB3\xA7d\0\0a\x18\xEB\x87a\x18\xE6\x87`\0\x01Q\x89a\x1BBV[a\x1BWV[\x10a\x18\xFEWP`\x01`\x01`\xFF\x1B\x03a\x19\x16V[a\x19\x13a\x11\x8E\x87a\x18\xE6\x87`\0\x01Q\x89a\x1BBV[\x90P[`\0a\x19*\x85` \x01Q\x86`@\x01Qa\x1A\xEDV[\x90P\x80a\x197\x83\x85a4\x1CV[a\x050\x91\x90a4\x1CV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x19YW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x19z\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x83\x86\x84\x84a\x18@V[\x96\x95PPPPPPV[`\0\x84\x86\x11\x15a\x19\xC4W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x07>V[`\0a\x19\xD4\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xE6\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x19\xF4\x82\x84a3\x8CV[\x13\x15a\x1A\x1DW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x07>V[`\0a\x1A)\x89\x89a0\x93V[\x90P`\0[`\x02a\x1A:\x8A\x8Ca0\x80V[a\x1AD\x91\x90a4\xC6V[\x94P`\0a\x1AV\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1Ad\x86\x83a3\x8CV[\x13a\x1AqW\x85\x99Pa\x1AxV[\x85\x9AP\x80\x94P[a\x1A\x82\x8B\x8Ba0\x93V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x1A\x96WP\x86\x81\x10[a\x1A.WPPPP\x96\x95PPPPPPV[`\0\x80a\x1A\xB5\x84\x84a!0V[\x90P`\0a\x1A\xC2\x82a\x17`V[\x90P`\0a\x1A\xCF\x82a\x17\xC9V[\x90Pa\x05\x94a\x1A\xE6\x82g\r\xE0\xB6\xB3\xA7d\0\0a0\x93V[\x88\x90a\x12\xF5V[`\0\x80a\x1A\xF9\x83a&\xFEV[a\x1B\x07\x90c;\x9A\xCA\0a4\xDAV[\x90Pa\t\xCD\x84\x82a\x1BBV[`\0\x80a\x1B2\x83a\x1B,\x86g\x1B\xC1mgN\xC8\0\0a'\xA2V[\x90a\x1BBV[\x90Pa\t\xCDg\x06\xF0[Y\xD3\xB2\0\0\x82[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x19AV[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x19AV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x1B\x85WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x1B\xADW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1B\xCEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\xDB\x83`\x02a3\x8CV[\x90P`\0a\x1B\xE8\x82a'\xCEV[\x90P`\0a\x1B\xFEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a*GV[\x90Pa\x12[\x81a4\0V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x1C$WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x1CkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07>V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\x1D\xCE\x91\x90a4\x85V[\x93P\x93P\x93P\x93Pa\x05\x94\x84\x84\x84\x89\x85a\x12dV[a\x1E@`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x1E^a\x1EO\x88\x86a4\x1CV[g\x1B\xC1mgN\xC8\0\0\x90a*\\V[\x90P`\0a\x1El\x85\x87a*\\V[a\x1Ev\x86\x89a4\x1CV[a\x1E\x80\x91\x90a3eV[\x90P`\0a\x1E\x96a\x1E\x91\x84\x84a*\x8FV[a'\xCEV[\x90P`\0a\x1E\xABg\x1B\xC1mgN\xC8\0\0a&\xFEV[a\x1E\xB9\x90c;\x9A\xCA\0a4\xDAV[\x90P`\0a\x1E\xCA\x87`@\x01Qa&\xFEV[a\x1E\xD8\x90c;\x9A\xCA\0a4\xDAV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a\x1F|g\x1B\xC1mgN\xC8\0\0a\x1Fv\x85``\x01Qa\x1Fp\x87`@\x01Q\x88`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a*\\V[\x90a*\x8FV[a\x1F\x85\x90a4\0V[\x90P`\0a\x1F\xB6\x84`\0\x01Qa\x1Fp\x86a\x01 \x01Qa\x1Fp\x88`@\x01Q\x89a\x01@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1F\xCCa\x1F\xC7\x83\x85a4\x1CV[a\x1C\tV[\x90P`\0a \x1Ca \x02\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xED\x90a4\0V[a\x1F\xF7\x91\x90a4\x1CV[`\xA0\x89\x01Q\x90a*\\V[\x87`\xC0\x01Qa \x11\x91\x90a4\x1CV[` \x88\x01Q\x90a*\\V[\x90P`\0a *\x83\x83a*\\V[\x90P`\0a I\x88`\x80\x01Q\x89`\xE0\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xE0\x01Q\x89`\xC0\x01Qa ]\x91\x90a4\x1CV[a g\x91\x90a3eV[\x90Pa\x050\x82\x82a*\x8FV[`\0\x80a \xB2\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a \x90\x90a4\0V[a \x9A\x91\x90a4\x1CV[` \x85\x01Qa\x1Fp\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a*\\V[\x90P`\0a \xDB\x84a\x01@\x01Qa\x1Fv\x86a\x01 \x01Q\x87`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\t\xCDa \xF8\x85`\0\x01Q\x83a \xF3\x91\x90a3eV[a%\x15V[\x83\x90a*\\V[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a!\x1B\x91\x90a4\x85V[\x93P\x93P\x93P\x93Pa\x05\x94\x84\x84\x84\x89\x85a\x15\xA5V[\x80Q` \x82\x01Q`@\x83\x01Q`\0\x92\x91\x90\x83a!L\x83\x83a\x1A\xEDV[\x90P`\0a!Z\x88\x86a%\x01V[\x90P`\0a!h\x85\x85a\x1B\x13V[\x90P\x82a\x17D\x82\x84a4\x1CV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a!\x8F\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x85\x84\x84\x84a\x18@V[a!\xFF`@Q\x80a\x01`\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\"\x0Ea\x1EO\x88\x86a4\x1CV[\x90P`\0a\"\x1C\x85\x87a*\\V[\x84Q\x86\x90a\"*\x90\x8Aa*\\V[a\"4\x91\x90a4\x1CV[a\">\x91\x90a3eV[\x90P`\0a\"Oa\x1E\x91\x84\x84a*\x8FV[\x90P`\0a\"dg\x1B\xC1mgN\xC8\0\0a&\xFEV[a\"r\x90c;\x9A\xCA\0a4\xDAV[\x90P`\0a\"\x83\x87`@\x01Qa&\xFEV[a\"\x91\x90c;\x9A\xCA\0a4\xDAV[`@\x80Qa\x01`\x81\x01\x82R\x94\x85R\x88Q` \x80\x87\x01\x91\x90\x91R\x89\x01Q\x85\x82\x01R\x97\x90\x97\x01Q``\x84\x01RP`\x80\x82\x01\x97\x90\x97R`\xA0\x81\x01\x98\x90\x98RPPP`\xC0\x85\x01\x93\x90\x93R`\xE0\x84\x01Ra\x01\0\x83\x01\x93\x90\x93Ra\x01 \x82\x01\x92\x90\x92Ra\x01@\x81\x01\x91\x90\x91R\x90V[`\0\x80a#,g\x1B\xC1mgN\xC8\0\0a\x1Fv\x85``\x01Qa\x1Fp\x87`@\x01Q\x88`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a#5\x90a4\0V[\x90P`\0a#f\x84`\0\x01Qa\x1Fp\x86a\x01@\x01Qa\x1Fp\x88`@\x01Q\x89a\x01 \x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a#wa\x1F\xC7\x83\x85a4\x1CV[\x90P`\0a#\xC1a#\x98\x87`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1F\xED\x90a4\0V[`\xC0\x88\x01Q` \x89\x01Qa#\xAB\x91a*\\V[a#\xB5\x91\x90a4\x1CV[a\x01\0\x88\x01Q\x90a*\\V[\x90P`\0a#\xCF\x83\x83a*\\V[\x90P`\0a ga#\xF1\x89`\x80\x01Q\x8A`\xE0\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xE0\x8A\x01Q`\xC0\x8B\x01Q` \x8C\x01Qa$\t\x91a*\\V[a$\x13\x91\x90a4\x1CV[a$\x1D\x91\x90a3eV[` \x8A\x01Q\x90a*\\V[`\0\x80a$[\x83`\x80\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a$E\x90a4\0V[a$O\x91\x90a4\x1CV[a\x01\0\x85\x01Q\x90a*\\V[\x90P`\0a$\x84\x84a\x01 \x01Qa\x1Fv\x86a\x01@\x01Q\x87`@\x01Qa*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a$\xA5a$\x9E\x86`\0\x01Q\x84a \xF3\x91\x90a3eV[\x84\x90a*\\V[\x90P`\0a$\xC8\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a*\\\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x19\x8D\x82\x82a*\x8FV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a$\xEE\x91\x90a4\x85V[\x93PP\x92P\x92Pa\x19\x8D\x83\x83\x87\x84a\x18@V[`\0a\x04\\a%\x10\x84\x84a\x12\xF5V[a*\xB3V[`\0\x81`\0\x03a%.WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a%EWP`\0\x91\x90PV[a%VgV\x98\xEE\xF0fp\0\0a4\0V[\x82\x13a%kWPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a%v\x83a,\x8EV[\x90P`\0a%\xAFg\r\xE0\xB6\xB3\xA7d\0\0a%\x98\x84g\x1B\xC1mgN\xC8\0\0a\x1BWV[a%\xAA\x90g\r\xE0\xB6\xB3\xA7d\0\0a4\x1CV[a,\xC5V[\x90P`\0\x80\x82a&\x10\x81a%\xFD\x81a%\xEB\x81a%\xD3\x81g\x02_\x0F\xE1\x05\xA3\x14\0a*GV[a%\xE6\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a4\x1CV[a*GV[a%\xE6\x90g\x14\xA8EL\x19\xE1\xAC\0a4\x1CV[a%\xE6\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a4\x1CV[a&\"\x90g\x03\xDE\xBD\x08;\x8C|\0a4\x1CV[\x91P\x83\x90Pa&\x8A\x81a&x\x81a&f\x81a&T\x81a&A\x81\x8Ba*GV[a%\xE6\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a4\x1CV[a%\xE6\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a4\x1CV[a%\xE6\x90g\x051\n\xA7\xD5!0\0a4\x1CV[a%\xE6\x90g\r\xE0\xCC=\x15a\0\0a4\x1CV[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a&\xA0\x87\x88a*GV[a&\xAC\x90`\0\x19a3\x8CV[a&\xB6\x91\x90a3eV[a&\xC0\x91\x90a4\x1CV[\x92PP`\0a&\xCE\x83a\x1C\tV[\x90P`\0a&\xDC\x85\x83a*GV[\x90P`\0\x88\x12a&\xECW\x80a\x050V[a\x050\x81g\x1B\xC1mgN\xC8\0\0a3eV[`\xB5\x81`\x01`\x88\x1B\x81\x10a'\x17W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a'3W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a'KW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a'aW`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0a\x04\\g\r\xE0\xB6\xB3\xA7d\0\0\x83a'\xBA\x86a*\xB3V[a'\xC4\x91\x90a3\x8CV[a\x1F\xC7\x91\x90a3\xD2V[`\0\x80\x82\x12\x80a'\xE5WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a(\x03W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a($W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a(LW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a(WW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a(\x7FWa(z\x83g\x1B\xC1mgN\xC8\0\0a3eV[a(\x81V[\x82[\x90P`\0a(\x97\x82g\x1B\xC1mgN\xC8\0\0a,\xC5V[\x90P\x80`\0\x03a(\xBAW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a(\xC5\x82a*\xB3V[\x90P`\0c;\x9A\xCA\0a(\xF0a(\xEBa(\xE5g\x1B\xC1mgN\xC8\0\0a4\0V[\x85a*GV[a&\xFEV[a(\xFA\x91\x90a3\x8CV[\x90P`\0\x80a)\x11\x83g\x03\xC1f\\z\xAB \0a*GV[a)#\x90g \x05\xFEO&\x8E\xA0\0a4\x1CV[\x90P`\0a)N\x84a)<\x86f\x9F2u$b\xA0\0a*GV[a%\xE6\x90g\r\xC5R\x7Fd, \0a4\x1CV[a)`\x90g\r\xE0\xB6\xB3\xA7d\0\0a4\x1CV[\x90Pa)\x84g\t\xD0(\xCCo _\xFF\x19\x85a)z\x85\x85a,\xC5V[a%\xE6\x91\x90a3eV[\x92PPP`\0[`\x02\x81\x10\x15a*\x1FW`\0\x86a)\xA0\x84a%\x15V[a)\xAA\x91\x90a3eV[\x90P`\0a)\xB8\x84\x85a*GV[a)\xC1\x90a4\0V[\x90P`\0a)\xCE\x82a\x1C\tV[\x90P`\0a)\xDC\x86\x85a*GV[a)\xEEg\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a*GV[a)\xF8\x91\x90a3eV[\x90Pa*\x04\x84\x82a,\xC5V[a*\x0E\x90\x87a4\x1CV[\x95P\x84`\x01\x01\x94PPPPPa)\x8BV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a*<Wa*7\x82a4\0V[a\x050V[P\x96\x95PPPPPPV[`\0a\x04\\\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a,\xD6V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a*~W`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a*\xADW`\0\x80\xFD[\x05\x91\x90PV[`\0\x80\x82\x13a*\xF0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07>V[`\0``a*\xFD\x84a,\xF5V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a,\xB4W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x18\x0EWP\x19`\x01\x01\x90V[`\0a\x04\\\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a,\xEEW`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a-2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07>V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a-\xB0W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a-\xDAW\x81\x81\x01Q\x83\x82\x01R` \x01a-\xC2V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra-\xFB\x81` \x86\x01` \x86\x01a-\xBFV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04\\` \x83\x01\x84a-\xE3V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a.8W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a.\x8DWa.\x8Da.TV[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a.\xA8W`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xE0\x81\x12\x15a.\xC1W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\xA0`?\x19\x82\x01\x12\x15a.\xDEW`\0\x80\xFD[Pa.\xE7a.jV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R`\xC0\x85\x015a/\x1A\x81a.\x93V[`\x80\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a/@W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a.\xA8W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a/zW`\0\x80\xFD[\x835\x92P` \x84\x015a/\x8C\x81a/WV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x19\x8D`\x80\x83\x01\x84a-\xE3V[`\0` \x82\x84\x03\x12\x15a/\xD6W`\0\x80\xFD[P5\x91\x90PV[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q\x90\x82\x01R`\x80\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\xA0\x81\x01a\x04_V[`\0` \x82\x84\x03\x12\x15a0-W`\0\x80\xFD[\x815a\x05L\x81a.\x93V[\x82\x81R`@` \x82\x01R`\0a\x05I`@\x83\x01\x84a-\xE3V[`\0` \x82\x84\x03\x12\x15a0cW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04_Wa\x04_a0jV[\x81\x81\x03\x81\x81\x11\x15a\x04_Wa\x04_a0jV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x05\x94\x81\x84\x01\x85a-\xE3V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a16W`\0\x80\xFD[\x86Qa1A\x81a/WV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a1\x82W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\x9AW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a1\xAEW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a1\xC0Wa1\xC0a.TV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a1\xE8Wa1\xE8a.TV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a2\x01W`\0\x80\xFD[a\x05\x94\x83` \x83\x01` \x88\x01a-\xBFV[`\0`\xA0\x82\x84\x03\x12\x15a2$W`\0\x80\xFD[a2,a.jV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Q``\x82\x01R`\x80\x82\x01Qa2^\x81a.\x93V[`\x80\x82\x01R\x92\x91PPV[`\0`\xA0\x82\x84\x03\x12\x15a2{W`\0\x80\xFD[a\x04\\\x83\x83a2\x12V[`\0` \x82\x84\x03\x12\x15a2\x97W`\0\x80\xFD[\x81Qa\x05L\x81a.\x93V[`\0\x80`\0``\x84\x86\x03\x12\x15a2\xB7W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\x06\x81\x10a2\xEEWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a3\0\x82\x86a2\xD0V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90Ra\x01\0\x81\x01a\x12[``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x80\x82\x01Q\x90\x83\x01R`\x80\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a3\x85Wa3\x85a0jV[P\x92\x91PPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a3\xA8Wa3\xA8a0jV[\x81\x81\x05\x83\x14\x82\x15\x17a\x04_Wa\x04_a0jV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a3\xE1Wa3\xE1a3\xBCV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a3\xFBWa3\xFBa0jV[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a4\x15Wa4\x15a0jV[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a4<Wa4<a0jV[PP\x92\x91PPV[`@\x81\x01a4R\x82\x85a2\xD0V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a4m\x82\x85a2\xD0V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80a\x01\0\x85\x87\x03\x12\x15a4\x9CW`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa4\xBB\x86``\x87\x01a2\x12V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a4\xD5Wa4\xD5a3\xBCV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04_Wa\x04_a0jV\xFE\xA2dipfsX\"\x12 Q\xCE\x08\xDF\x0FR\xB73\x04\xE9\xEB\xFD\xDF9\x80.\x19[\x99\xC5\x1B0y\x16\xD3\x94\xA2JN\xD9\t\xEBdsolcC\0\x08\x16\x003";
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
        AllocateGivenDeltaX(AllocateGivenDeltaXCall),
        AllocateGivenDeltaY(AllocateGivenDeltaYCall),
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
                Self::AllocateGivenDeltaX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenDeltaY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::AllocateGivenDeltaX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenDeltaY(element) => ::core::fmt::Display::fmt(element, f),
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

pub use geometric_mean_solver::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod geometric_mean_solver {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_strategy"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocateGivenX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocateGivenY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateDiffLower"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("calculateDiffRaise"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("v"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbLowerPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOptimalArbLowerPrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("computeOptimalArbRaisePrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeOptimalArbRaisePrice",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("vUpper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocateGivenX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocateGivenY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amountY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("fetchPoolParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct GeometricMeanParams",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("S"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct GeometricMeanParams",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextLiquidity"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveX"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getNextReserveY"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("rx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("L"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReservesAndLiquidity",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareControllerUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareControllerUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("prepareFeeUpdate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("data"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareWeightXUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "prepareWeightXUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetWeightX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("targetTimestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("simulateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("simulateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("strategy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("strategy"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_InvalidBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_InvalidBounds",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lower"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("upper"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("BisectionLib_RootOutsideBounds"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "BisectionLib_RootOutsideBounds",
                            ),
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
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GEOMETRICMEANSOLVER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0tW`\x1Fa%\\8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0yW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0tWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0tW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa$\xCC\x90\x81a\0\x90\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01gW\x80c%\th\xD9\x14a\x01bW\x80c0m\xB4k\x14a\x01]W\x80c3\"f\xF3\x14a\x01XW\x80c9(\xFF\x97\x14a\x01SW\x80c;M\x100\x14a\x01NW\x80cO\xD6|X\x14a\x01IW\x80cZ\x93\xB8\xCE\x14a\x01DW\x80cb7V\x9F\x14a\x01?W\x80c\x7F\x17@\x9C\x14a\x01:W\x80c\x81\xB5\xFA\xC2\x14a\x015W\x80c\x90.\xCA\xA2\x14a\x010W\x80c\xA8\xC6.v\x14a\x01+W\x80c\xB0\x9D\x04\xE5\x14a\x01&W\x80c\xCB\x1FU2\x14a\x01!W\x80c\xCE\x15;\xF4\x14a\x01\x1CW\x80c\xDE\xF1_\x92\x14a\x01\x17W\x80c\xEC)\xD8\xE6\x14a\x01\x12W\x80c\xEE>\x8C\xFB\x14a\x01\rW\x80c\xF2\xDEz{\x14a\x01\x08Wc\xF3\r7\xF2\x14a\x01\x03W`\0\x80\xFD[a\t\xB2V[a\t\x96V[a\tbV[a\tLV[a\x08\xE0V[a\x08/V[a\x07\xEAV[a\x07\xA6V[a\x07}V[a\x07TV[a\x07\0V[a\x06\xA0V[a\x06?V[a\x06\x1AV[a\x05\xF1V[a\x05\xBFV[a\x03.V[a\x02\xD6V[a\x02\x9FV[a\x026V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x01\xD5W6`#\x82\x01\x12\x15a\x01\xD5W\x80`\x04\x015\x91\x82\x11a\x01\xD5W6`$\x83\x83\x01\x01\x11a\x01\xD5Wa\x01\xD1\x91`$a\x01\xC1\x92\x01`\x045a\t\xE5V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`\0\x80\xFD[`\0[\x83\x81\x10a\x01\xEDWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xDDV[\x90` \x91a\x02\x16\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xDAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x023\x92\x81\x81R\x01\x90a\x01\xFDV[\x90V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02q\x81a\x08\x81V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[``\x90`\x03\x19\x01\x12a\x01\xD5W`\x045\x90`$5\x90`D5\x90V[4a\x01\xD5W` a\x02\xCEa\x02\xB26a\x02\x85V[\x90a\x02\xC5a\x02\xBF\x84a\x0CEV[\x93a\rrV[\x92\x91\x90\x91a\x0F\x1EV[`@Q\x90\x81R\xF3[4a\x01\xD5W` a\x02\xCEa\x02\xE96a\x02\x85V[\x90a\x02\xF6a\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x11IV[\x80\x15\x15\x03a\x01\xD5WV[\x90\x92`\x80\x92a\x023\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xFDV[4a\x01\xD5W``6`\x03\x19\x01\x12a\x01\xD5W`\x045`$5a\x03N\x81a\x02\xFFV[a\x04\xC4`D5\x91a\x03]a\n\x11V[a\x03\xADa\x03ha\n\x11V[\x94a\x03r\x87a\rrV[\x94\x91\x95\x90\x92` \x96\x87\x84\x01\x94`@\x97\x88\x86\x01R\x85R\x83R\x86\x8A\x87\x8Ba\x03\x96\x83a\x0CEV[\x98\x89\x93\x88Q\x90a\x03\xA7\x8BQ\x91a\x0CEV[\x91a\x12\xE2V[\x95\x15a\x05;WPa\x04\x0C\x93a\x03\xFEa\x03\xF9a\x04@\x99\x98\x95a\x03\xF3\x86a\x03\xDCa\x04\x05\x97a\x04\x19\x9C\x99\x01Q\x87a\x1D V[\x92a\x03\xEA\x8DQ\x8BQ\x90a\x1DLV[\x91\x01Q\x90a\x13$V[\x90a\x1D V[a\nWV[\x93Qa\nzV[\x8BRa\nzV[\x80\x86\x8A\x01R\x88Q\x8Aa\x0EeV[\x90a\x047a\x04,\x87\x8A\x01\x93\x80\x85Ra\nWV[\x80\x84R\x82Q\x11a\x0B!V[Q\x90Q\x90a\x0B\x14V[\x95[`\xC0\x86Q\x85\x88\x01\x92a\x04\x84\x84Q\x97a\x04v\x88\x8C\x01Q\x89Q\x9A\x8B\x96\x87\x94\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x08\xBEV[`\0Ta\x04\xA7\x90a\x04\x9B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0B\xAFV[\x03\x91Z\xFA\x94\x85\x15a\x056W`\0\x95a\x04\xF6W[P\x90a\x04\xEB\x91a\x01\xD1\x95\x96Q\x90Q\x90a\x14\xE4V[\x90Q\x94\x85\x94\x85a\x03\tV[a\x01\xD1\x95P\x90a\x05!a\x04\xEB\x93\x92`\xC0=`\xC0\x11a\x05/W[a\x05\x19\x81\x83a\x08\xBEV[\x81\x01\x90a\x0BxV[PPPPP\x95P\x90\x91a\x04\xD7V[P=a\x05\x0FV[a\x0B\xD3V[\x91\x96a\x05\xB0\x95a\x05\x9D\x94a\x05\x86a\x05\xA5\x97a\x05\x7Fa\x03\xF9\x8Ca\x03\xF3a\x05\xB9\x9Fa\x05wa\x05ma\x05\x90\x9C\x83\x01Q\x88a\x1D V[\x93Q\x8BQ\x90a\x1DLV[\x90Q\x90a\x13$V[\x94Qa\nzV[\x94\x01\x93\x84Ra\nzV[\x90\x81\x89\x8D\x01RQ\x8Ca\x0B\xDFV[\x80\x8ARa\nWV[\x80\x89R\x82Q\x11a\n\x87V[Q\x86Q\x90a\x0B\x14V[\x95a\x04BV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5W` a\x02\xCE`\x045a\x05\xEAa\x05\xE4\x82a\x0CEV[\x91a\rrV[P\x90a\x14\xE4V[4a\x01\xD5W` a\x02\xCEa\x06\x046a\x02\x85V[\x90a\x06\x11a\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x15\tV[4a\x01\xD5W` a\x02\xCEa\x069a\x0606a\x02\x85V[\x91\x92\x90\x92a\x0CEV[\x91a\x16\xA4V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x06{`\x045a\x01\xD1a\x06\x82a\x06sa\x06h\x84a\rrV[\x91\x90P`$5a\x16\xD1V[\x94\x90\x93a\x0CEV[\x84\x84a\x19\xE2V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5W`\x045a\x06\xDBa\x01\xD1a\x06\xE2a\x06\xD3a\x06\xC9\x85a\rrV[\x91P`$5a\x16\xFEV[\x93\x90\x94a\x0CEV[\x83\x85a\x16\xA4V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5W`\x80a\x07\x1E`\x045a\x0CEV[a\x07R`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x01\xD5W` a\x02\xCEa\x07g6a\x02\x85V[\x90a\x07ta\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x17%V[4a\x01\xD5W`\x006`\x03\x19\x01\x12a\x01\xD5W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02q\x81a\x08\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xD5WV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`\x045a\x08\n\x81a\x07\xD9V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02q\x81a\x08\xA2V[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1a\x08N`\x045a\rrV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[a\x08kV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[4a\x01\xD5W`\xC06`\x03\x19\x01\x12a\x01\xD5W`\x806`C\x19\x01\x12a\x01\xD5Wa\x01\xD1a\t@`@Qa\t\x0F\x81a\x08\x81V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\t0\x81a\x07\xD9V[``\x82\x01R`$5`\x045a\x18\xA2V[`@Q\x91\x82\x91\x82a\x02\"V[4a\x01\xD5W` a\x02\xCEa\x03\xA7a\x0606a\x02\x85V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x06{`\x045a\x01\xD1a\x06\x82a\x06sa\t\x8B\x84a\rrV[\x91\x90P`$5a\x16\xFEV[4a\x01\xD5W` a\x02\xCEa\t\xACa\x0606a\x02\x85V[\x91a\x19\xE2V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5W`\x045a\x06\xDBa\x01\xD1a\x06\xE2a\x06\xD3a\t\xDB\x85a\rrV[\x91P`$5a\x16\xD1V[\x91\x81``\x91\x81\x01\x03\x12a\x01\xD5Wa\t\xFEa\x023\x92a\x0CEV[\x90`@\x81\x015\x90` \x81\x015\x905a\x0E\x8BV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\neWV[a\nAV[\x90a\x03\xE8\x91\x82\x01\x80\x92\x11a\neWV[\x91\x90\x82\x01\x80\x92\x11a\neWV[\x15a\n\x8EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\neWV[\x90a\x03\xE8\x91\x82\x03\x91\x82\x11a\neWV[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\neWV[\x91\x90\x82\x03\x91\x82\x11a\neWV[\x15a\x0B(WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x01\xD5W\x81Qa\x0B\x8F\x81a\x02\xFFV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x023\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xFDV[`@Q=`\0\x82>=\x90\xFD[\x91a\x069a\x023\x93a\x0CEV[\x91\x90\x82`\x80\x91\x03\x12a\x01\xD5W`@Qa\x0C\x04\x81a\x08\x81V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x0C-\x83a\x07\xD9V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x01\xD5Wa\x023\x91a\x0B\xECV[\x90`@Qa\x0CR\x81a\x08\x81V[`\0\x90\x81\x81R\x81``` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x056W\x80\x92a\x0C\xB3W[Pa\x023\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0C1V[\x90\x91P=\x80\x82\x86>a\x0C\xC5\x81\x86a\x08\xBEV[\x84\x01\x90\x82\x85\x83\x03\x12a\r;W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\r>W\x01\x90\x82`\x1F\x83\x01\x12\x15a\r;W\x81Q\x95\x86\x11a\x08\x9DW`@Q\x92a\r\x11`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x08\xBEV[\x86\x84R\x84\x87\x84\x01\x01\x11a\r;WPa\x023\x93\x94a\r3\x91\x84\x80\x85\x01\x91\x01a\x01\xDAV[\x90\x83\x92a\x0C\x9FV[\x80\xFD[\x82\x80\xFD[\x90\x81` \x91\x03\x12a\x01\xD5WQa\x023\x81a\x07\xD9V[\x90\x81``\x91\x03\x12a\x01\xD5W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\r\x8Ea\x04\x9Ba\x04\x9B`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x056Wa\r\xD9\x93``\x92`\0\x91a\x0E6W[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x056W`\0\x80\x93`\0\x93a\r\xFFW[P\x92\x91\x90V[\x91\x93PPa\x0E%\x91P``=``\x11a\x0E/W[a\x0E\x1D\x81\x83a\x08\xBEV[\x81\x01\x90a\rWV[\x92\x90\x92\x918a\r\xF9V[P=a\x0E\x13V[a\x0EX\x91P` =` \x11a\x0E^W[a\x0EP\x81\x83a\x08\xBEV[\x81\x01\x90a\rBV[8a\r\xB8V[P=a\x0EFV[\x91a\t\xACa\x023\x93a\x0CEV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\neWV[\x92` a\x03\xEA\x84a\x0E\xAEa\x0E\xA6a\x03\xF3\x96\x97a\x0E\xB4\x99a \x87V[\x85Q\x90a\x13$V[\x95a \x87V[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\neW\x90V[\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Qa\x0F6\x90a\n\xDEV[\x91a\x0FA\x87\x85a \x87V[a\x0FK\x82\x82a\x13$V[\x92a\x0FU\x91a\x13$V[\x89Q\x85\x89\x85\x81a\x0Fe\x85\x8Da \xCAV[\x90a\x0Fo\x91a \xCAV[\x90a\x0Fy\x91a \xCAV[\x92a\x0F\x83\x90a \xA9V[a\x0F\x8C\x90a\n\xF4V[\x90a\x0F\x96\x91a\nzV[\x90a\x0F\xA0\x91a \xCAV[a\x0F\xA9\x86a\n\xDEV[a\x0F\xB2\x91a \xCAV[\x92a\x0F\xBC\x8Aa\njV[\x90a\x0F\xC6\x90a\x10\xF0V[a\x0F\xCF\x91a\x13$V[\x91a\x0F\xD9\x90a \xA9V[a\x0F\xE2\x86a\n\xDEV[a\x0F\xEB\x91a \xCAV[a\x0F\xF5\x90\x89a\nzV[\x92a\x0F\xFF\x91a\x0B\x14V[\x91a\x10\t\x91a \xCAV[\x89Qa\x10\x14\x90a\n\xDEV[a\x10\x1D\x90a hV[a\x10&\x91a\x13$V[a\x10/\x91a \xCAV[\x91\x88Qa\x10;\x90a\n\xDEV[a\x10D\x88a\njV[\x92a\x10O\x89\x89a \xCAV[\x90a\x10Y\x91a \xCAV[\x91a\x10c\x86a \xA9V[\x90a\x10m\x90a\n\xDEV[a\x10v\x91a \xCAV[\x92a\x10\x80\x91a \xCAV[\x91a\x10\x8A\x91a\nzV[a\x10\x93\x91a \xCAV[\x90a\x10\x9D\x84a\x10\xF0V[\x91a\x10\xA7\x91a \x87V[a\x10\xB0\x91a\x11-V[`\0\x13a\x10\xE5Wa\x023\x95a\x10\xE0\x93a\x10\xD2\x92`@Q\x96\x87\x95` \x87\x01a\x0E\xCDV[\x03`\x1F\x19\x81\x01\x83R\x82a\x08\xBEV[a\x1A0V[PPPPPP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\neW`\0\x03\x90V[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\neWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\neWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\neWV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x11d\x90a\n\xDEV[\x92a\x11o\x87\x87a \x87V[a\x11y\x82\x82a\x13$V[\x92a\x11\x83\x91a\x13$V[\x88Q\x87\x89\x85\x81a\x11\x93\x85\x8Ca \xCAV[\x90a\x11\x9D\x91a \xCAV[\x90a\x11\xA7\x91a \xCAV[\x92a\x11\xB2\x90\x88a \xCAV[a\x11\xBC\x90\x88a\x0B\x14V[\x90a\x11\xC6\x91a\nzV[\x90a\x11\xD0\x91a \xCAV[a\x11\xD9\x87a\n\xDEV[a\x11\xE2\x91a \xCAV[\x92a\x11\xED\x8A\x87a\nzV[\x90a\x11\xF7\x90a\x10\xF0V[a\x12\0\x91a\x13$V[\x91a\x12\x0B\x90\x86a \xCAV[a\x12\x14\x87a\n\xDEV[a\x12\x1D\x91a \xCAV[a\x12'\x90\x88a\nzV[\x92a\x121\x91a\x0B\x14V[\x91a\x12;\x91a \xCAV[\x88Qa\x12F\x90a\n\xDEV[a\x12O\x90a hV[a\x12X\x91a\x13$V[a\x12a\x91a \xCAV[\x96Qa\x12l\x90a\n\xDEV[\x93a\x12w\x87\x84a\nzV[\x96a\x12\x81\x91a \xCAV[\x90a\x12\x8B\x91a \xCAV[\x93a\x12\x95\x91a \xCAV[\x90a\x12\x9F\x90a\n\xDEV[a\x12\xA8\x91a \xCAV[\x92a\x12\xB2\x91a \xCAV[\x91a\x12\xBC\x91a\nzV[a\x12\xC5\x91a \xCAV[\x91a\x12\xCF\x90a\x10\xF0V[\x91a\x12\xD9\x91a \x87V[a\x023\x91a\x11-V[a\x023\x92\x91` a\x12\xF8a\x03\xF3\x93\x85Q\x90a\x13$V[\x93\x01Q\x90a\x13$V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\neW\x81\x84\x05\x14\x90\x15\x17\x15a\neWV[a\x14\xD1a\x023\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x14\xDF\x93a\x13Z`\0\x82\x13a\x1D\xC8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x13v\x82a!\nV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x13\x01V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1E\0V[a\x15\x03\x90a\x14\xFBa\x023\x94\x93` \x85\x01Q\x90a \x87V[\x92Q\x90a \x87V[\x90a \x87V[\x90\x92\x91\x85Q`@\x87\x01Qg\r\xE0\xB6\xB3\xA7d\0\0`\0\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\neWa\x15<\x83a\x11\x01V[a\x15E\x83a\x11\x13V[a\x15N\x91a\x13$V[\x90\x82a\x15Z\x85\x89a\x1F\xA9V[\x90a\x15d\x91a\x13$V[a\x15m\x81a\x1F\xC7V[\x92a\x15w\x83a\x11\x13V[a\x15\x81\x90\x85a\x1F\xF0V[a\x15\x8B\x90\x89a\x0ErV[\x91\x82\x91a\x15\x97\x88a\x11\x01V[a\x15\xA1\x90\x88a\x1F\xF0V[\x93a\x15\xAB\x91a\x1F\xF0V[a\x15\xB4\x87a\x1F\x8AV[a\x15\xBD\x91a\x13$V[\x92a\x15\xC7\x87a\x11\x13V[a\x15\xD1\x90\x8Ba\x1F\xF0V[\x91\x88a\x15\xDC\x89a\x1F\xC7V[\x90a\x15\xE6\x91a\x11-V[a\x15\xEF\x91a\x1F\xF0V[a\x15\xF8\x86a\x11\x13V[a\x16\x01\x91a\x1F\xF0V[\x92a\x16\x0B\x91a\x1F\xF0V[\x92a\x16\x16\x90\x89a\x1F\xF0V[\x91a\x16 \x91a\x0ErV[a\x16)\x91a\x1F\xF0V[a\x162\x91a\x11-V[\x92a\x16<\x85a\x11\x01V[a\x16E\x91a\x1F\xF0V[\x91a\x16O\x87a\x10\xF0V[\x91a\x16Y\x90a\x11\x13V[a\x16b\x91a\x1F\xF0V[a\x16k\x91a\x11-V[a\x16t\x91a\x1F\xF0V[a\x16}\x91a\x1F\xA9V[`\0\x13a\x10\xE5Wa\x023\x95a\x16\x9F\x93a\x10\xD2\x92`@Q\x96\x87\x95` \x87\x01a\x0E\xCDV[a\x1BUV[a\x16\xC4a\x023\x93\x92a\x16\xBEa\x16\xCB\x93` \x86\x01Q\x90a\x13$V[\x90a\x1DLV[\x91Qa\x1D|V[\x90a\x13$V[\x92\x91\x90a\x16\xE7a\x16\xE1\x82\x84a\x1DLV[\x85a\x1D V[\x93\x81\x03\x90\x81\x11a\neW\x92\x81\x03\x90\x81\x11a\neW\x90V[\x92\x91\x90a\x17\x0Ea\x16\xE1\x82\x84a\x1DLV[\x93\x81\x01\x80\x91\x11a\neW\x92\x81\x01\x80\x91\x11a\neW\x90V[\x92\x93\x94\x90\x91\x94`@\x82Q\x92\x01Q\x93g\r\xE0\xB6\xB3\xA7d\0\0`\0\x86\x82\x03\x96\x12\x81\x87\x12\x81\x16\x91\x87\x13\x90\x15\x16\x17a\neW\x82\x87\x94a\x17`\x86\x85a\x11-V[a\x17i\x83a\x11\x13V[a\x17r\x91a\x13$V[\x95a\x17|\x91a\x1F\xA9V[\x90a\x17\x86\x91a\x13$V[\x93a\x17\x91\x85\x84a\x1F\xF0V[\x94a\x17\x9B\x87a\x11\x13V[a\x17\xA5\x90\x87a\x1F\xF0V[a\x17\xAF\x90\x89a\x0ErV[\x92\x83\x92a\x17\xBC\x8B\x87a\x11-V[a\x17\xC6\x90\x88a\x1F\xF0V[\x94a\x17\xD0\x91a\x1F\xF0V[a\x17\xD9\x87a\x1F\x8AV[a\x17\xE2\x91a\x13$V[\x93a\x17\xEC\x87a\x11\x13V[a\x17\xF6\x90\x8Ba\x1F\xF0V[\x92\x8Ba\x18\x02\x89\x89a\x1F\xF0V[\x90a\x18\x0C\x91a\x11-V[a\x18\x15\x91a\x1F\xF0V[a\x18\x1E\x8Aa\x11\x13V[a\x18'\x91a\x1F\xF0V[\x93a\x181\x91a\x1F\xF0V[\x93a\x18;\x91a\x1F\xF0V[\x91a\x18E\x91a\x0ErV[a\x18N\x91a\x1F\xF0V[a\x18W\x91a\x11-V[\x95a\x18a\x91a\x11-V[a\x18j\x91a\x1F\xF0V[\x92a\x18t\x90a\x10\xF0V[\x91a\x18~\x90a\x11\x13V[a\x18\x87\x91a\x1F\xF0V[a\x18\x90\x91a\x11-V[a\x18\x99\x91a\x1F\xF0V[a\x023\x91a\x1F\xA9V[\x92\x91\x90\x83a\x18\xBDa\x18\xC2\x92a\x18\xBD` \x86\x01Q\x86Q\x90a \x87V[a \xCAV[\x90a\x18\xCE\x81\x83\x86a\x12\xE2V[\x93a\x18\xDB\x82\x86\x85\x84a\x0E\x8BV[\x85\x90`\0\x80\x82\x12\x15a\x19\xA4W[\x80\x82\x12a\x19\x86WPa\x19-a\x19z\x92a\x023\x96\x97\x98\x86\x93[a\x19\x14`@Q\x98\x89\x92\x8C\x8A` \x86\x01a \x1FV[\x03\x96a\x19(`\x1F\x19\x98\x89\x81\x01\x83R\x82a\x08\xBEV[a\x1C,V[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x08\xBEV[\x96a\x19\x91\x91Pa \xEBV[\x95a\x19\x9E\x84\x88\x87\x86a\x0E\x8BV[\x90a\x18\xE8V[\x96\x91\x96[\x80\x82\x13a\x19\xC4WPa\x19-a\x023\x95\x96\x97a\x19z\x93\x86\x93a\x19\0V[\x96a\x19\xCF\x91Pa\x1D\x9EV[\x95a\x19\xDC\x84\x88\x87\x86a\x0E\x8BV[\x90a\x19\xA8V[` a\x19\xFBa\x023\x94\x93a\x16\xBEa\x16\xCB\x94\x86Q\x90a\x13$V[\x92\x01Qa\x1D|V[\x91\x90a\x01\0\x83\x82\x03\x12a\x01\xD5W\x82Q\x92` \x81\x01Q\x92a\x023`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0B\xECV[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\x1B4Wa\x1AL\x81a!|V[a\x1AV\x85\x83a\"\xD5V[`\0a\x1Ab\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1As\x85\x96\x95a\x0B\x04V[`\x01\x94`\0\x91\x86\x80[a\x1A\x8DW[PPPPPPPP\x90PV[\x15a\x1A\xF0W[P\x85\x96\x97\x98P\x80\x91a\x1A\xAEa\x1A\xA8\x8B\x88a\nzV[`\x01\x1C\x90V[\x99a\x1A\xB9\x8B\x87a\"\xD5V[\x90\x83a\x1A\xC5\x87\x84a\x13\x01V[\x13a\x1A\xE4WPP\x89\x92[\x87a\x1A\xDA\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1A|V[\x8B\x97P\x90\x94P\x92a\x1A\xCFV[\x86\x10\x80a\x1B\nW[\x15a\x1B\x03W\x88a\x1A\x93V[\x80\x80a\x1A\x81V[Pa\x01\0\x82\x10a\x1A\xF8V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\x1B4Wa\x1Bq\x81a\"\xF6V[a\x1B{\x85\x83a$AV[`\0a\x1B\x87\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1B\x98\x85\x96\x95a\x0B\x04V[`\x01\x94`\0\x91\x86\x80[a\x1B\xB1WPPPPPPPP\x90PV[\x15a\x1C\x0EW[P\x85\x96\x97\x98P\x80\x91a\x1B\xCCa\x1A\xA8\x8B\x88a\nzV[\x99a\x1B\xD7\x8B\x87a$AV[\x90\x83a\x1B\xE3\x87\x84a\x13\x01V[\x13a\x1C\x02WPP\x89\x92[\x87a\x1B\xF8\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1B\xA1V[\x8B\x97P\x90\x94P\x92a\x1B\xEDV[\x86\x10\x80a\x1C!W[\x15a\x1B\x03W\x88a\x1B\xB7V[Pa\x01\0\x82\x10a\x1C\x16V[`\0\x93\x92\x91\x84\x91\x83\x82\x11a\x1D\0Wa\x1CD\x82\x82a$bV[a\x1CN\x85\x83a$bV[`\0a\x1CZ\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1Cl\x83\x86\x97\x96a\x0B\x14V[`\x01\x94`\0\x91\x86\x80[a\x1C\x85WPPPPPPPP\x90PV[\x15a\x1C\xE2W[P\x85\x96\x97\x98P\x80\x91a\x1C\xA0a\x1A\xA8\x8B\x88a\nzV[\x99a\x1C\xAB\x8B\x87a$bV[\x90\x83a\x1C\xB7\x87\x84a\x13\x01V[\x13a\x1C\xD6WPP\x89\x92[\x87a\x1C\xCC\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1CuV[\x8B\x97P\x90\x94P\x92a\x1C\xC1V[\x86\x10\x80a\x1C\xF5W[\x15a\x1B\x03W\x88a\x1C\x8BV[Pa\x01\0\x82\x10a\x1C\xEAV[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xD5W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a\x1D\xCFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x1F\x84Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1FPWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x01\xD5W\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x0F\x1C\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xD5W\x04\x90V[a\x03\xE8\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5Wa\x03\xE8\x90\x04\x90V[a!\x15\x81\x15\x15a\x1D\xC8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x80Q\x81\x01` \x01\x90` \x01\x90a!\x91\x91a\x1A\x03V[\x92\x91\x90\x83Q` \x85\x01Q`@\x86\x01Qa!\xA9\x90a\n\xDEV[\x91a!\xB4\x86\x86a \x87V[a!\xBE\x82\x82a\x13$V[\x92a!\xC8\x91a\x13$V[\x87Q\x86\x88\x85\x81a!\xD8\x85\x8Ba \xCAV[\x90a!\xE2\x91a \xCAV[\x90a!\xEC\x91a \xCAV[\x92a!\xF6\x90a \xA9V[a!\xFF\x90a\n\xF4V[\x90a\"\t\x91a\nzV[\x90a\"\x13\x91a \xCAV[a\"\x1C\x86a\n\xDEV[a\"%\x91a \xCAV[\x92a\"/\x89a\njV[\x90a\"9\x90a\x10\xF0V[a\"B\x91a\x13$V[\x91a\"L\x90a \xA9V[a\"U\x86a\n\xDEV[a\"^\x91a \xCAV[a\"h\x90\x87a\nzV[\x92a\"r\x91a\x0B\x14V[\x91a\"|\x91a \xCAV[\x87Qa\"\x87\x90a\n\xDEV[a\"\x90\x90a hV[a\"\x99\x91a\x13$V[a\"\xA2\x91a \xCAV[\x95Qa\"\xAD\x90a\n\xDEV[\x92a\"\xB7\x86a\njV[\x95a\"\xC1\x91a \xCAV[\x90a\"\xCB\x91a \xCAV[\x92a\x12\x95\x90a \xA9V[\x90a\"\xECa\x023\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[\x94\x93\x92\x90\x92a\x11IV[a#\t\x90` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[`@\x81\x95\x93\x95\x94\x92\x94Q\x91\x01Q\x92g\r\xE0\xB6\xB3\xA7d\0\0`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\neW\x81\x86\x93a#C\x85a\x11\x01V[a#L\x83a\x11\x13V[a#U\x91a\x13$V[\x94a#_\x91a\x1F\xA9V[\x90a#i\x91a\x13$V[\x92a#s\x84a\x1F\xC7V[\x93a#}\x86a\x11\x13V[a#\x87\x90\x86a\x1F\xF0V[a#\x91\x90\x88a\x0ErV[\x92\x83\x92a#\x9D\x8Aa\x11\x01V[a#\xA7\x90\x87a\x1F\xF0V[\x94a#\xB1\x91a\x1F\xF0V[a#\xBA\x86a\x1F\x8AV[a#\xC3\x91a\x13$V[\x93a#\xCD\x86a\x11\x13V[a#\xD7\x90\x8Aa\x1F\xF0V[\x92\x8Aa#\xE2\x88a\x1F\xC7V[\x90a#\xEC\x91a\x11-V[a#\xF5\x91a\x1F\xF0V[a#\xFE\x89a\x11\x13V[a$\x07\x91a\x1F\xF0V[\x93a$\x11\x91a\x1F\xF0V[\x93a$\x1B\x91a\x1F\xF0V[\x91a$%\x91a\x0ErV[a$.\x91a\x1F\xF0V[a$7\x91a\x11-V[\x94a\x18a\x90a\x11\x01V[\x90a$Xa\x023\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[\x94\x93\x92\x90\x92a\x17%V[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x01\xD5Wa\x023\x92a$\x90` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x0B\xECV[\x92a\x0E\x8BV\xFE\xA2dipfsX\"\x12 \xD1\x08\xA55\x03\x8Et\xC2T\x7F~\xE2\xF2 \xEA\\\xDFX\xC4 JC\x8D\x97\xDE\x94\x05I\xC5\xA5\x12\xA1dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\x005`\xE0\x1C\x80c\x0FAf\xB8\x14a\x01gW\x80c%\th\xD9\x14a\x01bW\x80c0m\xB4k\x14a\x01]W\x80c3\"f\xF3\x14a\x01XW\x80c9(\xFF\x97\x14a\x01SW\x80c;M\x100\x14a\x01NW\x80cO\xD6|X\x14a\x01IW\x80cZ\x93\xB8\xCE\x14a\x01DW\x80cb7V\x9F\x14a\x01?W\x80c\x7F\x17@\x9C\x14a\x01:W\x80c\x81\xB5\xFA\xC2\x14a\x015W\x80c\x90.\xCA\xA2\x14a\x010W\x80c\xA8\xC6.v\x14a\x01+W\x80c\xB0\x9D\x04\xE5\x14a\x01&W\x80c\xCB\x1FU2\x14a\x01!W\x80c\xCE\x15;\xF4\x14a\x01\x1CW\x80c\xDE\xF1_\x92\x14a\x01\x17W\x80c\xEC)\xD8\xE6\x14a\x01\x12W\x80c\xEE>\x8C\xFB\x14a\x01\rW\x80c\xF2\xDEz{\x14a\x01\x08Wc\xF3\r7\xF2\x14a\x01\x03W`\0\x80\xFD[a\t\xB2V[a\t\x96V[a\tbV[a\tLV[a\x08\xE0V[a\x08/V[a\x07\xEAV[a\x07\xA6V[a\x07}V[a\x07TV[a\x07\0V[a\x06\xA0V[a\x06?V[a\x06\x1AV[a\x05\xF1V[a\x05\xBFV[a\x03.V[a\x02\xD6V[a\x02\x9FV[a\x026V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`$5\x81\x81\x11a\x01\xD5W6`#\x82\x01\x12\x15a\x01\xD5W\x80`\x04\x015\x91\x82\x11a\x01\xD5W6`$\x83\x83\x01\x01\x11a\x01\xD5Wa\x01\xD1\x91`$a\x01\xC1\x92\x01`\x045a\t\xE5V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[`\0\x80\xFD[`\0[\x83\x81\x10a\x01\xEDWPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xDDV[\x90` \x91a\x02\x16\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x01\xDAV[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90` a\x023\x92\x81\x81R\x01\x90a\x01\xFDV[\x90V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`$5``\x82\x01R``\x81Ra\x02q\x81a\x08\x81V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[``\x90`\x03\x19\x01\x12a\x01\xD5W`\x045\x90`$5\x90`D5\x90V[4a\x01\xD5W` a\x02\xCEa\x02\xB26a\x02\x85V[\x90a\x02\xC5a\x02\xBF\x84a\x0CEV[\x93a\rrV[\x92\x91\x90\x91a\x0F\x1EV[`@Q\x90\x81R\xF3[4a\x01\xD5W` a\x02\xCEa\x02\xE96a\x02\x85V[\x90a\x02\xF6a\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x11IV[\x80\x15\x15\x03a\x01\xD5WV[\x90\x92`\x80\x92a\x023\x95\x94\x15\x15\x83R` \x83\x01R`@\x82\x01R\x81``\x82\x01R\x01\x90a\x01\xFDV[4a\x01\xD5W``6`\x03\x19\x01\x12a\x01\xD5W`\x045`$5a\x03N\x81a\x02\xFFV[a\x04\xC4`D5\x91a\x03]a\n\x11V[a\x03\xADa\x03ha\n\x11V[\x94a\x03r\x87a\rrV[\x94\x91\x95\x90\x92` \x96\x87\x84\x01\x94`@\x97\x88\x86\x01R\x85R\x83R\x86\x8A\x87\x8Ba\x03\x96\x83a\x0CEV[\x98\x89\x93\x88Q\x90a\x03\xA7\x8BQ\x91a\x0CEV[\x91a\x12\xE2V[\x95\x15a\x05;WPa\x04\x0C\x93a\x03\xFEa\x03\xF9a\x04@\x99\x98\x95a\x03\xF3\x86a\x03\xDCa\x04\x05\x97a\x04\x19\x9C\x99\x01Q\x87a\x1D V[\x92a\x03\xEA\x8DQ\x8BQ\x90a\x1DLV[\x91\x01Q\x90a\x13$V[\x90a\x1D V[a\nWV[\x93Qa\nzV[\x8BRa\nzV[\x80\x86\x8A\x01R\x88Q\x8Aa\x0EeV[\x90a\x047a\x04,\x87\x8A\x01\x93\x80\x85Ra\nWV[\x80\x84R\x82Q\x11a\x0B!V[Q\x90Q\x90a\x0B\x14V[\x95[`\xC0\x86Q\x85\x88\x01\x92a\x04\x84\x84Q\x97a\x04v\x88\x8C\x01Q\x89Q\x9A\x8B\x96\x87\x94\x85\x01`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x03`\x1F\x19\x81\x01\x84R\x83a\x08\xBEV[`\0Ta\x04\xA7\x90a\x04\x9B\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90\x86Q\x80\x99\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R0`\x04\x85\x01a\x0B\xAFV[\x03\x91Z\xFA\x94\x85\x15a\x056W`\0\x95a\x04\xF6W[P\x90a\x04\xEB\x91a\x01\xD1\x95\x96Q\x90Q\x90a\x14\xE4V[\x90Q\x94\x85\x94\x85a\x03\tV[a\x01\xD1\x95P\x90a\x05!a\x04\xEB\x93\x92`\xC0=`\xC0\x11a\x05/W[a\x05\x19\x81\x83a\x08\xBEV[\x81\x01\x90a\x0BxV[PPPPP\x95P\x90\x91a\x04\xD7V[P=a\x05\x0FV[a\x0B\xD3V[\x91\x96a\x05\xB0\x95a\x05\x9D\x94a\x05\x86a\x05\xA5\x97a\x05\x7Fa\x03\xF9\x8Ca\x03\xF3a\x05\xB9\x9Fa\x05wa\x05ma\x05\x90\x9C\x83\x01Q\x88a\x1D V[\x93Q\x8BQ\x90a\x1DLV[\x90Q\x90a\x13$V[\x94Qa\nzV[\x94\x01\x93\x84Ra\nzV[\x90\x81\x89\x8D\x01RQ\x8Ca\x0B\xDFV[\x80\x8ARa\nWV[\x80\x89R\x82Q\x11a\n\x87V[Q\x86Q\x90a\x0B\x14V[\x95a\x04BV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5W` a\x02\xCE`\x045a\x05\xEAa\x05\xE4\x82a\x0CEV[\x91a\rrV[P\x90a\x14\xE4V[4a\x01\xD5W` a\x02\xCEa\x06\x046a\x02\x85V[\x90a\x06\x11a\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x15\tV[4a\x01\xD5W` a\x02\xCEa\x069a\x0606a\x02\x85V[\x91\x92\x90\x92a\x0CEV[\x91a\x16\xA4V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x06{`\x045a\x01\xD1a\x06\x82a\x06sa\x06h\x84a\rrV[\x91\x90P`$5a\x16\xD1V[\x94\x90\x93a\x0CEV[\x84\x84a\x19\xE2V[\x92`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5W`\x045a\x06\xDBa\x01\xD1a\x06\xE2a\x06\xD3a\x06\xC9\x85a\rrV[\x91P`$5a\x16\xFEV[\x93\x90\x94a\x0CEV[\x83\x85a\x16\xA4V[\x91`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5W`\x80a\x07\x1E`\x045a\x0CEV[a\x07R`@Q\x80\x92``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\xF3[4a\x01\xD5W` a\x02\xCEa\x07g6a\x02\x85V[\x90a\x07ta\x02\xBF\x84a\x0CEV[\x92\x91\x90\x91a\x17%V[4a\x01\xD5W`\x006`\x03\x19\x01\x12a\x01\xD5W`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`@Q`\x01` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x02q\x81a\x08\xA2V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xD5WV[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1`\x045a\x08\n\x81a\x07\xD9V[`@\x80Q`\x03` \x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82\x82\x01R\x81Ra\x02q\x81a\x08\xA2V[4a\x01\xD5W` 6`\x03\x19\x01\x12a\x01\xD5Wa\x01\xD1a\x08N`\x045a\rrV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R\x90\x81\x90``\x82\x01\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[a\x08kV[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@RV[4a\x01\xD5W`\xC06`\x03\x19\x01\x12a\x01\xD5W`\x806`C\x19\x01\x12a\x01\xD5Wa\x01\xD1a\t@`@Qa\t\x0F\x81a\x08\x81V[`D5\x81R`d5` \x82\x01R`\x845`@\x82\x01R`\xA45a\t0\x81a\x07\xD9V[``\x82\x01R`$5`\x045a\x18\xA2V[`@Q\x91\x82\x91\x82a\x02\"V[4a\x01\xD5W` a\x02\xCEa\x03\xA7a\x0606a\x02\x85V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5Wa\x06{`\x045a\x01\xD1a\x06\x82a\x06sa\t\x8B\x84a\rrV[\x91\x90P`$5a\x16\xFEV[4a\x01\xD5W` a\x02\xCEa\t\xACa\x0606a\x02\x85V[\x91a\x19\xE2V[4a\x01\xD5W`@6`\x03\x19\x01\x12a\x01\xD5W`\x045a\x06\xDBa\x01\xD1a\x06\xE2a\x06\xD3a\t\xDB\x85a\rrV[\x91P`$5a\x16\xD1V[\x91\x81``\x91\x81\x01\x03\x12a\x01\xD5Wa\t\xFEa\x023\x92a\x0CEV[\x90`@\x81\x015\x90` \x81\x015\x905a\x0E\x8BV[`@Q\x90``\x82\x01\x82\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x08\x9DW`@R`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90`\x01\x82\x01\x80\x92\x11a\neWV[a\nAV[\x90a\x03\xE8\x91\x82\x01\x80\x92\x11a\neWV[\x91\x90\x82\x01\x80\x92\x11a\neWV[\x15a\n\x8EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\neWV[\x90a\x03\xE8\x91\x82\x03\x91\x82\x11a\neWV[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\neWV[\x91\x90\x82\x03\x91\x82\x11a\neWV[\x15a\x0B(WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x90\xFD[\x91\x90\x82`\xC0\x91\x03\x12a\x01\xD5W\x81Qa\x0B\x8F\x81a\x02\xFFV[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x023\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x01\xFDV[`@Q=`\0\x82>=\x90\xFD[\x91a\x069a\x023\x93a\x0CEV[\x91\x90\x82`\x80\x91\x03\x12a\x01\xD5W`@Qa\x0C\x04\x81a\x08\x81V[``\x80\x82\x94\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91a\x0C-\x83a\x07\xD9V[\x01RV[\x90`\x80\x82\x82\x03\x12a\x01\xD5Wa\x023\x91a\x0B\xECV[\x90`@Qa\x0CR\x81a\x08\x81V[`\0\x90\x81\x81R\x81``` \x92\x82\x84\x82\x01R\x82`@\x82\x01R\x01R\x81`\x01\x80`\xA0\x1B\x03\x81T\x16\x94`$`@Q\x80\x97\x81\x93c\xDC\x17\x83U`\xE0\x1B\x83R`\x04\x83\x01RZ\xFA\x91\x82\x15a\x056W\x80\x92a\x0C\xB3W[Pa\x023\x92\x93P\x80\x82Q\x83\x01\x01\x91\x01a\x0C1V[\x90\x91P=\x80\x82\x86>a\x0C\xC5\x81\x86a\x08\xBEV[\x84\x01\x90\x82\x85\x83\x03\x12a\r;W\x84Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x95\x86\x82\x11a\r>W\x01\x90\x82`\x1F\x83\x01\x12\x15a\r;W\x81Q\x95\x86\x11a\x08\x9DW`@Q\x92a\r\x11`\x1F\x88\x01`\x1F\x19\x16\x86\x01\x85a\x08\xBEV[\x86\x84R\x84\x87\x84\x01\x01\x11a\r;WPa\x023\x93\x94a\r3\x91\x84\x80\x85\x01\x91\x01a\x01\xDAV[\x90\x83\x92a\x0C\x9FV[\x80\xFD[\x82\x80\xFD[\x90\x81` \x91\x03\x12a\x01\xD5WQa\x023\x81a\x07\xD9V[\x90\x81``\x91\x03\x12a\x01\xD5W\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90`\x04` a\r\x8Ea\x04\x9Ba\x04\x9B`\0T`\x01\x80`\xA0\x1B\x03\x16\x90V[`@Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x92\x83\x91\x82\x90Z\xFA\x92\x83\x15a\x056Wa\r\xD9\x93``\x92`\0\x91a\x0E6W[P`@Q\x80\x80\x96\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x056W`\0\x80\x93`\0\x93a\r\xFFW[P\x92\x91\x90V[\x91\x93PPa\x0E%\x91P``=``\x11a\x0E/W[a\x0E\x1D\x81\x83a\x08\xBEV[\x81\x01\x90a\rWV[\x92\x90\x92\x918a\r\xF9V[P=a\x0E\x13V[a\x0EX\x91P` =` \x11a\x0E^W[a\x0EP\x81\x83a\x08\xBEV[\x81\x01\x90a\rBV[8a\r\xB8V[P=a\x0EFV[\x91a\t\xACa\x023\x93a\x0CEV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\neWV[\x92` a\x03\xEA\x84a\x0E\xAEa\x0E\xA6a\x03\xF3\x96\x97a\x0E\xB4\x99a \x87V[\x85Q\x90a\x13$V[\x95a \x87V[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\neW\x90V[\x90\x81R` \x80\x82\x01\x92\x90\x92R`@\x80\x82\x01\x93\x90\x93R``\x80\x82\x01\x94\x90\x94R\x84Q`\x80\x82\x01R\x90\x84\x01Q`\xA0\x82\x01R\x90\x83\x01Q`\xC0\x82\x01R\x91\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x82\x01Ra\x01\0\x01\x90V[V[\x90\x92\x91\x85Q` \x87\x01Q`@\x88\x01Qa\x0F6\x90a\n\xDEV[\x91a\x0FA\x87\x85a \x87V[a\x0FK\x82\x82a\x13$V[\x92a\x0FU\x91a\x13$V[\x89Q\x85\x89\x85\x81a\x0Fe\x85\x8Da \xCAV[\x90a\x0Fo\x91a \xCAV[\x90a\x0Fy\x91a \xCAV[\x92a\x0F\x83\x90a \xA9V[a\x0F\x8C\x90a\n\xF4V[\x90a\x0F\x96\x91a\nzV[\x90a\x0F\xA0\x91a \xCAV[a\x0F\xA9\x86a\n\xDEV[a\x0F\xB2\x91a \xCAV[\x92a\x0F\xBC\x8Aa\njV[\x90a\x0F\xC6\x90a\x10\xF0V[a\x0F\xCF\x91a\x13$V[\x91a\x0F\xD9\x90a \xA9V[a\x0F\xE2\x86a\n\xDEV[a\x0F\xEB\x91a \xCAV[a\x0F\xF5\x90\x89a\nzV[\x92a\x0F\xFF\x91a\x0B\x14V[\x91a\x10\t\x91a \xCAV[\x89Qa\x10\x14\x90a\n\xDEV[a\x10\x1D\x90a hV[a\x10&\x91a\x13$V[a\x10/\x91a \xCAV[\x91\x88Qa\x10;\x90a\n\xDEV[a\x10D\x88a\njV[\x92a\x10O\x89\x89a \xCAV[\x90a\x10Y\x91a \xCAV[\x91a\x10c\x86a \xA9V[\x90a\x10m\x90a\n\xDEV[a\x10v\x91a \xCAV[\x92a\x10\x80\x91a \xCAV[\x91a\x10\x8A\x91a\nzV[a\x10\x93\x91a \xCAV[\x90a\x10\x9D\x84a\x10\xF0V[\x91a\x10\xA7\x91a \x87V[a\x10\xB0\x91a\x11-V[`\0\x13a\x10\xE5Wa\x023\x95a\x10\xE0\x93a\x10\xD2\x92`@Q\x96\x87\x95` \x87\x01a\x0E\xCDV[\x03`\x1F\x19\x81\x01\x83R\x82a\x08\xBEV[a\x1A0V[PPPPPP`\0\x90V[`\x01`\xFF\x1B\x81\x14a\neW`\0\x03\x90V[\x90\x81a\x03\xE8\x01\x91\x82\x12`\x01\x16a\neWV[\x90\x81g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\neWV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\neWV[\x94\x93\x92\x90\x92\x84Q\x90` \x86\x01Q`@\x87\x01Qa\x11d\x90a\n\xDEV[\x92a\x11o\x87\x87a \x87V[a\x11y\x82\x82a\x13$V[\x92a\x11\x83\x91a\x13$V[\x88Q\x87\x89\x85\x81a\x11\x93\x85\x8Ca \xCAV[\x90a\x11\x9D\x91a \xCAV[\x90a\x11\xA7\x91a \xCAV[\x92a\x11\xB2\x90\x88a \xCAV[a\x11\xBC\x90\x88a\x0B\x14V[\x90a\x11\xC6\x91a\nzV[\x90a\x11\xD0\x91a \xCAV[a\x11\xD9\x87a\n\xDEV[a\x11\xE2\x91a \xCAV[\x92a\x11\xED\x8A\x87a\nzV[\x90a\x11\xF7\x90a\x10\xF0V[a\x12\0\x91a\x13$V[\x91a\x12\x0B\x90\x86a \xCAV[a\x12\x14\x87a\n\xDEV[a\x12\x1D\x91a \xCAV[a\x12'\x90\x88a\nzV[\x92a\x121\x91a\x0B\x14V[\x91a\x12;\x91a \xCAV[\x88Qa\x12F\x90a\n\xDEV[a\x12O\x90a hV[a\x12X\x91a\x13$V[a\x12a\x91a \xCAV[\x96Qa\x12l\x90a\n\xDEV[\x93a\x12w\x87\x84a\nzV[\x96a\x12\x81\x91a \xCAV[\x90a\x12\x8B\x91a \xCAV[\x93a\x12\x95\x91a \xCAV[\x90a\x12\x9F\x90a\n\xDEV[a\x12\xA8\x91a \xCAV[\x92a\x12\xB2\x91a \xCAV[\x91a\x12\xBC\x91a\nzV[a\x12\xC5\x91a \xCAV[\x91a\x12\xCF\x90a\x10\xF0V[\x91a\x12\xD9\x91a \x87V[a\x023\x91a\x11-V[a\x023\x92\x91` a\x12\xF8a\x03\xF3\x93\x85Q\x90a\x13$V[\x93\x01Q\x90a\x13$V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\neW\x81\x84\x05\x14\x90\x15\x17\x15a\neWV[a\x14\xD1a\x023\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x14\xDF\x93a\x13Z`\0\x82\x13a\x1D\xC8V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x13v\x82a!\nV[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\x13\x01V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1E\0V[a\x15\x03\x90a\x14\xFBa\x023\x94\x93` \x85\x01Q\x90a \x87V[\x92Q\x90a \x87V[\x90a \x87V[\x90\x92\x91\x85Q`@\x87\x01Qg\r\xE0\xB6\xB3\xA7d\0\0`\0\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\neWa\x15<\x83a\x11\x01V[a\x15E\x83a\x11\x13V[a\x15N\x91a\x13$V[\x90\x82a\x15Z\x85\x89a\x1F\xA9V[\x90a\x15d\x91a\x13$V[a\x15m\x81a\x1F\xC7V[\x92a\x15w\x83a\x11\x13V[a\x15\x81\x90\x85a\x1F\xF0V[a\x15\x8B\x90\x89a\x0ErV[\x91\x82\x91a\x15\x97\x88a\x11\x01V[a\x15\xA1\x90\x88a\x1F\xF0V[\x93a\x15\xAB\x91a\x1F\xF0V[a\x15\xB4\x87a\x1F\x8AV[a\x15\xBD\x91a\x13$V[\x92a\x15\xC7\x87a\x11\x13V[a\x15\xD1\x90\x8Ba\x1F\xF0V[\x91\x88a\x15\xDC\x89a\x1F\xC7V[\x90a\x15\xE6\x91a\x11-V[a\x15\xEF\x91a\x1F\xF0V[a\x15\xF8\x86a\x11\x13V[a\x16\x01\x91a\x1F\xF0V[\x92a\x16\x0B\x91a\x1F\xF0V[\x92a\x16\x16\x90\x89a\x1F\xF0V[\x91a\x16 \x91a\x0ErV[a\x16)\x91a\x1F\xF0V[a\x162\x91a\x11-V[\x92a\x16<\x85a\x11\x01V[a\x16E\x91a\x1F\xF0V[\x91a\x16O\x87a\x10\xF0V[\x91a\x16Y\x90a\x11\x13V[a\x16b\x91a\x1F\xF0V[a\x16k\x91a\x11-V[a\x16t\x91a\x1F\xF0V[a\x16}\x91a\x1F\xA9V[`\0\x13a\x10\xE5Wa\x023\x95a\x16\x9F\x93a\x10\xD2\x92`@Q\x96\x87\x95` \x87\x01a\x0E\xCDV[a\x1BUV[a\x16\xC4a\x023\x93\x92a\x16\xBEa\x16\xCB\x93` \x86\x01Q\x90a\x13$V[\x90a\x1DLV[\x91Qa\x1D|V[\x90a\x13$V[\x92\x91\x90a\x16\xE7a\x16\xE1\x82\x84a\x1DLV[\x85a\x1D V[\x93\x81\x03\x90\x81\x11a\neW\x92\x81\x03\x90\x81\x11a\neW\x90V[\x92\x91\x90a\x17\x0Ea\x16\xE1\x82\x84a\x1DLV[\x93\x81\x01\x80\x91\x11a\neW\x92\x81\x01\x80\x91\x11a\neW\x90V[\x92\x93\x94\x90\x91\x94`@\x82Q\x92\x01Q\x93g\r\xE0\xB6\xB3\xA7d\0\0`\0\x86\x82\x03\x96\x12\x81\x87\x12\x81\x16\x91\x87\x13\x90\x15\x16\x17a\neW\x82\x87\x94a\x17`\x86\x85a\x11-V[a\x17i\x83a\x11\x13V[a\x17r\x91a\x13$V[\x95a\x17|\x91a\x1F\xA9V[\x90a\x17\x86\x91a\x13$V[\x93a\x17\x91\x85\x84a\x1F\xF0V[\x94a\x17\x9B\x87a\x11\x13V[a\x17\xA5\x90\x87a\x1F\xF0V[a\x17\xAF\x90\x89a\x0ErV[\x92\x83\x92a\x17\xBC\x8B\x87a\x11-V[a\x17\xC6\x90\x88a\x1F\xF0V[\x94a\x17\xD0\x91a\x1F\xF0V[a\x17\xD9\x87a\x1F\x8AV[a\x17\xE2\x91a\x13$V[\x93a\x17\xEC\x87a\x11\x13V[a\x17\xF6\x90\x8Ba\x1F\xF0V[\x92\x8Ba\x18\x02\x89\x89a\x1F\xF0V[\x90a\x18\x0C\x91a\x11-V[a\x18\x15\x91a\x1F\xF0V[a\x18\x1E\x8Aa\x11\x13V[a\x18'\x91a\x1F\xF0V[\x93a\x181\x91a\x1F\xF0V[\x93a\x18;\x91a\x1F\xF0V[\x91a\x18E\x91a\x0ErV[a\x18N\x91a\x1F\xF0V[a\x18W\x91a\x11-V[\x95a\x18a\x91a\x11-V[a\x18j\x91a\x1F\xF0V[\x92a\x18t\x90a\x10\xF0V[\x91a\x18~\x90a\x11\x13V[a\x18\x87\x91a\x1F\xF0V[a\x18\x90\x91a\x11-V[a\x18\x99\x91a\x1F\xF0V[a\x023\x91a\x1F\xA9V[\x92\x91\x90\x83a\x18\xBDa\x18\xC2\x92a\x18\xBD` \x86\x01Q\x86Q\x90a \x87V[a \xCAV[\x90a\x18\xCE\x81\x83\x86a\x12\xE2V[\x93a\x18\xDB\x82\x86\x85\x84a\x0E\x8BV[\x85\x90`\0\x80\x82\x12\x15a\x19\xA4W[\x80\x82\x12a\x19\x86WPa\x19-a\x19z\x92a\x023\x96\x97\x98\x86\x93[a\x19\x14`@Q\x98\x89\x92\x8C\x8A` \x86\x01a \x1FV[\x03\x96a\x19(`\x1F\x19\x98\x89\x81\x01\x83R\x82a\x08\xBEV[a\x1C,V[\x81Q`@\x80\x84\x01Q``\x94\x85\x01Q\x82Q` \x81\x01\x98\x90\x98R\x91\x87\x01\x99\x90\x99R\x92\x85\x01\x91\x90\x91R`\x80\x84\x01R`\xA0\x83\x01\x95\x90\x95R`\x01`\x01`\xA0\x1B\x03\x90\x94\x16`\xC0\x82\x01R\x92\x83\x90`\xE0\x82\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x08\xBEV[\x96a\x19\x91\x91Pa \xEBV[\x95a\x19\x9E\x84\x88\x87\x86a\x0E\x8BV[\x90a\x18\xE8V[\x96\x91\x96[\x80\x82\x13a\x19\xC4WPa\x19-a\x023\x95\x96\x97a\x19z\x93\x86\x93a\x19\0V[\x96a\x19\xCF\x91Pa\x1D\x9EV[\x95a\x19\xDC\x84\x88\x87\x86a\x0E\x8BV[\x90a\x19\xA8V[` a\x19\xFBa\x023\x94\x93a\x16\xBEa\x16\xCB\x94\x86Q\x90a\x13$V[\x92\x01Qa\x1D|V[\x91\x90a\x01\0\x83\x82\x03\x12a\x01\xD5W\x82Q\x92` \x81\x01Q\x92a\x023`@\x83\x01Q\x93`\x80``\x85\x01Q\x94\x01a\x0B\xECV[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\x1B4Wa\x1AL\x81a!|V[a\x1AV\x85\x83a\"\xD5V[`\0a\x1Ab\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1As\x85\x96\x95a\x0B\x04V[`\x01\x94`\0\x91\x86\x80[a\x1A\x8DW[PPPPPPPP\x90PV[\x15a\x1A\xF0W[P\x85\x96\x97\x98P\x80\x91a\x1A\xAEa\x1A\xA8\x8B\x88a\nzV[`\x01\x1C\x90V[\x99a\x1A\xB9\x8B\x87a\"\xD5V[\x90\x83a\x1A\xC5\x87\x84a\x13\x01V[\x13a\x1A\xE4WPP\x89\x92[\x87a\x1A\xDA\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1A|V[\x8B\x97P\x90\x94P\x92a\x1A\xCFV[\x86\x10\x80a\x1B\nW[\x15a\x1B\x03W\x88a\x1A\x93V[\x80\x80a\x1A\x81V[Pa\x01\0\x82\x10a\x1A\xF8V[`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x92\x90\x92R`$\x82\x01R`D\x90\xFD[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81Ra\x03\xE8`\x04\x82\x01R`$\x81\x01\x85\x90R`D\x90\xFD[\x91\x90a\x03\xE8\x92`\0\x93`\0\x91\x83\x82\x11a\x1B4Wa\x1Bq\x81a\"\xF6V[a\x1B{\x85\x83a$AV[`\0a\x1B\x87\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1B\x98\x85\x96\x95a\x0B\x04V[`\x01\x94`\0\x91\x86\x80[a\x1B\xB1WPPPPPPPP\x90PV[\x15a\x1C\x0EW[P\x85\x96\x97\x98P\x80\x91a\x1B\xCCa\x1A\xA8\x8B\x88a\nzV[\x99a\x1B\xD7\x8B\x87a$AV[\x90\x83a\x1B\xE3\x87\x84a\x13\x01V[\x13a\x1C\x02WPP\x89\x92[\x87a\x1B\xF8\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1B\xA1V[\x8B\x97P\x90\x94P\x92a\x1B\xEDV[\x86\x10\x80a\x1C!W[\x15a\x1B\x03W\x88a\x1B\xB7V[Pa\x01\0\x82\x10a\x1C\x16V[`\0\x93\x92\x91\x84\x91\x83\x82\x11a\x1D\0Wa\x1CD\x82\x82a$bV[a\x1CN\x85\x83a$bV[`\0a\x1CZ\x82\x84a\x13\x01V[\x13a\x1B\x15WPa\x1Cl\x83\x86\x97\x96a\x0B\x14V[`\x01\x94`\0\x91\x86\x80[a\x1C\x85WPPPPPPPP\x90PV[\x15a\x1C\xE2W[P\x85\x96\x97\x98P\x80\x91a\x1C\xA0a\x1A\xA8\x8B\x88a\nzV[\x99a\x1C\xAB\x8B\x87a$bV[\x90\x83a\x1C\xB7\x87\x84a\x13\x01V[\x13a\x1C\xD6WPP\x89\x92[\x87a\x1C\xCC\x88\x86a\x0B\x14V[\x92\x01\x93\x99\x98a\x1CuV[\x8B\x97P\x90\x94P\x92a\x1C\xC1V[\x86\x10\x80a\x1C\xF5W[\x15a\x1B\x03W\x88a\x1C\x8BV[Pa\x01\0\x82\x10a\x1C\xEAV[`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x85\x90R`D\x90\xFD[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xD5W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x0F\xFF\xFF\xFF\xFF\x04`\x01\x01\x90V[a\x03\xE9\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5W`\x01a\x03\xE8`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x15a\x1D\xCFWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x1F\x84Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x1FPWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x82\x05\x14\x82\x15\x15\x16\x15a\x01\xD5W\x05\x90V[a\x03\xE8\x81\x81\x02\x91`\x01`\xFF\x1B\x81\x13`\x01\x17\x91\x83\x05\x14\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x15\x82\x84\x05\x82\x14\x17`\0\x19\x90\x92\x10`\x01`\xFF\x1B\x90\x91\x13\x17\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x0F\x1C\x93``\x92\x96\x95\x93`\xE0\x83\x01\x97\x83R` \x83\x01R`@\x82\x01R\x01\x90``\x90\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[`\x01\x81\x15\x15\x16\x15a\x01\xD5Wn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xD5W\x04\x90V[a\x03\xE8\x90\x80\x82\x02\x91\x82\x04\x14`\x01\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[a\x03\xE7\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xD5Wa\x03\xE8\x90\x04\x90V[a!\x15\x81\x15\x15a\x1D\xC8V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[\x80Q\x81\x01` \x01\x90` \x01\x90a!\x91\x91a\x1A\x03V[\x92\x91\x90\x83Q` \x85\x01Q`@\x86\x01Qa!\xA9\x90a\n\xDEV[\x91a!\xB4\x86\x86a \x87V[a!\xBE\x82\x82a\x13$V[\x92a!\xC8\x91a\x13$V[\x87Q\x86\x88\x85\x81a!\xD8\x85\x8Ba \xCAV[\x90a!\xE2\x91a \xCAV[\x90a!\xEC\x91a \xCAV[\x92a!\xF6\x90a \xA9V[a!\xFF\x90a\n\xF4V[\x90a\"\t\x91a\nzV[\x90a\"\x13\x91a \xCAV[a\"\x1C\x86a\n\xDEV[a\"%\x91a \xCAV[\x92a\"/\x89a\njV[\x90a\"9\x90a\x10\xF0V[a\"B\x91a\x13$V[\x91a\"L\x90a \xA9V[a\"U\x86a\n\xDEV[a\"^\x91a \xCAV[a\"h\x90\x87a\nzV[\x92a\"r\x91a\x0B\x14V[\x91a\"|\x91a \xCAV[\x87Qa\"\x87\x90a\n\xDEV[a\"\x90\x90a hV[a\"\x99\x91a\x13$V[a\"\xA2\x91a \xCAV[\x95Qa\"\xAD\x90a\n\xDEV[\x92a\"\xB7\x86a\njV[\x95a\"\xC1\x91a \xCAV[\x90a\"\xCB\x91a \xCAV[\x92a\x12\x95\x90a \xA9V[\x90a\"\xECa\x023\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[\x94\x93\x92\x90\x92a\x11IV[a#\t\x90` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[`@\x81\x95\x93\x95\x94\x92\x94Q\x91\x01Q\x92g\r\xE0\xB6\xB3\xA7d\0\0`\0\x85\x82\x03\x95\x12\x81\x86\x12\x81\x16\x91\x86\x13\x90\x15\x16\x17a\neW\x81\x86\x93a#C\x85a\x11\x01V[a#L\x83a\x11\x13V[a#U\x91a\x13$V[\x94a#_\x91a\x1F\xA9V[\x90a#i\x91a\x13$V[\x92a#s\x84a\x1F\xC7V[\x93a#}\x86a\x11\x13V[a#\x87\x90\x86a\x1F\xF0V[a#\x91\x90\x88a\x0ErV[\x92\x83\x92a#\x9D\x8Aa\x11\x01V[a#\xA7\x90\x87a\x1F\xF0V[\x94a#\xB1\x91a\x1F\xF0V[a#\xBA\x86a\x1F\x8AV[a#\xC3\x91a\x13$V[\x93a#\xCD\x86a\x11\x13V[a#\xD7\x90\x8Aa\x1F\xF0V[\x92\x8Aa#\xE2\x88a\x1F\xC7V[\x90a#\xEC\x91a\x11-V[a#\xF5\x91a\x1F\xF0V[a#\xFE\x89a\x11\x13V[a$\x07\x91a\x1F\xF0V[\x93a$\x11\x91a\x1F\xF0V[\x93a$\x1B\x91a\x1F\xF0V[\x91a$%\x91a\x0ErV[a$.\x91a\x1F\xF0V[a$7\x91a\x11-V[\x94a\x18a\x90a\x11\x01V[\x90a$Xa\x023\x92` \x80\x82Q\x83\x01\x01\x91\x01a\x1A\x03V[\x94\x93\x92\x90\x92a\x17%V[\x80Q\x81\x01\x91`\xE0\x82\x84\x03\x12a\x01\xD5Wa\x023\x92a$\x90` \x84\x01Q\x93`\x80` `@\x83\x01Q\x94\x01\x91\x01a\x0B\xECV[\x92a\x0E\x8BV\xFE\xA2dipfsX\"\x12 \xD1\x08\xA55\x03\x8Et\xC2T\x7F~\xE2\xF2 \xEA\\\xDFX\xC4 JC\x8D\x97\xDE\x94\x05I\xC5\xA5\x12\xA1dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeometricMeanSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMeanSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMeanSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMeanSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMeanSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMeanSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMeanSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEOMETRICMEANSOLVER_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
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
                GEOMETRICMEANSOLVER_ABI.clone(),
                GEOMETRICMEANSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allocateGivenX` (0xee3e8cfb) function
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
        ///Calls the contract's `allocateGivenY` (0x7f17409c) function
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
        ///Calls the contract's `calculateDiffLower` (0x332266f3) function
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
        ///Calls the contract's `calculateDiffRaise` (0x902ecaa2) function
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
        ///Calls the contract's `checkSwapConstant` (0x0f4166b8) function
        pub fn check_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([15, 65, 102, 184], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeOptimalArbLowerPrice` (0x306db46b) function
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
        ///Calls the contract's `computeOptimalArbRaisePrice` (0x4fd67c58) function
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
        ///Calls the contract's `deallocateGivenX` (0x6237569f) function
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
        ///Calls the contract's `deallocateGivenY` (0xf30d37f2) function
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
        ///Calls the contract's `fetchPoolParams` (0x81b5fac2) function
        pub fn fetch_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GeometricMeanParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInitialPoolData` (0xdef15f92) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: GeometricMeanParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([222, 241, 95, 146], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextLiquidity` (0xec29d8e6) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([236, 41, 216, 230], (pool_id, rx, ry))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveX` (0x5a93b8ce) function
        pub fn get_next_reserve_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([90, 147, 184, 206], (pool_id, ry, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNextReserveY` (0xf2de7a7b) function
        pub fn get_next_reserve_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            rx: ::ethers::core::types::U256,
            l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([242, 222, 122, 123], (pool_id, rx, l))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
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
        ///Calls the contract's `internalPrice` (0x3b4d1030) function
        pub fn internal_price(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 77, 16, 48], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareControllerUpdate` (0xcb1f5532) function
        pub fn prepare_controller_update(
            &self,
            controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([203, 31, 85, 50], controller)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareFeeUpdate` (0xb09d04e5) function
        pub fn prepare_fee_update(
            &self,
            swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([176, 157, 4, 229], swap_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prepareWeightXUpdate` (0x250968d9) function
        pub fn prepare_weight_x_update(
            &self,
            target_weight_x: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([37, 9, 104, 217], (target_weight_x, target_timestamp))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `simulateSwap` (0x3928ff97) function
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
        ///Calls the contract's `strategy` (0xa8c62e76) function
        pub fn strategy(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([168, 198, 46, 118], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GeometricMeanSolver<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BisectionLib_InvalidBounds` with signature `BisectionLib_InvalidBounds(uint256,uint256)` and selector `0x6105bfb6`
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
        Hash
    )]
    #[etherror(
        name = "BisectionLib_InvalidBounds",
        abi = "BisectionLib_InvalidBounds(uint256,uint256)"
    )]
    pub struct BisectionLib_InvalidBounds {
        pub lower: ::ethers::core::types::U256,
        pub upper: ::ethers::core::types::U256,
    }
    ///Custom Error type `BisectionLib_RootOutsideBounds` with signature `BisectionLib_RootOutsideBounds(int256,int256)` and selector `0x1bc6f974`
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
        Hash
    )]
    #[etherror(
        name = "BisectionLib_RootOutsideBounds",
        abi = "BisectionLib_RootOutsideBounds(int256,int256)"
    )]
    pub struct BisectionLib_RootOutsideBounds {
        pub lower_result: ::ethers::core::types::I256,
        pub upper_result: ::ethers::core::types::I256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum GeometricMeanSolverErrors {
        BisectionLib_InvalidBounds(BisectionLib_InvalidBounds),
        BisectionLib_RootOutsideBounds(BisectionLib_RootOutsideBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanSolverErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BisectionLib_InvalidBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_InvalidBounds(decoded));
            }
            if let Ok(decoded) = <BisectionLib_RootOutsideBounds as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BisectionLib_RootOutsideBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeometricMeanSolverErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GeometricMeanSolverErrors {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanSolverErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BisectionLib_InvalidBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BisectionLib_RootOutsideBounds(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GeometricMeanSolverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds>
    for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds>
    for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    ///Container type for all input parameters for the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
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
        Hash
    )]
    #[ethcall(name = "allocateGivenX", abi = "allocateGivenX(uint256,uint256)")]
    pub struct AllocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
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
        Hash
    )]
    #[ethcall(name = "allocateGivenY", abi = "allocateGivenY(uint256,uint256)")]
    pub struct AllocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `calculateDiffLower` function with signature `calculateDiffLower(uint256,uint256,uint256)` and selector `0x332266f3`
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
        Hash
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
    ///Container type for all input parameters for the `calculateDiffRaise` function with signature `calculateDiffRaise(uint256,uint256,uint256)` and selector `0x902ecaa2`
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
        Hash
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
    ///Container type for all input parameters for the `checkSwapConstant` function with signature `checkSwapConstant(uint256,bytes)` and selector `0x0f4166b8`
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
        Hash
    )]
    #[ethcall(name = "checkSwapConstant", abi = "checkSwapConstant(uint256,bytes)")]
    pub struct CheckSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `computeOptimalArbLowerPrice` function with signature `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector `0x306db46b`
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
        Hash
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
    ///Container type for all input parameters for the `computeOptimalArbRaisePrice` function with signature `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector `0x4fd67c58`
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
        Hash
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
    ///Container type for all input parameters for the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
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
        Hash
    )]
    #[ethcall(name = "deallocateGivenX", abi = "deallocateGivenX(uint256,uint256)")]
    pub struct DeallocateGivenXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
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
        Hash
    )]
    #[ethcall(name = "deallocateGivenY", abi = "deallocateGivenY(uint256,uint256)")]
    pub struct DeallocateGivenYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
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
        Hash
    )]
    #[ethcall(name = "fetchPoolParams", abi = "fetchPoolParams(uint256)")]
    pub struct FetchPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))` and selector `0xdef15f92`
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
        Hash
    )]
    #[ethcall(
        name = "getInitialPoolData",
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub rx: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
        pub params: GeometricMeanParams,
    }
    ///Container type for all input parameters for the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
        Hash
    )]
    #[ethcall(
        name = "getNextLiquidity",
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256)` and selector `0x5a93b8ce`
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
        Hash
    )]
    #[ethcall(
        name = "getNextReserveX",
        abi = "getNextReserveX(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256)` and selector `0xf2de7a7b`
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
        Hash
    )]
    #[ethcall(
        name = "getNextReserveY",
        abi = "getNextReserveY(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
        Hash
    )]
    #[ethcall(
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
        Hash
    )]
    #[ethcall(name = "internalPrice", abi = "internalPrice(uint256)")]
    pub struct InternalPriceCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
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
        Hash
    )]
    #[ethcall(
        name = "prepareControllerUpdate",
        abi = "prepareControllerUpdate(address)"
    )]
    pub struct PrepareControllerUpdateCall {
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
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
        Hash
    )]
    #[ethcall(name = "prepareFeeUpdate", abi = "prepareFeeUpdate(uint256)")]
    pub struct PrepareFeeUpdateCall {
        pub swap_fee: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
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
        Hash
    )]
    #[ethcall(
        name = "prepareWeightXUpdate",
        abi = "prepareWeightXUpdate(uint256,uint256)"
    )]
    pub struct PrepareWeightXUpdateCall {
        pub target_weight_x: ::ethers::core::types::U256,
        pub target_timestamp: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
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
        Hash
    )]
    #[ethcall(name = "simulateSwap", abi = "simulateSwap(uint256,bool,uint256)")]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub swap_x_in: bool,
        pub amount_in: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
        Hash
    )]
    #[ethcall(name = "strategy", abi = "strategy()")]
    pub struct StrategyCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum GeometricMeanSolverCalls {
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        CheckSwapConstant(CheckSwapConstantCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenY(DeallocateGivenYCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareWeightXUpdate(PrepareWeightXUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllocateGivenX(decoded));
            }
            if let Ok(decoded) = <AllocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllocateGivenY(decoded));
            }
            if let Ok(decoded) = <CalculateDiffLowerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDiffLower(decoded));
            }
            if let Ok(decoded) = <CalculateDiffRaiseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CalculateDiffRaise(decoded));
            }
            if let Ok(decoded) = <CheckSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSwapConstant(decoded));
            }
            if let Ok(decoded) = <ComputeOptimalArbLowerPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOptimalArbLowerPrice(decoded));
            }
            if let Ok(decoded) = <ComputeOptimalArbRaisePriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeOptimalArbRaisePrice(decoded));
            }
            if let Ok(decoded) = <DeallocateGivenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeallocateGivenX(decoded));
            }
            if let Ok(decoded) = <DeallocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeallocateGivenY(decoded));
            }
            if let Ok(decoded) = <FetchPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FetchPoolParams(decoded));
            }
            if let Ok(decoded) = <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInitialPoolData(decoded));
            }
            if let Ok(decoded) = <GetNextLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextLiquidity(decoded));
            }
            if let Ok(decoded) = <GetNextReserveXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveX(decoded));
            }
            if let Ok(decoded) = <GetNextReserveYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNextReserveY(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InternalPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalPrice(decoded));
            }
            if let Ok(decoded) = <PrepareControllerUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareControllerUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareFeeUpdate(decoded));
            }
            if let Ok(decoded) = <PrepareWeightXUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PrepareWeightXUpdate(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <StrategyCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Strategy(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeometricMeanSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllocateGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AllocateGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDiffLower(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CalculateDiffRaise(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CheckSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FetchPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareWeightXUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Strategy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffLower(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CalculateDiffRaise(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CheckSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightXUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateGivenXCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenXCall) -> Self {
            Self::AllocateGivenX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenYCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenYCall) -> Self {
            Self::AllocateGivenY(value)
        }
    }
    impl ::core::convert::From<CalculateDiffLowerCall> for GeometricMeanSolverCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for GeometricMeanSolverCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<CheckSwapConstantCall> for GeometricMeanSolverCalls {
        fn from(value: CheckSwapConstantCall) -> Self {
            Self::CheckSwapConstant(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall>
    for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall>
    for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<FetchPoolParamsCall> for GeometricMeanSolverCalls {
        fn from(value: FetchPoolParamsCall) -> Self {
            Self::FetchPoolParams(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for GeometricMeanSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetNextReserveXCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextReserveXCall) -> Self {
            Self::GetNextReserveX(value)
        }
    }
    impl ::core::convert::From<GetNextReserveYCall> for GeometricMeanSolverCalls {
        fn from(value: GetNextReserveYCall) -> Self {
            Self::GetNextReserveY(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall>
    for GeometricMeanSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for GeometricMeanSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall>
    for GeometricMeanSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWeightXUpdateCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareWeightXUpdateCall) -> Self {
            Self::PrepareWeightXUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for GeometricMeanSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for GeometricMeanSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    ///Container type for all return fields from the `allocateGivenX` function with signature `allocateGivenX(uint256,uint256)` and selector `0xee3e8cfb`
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
        Hash
    )]
    pub struct AllocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `allocateGivenY` function with signature `allocateGivenY(uint256,uint256)` and selector `0x7f17409c`
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
        Hash
    )]
    pub struct AllocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `calculateDiffLower` function with signature `calculateDiffLower(uint256,uint256,uint256)` and selector `0x332266f3`
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
        Hash
    )]
    pub struct CalculateDiffLowerReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `calculateDiffRaise` function with signature `calculateDiffRaise(uint256,uint256,uint256)` and selector `0x902ecaa2`
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
        Hash
    )]
    pub struct CalculateDiffRaiseReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `checkSwapConstant` function with signature `checkSwapConstant(uint256,bytes)` and selector `0x0f4166b8`
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
        Hash
    )]
    pub struct CheckSwapConstantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `computeOptimalArbLowerPrice` function with signature `computeOptimalArbLowerPrice(uint256,uint256,uint256)` and selector `0x306db46b`
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
        Hash
    )]
    pub struct ComputeOptimalArbLowerPriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `computeOptimalArbRaisePrice` function with signature `computeOptimalArbRaisePrice(uint256,uint256,uint256)` and selector `0x4fd67c58`
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
        Hash
    )]
    pub struct ComputeOptimalArbRaisePriceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `deallocateGivenX` function with signature `deallocateGivenX(uint256,uint256)` and selector `0x6237569f`
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
        Hash
    )]
    pub struct DeallocateGivenXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocateGivenY` function with signature `deallocateGivenY(uint256,uint256)` and selector `0xf30d37f2`
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
        Hash
    )]
    pub struct DeallocateGivenYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `fetchPoolParams` function with signature `fetchPoolParams(uint256)` and selector `0x81b5fac2`
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
        Hash
    )]
    pub struct FetchPoolParamsReturn(pub GeometricMeanParams);
    ///Container type for all return fields from the `getInitialPoolData` function with signature `getInitialPoolData(uint256,uint256,(uint256,uint256,uint256,address))` and selector `0xdef15f92`
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
        Hash
    )]
    pub struct GetInitialPoolDataReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getNextLiquidity` function with signature `getNextLiquidity(uint256,uint256,uint256)` and selector `0xec29d8e6`
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
        Hash
    )]
    pub struct GetNextLiquidityReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveX` function with signature `getNextReserveX(uint256,uint256,uint256)` and selector `0x5a93b8ce`
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
        Hash
    )]
    pub struct GetNextReserveXReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getNextReserveY` function with signature `getNextReserveY(uint256,uint256,uint256)` and selector `0xf2de7a7b`
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
        Hash
    )]
    pub struct GetNextReserveYReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
        Hash
    )]
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `internalPrice` function with signature `internalPrice(uint256)` and selector `0x3b4d1030`
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
        Hash
    )]
    pub struct InternalPriceReturn {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `prepareControllerUpdate` function with signature `prepareControllerUpdate(address)` and selector `0xcb1f5532`
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
        Hash
    )]
    pub struct PrepareControllerUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `prepareFeeUpdate` function with signature `prepareFeeUpdate(uint256)` and selector `0xb09d04e5`
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
        Hash
    )]
    pub struct PrepareFeeUpdateReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all return fields from the `prepareWeightXUpdate` function with signature `prepareWeightXUpdate(uint256,uint256)` and selector `0x250968d9`
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
        Hash
    )]
    pub struct PrepareWeightXUpdateReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `simulateSwap` function with signature `simulateSwap(uint256,bool,uint256)` and selector `0x3928ff97`
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
        Hash
    )]
    pub struct SimulateSwapReturn(
        pub bool,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all return fields from the `strategy` function with signature `strategy()` and selector `0xa8c62e76`
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
        Hash
    )]
    pub struct StrategyReturn(pub ::ethers::core::types::Address);
    ///`GeometricMeanParams(uint256,uint256,uint256,address)`
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
        Hash
    )]
    pub struct GeometricMeanParams {
        pub w_x: ::ethers::core::types::U256,
        pub w_y: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}

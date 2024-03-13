pub use geometric_mean_solver::*;
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
pub mod geometric_mean_solver {
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
                    ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("checkSwapConstant"),
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
                    ::std::borrow::ToOwned::to_owned("deallocateGivenXReturnDeltas"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenXReturnDeltas",),
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
                    ::std::borrow::ToOwned::to_owned("deallocateGivenYReturnDeltas"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocateGivenYReturnDeltas",),
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
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct GeometricMeanParams",),
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
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct GeometricMeanParams",),
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
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct GeometricMeanParams",),
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
                            name: ::std::borrow::ToOwned::to_owned("data"),
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
                    ::std::borrow::ToOwned::to_owned("prepareWeightXUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareWeightXUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetWeightX"),
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
            ]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static GEOMETRICMEANSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0(*8\x03\x80b\0(*\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0ZV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\0\x8CV[`\0` \x82\x84\x03\x12\x15b\0\0mW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x85W`\0\x80\xFD[\x93\x92PPPV[a'\x8E\x80b\0\0\x9C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80c\x90.\xCA\xA2\x11a\0\xDEW\x80c\xDE^\xE1\xC3\x11a\0\x97W\x80c\xEC)\xD8\xE6\x11a\0qW\x80c\xEC)\xD8\xE6\x14a\x03\xA7W\x80c\xEE>\x8C\xFB\x14a\x03\xBAW\x80c\xF2\xDEz{\x14a\x03\xCDW\x80c\xF3\r7\xF2\x14a\x03\xE0W`\0\x80\xFD[\x80c\xDE^\xE1\xC3\x14a\x03nW\x80c\xDE\xF1_\x92\x14a\x03\x81W\x80c\xE2i\xADc\x14a\x03\x94W`\0\x80\xFD[\x80c\x90.\xCA\xA2\x14a\x02\xF7W\x80c\xA8\xC6.v\x14a\x03\nW\x80c\xB0\x9D\x04\xE5\x14a\x035W\x80c\xCB\x1FU2\x14a\x03HW\x80c\xCE\x15;\xF4\x14a\x03[W\x80c\xDC\x17\x83U\x14a\x02\xD7W`\0\x80\xFD[\x80cO\xD6|X\x11a\x01KW\x80cb7V\x9F\x11a\x01%W\x80cb7V\x9F\x14a\x02\x89W\x80c}\xDA\x1A#\x14a\x02\x9CW\x80c\x7F\x17@\x9C\x14a\x02\xC4W\x80c\x81\xB5\xFA\xC2\x14a\x02\xD7W`\0\x80\xFD[\x80cO\xD6|X\x14a\x025W\x80cZ\x93\xB8\xCE\x14a\x02HW\x80cZ\x9C\xA5S\x14a\x02[W`\0\x80\xFD[\x80c\x0FAf\xB8\x14a\x01\x93W\x80c%\th\xD9\x14a\x01\xB9W\x80c0m\xB4k\x14a\x01\xD9W\x80c3\"f\xF3\x14a\x01\xECW\x80c9(\xFF\x97\x14a\x01\xFFW\x80c;M\x100\x14a\x02\"W[`\0\x80\xFD[a\x01\xA6a\x01\xA16`\x04a\x1F\xD1V[a\x03\xF3V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCCa\x01\xC76`\x04a MV[a\x04/V[`@Qa\x01\xB0\x91\x90a \xBFV[a\x01\xA6a\x01\xE76`\x04a \xD2V[a\x04DV[a\x01\xA6a\x01\xFA6`\x04a \xD2V[a\x04tV[a\x02\x12a\x02\r6`\x04a!\x0FV[a\x04\xA4V[`@Qa\x01\xB0\x94\x93\x92\x91\x90a!GV[a\x01\xA6a\x0206`\x04a!nV[a\x08\xDDV[a\x01\xA6a\x02C6`\x04a \xD2V[a\t\x10V[a\x01\xA6a\x02V6`\x04a \xD2V[a\t@V[a\x02na\x02i6`\x04a MV[a\t]V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xB0V[a\x02na\x02\x976`\x04a MV[a\t\xC9V[a\x02\xAFa\x02\xAA6`\x04a MV[a\n\x12V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xB0V[a\x02na\x02\xD26`\x04a MV[a\nnV[a\x02\xEAa\x02\xE56`\x04a!nV[a\n\xB6V[`@Qa\x01\xB0\x91\x90a!\xB3V[a\x01\xA6a\x03\x056`\x04a \xD2V[a\x0BnV[`\0Ta\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xB0V[a\x01\xCCa\x03C6`\x04a!nV[a\x0B\x9EV[a\x01\xCCa\x03V6`\x04a!\xD6V[a\x0B\xA9V[a\x02na\x03i6`\x04a!nV[a\x0B\xB4V[a\x02\xAFa\x03|6`\x04a MV[a\x0C\xAAV[a\x01\xCCa\x03\x8F6`\x04a\"9V[a\x0C\xF6V[a\x02na\x03\xA26`\x04a MV[a\r\x03V[a\x01\xA6a\x03\xB56`\x04a \xD2V[a\rRV[a\x02na\x03\xC86`\x04a MV[a\rbV[a\x01\xA6a\x03\xDB6`\x04a \xD2V[a\r\x88V[a\x02na\x03\xEE6`\x04a MV[a\r\x9DV[`\0\x80\x80\x80a\x04\x04\x85\x87\x01\x87a \xD2V[\x92P\x92P\x92P`\0a\x04\x15\x88a\n\xB6V[\x90Pa\x04#\x84\x84\x84\x84a\r\xC3V[\x98\x97PPPPPPPPV[``a\x04;\x83\x83a\x0E!V[\x90P[\x92\x91PPV[`\0\x80a\x04P\x85a\n\xB6V[\x90P`\0\x80`\0a\x04`\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0EPV[`\0\x80a\x04\x80\x85a\n\xB6V[\x90P`\0\x80`\0a\x04\x90\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0E\xC6V[`\0\x80`\0``a\x04\xCF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xF3`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xFC\x89a\x0B\xB4V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x13\x8Aa\n\xB6V[\x90P`\0\x80a\x053\x85`\0\x01Q\x86` \x01Qa\x05.\x8Fa\n\xB6V[a\x0E\xFBV[\x90P\x8A\x15a\x06qW`\0a\x05T\x84`@\x01Q\x8Ca\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05\x81\x85` \x01Qa\x05{\x89`\0\x01Q\x8A` \x01Qa\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0FPV[\x90P`\0a\x05\x8F\x83\x83a\x0F&V[\x90Pa\x05\x9C`\x01\x82a\"\xC5V[\x88Q\x90\x91Pa\x05\xAC\x90\x8E\x90a\"\xC5V[\x87Ra\x05\xB8\x81\x85a\"\xC5V[\x87`@\x01\x81\x81RPPa\x05\xD4\x8F\x88`\0\x01Q\x89`@\x01Qa\r\x88V[` \x88\x01\x81\x81R`\x01\x91a\x05\xE9\x90\x83\x90a\"\xC5V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06g\x91\x90a\"\xD8V[\x94PPPPa\x07\x86V[`\0a\x06\x8A\x84`@\x01Q\x8Ca\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q` \x88\x01Q\x88Q\x92\x93P`\0\x92a\x06\xA8\x92\x91a\x05{\x91\x90a\x0F;V[\x90P`\0a\x06\xB6\x83\x83a\x0F&V[\x90Pa\x06\xC3`\x01\x82a\"\xC5V[\x90P\x8C\x88` \x01Qa\x06\xD5\x91\x90a\"\xC5V[` \x88\x01Ra\x06\xE4\x81\x85a\"\xC5V[\x87`@\x01\x81\x81RPPa\x07\0\x8F\x88` \x01Q\x89`@\x01Qa\t@V[\x80\x88R`\x01\x90\x88\x90a\x07\x13\x90\x83\x90a\"\xC5V[\x90RP\x87Q\x87Q\x10a\x07rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06JV[\x86Q\x88Qa\x07\x80\x91\x90a\"\xD8V[\x94PPPP[\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\xE0\x83\x01\x82R`\0\x80\x84R` \x84\x01\x81\x90R\x91\x83\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x91\x90\x91R\x91P\x86`\0\x01Q\x81``\x01\x81\x81RPP\x86` \x01Q\x81`\x80\x01\x81\x81RPP\x82\x81`\xA0\x01\x81\x81RPP`\0\x8E\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ce\xC9\xFF\xC20\x84\x86\x88`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08e\x94\x93\x92\x91\x90a\"\xEBV[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA6\x91\x90a#bV[PPPPP\x90P\x80\x86a\x08\xC2\x8A`\0\x01Q\x8B` \x01Q\x8Ba\x0F\x81V[\x86\x9CP\x9CP\x9CP\x9CPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x08\xE9\x83a\n\xB6V[\x90P`\0\x80a\x08\xF7\x85a\x0B\xB4V[P\x91P\x91Pa\t\x07\x82\x82\x85a\x0F\x81V[\x95\x94PPPPPV[`\0\x80a\t\x1C\x85a\n\xB6V[\x90P`\0\x80`\0a\t,\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0F\xBAV[`\0a\tU\x83\x83a\tP\x87a\n\xB6V[a\x10 V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80a\to\x88a\x0B\xB4V[\x92P\x92P\x92P`\0\x80a\t\x85`\0\x8A\x86\x86a\x10]V[\x91P\x91P`\0a\t\x96\x8B\x84\x84a\t@V[\x90Pa\t\xA2\x81\x87a\"\xD8V[a\t\xAC\x84\x87a\"\xD8V[a\t\xB6\x84\x87a\"\xD8V[\x98P\x98P\x98PPPPPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\xDA\x87a\x0B\xB4V[\x92PP\x91P`\0\x80a\t\xEF`\0\x89\x86\x86a\x10]V[\x91P\x91P`\0a\n\0\x8A\x84\x84a\r\x88V[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80a\n!\x86a\x0B\xB4V[P\x91P\x91P`\0a\n1\x87a\n\xB6V[\x90P`\0a\n@\x84\x84\x84a\x0F\x81V[\x90P`\0a\nO\x88\x83\x85a\x10\xC6V[\x90P`\0a\n^\x89\x84\x86a\x11\x0FV[\x9A\x91\x99P\x90\x97PPPPPPPPV[`\0\x80`\0\x80`\0a\n\x7F\x87a\x0B\xB4V[\x92P\x92PP`\0\x80a\n\x94`\x01\x89\x86\x86a\x10]V[\x91P\x91P`\0a\n\xA5\x8A\x84\x84a\t@V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[a\n\xEA`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B[\x91\x90\x81\x01\x90a#\xB5V[\x80` \x01\x90Q\x81\x01\x90a\x04>\x91\x90a$\xA4V[`\0\x80a\x0Bz\x85a\n\xB6V[\x90P`\0\x80`\0a\x0B\x8A\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x11:V[``a\x04>\x82a\x11\xDEV[``a\x04>\x82a\x12\nV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C/\x91\x90a$\xC0V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\\\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CyW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9D\x91\x90a$\xDDV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80a\x0C\xB9\x86a\x0B\xB4V[P\x91P\x91P`\0a\x0C\xC9\x87a\n\xB6V[\x90P`\0a\x0C\xD8\x84\x84\x84a\x0F\x81V[\x90P`\0a\x0C\xE7\x88\x83\x85a\x12 V[\x90P`\0a\n^\x89\x84\x86a\x12OV[``a\tU\x84\x84\x84a\x12wV[`\0\x80`\0\x80`\0\x80a\r\x15\x88a\x0B\xB4V[\x92P\x92P\x92P`\0\x80a\r+`\0\x8A\x87\x86a\x10]V[\x91P\x91P`\0a\r<\x8B\x84\x84a\r\x88V[\x90Pa\rH\x83\x87a\"\xD8V[a\t\xAC\x82\x87a\"\xD8V[`\0a\tU\x83\x83a\x05.\x87a\n\xB6V[`\0\x80`\0\x80`\0a\rs\x87a\x0B\xB4V[\x92PP\x91P`\0\x80a\t\xEF`\x01\x89\x86\x86a\x10]V[`\0a\tU\x83\x83a\r\x98\x87a\n\xB6V[a\x13\x15V[`\0\x80`\0\x80`\0a\r\xAE\x87a\x0B\xB4V[\x92P\x92PP`\0\x80a\n\x94`\0\x89\x86\x86a\x10]V[\x80Q`\0\x90\x81\x90a\r\xD8\x90a\x05{\x88\x87a\x13JV[\x90P`\0a\r\xF7\x84` \x01Qa\x05{\x87\x89a\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x0C\x83\x83a\x0F&V[a\x0E\x16\x91\x90a%\x0BV[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\x0E9\x93\x92\x91\x90a%TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x82a\x03\xE8\x82a\x0Ee\x8A\x8A\x8A\x8A\x86\x8Aa\x0E\xC6V[\x90P`\0\x81\x12\x15a\x0E|W`\0\x93PPPPa\x0E\xBCV[a\x0E\xB6\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x0E\x98\x95\x94\x93\x92\x91\x90a%sV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x13_a\x13\x94V[\x93PPPP[\x96\x95PPPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xE1\x91\x90a\"\xD8V[\x90Pa\x04#\x83`\0\x01Q\x84` \x01Q\x89\x89\x8C\x8A\x8A\x88a\x14\xA5V[`\0a\tUa\x0F\x17\x83` \x01Q\x85a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x0F$\x90\x87\x90a\x0FPV[\x90[`\0a\x04;\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15TV[`\0a\x04;\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15TV[`\0a\x04;g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0Fh\x86a\x15\x82V[a\x0Fr\x91\x90a%\x9EV[a\x0F|\x91\x90a%\xE4V[a\x17]V[`\0\x80a\x0F\x9B\x83` \x01Q\x85a\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\x0F\xAE\x90\x87\x90a\x13JV[\x90Pa\x0E\xBC\x82\x82a\x13JV[`\0\x82a\x03\xE8\x82a\x0F\xCF\x8A\x8A\x8A\x8A\x86\x8Aa\x11:V[\x90P`\0\x81\x12\x15a\x0F\xE6W`\0\x93PPPPa\x0E\xBCV[a\x0E\xB6\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x10\x02\x95\x94\x93\x92\x91\x90a%sV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x19\x06a\x13\x94V[\x80Q`\0\x90a\tU\x90a\x10<\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0F;V[a\x05{a\x10V\x85` \x01Q\x88a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0F;V[`\0\x80\x80a\x10k\x84\x86a\x0F;V[\x90P`\0a\x10y\x87\x83a\x0F&V[\x90P\x87a\x10\x8FWa\x10\x8A\x87\x87a\"\xD8V[a\x10\x99V[a\x10\x99\x87\x87a\"\xC5V[\x93P\x87a\x10\xAFWa\x10\xAA\x81\x86a\"\xD8V[a\x10\xB9V[a\x10\xB9\x81\x86a\"\xC5V[\x92PPP\x94P\x94\x92PPPV[`\0\x80a\x10\xE8\x84a\x0F$\x85`\0\x01Q\x86` \x01Qa\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\x03\x84` \x01Q\x83a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\xBC\x86\x82a\x0F&V[`\0a\tU\x84a\x114\x85a\x114\x86`\0\x01Q\x87` \x01Qa\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x19;V[`\0\x80a\x11T\x83`\0\x01Q\x88\x86\x89\x89\x8D\x89`@\x01Qa\x19PV[\x90P`\0a\x11a\x82a\x1A\x95V[\x90P`\0\x80a\x11{a\x11s\x8A\x89a&\x12V[\x87Q\x90a\x1B\xBAV[\x90P`\0a\x11\xAF\x85`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x99\x90a&:V[a\x11\xA3\x91\x90a&\x12V[a\x01 \x87\x01Q\x90a\x1B\xBAV[a\x11\xB8\x8Aa&:V[a\x11\xC2\x91\x90a&\x12V[\x90Pa\x11\xCE\x82\x82a\x1B\xBAV[\x92Pa\x0E\xB6\x91P\x83\x90P\x82a\x1B\xEDV[```\x01\x82`@Q` \x01a\x11\xF4\x92\x91\x90a&VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x03\x82`@Q` \x01a\x11\xF4\x92\x91\x90a&qV[`\0a\tUa\x12<\x84\x84` \x01Qa\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x12I\x90\x87\x90a\x0F&V[\x90a\x0F;V[`\0a\tU\x84a\x114a\x12o\x86\x86` \x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90a\x13JV[```\0a\x12\x86\x85\x85\x85a\x11\x0FV[\x90P`\0a\x12\x95\x86\x83\x86a\x1C\x11V[\x90P`\0a\x12\xA5\x87\x84\x84\x88a\r\xC3V[\x90Pa\x12\xB4\x87\x84\x83\x85\x89a\x1CJV[\x85Q`@\x80\x88\x01Q``\x80\x8A\x01Q\x83Q` \x81\x01\x8E\x90R\x93\x84\x01\x89\x90R\x90\x83\x01\x85\x90R`\x80\x83\x01\x93\x90\x93R`\xA0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x82\x01R\x90\x92P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\tUa\x139\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05{\x90a\x10V\x90\x88\x90a\x0FPV[`\0a\x04;\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C\xFCV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x13|\x91\x90a&\x97V[\x94P\x94P\x94P\x94P\x94Pa\x04#\x85\x85\x85\x85\x8B\x86a\x0E\xC6V[`\0\x84\x86\x11\x15a\x13\xC1W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06JV[`\0a\x13\xD1\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xE3\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xF1\x82\x84a%\x9EV[\x13\x15a\x14\x1AW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06JV[`\0a\x14&\x89\x89a\"\xD8V[\x90P`\0[`\x02a\x147\x8A\x8Ca\"\xC5V[a\x14A\x91\x90a&\xE2V[\x94P`\0a\x14S\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14a\x86\x83a%\x9EV[\x13a\x14nW\x85\x99Pa\x14uV[\x85\x9AP\x80\x94P[a\x14\x7F\x8B\x8Ba\"\xD8V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x14\x93WP\x86\x81\x10[a\x14+WPPPP\x96\x95PPPPPPV[`\0\x80a\x14\xB2\x87\x89a\x13JV[\x90P`\0`@Q\x80a\x01\0\x01`@R\x80\x8C\x81R` \x01\x8A\x81R` \x01\x89\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a\x14\xF3\x8D\x85a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x15\x02\x84\x8Da\x0FPV[\x81R` \x01\x85\x90R\x90P`\0a\x15\x17\x82a\x1D\x1BV[\x90P`\0a\x15$\x83a\x1EYV[\x90Pa\x150\x82\x82a\x13JV[a\x159\x8Aa&:V[a\x15C\x91\x90a&\x12V[\x9D\x9CPPPPPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15lW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x15\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06JV[`\0``a\x15\xCC\x84a\x1E\xFCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x17xWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06JV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x19#\x91\x90a&\x97V[\x94P\x94P\x94P\x94P\x94Pa\x04#\x85\x85\x85\x85\x8B\x86a\x11:V[`\0a\x04;\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xFCV[a\x19\xA6`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x19\xD1\x89a\x19\xBDg\r\xE0\xB6\xB3\xA7d\0\0a&:V[a\x19\xC7\x91\x90a&\x12V[a\x05{\x88\x8Aa&\x12V[\x90P`\0a\x19\xE3\x8Aa\x05{\x8B\x8Aa\x1B\xEDV[\x90P`\0a\x19\xF1\x89\x83a\x1B\xBAV[\x90P`\0a\x1A\x07\x86g\r\xE0\xB6\xB3\xA7d\0\0a%\x0BV[\x90P`\0a\x1A1\x82a\x1A g\r\xE0\xB6\xB3\xA7d\0\0a&:V[a\x1A*\x91\x90a&\x12V[\x84\x90a\x1B\xBAV[a\x1A;\x90\x8Aa%\x0BV[\x90P`@Q\x80a\x01@\x01`@R\x80\x8E\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x82\x81R` \x01\x86\x81R` \x01\x8A\x81R` \x01\x85\x81R` \x01\x83\x81R` \x01\x89\x81R` \x01\x84\x81RP\x95PPPPPP\x97\x96PPPPPPPV[`\0\x80a\x1A\xB7\x83`@\x01Q\x84` \x01Qa\x1A\xAF\x91\x90a&\x12V[\x84Q\x90a\x1B\xBAV[\x90P`\0a\x1A\xF0a\x1A\xDD\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xED\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x86\x01Q`\x80\x87\x01Qa\x05{\x91a\x1B\xBAV[\x90P`\0a\x1B#\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x0E\x90a&:V[a\x1B\x18\x91\x90a&\x12V[`\xA0\x87\x01Q\x90a\x1B\xBAV[\x90P`\0a\x1B\x81\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1BA\x90a&:V[a\x1BK\x91\x90a&\x12V[`@\x88\x01Q\x88Q` \x8A\x01Qa\x1B{\x92\x91a\x1Bf\x91\x90a\x1B\xBAV[a\x1Bp\x91\x90a&\x12V[`\xC0\x8A\x01Q\x90a\x1B\xBAV[\x90a\x1B\xBAV[\x90Pa\x1B\xA0a\x1B\x90\x82\x84a%\x0BV[a\x01\0\x88\x01Qa\x1B{\x90\x86a\x1B\xBAV[``\x87\x01Qa\x1B\xB0\x90\x86\x90a\x1B\xBAV[a\x0E\xBC\x91\x90a&\x12V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x1B\xDCW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1C\x0BW`\0\x80\xFD[\x05\x91\x90PV[\x80Q`\0\x90\x81\x90a\x1C#\x90\x86\x90a\x0FPV[\x90P`\0a\x1C>\x84` \x01Q\x86a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\xBC\x82\x82a\x0F&V[`\0\x82\x80\x85\x83\x81\x12\x15a\x1C\x8AW[`\0\x81\x12\x15a\x1C\x85Wa\x1Cp\x82a\x03\xE7a\x03\xE8a\x1C\xFCV[\x91Pa\x1C~\x89\x89\x84\x88a\r\xC3V[\x90Pa\x1CXV[a\x1C\xB7V[`\0\x81\x13\x15a\x1C\xB7Wa\x1C\xA2\x83a\x03\xE9a\x03\xE8a\x15TV[\x92Pa\x1C\xB0\x89\x89\x85\x88a\r\xC3V[\x90Pa\x1C\x8AV[a\x1C\xEF\x89\x89\x83\x88`@Q` \x01a\x1C\xD1\x94\x93\x92\x91\x90a&\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1F\xA4a\x13\x94V[\x99\x98PPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D\x14W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x1DI\x83`\xA0\x01Qa\x114\x85` \x01Qa\x114\x87`\0\x01Q\x88``\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\xA9\x84`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dh\x91\x90a\"\xD8V[a\x114\x86`@\x01Q\x87` \x01Qa\x1D\x90\x89`\0\x01Q\x8A`\x80\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01Qa\x1D\x9F\x91\x90a\"\xD8V[a\x114\x91\x90a\"\xC5V[\x90P`\0a\x1D\xD2\x85`\0\x01Qa\x1D\xBE\x90a&:V[\x86` \x01Q\x87`\x80\x01Qa\x05{\x91\x90a\"\xC5V[\x90P`\0a\x1E\x04\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1D\xF1\x91\x90a\"\xD8V[`\xC0\x88\x01Q`\x80\x89\x01Qa\x114\x91a\x19;V[\x86``\x01Qa\x1E\x13\x91\x90a\"\xC5V[\x90Pa\x0E\xBCa\x1EOa\x1EE\x88`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E6\x91\x90a\"\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x13JV[a\x05{\x85\x85a\x19;V[a\x114\x85\x87a\"\xD8V[\x80Q`\0\x90\x81\x90a\x1Er\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD8V[\x90P`\0\x83` \x01Q\x84`\x80\x01Qa\x1E\x8A\x91\x90a\"\xC5V[\x90P`\0a\x1E\xB1\x85`\xA0\x01Qa\x114\x87` \x01Q\x88``\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1E\xE3\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xD0\x91\x90a\"\xD8V[`@\x88\x01Q`\x80\x89\x01Qa\x114\x91a\x19;V[\x90Pa\x0E\xBCa\x1E\xF2\x82\x84a\"\xC5V[a\x114\x86\x86a\x19;V[`\0\x80\x82\x11a\x1F9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06JV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1F\xBE\x91\x90a'\x18V[\x93PP\x92P\x92Pa\x0E\xBC\x83\x83\x87\x84a\r\xC3V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F\xE6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \x05W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a \x19W`\0\x80\xFD[\x815\x81\x81\x11\x15a (W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a :W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a `W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a \x8AW\x81\x81\x01Q\x83\x82\x01R` \x01a rV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra \xAB\x81` \x86\x01` \x86\x01a oV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04;` \x83\x01\x84a \x93V[`\0\x80`\0``\x84\x86\x03\x12\x15a \xE7W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!\x0CW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!$W`\0\x80\xFD[\x835\x92P` \x84\x015a!6\x81a \xFEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0E\xBC`\x80\x83\x01\x84a \x93V[`\0` \x82\x84\x03\x12\x15a!\x80W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x04>\x82\x84a!\x87V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x0CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\xE8W`\0\x80\xFD[\x815a!\xF3\x81a!\xC1V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"3Wa\"3a!\xFAV[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\"OW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\"lW`\0\x80\xFD[Pa\"ua\"\x10V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a\"\x9E\x81a!\xC1V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04>Wa\x04>a\"\xAFV[\x81\x81\x03\x81\x81\x11\x15a\x04>Wa\x04>a\"\xAFV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x0E\x16\x81\x84\x01\x85a \x93V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#{W`\0\x80\xFD[\x86Qa#\x86\x81a \xFEV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a#\xC7W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xDFW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a#\xF3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a$\x05Wa$\x05a!\xFAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a$-Wa$-a!\xFAV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a$FW`\0\x80\xFD[a\x0E\x16\x83` \x83\x01` \x88\x01a oV[`\0`\x80\x82\x84\x03\x12\x15a$iW`\0\x80\xFD[a$qa\"\x10V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa$\x99\x81a!\xC1V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a$\xB6W`\0\x80\xFD[a\x04;\x83\x83a$WV[`\0` \x82\x84\x03\x12\x15a$\xD2W`\0\x80\xFD[\x81Qa!\xF3\x81a!\xC1V[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xF2W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a%+Wa%+a\"\xAFV[P\x92\x91PPV[`\x04\x81\x10a%PWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a%b\x82\x86a%2V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`\0a\x01\0\x82\x01\x90P\x86\x82R\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01Ra\x0E\xBC`\x80\x83\x01\x84a!\x87V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a%\xBAWa%\xBAa\"\xAFV[\x81\x81\x05\x83\x14\x82\x15\x17a\x04>Wa\x04>a\"\xAFV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a%\xF3Wa%\xF3a%\xCEV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\rWa&\ra\"\xAFV[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&2Wa&2a\"\xAFV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a&OWa&Oa\"\xAFV[P`\0\x03\x90V[`@\x81\x01a&d\x82\x85a%2V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a&\x7F\x82\x85a%2V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x01\0\x86\x88\x03\x12\x15a&\xB0W`\0\x80\xFD[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa&\xD6\x87`\x80\x88\x01a$WV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82a&\xF1Wa&\xF1a%\xCEV[P\x04\x90V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\t\x07``\x83\x01\x84a!\x87V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a'.W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa'M\x86``\x87\x01a$WV[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 \x96u\x0Cf\x10\xA74J\xA6\xA8\xB5\xE7\x18\x7F\xFC9\xCC\xEE\x12\x83\x01u\xE5\xC2\xD8\xE7\xC2\xFA\xA1jM\xADdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x8EW`\x005`\xE0\x1C\x80c\x90.\xCA\xA2\x11a\0\xDEW\x80c\xDE^\xE1\xC3\x11a\0\x97W\x80c\xEC)\xD8\xE6\x11a\0qW\x80c\xEC)\xD8\xE6\x14a\x03\xA7W\x80c\xEE>\x8C\xFB\x14a\x03\xBAW\x80c\xF2\xDEz{\x14a\x03\xCDW\x80c\xF3\r7\xF2\x14a\x03\xE0W`\0\x80\xFD[\x80c\xDE^\xE1\xC3\x14a\x03nW\x80c\xDE\xF1_\x92\x14a\x03\x81W\x80c\xE2i\xADc\x14a\x03\x94W`\0\x80\xFD[\x80c\x90.\xCA\xA2\x14a\x02\xF7W\x80c\xA8\xC6.v\x14a\x03\nW\x80c\xB0\x9D\x04\xE5\x14a\x035W\x80c\xCB\x1FU2\x14a\x03HW\x80c\xCE\x15;\xF4\x14a\x03[W\x80c\xDC\x17\x83U\x14a\x02\xD7W`\0\x80\xFD[\x80cO\xD6|X\x11a\x01KW\x80cb7V\x9F\x11a\x01%W\x80cb7V\x9F\x14a\x02\x89W\x80c}\xDA\x1A#\x14a\x02\x9CW\x80c\x7F\x17@\x9C\x14a\x02\xC4W\x80c\x81\xB5\xFA\xC2\x14a\x02\xD7W`\0\x80\xFD[\x80cO\xD6|X\x14a\x025W\x80cZ\x93\xB8\xCE\x14a\x02HW\x80cZ\x9C\xA5S\x14a\x02[W`\0\x80\xFD[\x80c\x0FAf\xB8\x14a\x01\x93W\x80c%\th\xD9\x14a\x01\xB9W\x80c0m\xB4k\x14a\x01\xD9W\x80c3\"f\xF3\x14a\x01\xECW\x80c9(\xFF\x97\x14a\x01\xFFW\x80c;M\x100\x14a\x02\"W[`\0\x80\xFD[a\x01\xA6a\x01\xA16`\x04a\x1F\xD1V[a\x03\xF3V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\xCCa\x01\xC76`\x04a MV[a\x04/V[`@Qa\x01\xB0\x91\x90a \xBFV[a\x01\xA6a\x01\xE76`\x04a \xD2V[a\x04DV[a\x01\xA6a\x01\xFA6`\x04a \xD2V[a\x04tV[a\x02\x12a\x02\r6`\x04a!\x0FV[a\x04\xA4V[`@Qa\x01\xB0\x94\x93\x92\x91\x90a!GV[a\x01\xA6a\x0206`\x04a!nV[a\x08\xDDV[a\x01\xA6a\x02C6`\x04a \xD2V[a\t\x10V[a\x01\xA6a\x02V6`\x04a \xD2V[a\t@V[a\x02na\x02i6`\x04a MV[a\t]V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\xB0V[a\x02na\x02\x976`\x04a MV[a\t\xC9V[a\x02\xAFa\x02\xAA6`\x04a MV[a\n\x12V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01\xB0V[a\x02na\x02\xD26`\x04a MV[a\nnV[a\x02\xEAa\x02\xE56`\x04a!nV[a\n\xB6V[`@Qa\x01\xB0\x91\x90a!\xB3V[a\x01\xA6a\x03\x056`\x04a \xD2V[a\x0BnV[`\0Ta\x03\x1D\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\xB0V[a\x01\xCCa\x03C6`\x04a!nV[a\x0B\x9EV[a\x01\xCCa\x03V6`\x04a!\xD6V[a\x0B\xA9V[a\x02na\x03i6`\x04a!nV[a\x0B\xB4V[a\x02\xAFa\x03|6`\x04a MV[a\x0C\xAAV[a\x01\xCCa\x03\x8F6`\x04a\"9V[a\x0C\xF6V[a\x02na\x03\xA26`\x04a MV[a\r\x03V[a\x01\xA6a\x03\xB56`\x04a \xD2V[a\rRV[a\x02na\x03\xC86`\x04a MV[a\rbV[a\x01\xA6a\x03\xDB6`\x04a \xD2V[a\r\x88V[a\x02na\x03\xEE6`\x04a MV[a\r\x9DV[`\0\x80\x80\x80a\x04\x04\x85\x87\x01\x87a \xD2V[\x92P\x92P\x92P`\0a\x04\x15\x88a\n\xB6V[\x90Pa\x04#\x84\x84\x84\x84a\r\xC3V[\x98\x97PPPPPPPPV[``a\x04;\x83\x83a\x0E!V[\x90P[\x92\x91PPV[`\0\x80a\x04P\x85a\n\xB6V[\x90P`\0\x80`\0a\x04`\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0EPV[`\0\x80a\x04\x80\x85a\n\xB6V[\x90P`\0\x80`\0a\x04\x90\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0E\xC6V[`\0\x80`\0``a\x04\xCF`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xF3`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x04\xFC\x89a\x0B\xB4V[`@\x85\x01R` \x84\x01R\x82R`\0a\x05\x13\x8Aa\n\xB6V[\x90P`\0\x80a\x053\x85`\0\x01Q\x86` \x01Qa\x05.\x8Fa\n\xB6V[a\x0E\xFBV[\x90P\x8A\x15a\x06qW`\0a\x05T\x84`@\x01Q\x8Ca\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x05\x81\x85` \x01Qa\x05{\x89`\0\x01Q\x8A` \x01Qa\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x0FPV[\x90P`\0a\x05\x8F\x83\x83a\x0F&V[\x90Pa\x05\x9C`\x01\x82a\"\xC5V[\x88Q\x90\x91Pa\x05\xAC\x90\x8E\x90a\"\xC5V[\x87Ra\x05\xB8\x81\x85a\"\xC5V[\x87`@\x01\x81\x81RPPa\x05\xD4\x8F\x88`\0\x01Q\x89`@\x01Qa\r\x88V[` \x88\x01\x81\x81R`\x01\x91a\x05\xE9\x90\x83\x90a\"\xC5V[\x90RP` \x80\x89\x01Q\x90\x88\x01Q\x10a\x06SW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x86` \x01Q\x88` \x01Qa\x06g\x91\x90a\"\xD8V[\x94PPPPa\x07\x86V[`\0a\x06\x8A\x84`@\x01Q\x8Ca\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84Q` \x88\x01Q\x88Q\x92\x93P`\0\x92a\x06\xA8\x92\x91a\x05{\x91\x90a\x0F;V[\x90P`\0a\x06\xB6\x83\x83a\x0F&V[\x90Pa\x06\xC3`\x01\x82a\"\xC5V[\x90P\x8C\x88` \x01Qa\x06\xD5\x91\x90a\"\xC5V[` \x88\x01Ra\x06\xE4\x81\x85a\"\xC5V[\x87`@\x01\x81\x81RPPa\x07\0\x8F\x88` \x01Q\x89`@\x01Qa\t@V[\x80\x88R`\x01\x90\x88\x90a\x07\x13\x90\x83\x90a\"\xC5V[\x90RP\x87Q\x87Q\x10a\x07rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x06JV[\x86Q\x88Qa\x07\x80\x91\x90a\"\xD8V[\x94PPPP[\x83Q` \x80\x86\x01Q`@\x80\x88\x01Q\x81Q\x93\x84\x01\x94\x90\x94R\x82\x01R``\x81\x01\x91\x90\x91R`\0\x90`\x80\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R`\xE0\x83\x01\x82R`\0\x80\x84R` \x84\x01\x81\x90R\x91\x83\x01\x82\x90R``\x83\x01\x82\x90R`\x80\x83\x01\x82\x90R`\xA0\x83\x01\x82\x90R`\xC0\x83\x01\x91\x90\x91R\x91P\x86`\0\x01Q\x81``\x01\x81\x81RPP\x86` \x01Q\x81`\x80\x01\x81\x81RPP\x82\x81`\xA0\x01\x81\x81RPP`\0\x8E\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ce\xC9\xFF\xC20\x84\x86\x88`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08e\x94\x93\x92\x91\x90a\"\xEBV[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xA6\x91\x90a#bV[PPPPP\x90P\x80\x86a\x08\xC2\x8A`\0\x01Q\x8B` \x01Q\x8Ba\x0F\x81V[\x86\x9CP\x9CP\x9CP\x9CPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80a\x08\xE9\x83a\n\xB6V[\x90P`\0\x80a\x08\xF7\x85a\x0B\xB4V[P\x91P\x91Pa\t\x07\x82\x82\x85a\x0F\x81V[\x95\x94PPPPPV[`\0\x80a\t\x1C\x85a\n\xB6V[\x90P`\0\x80`\0a\t,\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x0F\xBAV[`\0a\tU\x83\x83a\tP\x87a\n\xB6V[a\x10 V[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80a\to\x88a\x0B\xB4V[\x92P\x92P\x92P`\0\x80a\t\x85`\0\x8A\x86\x86a\x10]V[\x91P\x91P`\0a\t\x96\x8B\x84\x84a\t@V[\x90Pa\t\xA2\x81\x87a\"\xD8V[a\t\xAC\x84\x87a\"\xD8V[a\t\xB6\x84\x87a\"\xD8V[\x98P\x98P\x98PPPPPPP\x92P\x92P\x92V[`\0\x80`\0\x80`\0a\t\xDA\x87a\x0B\xB4V[\x92PP\x91P`\0\x80a\t\xEF`\0\x89\x86\x86a\x10]V[\x91P\x91P`\0a\n\0\x8A\x84\x84a\r\x88V[\x92\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[`\0\x80`\0\x80a\n!\x86a\x0B\xB4V[P\x91P\x91P`\0a\n1\x87a\n\xB6V[\x90P`\0a\n@\x84\x84\x84a\x0F\x81V[\x90P`\0a\nO\x88\x83\x85a\x10\xC6V[\x90P`\0a\n^\x89\x84\x86a\x11\x0FV[\x9A\x91\x99P\x90\x97PPPPPPPPV[`\0\x80`\0\x80`\0a\n\x7F\x87a\x0B\xB4V[\x92P\x92PP`\0\x80a\n\x94`\x01\x89\x86\x86a\x10]V[\x91P\x91P`\0a\n\xA5\x8A\x84\x84a\t@V[\x97P\x91\x95P\x93PPPP\x92P\x92P\x92V[a\n\xEA`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0B[\x91\x90\x81\x01\x90a#\xB5V[\x80` \x01\x90Q\x81\x01\x90a\x04>\x91\x90a$\xA4V[`\0\x80a\x0Bz\x85a\n\xB6V[\x90P`\0\x80`\0a\x0B\x8A\x88a\x0B\xB4V[\x92P\x92P\x92Pa\x04#\x87\x84\x84\x84\x8A\x89a\x11:V[``a\x04>\x82a\x11\xDEV[``a\x04>\x82a\x12\nV[`\0\x80`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x0BW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C/\x91\x90a$\xC0V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0C\\\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0CyW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C\x9D\x91\x90a$\xDDV[\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0\x80`\0\x80a\x0C\xB9\x86a\x0B\xB4V[P\x91P\x91P`\0a\x0C\xC9\x87a\n\xB6V[\x90P`\0a\x0C\xD8\x84\x84\x84a\x0F\x81V[\x90P`\0a\x0C\xE7\x88\x83\x85a\x12 V[\x90P`\0a\n^\x89\x84\x86a\x12OV[``a\tU\x84\x84\x84a\x12wV[`\0\x80`\0\x80`\0\x80a\r\x15\x88a\x0B\xB4V[\x92P\x92P\x92P`\0\x80a\r+`\0\x8A\x87\x86a\x10]V[\x91P\x91P`\0a\r<\x8B\x84\x84a\r\x88V[\x90Pa\rH\x83\x87a\"\xD8V[a\t\xAC\x82\x87a\"\xD8V[`\0a\tU\x83\x83a\x05.\x87a\n\xB6V[`\0\x80`\0\x80`\0a\rs\x87a\x0B\xB4V[\x92PP\x91P`\0\x80a\t\xEF`\x01\x89\x86\x86a\x10]V[`\0a\tU\x83\x83a\r\x98\x87a\n\xB6V[a\x13\x15V[`\0\x80`\0\x80`\0a\r\xAE\x87a\x0B\xB4V[\x92P\x92PP`\0\x80a\n\x94`\0\x89\x86\x86a\x10]V[\x80Q`\0\x90\x81\x90a\r\xD8\x90a\x05{\x88\x87a\x13JV[\x90P`\0a\r\xF7\x84` \x01Qa\x05{\x87\x89a\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\x0C\x83\x83a\x0F&V[a\x0E\x16\x91\x90a%\x0BV[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\x0E9\x93\x92\x91\x90a%TV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x82a\x03\xE8\x82a\x0Ee\x8A\x8A\x8A\x8A\x86\x8Aa\x0E\xC6V[\x90P`\0\x81\x12\x15a\x0E|W`\0\x93PPPPa\x0E\xBCV[a\x0E\xB6\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x0E\x98\x95\x94\x93\x92\x91\x90a%sV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x13_a\x13\x94V[\x93PPPP[\x96\x95PPPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0E\xE1\x91\x90a\"\xD8V[\x90Pa\x04#\x83`\0\x01Q\x84` \x01Q\x89\x89\x8C\x8A\x8A\x88a\x14\xA5V[`\0a\tUa\x0F\x17\x83` \x01Q\x85a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x0F$\x90\x87\x90a\x0FPV[\x90[`\0a\x04;\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x15TV[`\0a\x04;\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x15TV[`\0a\x04;g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0Fh\x86a\x15\x82V[a\x0Fr\x91\x90a%\x9EV[a\x0F|\x91\x90a%\xE4V[a\x17]V[`\0\x80a\x0F\x9B\x83` \x01Q\x85a\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\x0F\xAE\x90\x87\x90a\x13JV[\x90Pa\x0E\xBC\x82\x82a\x13JV[`\0\x82a\x03\xE8\x82a\x0F\xCF\x8A\x8A\x8A\x8A\x86\x8Aa\x11:V[\x90P`\0\x81\x12\x15a\x0F\xE6W`\0\x93PPPPa\x0E\xBCV[a\x0E\xB6\x8A\x8A\x8A\x8A\x89`@Q` \x01a\x10\x02\x95\x94\x93\x92\x91\x90a%sV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x19\x06a\x13\x94V[\x80Q`\0\x90a\tU\x90a\x10<\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0F;V[a\x05{a\x10V\x85` \x01Q\x88a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0F;V[`\0\x80\x80a\x10k\x84\x86a\x0F;V[\x90P`\0a\x10y\x87\x83a\x0F&V[\x90P\x87a\x10\x8FWa\x10\x8A\x87\x87a\"\xD8V[a\x10\x99V[a\x10\x99\x87\x87a\"\xC5V[\x93P\x87a\x10\xAFWa\x10\xAA\x81\x86a\"\xD8V[a\x10\xB9V[a\x10\xB9\x81\x86a\"\xC5V[\x92PPP\x94P\x94\x92PPPV[`\0\x80a\x10\xE8\x84a\x0F$\x85`\0\x01Q\x86` \x01Qa\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x11\x03\x84` \x01Q\x83a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\xBC\x86\x82a\x0F&V[`\0a\tU\x84a\x114\x85a\x114\x86`\0\x01Q\x87` \x01Qa\x13J\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x19;V[`\0\x80a\x11T\x83`\0\x01Q\x88\x86\x89\x89\x8D\x89`@\x01Qa\x19PV[\x90P`\0a\x11a\x82a\x1A\x95V[\x90P`\0\x80a\x11{a\x11s\x8A\x89a&\x12V[\x87Q\x90a\x1B\xBAV[\x90P`\0a\x11\xAF\x85`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x11\x99\x90a&:V[a\x11\xA3\x91\x90a&\x12V[a\x01 \x87\x01Q\x90a\x1B\xBAV[a\x11\xB8\x8Aa&:V[a\x11\xC2\x91\x90a&\x12V[\x90Pa\x11\xCE\x82\x82a\x1B\xBAV[\x92Pa\x0E\xB6\x91P\x83\x90P\x82a\x1B\xEDV[```\x01\x82`@Q` \x01a\x11\xF4\x92\x91\x90a&VV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x03\x82`@Q` \x01a\x11\xF4\x92\x91\x90a&qV[`\0a\tUa\x12<\x84\x84` \x01Qa\x0F&\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x12I\x90\x87\x90a\x0F&V[\x90a\x0F;V[`\0a\tU\x84a\x114a\x12o\x86\x86` \x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x85Q\x90a\x13JV[```\0a\x12\x86\x85\x85\x85a\x11\x0FV[\x90P`\0a\x12\x95\x86\x83\x86a\x1C\x11V[\x90P`\0a\x12\xA5\x87\x84\x84\x88a\r\xC3V[\x90Pa\x12\xB4\x87\x84\x83\x85\x89a\x1CJV[\x85Q`@\x80\x88\x01Q``\x80\x8A\x01Q\x83Q` \x81\x01\x8E\x90R\x93\x84\x01\x89\x90R\x90\x83\x01\x85\x90R`\x80\x83\x01\x93\x90\x93R`\xA0\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\xC0\x82\x01R\x90\x92P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x93PPPP\x93\x92PPPV[`\0a\tUa\x139\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0F;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05{\x90a\x10V\x90\x88\x90a\x0FPV[`\0a\x04;\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1C\xFCV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x13|\x91\x90a&\x97V[\x94P\x94P\x94P\x94P\x94Pa\x04#\x85\x85\x85\x85\x8B\x86a\x0E\xC6V[`\0\x84\x86\x11\x15a\x13\xC1W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x87\x90R`$\x81\x01\x86\x90R`D\x01a\x06JV[`\0a\x13\xD1\x88\x88\x85c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xE3\x89\x88\x86c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xF1\x82\x84a%\x9EV[\x13\x15a\x14\x1AW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x06JV[`\0a\x14&\x89\x89a\"\xD8V[\x90P`\0[`\x02a\x147\x8A\x8Ca\"\xC5V[a\x14A\x91\x90a&\xE2V[\x94P`\0a\x14S\x8C\x87\x89c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14a\x86\x83a%\x9EV[\x13a\x14nW\x85\x99Pa\x14uV[\x85\x9AP\x80\x94P[a\x14\x7F\x8B\x8Ba\"\xD8V[\x92PP`\x01\x01\x87\x82\x11\x80\x15a\x14\x93WP\x86\x81\x10[a\x14+WPPPP\x96\x95PPPPPPV[`\0\x80a\x14\xB2\x87\x89a\x13JV[\x90P`\0`@Q\x80a\x01\0\x01`@R\x80\x8C\x81R` \x01\x8A\x81R` \x01\x89\x81R` \x01\x87\x81R` \x01\x86\x81R` \x01a\x14\xF3\x8D\x85a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x81R` \x01a\x15\x02\x84\x8Da\x0FPV[\x81R` \x01\x85\x90R\x90P`\0a\x15\x17\x82a\x1D\x1BV[\x90P`\0a\x15$\x83a\x1EYV[\x90Pa\x150\x82\x82a\x13JV[a\x159\x8Aa&:V[a\x15C\x91\x90a&\x12V[\x9D\x9CPPPPPPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x15lW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x13a\x15\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06JV[`\0``a\x15\xCC\x84a\x1E\xFCV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x17xWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x17\xBFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x06JV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x80`\0\x80`\0\x80\x87\x80` \x01\x90Q\x81\x01\x90a\x19#\x91\x90a&\x97V[\x94P\x94P\x94P\x94P\x94Pa\x04#\x85\x85\x85\x85\x8B\x86a\x11:V[`\0a\x04;\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xFCV[a\x19\xA6`@Q\x80a\x01@\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x19\xD1\x89a\x19\xBDg\r\xE0\xB6\xB3\xA7d\0\0a&:V[a\x19\xC7\x91\x90a&\x12V[a\x05{\x88\x8Aa&\x12V[\x90P`\0a\x19\xE3\x8Aa\x05{\x8B\x8Aa\x1B\xEDV[\x90P`\0a\x19\xF1\x89\x83a\x1B\xBAV[\x90P`\0a\x1A\x07\x86g\r\xE0\xB6\xB3\xA7d\0\0a%\x0BV[\x90P`\0a\x1A1\x82a\x1A g\r\xE0\xB6\xB3\xA7d\0\0a&:V[a\x1A*\x91\x90a&\x12V[\x84\x90a\x1B\xBAV[a\x1A;\x90\x8Aa%\x0BV[\x90P`@Q\x80a\x01@\x01`@R\x80\x8E\x81R` \x01\x8C\x81R` \x01\x8B\x81R` \x01\x82\x81R` \x01\x86\x81R` \x01\x8A\x81R` \x01\x85\x81R` \x01\x83\x81R` \x01\x89\x81R` \x01\x84\x81RP\x95PPPPPP\x97\x96PPPPPPPV[`\0\x80a\x1A\xB7\x83`@\x01Q\x84` \x01Qa\x1A\xAF\x91\x90a&\x12V[\x84Q\x90a\x1B\xBAV[\x90P`\0a\x1A\xF0a\x1A\xDD\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\xED\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[``\x86\x01Q`\x80\x87\x01Qa\x05{\x91a\x1B\xBAV[\x90P`\0a\x1B#\x85`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1B\x0E\x90a&:V[a\x1B\x18\x91\x90a&\x12V[`\xA0\x87\x01Q\x90a\x1B\xBAV[\x90P`\0a\x1B\x81\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1BA\x90a&:V[a\x1BK\x91\x90a&\x12V[`@\x88\x01Q\x88Q` \x8A\x01Qa\x1B{\x92\x91a\x1Bf\x91\x90a\x1B\xBAV[a\x1Bp\x91\x90a&\x12V[`\xC0\x8A\x01Q\x90a\x1B\xBAV[\x90a\x1B\xBAV[\x90Pa\x1B\xA0a\x1B\x90\x82\x84a%\x0BV[a\x01\0\x88\x01Qa\x1B{\x90\x86a\x1B\xBAV[``\x87\x01Qa\x1B\xB0\x90\x86\x90a\x1B\xBAV[a\x0E\xBC\x91\x90a&\x12V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x1B\xDCW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x1C\x0BW`\0\x80\xFD[\x05\x91\x90PV[\x80Q`\0\x90\x81\x90a\x1C#\x90\x86\x90a\x0FPV[\x90P`\0a\x1C>\x84` \x01Q\x86a\x0FP\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x0E\xBC\x82\x82a\x0F&V[`\0\x82\x80\x85\x83\x81\x12\x15a\x1C\x8AW[`\0\x81\x12\x15a\x1C\x85Wa\x1Cp\x82a\x03\xE7a\x03\xE8a\x1C\xFCV[\x91Pa\x1C~\x89\x89\x84\x88a\r\xC3V[\x90Pa\x1CXV[a\x1C\xB7V[`\0\x81\x13\x15a\x1C\xB7Wa\x1C\xA2\x83a\x03\xE9a\x03\xE8a\x15TV[\x92Pa\x1C\xB0\x89\x89\x85\x88a\r\xC3V[\x90Pa\x1C\x8AV[a\x1C\xEF\x89\x89\x83\x88`@Q` \x01a\x1C\xD1\x94\x93\x92\x91\x90a&\xF6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\x1F\xA4a\x13\x94V[\x99\x98PPPPPPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1D\x14W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80a\x1DI\x83`\xA0\x01Qa\x114\x85` \x01Qa\x114\x87`\0\x01Q\x88``\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1D\xA9\x84`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1Dh\x91\x90a\"\xD8V[a\x114\x86`@\x01Q\x87` \x01Qa\x1D\x90\x89`\0\x01Q\x8A`\x80\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x89`\x80\x01Qa\x1D\x9F\x91\x90a\"\xD8V[a\x114\x91\x90a\"\xC5V[\x90P`\0a\x1D\xD2\x85`\0\x01Qa\x1D\xBE\x90a&:V[\x86` \x01Q\x87`\x80\x01Qa\x05{\x91\x90a\"\xC5V[\x90P`\0a\x1E\x04\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1D\xF1\x91\x90a\"\xD8V[`\xC0\x88\x01Q`\x80\x89\x01Qa\x114\x91a\x19;V[\x86``\x01Qa\x1E\x13\x91\x90a\"\xC5V[\x90Pa\x0E\xBCa\x1EOa\x1EE\x88`\0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E6\x91\x90a\"\xD8V[g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x13JV[a\x05{\x85\x85a\x19;V[a\x114\x85\x87a\"\xD8V[\x80Q`\0\x90\x81\x90a\x1Er\x90g\r\xE0\xB6\xB3\xA7d\0\0a\"\xD8V[\x90P`\0\x83` \x01Q\x84`\x80\x01Qa\x1E\x8A\x91\x90a\"\xC5V[\x90P`\0a\x1E\xB1\x85`\xA0\x01Qa\x114\x87` \x01Q\x88``\x01Qa\x19;\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x1E\xE3\x86`\xE0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xD0\x91\x90a\"\xD8V[`@\x88\x01Q`\x80\x89\x01Qa\x114\x91a\x19;V[\x90Pa\x0E\xBCa\x1E\xF2\x82\x84a\"\xC5V[a\x114\x86\x86a\x19;V[`\0\x80\x82\x11a\x1F9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x06JV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x1F\xBE\x91\x90a'\x18V[\x93PP\x92P\x92Pa\x0E\xBC\x83\x83\x87\x84a\r\xC3V[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x1F\xE6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a \x05W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a \x19W`\0\x80\xFD[\x815\x81\x81\x11\x15a (W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a :W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a `W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a \x8AW\x81\x81\x01Q\x83\x82\x01R` \x01a rV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra \xAB\x81` \x86\x01` \x86\x01a oV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x04;` \x83\x01\x84a \x93V[`\0\x80`\0``\x84\x86\x03\x12\x15a \xE7W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!\x0CW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!$W`\0\x80\xFD[\x835\x92P` \x84\x015a!6\x81a \xFEV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0E\xBC`\x80\x83\x01\x84a \x93V[`\0` \x82\x84\x03\x12\x15a!\x80W`\0\x80\xFD[P5\x91\x90PV[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x04>\x82\x84a!\x87V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\x0CW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a!\xE8W`\0\x80\xFD[\x815a!\xF3\x81a!\xC1V[\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"3Wa\"3a!\xFAV[`@R\x90V[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\"OW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\"lW`\0\x80\xFD[Pa\"ua\"\x10V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a\"\x9E\x81a!\xC1V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x04>Wa\x04>a\"\xAFV[\x81\x81\x03\x81\x81\x11\x15a\x04>Wa\x04>a\"\xAFV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x0E\x16\x81\x84\x01\x85a \x93V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a#{W`\0\x80\xFD[\x86Qa#\x86\x81a \xFEV[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a#\xC7W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a#\xDFW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a#\xF3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a$\x05Wa$\x05a!\xFAV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a$-Wa$-a!\xFAV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a$FW`\0\x80\xFD[a\x0E\x16\x83` \x83\x01` \x88\x01a oV[`\0`\x80\x82\x84\x03\x12\x15a$iW`\0\x80\xFD[a$qa\"\x10V[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa$\x99\x81a!\xC1V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a$\xB6W`\0\x80\xFD[a\x04;\x83\x83a$WV[`\0` \x82\x84\x03\x12\x15a$\xD2W`\0\x80\xFD[\x81Qa!\xF3\x81a!\xC1V[`\0\x80`\0``\x84\x86\x03\x12\x15a$\xF2W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a%+Wa%+a\"\xAFV[P\x92\x91PPV[`\x04\x81\x10a%PWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a%b\x82\x86a%2V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`\0a\x01\0\x82\x01\x90P\x86\x82R\x85` \x83\x01R\x84`@\x83\x01R\x83``\x83\x01Ra\x0E\xBC`\x80\x83\x01\x84a!\x87V[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a%\xBAWa%\xBAa\"\xAFV[\x81\x81\x05\x83\x14\x82\x15\x17a\x04>Wa\x04>a\"\xAFV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a%\xF3Wa%\xF3a%\xCEV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a&\rWa&\ra\"\xAFV[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a&2Wa&2a\"\xAFV[PP\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a&OWa&Oa\"\xAFV[P`\0\x03\x90V[`@\x81\x01a&d\x82\x85a%2V[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a&\x7F\x82\x85a%2V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\0\x80`\0\x80`\0a\x01\0\x86\x88\x03\x12\x15a&\xB0W`\0\x80\xFD[\x85Q\x94P` \x86\x01Q\x93P`@\x86\x01Q\x92P``\x86\x01Q\x91Pa&\xD6\x87`\x80\x88\x01a$WV[\x90P\x92\x95P\x92\x95\x90\x93PV[`\0\x82a&\xF1Wa&\xF1a%\xCEV[P\x04\x90V[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\t\x07``\x83\x01\x84a!\x87V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a'.W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa'M\x86``\x87\x01a$WV[\x90P\x92\x95\x91\x94P\x92PV\xFE\xA2dipfsX\"\x12 \x96u\x0Cf\x10\xA74J\xA6\xA8\xB5\xE7\x18\x7F\xFC9\xCC\xEE\x12\x83\x01u\xE5\xC2\xD8\xE7\xC2\xFA\xA1jM\xADdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                GEOMETRICMEANSOLVER_ABI.clone(),
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
                GEOMETRICMEANSOLVER_ABI.clone(),
                GEOMETRICMEANSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        /// Calls the contract's `checkSwapConstant` (0x0f4166b8) function
        pub fn check_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([15, 65, 102, 184], (pool_id, data))
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
        /// Calls the contract's `deallocateGivenXReturnDeltas` (0xe269ad63)
        /// function
        pub fn deallocate_given_x_return_deltas(
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
                .method_hash([226, 105, 173, 99], (pool_id, amount_x))
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
        /// Calls the contract's `deallocateGivenYReturnDeltas` (0x5a9ca553)
        /// function
        pub fn deallocate_given_y_return_deltas(
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
                .method_hash([90, 156, 165, 83], (pool_id, amount_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `fetchPoolParams` (0x81b5fac2) function
        pub fn fetch_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GeometricMeanParams> {
            self.0
                .method_hash([129, 181, 250, 194], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getInitialPoolData` (0xdef15f92) function
        pub fn get_initial_pool_data(
            &self,
            rx: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
            params: GeometricMeanParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([222, 241, 95, 146], (rx, s, params))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getNextLiquidity` (0xec29d8e6) function
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
        /// Calls the contract's `getNextReserveX` (0x5a93b8ce) function
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
        /// Calls the contract's `getNextReserveY` (0xf2de7a7b) function
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
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, GeometricMeanParams> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
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
        /// Calls the contract's `prepareWeightXUpdate` (0x250968d9) function
        pub fn prepare_weight_x_update(
            &self,
            target_weight_x: ::ethers::core::types::U256,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([37, 9, 104, 217], (target_weight_x, target_timestamp))
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
        for GeometricMeanSolver<M>
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
                Self::BisectionLib_InvalidBounds(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<BisectionLib_InvalidBounds> for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for GeometricMeanSolverErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
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
    /// Container type for all input parameters for the `checkSwapConstant`
    /// function with signature `checkSwapConstant(uint256,bytes)` and selector
    /// `0x0f4166b8`
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
    #[ethcall(name = "checkSwapConstant", abi = "checkSwapConstant(uint256,bytes)")]
    pub struct CheckSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    /// Container type for all input parameters for the
    /// `deallocateGivenXReturnDeltas` function with signature
    /// `deallocateGivenXReturnDeltas(uint256,uint256)` and selector
    /// `0xe269ad63`
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
        name = "deallocateGivenXReturnDeltas",
        abi = "deallocateGivenXReturnDeltas(uint256,uint256)"
    )]
    pub struct DeallocateGivenXReturnDeltasCall {
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
    /// Container type for all input parameters for the
    /// `deallocateGivenYReturnDeltas` function with signature
    /// `deallocateGivenYReturnDeltas(uint256,uint256)` and selector
    /// `0x5a9ca553`
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
        name = "deallocateGivenYReturnDeltas",
        abi = "deallocateGivenYReturnDeltas(uint256,uint256)"
    )]
    pub struct DeallocateGivenYReturnDeltasCall {
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
        pub params: GeometricMeanParams,
    }
    /// Container type for all input parameters for the `getNextLiquidity`
    /// function with signature `getNextLiquidity(uint256,uint256,uint256)` and
    /// selector `0xec29d8e6`
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
        abi = "getNextLiquidity(uint256,uint256,uint256)"
    )]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getNextReserveX`
    /// function with signature `getNextReserveX(uint256,uint256,uint256)` and
    /// selector `0x5a93b8ce`
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
        abi = "getNextReserveX(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getNextReserveY`
    /// function with signature `getNextReserveY(uint256,uint256,uint256)` and
    /// selector `0xf2de7a7b`
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
        abi = "getNextReserveY(uint256,uint256,uint256)"
    )]
    pub struct GetNextReserveYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub rx: ::ethers::core::types::U256,
        pub l: ::ethers::core::types::U256,
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
    /// Container type for all input parameters for the `prepareWeightXUpdate`
    /// function with signature `prepareWeightXUpdate(uint256,uint256)` and
    /// selector `0x250968d9`
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
        name = "prepareWeightXUpdate",
        abi = "prepareWeightXUpdate(uint256,uint256)"
    )]
    pub struct PrepareWeightXUpdateCall {
        pub target_weight_x: ::ethers::core::types::U256,
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
    pub enum GeometricMeanSolverCalls {
        AllocateGivenDeltaX(AllocateGivenDeltaXCall),
        AllocateGivenDeltaY(AllocateGivenDeltaYCall),
        AllocateGivenX(AllocateGivenXCall),
        AllocateGivenY(AllocateGivenYCall),
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        CheckSwapConstant(CheckSwapConstantCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        DeallocateGivenX(DeallocateGivenXCall),
        DeallocateGivenXReturnDeltas(DeallocateGivenXReturnDeltasCall),
        DeallocateGivenY(DeallocateGivenYCall),
        DeallocateGivenYReturnDeltas(DeallocateGivenYReturnDeltasCall),
        FetchPoolParams(FetchPoolParamsCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPoolParams(GetPoolParamsCall),
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
                <CheckSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSwapConstant(decoded));
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
                <DeallocateGivenXReturnDeltasCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenXReturnDeltas(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenYCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenY(decoded));
            }
            if let Ok(decoded) =
                <DeallocateGivenYReturnDeltasCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DeallocateGivenYReturnDeltas(decoded));
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
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
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
                <PrepareWeightXUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareWeightXUpdate(decoded));
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::CheckSwapConstant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ComputeOptimalArbLowerPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ComputeOptimalArbRaisePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenXReturnDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeallocateGivenY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DeallocateGivenYReturnDeltas(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FetchPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareWeightXUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateGivenDeltaX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenDeltaY(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffLower(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffRaise(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbLowerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbRaisePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenXReturnDeltas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeallocateGivenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateGivenYReturnDeltas(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FetchPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightXUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateGivenDeltaXCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenDeltaXCall) -> Self {
            Self::AllocateGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<AllocateGivenDeltaYCall> for GeometricMeanSolverCalls {
        fn from(value: AllocateGivenDeltaYCall) -> Self {
            Self::AllocateGivenDeltaY(value)
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
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall> for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall> for GeometricMeanSolverCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenXCall) -> Self {
            Self::DeallocateGivenX(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenXReturnDeltasCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenXReturnDeltasCall) -> Self {
            Self::DeallocateGivenXReturnDeltas(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenYCall) -> Self {
            Self::DeallocateGivenY(value)
        }
    }
    impl ::core::convert::From<DeallocateGivenYReturnDeltasCall> for GeometricMeanSolverCalls {
        fn from(value: DeallocateGivenYReturnDeltasCall) -> Self {
            Self::DeallocateGivenYReturnDeltas(value)
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
    impl ::core::convert::From<GetPoolParamsCall> for GeometricMeanSolverCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for GeometricMeanSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InternalPriceCall> for GeometricMeanSolverCalls {
        fn from(value: InternalPriceCall) -> Self {
            Self::InternalPrice(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for GeometricMeanSolverCalls {
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
    pub struct AllocateGivenDeltaXReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    pub struct AllocateGivenDeltaYReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
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
    /// Container type for all return fields from the `checkSwapConstant`
    /// function with signature `checkSwapConstant(uint256,bytes)` and selector
    /// `0x0f4166b8`
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
    pub struct CheckSwapConstantReturn(pub ::ethers::core::types::I256);
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
    /// Container type for all return fields from the
    /// `deallocateGivenXReturnDeltas` function with signature
    /// `deallocateGivenXReturnDeltas(uint256,uint256)` and selector
    /// `0xe269ad63`
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
    pub struct DeallocateGivenXReturnDeltasReturn(
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
    /// Container type for all return fields from the
    /// `deallocateGivenYReturnDeltas` function with signature
    /// `deallocateGivenYReturnDeltas(uint256,uint256)` and selector
    /// `0x5a9ca553`
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
    pub struct DeallocateGivenYReturnDeltasReturn(
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
    pub struct FetchPoolParamsReturn(pub GeometricMeanParams);
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
    /// function with signature `getNextLiquidity(uint256,uint256,uint256)` and
    /// selector `0xec29d8e6`
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
    /// with signature `getNextReserveX(uint256,uint256,uint256)` and selector
    /// `0x5a93b8ce`
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
    /// with signature `getNextReserveY(uint256,uint256,uint256)` and selector
    /// `0xf2de7a7b`
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
    pub struct GetPoolParamsReturn {
        pub params: GeometricMeanParams,
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
    pub struct PrepareFeeUpdateReturn {
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all return fields from the `prepareWeightXUpdate`
    /// function with signature `prepareWeightXUpdate(uint256,uint256)` and
    /// selector `0x250968d9`
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
    pub struct PrepareWeightXUpdateReturn(pub ::ethers::core::types::Bytes);
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
    /// `GeometricMeanParams(uint256,uint256,uint256,address)`
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
    pub struct GeometricMeanParams {
        pub w_x: ::ethers::core::types::U256,
        pub w_y: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}

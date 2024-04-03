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
                    name: ::std::borrow::ToOwned::to_owned("strategy_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("prepareAllocationDeltasGivenDeltaL"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "prepareAllocationDeltasGivenDeltaL",
                        ),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("prepareAllocationDeltasGivenDeltaX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "prepareAllocationDeltasGivenDeltaX",
                        ),
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
                    ::std::borrow::ToOwned::to_owned("prepareAllocationDeltasGivenDeltaY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned(
                            "prepareAllocationDeltasGivenDeltaY",
                        ),
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
                                ::std::borrow::ToOwned::to_owned("contract IStrategy"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1E\xD38\x03\x80a\x1E\xD3\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x1E@\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xB0\x9D\x04\xE5\x11a\0\x97W\x80c\xCE\x15;\xF4\x11a\0fW\x80c\xCE\x15;\xF4\x14a\x02\x16W\x80c\xDC\x17\x83U\x14a\x02DW\x80c\xDE\xF1_\x92\x14a\x02dW\x80c\xF2\xDEz{\x14a\x02wW`\0\x80\xFD[\x80c\xB0\x9D\x04\xE5\x14a\x01\xBBW\x80c\xC2\x93\x87\xE5\x14a\x01\xCEW\x80c\xC6a\xDB\xF5\x14a\x01\xF0W\x80c\xCB\x1FU2\x14a\x02\x03W`\0\x80\xFD[\x80c;M\x100\x11a\0\xD3W\x80c;M\x100\x14a\x01WW\x80cZ\x93\xB8\xCE\x14a\x01jW\x80c\x8C5\x82M\x14a\x01}W\x80c\xA8\xC6.v\x14a\x01\x90W`\0\x80\xFD[\x80c\x08TQ[\x14a\0\xFAW\x80c\x0FAf\xB8\x14a\x01#W\x80c%\th\xD9\x14a\x01DW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x14,V[a\x02\x8AV[`@Qa\x01\x1A\x91\x90a\x14\x9EV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a\x14\xB1V[a\x02\xB8V[`@Q\x90\x81R` \x01a\x01\x1AV[a\x01\ra\x01R6`\x04a\x14,V[a\x02\xF4V[a\x016a\x01e6`\x04a\x15-V[a\x03\x07V[a\x016a\x01x6`\x04a\x15FV[a\x03:V[a\x01\ra\x01\x8B6`\x04a\x14,V[a\x03WV[`\0Ta\x01\xA3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x01\ra\x01\xC96`\x04a\x15-V[a\x03yV[a\x01\xE1a\x01\xDC6`\x04a\x15rV[a\x03\x84V[`@Qa\x01\x1A\x93\x92\x91\x90a\x15\xA4V[a\x01\ra\x01\xFE6`\x04a\x14,V[a\x06\xD1V[a\x01\ra\x02\x116`\x04a\x15\xDDV[a\x06\xF3V[a\x02)a\x02$6`\x04a\x15-V[a\x06\xFEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x1AV[a\x02Wa\x02R6`\x04a\x15-V[a\x08>V[`@Qa\x01\x1A\x91\x90a\x15\xFAV[a\x01\ra\x02r6`\x04a\x16\xC1V[a\x08\xF6V[a\x016a\x02\x856`\x04a\x15FV[a\t\x03V[```\0\x80`\0a\x02\x9A\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\t\x18V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80a\x02\xC9\x85\x87\x01\x87a\x15FV[\x92P\x92P\x92P`\0a\x02\xDA\x88a\x08>V[\x90Pa\x02\xE8\x84\x84\x84\x84a\tpV[\x98\x97PPPPPPPPV[``a\x03\0\x83\x83a\t\xCEV[\x93\x92PPPV[`\0\x80a\x03\x13\x83a\x08>V[\x90P`\0\x80a\x03!\x85a\x06\xFEV[P\x91P\x91Pa\x031\x82\x82\x85a\t\xFDV[\x95\x94PPPPPV[`\0a\x03O\x83\x83a\x03J\x87a\x08>V[a\n@V[\x94\x93PPPPV[```\0\x80`\0a\x03g\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\n}V[``a\x02\xB2\x82a\n\xBEV[`\0\x80```\0a\x03\x94\x88a\x08>V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0E\x91\x90a\x17GV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04;\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x80\x91\x90\x81\x01\x90a\x18_V[\x90Pa\x04\xCA`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81`@\x01Q\x89\x81Q\x81\x10a\x04\xE0Wa\x04\xE0a\x19:V[` \x02` \x01\x01Q\x81`@\x01\x81\x81RPP\x81`@\x01Q\x88\x81Q\x81\x10a\x05\x07Wa\x05\x07a\x19:V[` \x02` \x01\x01Q\x81``\x01\x81\x81RPP\x88`\0\x03a\x056W\x82Q`\x80\x82\x01R` \x83\x01Q`\xA0\x82\x01Ra\x05HV[` \x83\x01Q`\x80\x82\x01R\x82Q`\xA0\x82\x01R[a\x05e\x87\x82`@\x01Q\x84``\x01Q\x84`\x80\x01Q\x87`@\x01Qa\n\xEAV[`\xC0\x82\x01\x81\x90R``\x83\x01Q`\0\x91a\x05}\x91a\x19fV[\x90P`\0a\x05\x9F\x83`\x80\x01Q\x8A\x85`@\x01Qa\x05\x99\x91\x90a\x19fV[\x90a\x0B\x0BV[\x90P`\0a\x05\xCFa\x05\xC5\x85`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\x99\x85\x85a\x0B<V[\x90P\x80\x84``\x01Qa\x05\xE1\x91\x90a\x19yV[` \x85\x81\x01\x82\x90R`@\x80Q\x91\x82\x01\x8F\x90R\x81\x01\x8D\x90R``\x81\x01\x8C\x90R`\x80\x81\x01\x91\x90\x91R`\0\x93P`\xA0\x01\x91Pa\x06\x17\x90PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x8E\x87\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06p\x94\x93\x92\x91\x90a\x19\xC8V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB1\x91\x90a\x1A\xB1V[PPPP` \x95\x90\x95\x01Q\x91\x9E\x91\x9DP\x92\x9BP\x99PPPPPPPPPPV[```\0\x80`\0a\x06\xE1\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\x0BQV[``a\x02\xB2\x82a\x0B\x92V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07y\x91\x90a\x17GV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xA6\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xEB\x91\x90\x81\x01\x90a\x18_V[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\x08\x04Wa\x08\x04a\x19:V[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x08#Wa\x08#a\x19:V[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\x08r`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xE3\x91\x90\x81\x01\x90a\x1B\x13V[\x80` \x01\x90Q\x81\x01\x90a\x02\xB2\x91\x90a\x1B\xF4V[``a\x03O\x84\x84\x84a\x0B\xA8V[`\0a\x03O\x83\x83a\t\x13\x87a\x08>V[a\x0C\x8CV[```\0a\t'\x86\x86\x85a\x0C\xC1V[\x90P`\0a\t6\x87\x86\x86a\x0C\xC1V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[\x80Q`\0\x90\x81\x90a\t\x85\x90a\x05\x99\x88\x87a\x0B<V[\x90P`\0a\t\xA4\x84` \x01Qa\x05\x99\x87\x89a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\t\xB9\x83\x83a\x0C\xCEV[a\t\xC3\x91\x90a\x1C\x10V[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\t\xE6\x93\x92\x91\x90a\x1CYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\n\x17\x83` \x01Q\x85a\x0C\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\n*\x90\x87\x90a\x0C\xE3V[\x90Pa\n6\x82\x82a\x0C\xE3V[\x96\x95PPPPPPV[\x80Q`\0\x90a\x03O\x90a\n\\\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0B<V[a\x05\x99a\nv\x85` \x01Q\x88a\x0B\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0B<V[```\0a\n\x8C\x86\x86\x86a\x0C\xF8V[\x90P`\0a\n\x9B\x87\x85\x87a\r\x05V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\tUV[```\x01\x82`@Q` \x01a\n\xD4\x92\x91\x90a\x1CxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\n6a\n\xF9\x87\x87a\x0B<V[a\x0B\x05\x86\x81\x87\x87a\x0C\xCEV[\x90a\x0C\xCEV[`\0a\x03\0g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0B#\x86a\r\x12V[a\x0B-\x91\x90a\x1C\x93V[a\x0B7\x91\x90a\x1C\xD9V[a\x0E\xF2V[`\0a\x03\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x9BV[```\0a\x0B`\x86\x86\x86a\x0C\xC1V[\x90P`\0a\x0Bo\x87\x85\x88a\r\x05V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\tUV[```\x03\x82`@Q` \x01a\n\xD4\x92\x91\x90a\x1D\x07V[```\0a\x0B\xB7\x85\x85\x85a\x10\xC9V[\x90P`\0a\x0B\xC6\x86\x83\x86a\x10\xF4V[\x90P`\0a\x0B\xD6\x87\x84\x84\x88a\tpV[\x90Pa\x0B\xE5\x87\x84\x83\x85\x89a\x11-V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x94P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x0C\x1FWa\x0C\x1Fa\x19:V[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\x0C?Wa\x0C?a\x19:V[` \x02` \x01\x01\x81\x81RPP\x80\x83\x87`\0\x01Q\x88`@\x01Q\x89``\x01Q`@Q` \x01a\x0Cp\x95\x94\x93\x92\x91\x90a\x1D-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0a\x03Oa\x0C\xB0\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05\x99\x90a\nv\x90\x88\x90a\x0B\x0BV[`\0a\x03O\x82\x85\x85a\x10\x9BV[`\0a\x03\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x9BV[`\0a\x03\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12\x07V[`\0a\x03O\x83\x85\x84a\x10\x9BV[`\0a\x03O\x83\x85\x84a\x12\x07V[`\0\x80\x82\x13a\rTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\ra\x84a\x12&V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0F\rWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0FTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\rKV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xB3W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0a\x03O\x84a\x10\xEE\x85a\x10\xEE\x86`\0\x01Q\x87` \x01Qa\x0C\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x12\xCEV[\x80Q`\0\x90\x81\x90a\x11\x06\x90\x86\x90a\x0B\x0BV[\x90P`\0a\x11!\x84` \x01Q\x86a\x0B\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\n6\x82\x82a\x0C\xCEV[`\0\x82\x80\x85\x83\x81\x12\x15a\x11mW[`\0\x81\x12\x15a\x11hWa\x11S\x82a\x03\xE7a\x03\xE8a\x12\x07V[\x91Pa\x11a\x89\x89\x84\x88a\tpV[\x90Pa\x11;V[a\x11\x9AV[`\0\x81\x13\x15a\x11\x9AWa\x11\x85\x83a\x03\xE9a\x03\xE8a\x10\x9BV[\x92Pa\x11\x93\x89\x89\x85\x88a\tpV[\x90Pa\x11mV[`\0\x80a\x11\xD5\x8B\x8B\x85\x8A`@Q` \x01a\x11\xB7\x94\x93\x92\x91\x90a\x1DmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01a\x01\0a\x12\xE3a\x13\x10V[\x92PP\x91Pa\x11\xE6\x8B\x8B\x84\x8Aa\tpV[`\0\x03a\x11\xF5W\x81\x95Pa\x11\xF9V[\x80\x95P[PPPPP\x95\x94PPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12\x1FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x11a\x12cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\rKV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x03\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x07V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x12\xFD\x91\x90a\x1D\xB6V[\x93PP\x92P\x92Pa\x02\xAC\x83\x83\x87\x84a\tpV[`\0\x80`\0\x86\x88\x11\x15a\x13@W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\rKV[`\0a\x13P\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13b\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13p\x82\x84a\x1C\x93V[\x13\x15a\x13\x99W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\rKV[`\0a\x13\xA5\x8B\x8Ba\x19yV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x13\xBC\x87\x87a\x19fV[a\x13\xC6\x91\x90a\x1D\xF6V[\x96P`\0a\x13\xD8\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xE6\x86\x83a\x1C\x93V[\x13a\x13\xF3W\x87\x96Pa\x13\xFAV[\x87\x95P\x80\x94P[a\x14\x04\x8D\x8Da\x19yV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x14\x18WP\x88\x81\x10[a\x13\xB0WPPPP\x96P\x96P\x96\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14?W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x14iW\x81\x81\x01Q\x83\x82\x01R` \x01a\x14QV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\x8A\x81` \x86\x01` \x86\x01a\x14NV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\0` \x83\x01\x84a\x14rV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x14\xC6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE5W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x14\xF9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x15\x08W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x15\x1AW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x15?W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15[W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x88W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x031``\x83\x01\x84a\x14rV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xDAW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x15\xEFW`\0\x80\xFD[\x815a\x03\0\x81a\x15\xC5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x02\xB2V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16gWa\x16ga\x16.V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16gWa\x16ga\x16.V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16\xB9Wa\x16\xB9a\x16.V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\x16\xD7W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\x16\xF4W`\0\x80\xFD[Pa\x16\xFDa\x16DV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a\x17&\x81a\x15\xC5V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[\x80Qa\x17B\x81a\x15\xC5V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17YW`\0\x80\xFD[\x81Qa\x03\0\x81a\x15\xC5V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x17~Wa\x17~a\x16.V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x17\x99W`\0\x80\xFD[\x81Q` a\x17\xAEa\x17\xA9\x83a\x17dV[a\x16\x90V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17\xD0W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x17\xF5W\x80Qa\x17\xE8\x81a\x15\xC5V[\x83R\x91\x83\x01\x91\x83\x01a\x17\xD5V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x18\x11W`\0\x80\xFD[\x81Q` a\x18!a\x17\xA9\x83a\x17dV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x18CW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x17\xF5W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x18HV[`\0` \x82\x84\x03\x12\x15a\x18qW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x89W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x18\x9DW`\0\x80\xFD[a\x18\xA5a\x16mV[a\x18\xAE\x83a\x177V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x18\xC2W`\0\x80\xFD[a\x18\xCE\x87\x82\x86\x01a\x17\x88V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x18\xE6W`\0\x80\xFD[a\x18\xF2\x87\x82\x86\x01a\x18\0V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x19\x0E`\x80\x84\x01a\x177V[`\x80\x82\x01Ra\x19\x1F`\xA0\x84\x01a\x177V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xB2Wa\x02\xB2a\x19PV[\x81\x81\x03\x81\x81\x11\x15a\x02\xB2Wa\x02\xB2a\x19PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x19\xBDW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x19\xA1V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x1A6W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1A\x14V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x1AT\x81\x86a\x19\x8CV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x1A\x7Fa\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\t\xC3\x81\x85a\x14rV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[\x87Q\x80\x15\x15\x81\x14a\x1A\xDCW`\0\x80\xFD[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B%W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B=W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x1BQW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1BcWa\x1Bca\x16.V[a\x1Bv`\x1F\x82\x01`\x1F\x19\x16` \x01a\x16\x90V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x1B\x8DW`\0\x80\xFD[a\x1B\x9E\x81` \x84\x01` \x86\x01a\x14NV[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1B\xB9W`\0\x80\xFD[a\x1B\xC1a\x16DV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa\x1B\xE9\x81a\x15\xC5V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x06W`\0\x80\xFD[a\x03\0\x83\x83a\x1B\xA7V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C0Wa\x1C0a\x19PV[P\x92\x91PPV[`\x04\x81\x10a\x1CUWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a\x1Cg\x82\x86a\x1C7V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`@\x81\x01a\x1C\x86\x82\x85a\x1C7V[\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1C\xAFWa\x1C\xAFa\x19PV[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xB2Wa\x02\xB2a\x19PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1C\xE8Wa\x1C\xE8a\x1C\xC3V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1D\x02Wa\x1D\x02a\x19PV[P\x05\x90V[`@\x81\x01a\x1D\x15\x82\x85a\x1C7V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xA0\x81R`\0a\x1D@`\xA0\x83\x01\x88a\x19\x8CV[` \x83\x01\x96\x90\x96RP`@\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x90\x91\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x031``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x1D\xCCW`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa\x1D\xEB\x86``\x87\x01a\x1B\xA7V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a\x1E\x05Wa\x1E\x05a\x1C\xC3V[P\x04\x90V\xFE\xA2dipfsX\"\x12 Aj+\xBB:'\x01\xDB\x07\x1C\xC3\x9D\x8C6\xA0\xD5\xB6\xE4\xA6\x1D@=\x1E<\xF3\xCC\xC2\x1B\x10T\xFE\x83dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEANSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xF5W`\x005`\xE0\x1C\x80c\xB0\x9D\x04\xE5\x11a\0\x97W\x80c\xCE\x15;\xF4\x11a\0fW\x80c\xCE\x15;\xF4\x14a\x02\x16W\x80c\xDC\x17\x83U\x14a\x02DW\x80c\xDE\xF1_\x92\x14a\x02dW\x80c\xF2\xDEz{\x14a\x02wW`\0\x80\xFD[\x80c\xB0\x9D\x04\xE5\x14a\x01\xBBW\x80c\xC2\x93\x87\xE5\x14a\x01\xCEW\x80c\xC6a\xDB\xF5\x14a\x01\xF0W\x80c\xCB\x1FU2\x14a\x02\x03W`\0\x80\xFD[\x80c;M\x100\x11a\0\xD3W\x80c;M\x100\x14a\x01WW\x80cZ\x93\xB8\xCE\x14a\x01jW\x80c\x8C5\x82M\x14a\x01}W\x80c\xA8\xC6.v\x14a\x01\x90W`\0\x80\xFD[\x80c\x08TQ[\x14a\0\xFAW\x80c\x0FAf\xB8\x14a\x01#W\x80c%\th\xD9\x14a\x01DW[`\0\x80\xFD[a\x01\ra\x01\x086`\x04a\x14,V[a\x02\x8AV[`@Qa\x01\x1A\x91\x90a\x14\x9EV[`@Q\x80\x91\x03\x90\xF3[a\x016a\x0116`\x04a\x14\xB1V[a\x02\xB8V[`@Q\x90\x81R` \x01a\x01\x1AV[a\x01\ra\x01R6`\x04a\x14,V[a\x02\xF4V[a\x016a\x01e6`\x04a\x15-V[a\x03\x07V[a\x016a\x01x6`\x04a\x15FV[a\x03:V[a\x01\ra\x01\x8B6`\x04a\x14,V[a\x03WV[`\0Ta\x01\xA3\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01\x1AV[a\x01\ra\x01\xC96`\x04a\x15-V[a\x03yV[a\x01\xE1a\x01\xDC6`\x04a\x15rV[a\x03\x84V[`@Qa\x01\x1A\x93\x92\x91\x90a\x15\xA4V[a\x01\ra\x01\xFE6`\x04a\x14,V[a\x06\xD1V[a\x01\ra\x02\x116`\x04a\x15\xDDV[a\x06\xF3V[a\x02)a\x02$6`\x04a\x15-V[a\x06\xFEV[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01\x1AV[a\x02Wa\x02R6`\x04a\x15-V[a\x08>V[`@Qa\x01\x1A\x91\x90a\x15\xFAV[a\x01\ra\x02r6`\x04a\x16\xC1V[a\x08\xF6V[a\x016a\x02\x856`\x04a\x15FV[a\t\x03V[```\0\x80`\0a\x02\x9A\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\t\x18V[\x93PPPP[\x92\x91PPV[`\0\x80\x80\x80a\x02\xC9\x85\x87\x01\x87a\x15FV[\x92P\x92P\x92P`\0a\x02\xDA\x88a\x08>V[\x90Pa\x02\xE8\x84\x84\x84\x84a\tpV[\x98\x97PPPPPPPPV[``a\x03\0\x83\x83a\t\xCEV[\x93\x92PPPV[`\0\x80a\x03\x13\x83a\x08>V[\x90P`\0\x80a\x03!\x85a\x06\xFEV[P\x91P\x91Pa\x031\x82\x82\x85a\t\xFDV[\x95\x94PPPPPV[`\0a\x03O\x83\x83a\x03J\x87a\x08>V[a\n@V[\x94\x93PPPPV[```\0\x80`\0a\x03g\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\n}V[``a\x02\xB2\x82a\n\xBEV[`\0\x80```\0a\x03\x94\x88a\x08>V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xEAW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x0E\x91\x90a\x17GV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04;\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04XW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x80\x91\x90\x81\x01\x90a\x18_V[\x90Pa\x04\xCA`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81`@\x01Q\x89\x81Q\x81\x10a\x04\xE0Wa\x04\xE0a\x19:V[` \x02` \x01\x01Q\x81`@\x01\x81\x81RPP\x81`@\x01Q\x88\x81Q\x81\x10a\x05\x07Wa\x05\x07a\x19:V[` \x02` \x01\x01Q\x81``\x01\x81\x81RPP\x88`\0\x03a\x056W\x82Q`\x80\x82\x01R` \x83\x01Q`\xA0\x82\x01Ra\x05HV[` \x83\x01Q`\x80\x82\x01R\x82Q`\xA0\x82\x01R[a\x05e\x87\x82`@\x01Q\x84``\x01Q\x84`\x80\x01Q\x87`@\x01Qa\n\xEAV[`\xC0\x82\x01\x81\x90R``\x83\x01Q`\0\x91a\x05}\x91a\x19fV[\x90P`\0a\x05\x9F\x83`\x80\x01Q\x8A\x85`@\x01Qa\x05\x99\x91\x90a\x19fV[\x90a\x0B\x0BV[\x90P`\0a\x05\xCFa\x05\xC5\x85`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\x99\x85\x85a\x0B<V[\x90P\x80\x84``\x01Qa\x05\xE1\x91\x90a\x19yV[` \x85\x81\x01\x82\x90R`@\x80Q\x91\x82\x01\x8F\x90R\x81\x01\x8D\x90R``\x81\x01\x8C\x90R`\x80\x81\x01\x91\x90\x91R`\0\x93P`\xA0\x01\x91Pa\x06\x17\x90PV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x8E\x87\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06p\x94\x93\x92\x91\x90a\x19\xC8V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x8DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xB1\x91\x90a\x1A\xB1V[PPPP` \x95\x90\x95\x01Q\x91\x9E\x91\x9DP\x92\x9BP\x99PPPPPPPPPPV[```\0\x80`\0a\x06\xE1\x86a\x06\xFEV[\x92P\x92P\x92Pa\x02\xAC\x85\x84\x84\x84a\x0BQV[``a\x02\xB2\x82a\x0B\x92V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07UW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07y\x91\x90a\x17GV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07\xA6\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\xC3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xEB\x91\x90\x81\x01\x90a\x18_V[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\x08\x04Wa\x08\x04a\x19:V[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x08#Wa\x08#a\x19:V[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\x08r`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\xBBW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xE3\x91\x90\x81\x01\x90a\x1B\x13V[\x80` \x01\x90Q\x81\x01\x90a\x02\xB2\x91\x90a\x1B\xF4V[``a\x03O\x84\x84\x84a\x0B\xA8V[`\0a\x03O\x83\x83a\t\x13\x87a\x08>V[a\x0C\x8CV[```\0a\t'\x86\x86\x85a\x0C\xC1V[\x90P`\0a\t6\x87\x86\x86a\x0C\xC1V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[\x80Q`\0\x90\x81\x90a\t\x85\x90a\x05\x99\x88\x87a\x0B<V[\x90P`\0a\t\xA4\x84` \x01Qa\x05\x99\x87\x89a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0a\t\xB9\x83\x83a\x0C\xCEV[a\t\xC3\x91\x90a\x1C\x10V[\x97\x96PPPPPPPV[```\x02\x83\x83`@Q` \x01a\t\xE6\x93\x92\x91\x90a\x1CYV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\n\x17\x83` \x01Q\x85a\x0C\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Q\x90\x91P`\0\x90a\n*\x90\x87\x90a\x0C\xE3V[\x90Pa\n6\x82\x82a\x0C\xE3V[\x96\x95PPPPPPV[\x80Q`\0\x90a\x03O\x90a\n\\\x90g\r\xE0\xB6\xB3\xA7d\0\0\x90a\x0B<V[a\x05\x99a\nv\x85` \x01Q\x88a\x0B\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x86\x90a\x0B<V[```\0a\n\x8C\x86\x86\x86a\x0C\xF8V[\x90P`\0a\n\x9B\x87\x85\x87a\r\x05V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\tUV[```\x01\x82`@Q` \x01a\n\xD4\x92\x91\x90a\x1CxV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\n6a\n\xF9\x87\x87a\x0B<V[a\x0B\x05\x86\x81\x87\x87a\x0C\xCEV[\x90a\x0C\xCEV[`\0a\x03\0g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0B#\x86a\r\x12V[a\x0B-\x91\x90a\x1C\x93V[a\x0B7\x91\x90a\x1C\xD9V[a\x0E\xF2V[`\0a\x03\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x10\x9BV[```\0a\x0B`\x86\x86\x86a\x0C\xC1V[\x90P`\0a\x0Bo\x87\x85\x88a\r\x05V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\tUV[```\x03\x82`@Q` \x01a\n\xD4\x92\x91\x90a\x1D\x07V[```\0a\x0B\xB7\x85\x85\x85a\x10\xC9V[\x90P`\0a\x0B\xC6\x86\x83\x86a\x10\xF4V[\x90P`\0a\x0B\xD6\x87\x84\x84\x88a\tpV[\x90Pa\x0B\xE5\x87\x84\x83\x85\x89a\x11-V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x94P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x0C\x1FWa\x0C\x1Fa\x19:V[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\x0C?Wa\x0C?a\x19:V[` \x02` \x01\x01\x81\x81RPP\x80\x83\x87`\0\x01Q\x88`@\x01Q\x89``\x01Q`@Q` \x01a\x0Cp\x95\x94\x93\x92\x91\x90a\x1D-V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0a\x03Oa\x0C\xB0\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0B<\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x83Qa\x05\x99\x90a\nv\x90\x88\x90a\x0B\x0BV[`\0a\x03O\x82\x85\x85a\x10\x9BV[`\0a\x03\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x10\x9BV[`\0a\x03\0\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x12\x07V[`\0a\x03O\x83\x85\x84a\x10\x9BV[`\0a\x03O\x83\x85\x84a\x12\x07V[`\0\x80\x82\x13a\rTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\ra\x84a\x12&V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x0F\rWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x0FTW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\rKV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x10\xB3W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0a\x03O\x84a\x10\xEE\x85a\x10\xEE\x86`\0\x01Q\x87` \x01Qa\x0C\xE3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90a\x12\xCEV[\x80Q`\0\x90\x81\x90a\x11\x06\x90\x86\x90a\x0B\x0BV[\x90P`\0a\x11!\x84` \x01Q\x86a\x0B\x0B\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\n6\x82\x82a\x0C\xCEV[`\0\x82\x80\x85\x83\x81\x12\x15a\x11mW[`\0\x81\x12\x15a\x11hWa\x11S\x82a\x03\xE7a\x03\xE8a\x12\x07V[\x91Pa\x11a\x89\x89\x84\x88a\tpV[\x90Pa\x11;V[a\x11\x9AV[`\0\x81\x13\x15a\x11\x9AWa\x11\x85\x83a\x03\xE9a\x03\xE8a\x10\x9BV[\x92Pa\x11\x93\x89\x89\x85\x88a\tpV[\x90Pa\x11mV[`\0\x80a\x11\xD5\x8B\x8B\x85\x8A`@Q` \x01a\x11\xB7\x94\x93\x92\x91\x90a\x1DmV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01a\x01\0a\x12\xE3a\x13\x10V[\x92PP\x91Pa\x11\xE6\x8B\x8B\x84\x8Aa\tpV[`\0\x03a\x11\xF5W\x81\x95Pa\x11\xF9V[\x80\x95P[PPPPP\x95\x94PPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x12\x1FW`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x11a\x12cW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\rKV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0a\x03\0\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x12\x07V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x12\xFD\x91\x90a\x1D\xB6V[\x93PP\x92P\x92Pa\x02\xAC\x83\x83\x87\x84a\tpV[`\0\x80`\0\x86\x88\x11\x15a\x13@W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\rKV[`\0a\x13P\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13b\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13p\x82\x84a\x1C\x93V[\x13\x15a\x13\x99W`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\rKV[`\0a\x13\xA5\x8B\x8Ba\x19yV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x13\xBC\x87\x87a\x19fV[a\x13\xC6\x91\x90a\x1D\xF6V[\x96P`\0a\x13\xD8\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x13\xE6\x86\x83a\x1C\x93V[\x13a\x13\xF3W\x87\x96Pa\x13\xFAV[\x87\x95P\x80\x94P[a\x14\x04\x8D\x8Da\x19yV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x14\x18WP\x88\x81\x10[a\x13\xB0WPPPP\x96P\x96P\x96\x93PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x14?W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x14iW\x81\x81\x01Q\x83\x82\x01R` \x01a\x14QV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\x8A\x81` \x86\x01` \x86\x01a\x14NV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\0` \x83\x01\x84a\x14rV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x14\xC6W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x14\xE5W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x14\xF9W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x15\x08W`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x15\x1AW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x15?W`\0\x80\xFD[P5\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x15[W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x88W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x031``\x83\x01\x84a\x14rV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x15\xDAW`\0\x80\xFD[PV[`\0` \x82\x84\x03\x12\x15a\x15\xEFW`\0\x80\xFD[\x815a\x03\0\x81a\x15\xC5V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q\x90\x82\x01R``\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R`\x80\x81\x01a\x02\xB2V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16gWa\x16ga\x16.V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16gWa\x16ga\x16.V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x16\xB9Wa\x16\xB9a\x16.V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\x16\xD7W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\x16\xF4W`\0\x80\xFD[Pa\x16\xFDa\x16DV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a\x17&\x81a\x15\xC5V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[\x80Qa\x17B\x81a\x15\xC5V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x17YW`\0\x80\xFD[\x81Qa\x03\0\x81a\x15\xC5V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x17~Wa\x17~a\x16.V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x17\x99W`\0\x80\xFD[\x81Q` a\x17\xAEa\x17\xA9\x83a\x17dV[a\x16\x90V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17\xD0W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x17\xF5W\x80Qa\x17\xE8\x81a\x15\xC5V[\x83R\x91\x83\x01\x91\x83\x01a\x17\xD5V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x18\x11W`\0\x80\xFD[\x81Q` a\x18!a\x17\xA9\x83a\x17dV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x18CW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x17\xF5W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x18HV[`\0` \x82\x84\x03\x12\x15a\x18qW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x18\x89W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x18\x9DW`\0\x80\xFD[a\x18\xA5a\x16mV[a\x18\xAE\x83a\x177V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x18\xC2W`\0\x80\xFD[a\x18\xCE\x87\x82\x86\x01a\x17\x88V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x18\xE6W`\0\x80\xFD[a\x18\xF2\x87\x82\x86\x01a\x18\0V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x19\x0E`\x80\x84\x01a\x177V[`\x80\x82\x01Ra\x19\x1F`\xA0\x84\x01a\x177V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xB2Wa\x02\xB2a\x19PV[\x81\x81\x03\x81\x81\x11\x15a\x02\xB2Wa\x02\xB2a\x19PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x19\xBDW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x19\xA1V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x1A6W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x1A\x14V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x1AT\x81\x86a\x19\x8CV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x1A\x7Fa\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\t\xC3\x81\x85a\x14rV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x1A\xCCW`\0\x80\xFD[\x87Q\x80\x15\x15\x81\x14a\x1A\xDCW`\0\x80\xFD[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1B%W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1B=W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x1BQW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1BcWa\x1Bca\x16.V[a\x1Bv`\x1F\x82\x01`\x1F\x19\x16` \x01a\x16\x90V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x1B\x8DW`\0\x80\xFD[a\x1B\x9E\x81` \x84\x01` \x86\x01a\x14NV[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a\x1B\xB9W`\0\x80\xFD[a\x1B\xC1a\x16DV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa\x1B\xE9\x81a\x15\xC5V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x06W`\0\x80\xFD[a\x03\0\x83\x83a\x1B\xA7V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C0Wa\x1C0a\x19PV[P\x92\x91PPV[`\x04\x81\x10a\x1CUWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a\x1Cg\x82\x86a\x1C7V[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[`@\x81\x01a\x1C\x86\x82\x85a\x1C7V[\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1C\xAFWa\x1C\xAFa\x19PV[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xB2Wa\x02\xB2a\x19PV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1C\xE8Wa\x1C\xE8a\x1C\xC3V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1D\x02Wa\x1D\x02a\x19PV[P\x05\x90V[`@\x81\x01a\x1D\x15\x82\x85a\x1C7V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xA0\x81R`\0a\x1D@`\xA0\x83\x01\x88a\x19\x8CV[` \x83\x01\x96\x90\x96RP`@\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R`\x01`\x01`\xA0\x1B\x03\x16`\x80\x90\x91\x01R\x91\x90PV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x031``\x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x1D\xCCW`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa\x1D\xEB\x86``\x87\x01a\x1B\xA7V[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a\x1E\x05Wa\x1E\x05a\x1C\xC3V[P\x04\x90V\xFE\xA2dipfsX\"\x12 Aj+\xBB:'\x01\xDB\x07\x1C\xC3\x9D\x8C6\xA0\xD5\xB6\xE4\xA6\x1D@=\x1E<\xF3\xCC\xC2\x1B\x10T\xFE\x83dsolcC\0\x08\x16\x003";
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
        /// Calls the contract's `prepareAllocationDeltasGivenDeltaL`
        /// (0x0854515b) function
        pub fn prepare_allocation_deltas_given_delta_l(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_l: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([8, 84, 81, 91], (pool_id, delta_l))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareAllocationDeltasGivenDeltaX`
        /// (0xc661dbf5) function
        pub fn prepare_allocation_deltas_given_delta_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([198, 97, 219, 245], (pool_id, delta_x))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareAllocationDeltasGivenDeltaY`
        /// (0x8c35824d) function
        pub fn prepare_allocation_deltas_given_delta_y(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([140, 53, 130, 77], (pool_id, delta_y))
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
        /// Calls the contract's `simulateSwap` (0xc29387e5) function
        pub fn simulate_swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            token_in_index: ::ethers::core::types::U256,
            token_out_index: ::ethers::core::types::U256,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash(
                    [194, 147, 135, 229],
                    (pool_id, token_in_index, token_out_index, amount_in),
                )
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
    /// `prepareAllocationDeltasGivenDeltaL` function with signature
    /// `prepareAllocationDeltasGivenDeltaL(uint256,uint256)` and selector
    /// `0x0854515b`
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
        name = "prepareAllocationDeltasGivenDeltaL",
        abi = "prepareAllocationDeltasGivenDeltaL(uint256,uint256)"
    )]
    pub struct PrepareAllocationDeltasGivenDeltaLCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `prepareAllocationDeltasGivenDeltaX` function with signature
    /// `prepareAllocationDeltasGivenDeltaX(uint256,uint256)` and selector
    /// `0xc661dbf5`
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
        name = "prepareAllocationDeltasGivenDeltaX",
        abi = "prepareAllocationDeltasGivenDeltaX(uint256,uint256)"
    )]
    pub struct PrepareAllocationDeltasGivenDeltaXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `prepareAllocationDeltasGivenDeltaY` function with signature
    /// `prepareAllocationDeltasGivenDeltaY(uint256,uint256)` and selector
    /// `0x8c35824d`
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
        name = "prepareAllocationDeltasGivenDeltaY",
        abi = "prepareAllocationDeltasGivenDeltaY(uint256,uint256)"
    )]
    pub struct PrepareAllocationDeltasGivenDeltaYCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
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
    /// with signature `simulateSwap(uint256,uint256,uint256,uint256)` and
    /// selector `0xc29387e5`
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
        name = "simulateSwap",
        abi = "simulateSwap(uint256,uint256,uint256,uint256)"
    )]
    pub struct SimulateSwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub token_in_index: ::ethers::core::types::U256,
        pub token_out_index: ::ethers::core::types::U256,
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
        CheckSwapConstant(CheckSwapConstantCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPoolParams(GetPoolParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareAllocationDeltasGivenDeltaL(PrepareAllocationDeltasGivenDeltaLCall),
        PrepareAllocationDeltasGivenDeltaX(PrepareAllocationDeltasGivenDeltaXCall),
        PrepareAllocationDeltasGivenDeltaY(PrepareAllocationDeltasGivenDeltaYCall),
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
                <CheckSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSwapConstant(decoded));
            }
            if let Ok(decoded) =
                <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInitialPoolData(decoded));
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
                <PrepareAllocationDeltasGivenDeltaLCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PrepareAllocationDeltasGivenDeltaL(decoded));
            }
            if let Ok(decoded) =
                <PrepareAllocationDeltasGivenDeltaXCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PrepareAllocationDeltasGivenDeltaX(decoded));
            }
            if let Ok(decoded) =
                <PrepareAllocationDeltasGivenDeltaYCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PrepareAllocationDeltasGivenDeltaY(decoded));
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
                Self::CheckSwapConstant(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextReserveX(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetNextReserveY(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InternalPrice(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareAllocationDeltasGivenDeltaL(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareAllocationDeltasGivenDeltaX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareAllocationDeltasGivenDeltaY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::CheckSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareAllocationDeltasGivenDeltaL(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareAllocationDeltasGivenDeltaX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareAllocationDeltasGivenDeltaY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightXUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSwapConstantCall> for GeometricMeanSolverCalls {
        fn from(value: CheckSwapConstantCall) -> Self {
            Self::CheckSwapConstant(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for GeometricMeanSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
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
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaLCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaLCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaL(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaXCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaXCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaYCall> for GeometricMeanSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaYCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaY(value)
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
    /// Container type for all return fields from the
    /// `prepareAllocationDeltasGivenDeltaL` function with signature
    /// `prepareAllocationDeltasGivenDeltaL(uint256,uint256)` and selector
    /// `0x0854515b`
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
    pub struct PrepareAllocationDeltasGivenDeltaLReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the
    /// `prepareAllocationDeltasGivenDeltaX` function with signature
    /// `prepareAllocationDeltasGivenDeltaX(uint256,uint256)` and selector
    /// `0xc661dbf5`
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
    pub struct PrepareAllocationDeltasGivenDeltaXReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the
    /// `prepareAllocationDeltasGivenDeltaY` function with signature
    /// `prepareAllocationDeltasGivenDeltaY(uint256,uint256)` and selector
    /// `0x8c35824d`
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
    pub struct PrepareAllocationDeltasGivenDeltaYReturn(pub ::ethers::core::types::Bytes);
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
    /// with signature `simulateSwap(uint256,uint256,uint256,uint256)` and
    /// selector `0xc29387e5`
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

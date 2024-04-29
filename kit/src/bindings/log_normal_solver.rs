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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0+F8\x03\x80b\0+F\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0ZV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ub\0\0\x8CV[`\0` \x82\x84\x03\x12\x15b\0\0mW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\x85W`\0\x80\xFD[\x93\x92PPPV[a*\xAA\x80b\0\0\x9C`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xADW\x80c\xCE\x15;\xF4\x11a\0qW\x80c\xCE\x15;\xF4\x14a\x02\x8FW\x80c\xDC\x17\x83U\x14a\x02\xBDW\x80c\xDE\xF1_\x92\x14a\x02\xDDW\x80c\xEA\xAE\x17\xBA\x14a\x01ZW\x80c\xF9\xC2\x82\x11\x14a\x02\xF0W`\0\x80\xFD[\x80c\xA8\xC6.v\x14a\x02\x18W\x80c\xAFNC\x7F\x14a\x02CW\x80c\xB0\x9D\x04\xE5\x14a\x02VW\x80c\xC6a\xDB\xF5\x14a\x02iW\x80c\xCB\x1FU2\x14a\x02|W`\0\x80\xFD[\x80c;M\x100\x11a\0\xF4W\x80c;M\x100\x14a\x01\xC4W\x80cN\x81\x7F\xD9\x14a\x01\xD7W\x80c^\xB4\x08\xFC\x14a\x01\xEAW\x80cme\"\x99\x14a\x01\xFDW\x80c\x8C5\x82M\x14a\x02\x05W`\0\x80\xFD[\x80c\x08TQ[\x14a\x011W\x80c\x0F\x85z\xB9\x14a\x01ZW\x80c\x12\x06I\xC5\x14a\x01mW\x80c\x1E\x97\x8C\xB0\x14a\x01\x8EW\x80c9(\xFF\x97\x14a\x01\xA1W[`\0\x80\xFD[a\x01Da\x01?6`\x04a \xD0V[a\x02\xF8V[`@Qa\x01Q\x91\x90a!BV[`@Q\x80\x91\x03\x90\xF3[a\x01Da\x01h6`\x04a \xD0V[a\x03&V[a\x01\x80a\x01{6`\x04a!UV[a\x039V[`@Q\x90\x81R` \x01a\x01QV[a\x01\x80a\x01\x9C6`\x04a!\x87V[a\x03\x81V[a\x01\xB4a\x01\xAF6`\x04a!\xC4V[a\x03\x96V[`@Qa\x01Q\x94\x93\x92\x91\x90a!\xFCV[a\x01\x80a\x01\xD26`\x04a\"#V[a\x07\xD9V[a\x01\x80a\x01\xE56`\x04a!\x87V[a\x07\xFAV[a\x01\x80a\x01\xF86`\x04a!UV[a\x08\x0FV[a\x01\x80`\0\x81V[a\x01Da\x02\x136`\x04a \xD0V[a\x08IV[`\0Ta\x02+\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01QV[a\x01\x80a\x02Q6`\x04a!UV[a\x08kV[a\x01Da\x02d6`\x04a\"#V[a\x08\xA1V[a\x01Da\x02w6`\x04a \xD0V[a\x08\xACV[a\x01Da\x02\x8A6`\x04a\"QV[a\x08\xCEV[a\x02\xA2a\x02\x9D6`\x04a\"#V[a\x08\xD9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01QV[a\x02\xD0a\x02\xCB6`\x04a\"#V[a\n\x19V[`@Qa\x01Q\x91\x90a\"\x9AV[a\x01Da\x02\xEB6`\x04a#;V[a\n\xD1V[a\x01\x80`x\x81V[```\0\x80`\0a\x03\x08\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\n\xDEV[\x93PPPP[\x92\x91PPV[``a\x032\x83\x83a\x0B6V[\x93\x92PPPV[`\0\x80a\x03E\x86a\n\x19V[\x90P`\0a\x03T\x85\x85\x84a\x0BeV[\x90P`\0a\x03d\x87\x83\x88\x86a\x0B\xACV[\x90Pa\x03s\x87\x87\x83\x85\x87a\x0C\x05V[\x93PPPP[\x94\x93PPPPV[`\0a\x03y\x83\x83a\x03\x91\x87a\n\x19V[a\x0C\xDEV[`\0\x80`\0``a\x03\xC1`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80`\0a\x03\xCF\x8Ba\x08\xD9V[\x92P\x92P\x92P`\0a\x03\xE0\x8Ca\n\x19V[\x90Pa\x04\x06`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x04\x14\x8E\x87\x87\x87a\x08kV[\x90P\x8C\x15a\x04\xF9Wa\x04)\x8C\x87\x87\x87\x87a\rNV[` \x83\x01Ra\x048\x8C\x87a#\xC7V[\x87R` \x82\x01Qa\x04I\x90\x82a#\xC7V[\x87`@\x01\x81\x81RPP`\0a\x04g\x8F\x89`\0\x01Q\x8A`@\x01Qa\x03\x81V[\x90Pa\x04}\x8F\x89`\0\x01Q\x8A`@\x01Q\x84a\x039V[` \x89\x01\x81\x90R\x86\x11a\x04\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[` \x88\x01Qa\x04\xF1\x90\x87a#\xDAV[\x83RPa\x05\xCBV[a\x05\x06\x8C\x87\x87\x87\x87a\r\xB1V[` \x83\x01Ra\x05\x15\x8C\x86a#\xC7V[` \x80\x89\x01\x91\x90\x91R\x82\x01Qa\x05+\x90\x82a#\xC7V[\x87`@\x01\x81\x81RPP`\0a\x05I\x8F\x89` \x01Q\x8A`@\x01Qa\x07\xFAV[\x90Pa\x05_\x8F\x89` \x01Q\x8A`@\x01Q\x84a\x08\x0FV[\x80\x89R\x87\x11a\x05\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x04\xD9V[\x87Qa\x05\xC7\x90\x88a#\xDAV[\x83RP[Pa\x06'`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837PPP`@\x82\x01\x81\x90R\x87Q\x81Q\x90\x91\x90`\0\x90a\x06aWa\x06aa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x86` \x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x06\x89Wa\x06\x89a#\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R``\x80\x82\x01\x85\x90R\x8D\x15a\x06\xDAWP\x81Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8F\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x07\x0EV[P\x81Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8F\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x8F\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x84\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07[\x94\x93\x92\x91\x90a$?V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x9C\x91\x90a%(V[PPPPPP\x90P\x80\x85`\0\x01Qa\x07\xBD\x8C`\0\x01Q\x8D`@\x01Q\x8Aa\x0C\xDEV[\x85\x9DP\x9DP\x9DP\x9DPPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\x07\xE7\x84a\x08\xD9V[\x92PP\x91Pa\x03y\x82\x82a\x03\x91\x87a\n\x19V[`\0a\x03y\x83\x83a\x08\n\x87a\n\x19V[a\r\xFEV[`\0\x80a\x08\x1B\x86a\n\x19V[\x90P`\0a\x08*\x85\x85\x84a\x0EXV[\x90P`\0a\x08:\x82\x88\x88\x86a\x0B\xACV[\x90Pa\x03s\x87\x87\x83\x85\x87a\x0E\x9DV[```\0\x80`\0a\x08Y\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x0FwV[`\0\x80a\x08w\x86a\n\x19V[\x90P`\0a\x08\x87\x86\x86\x86\x85a\x0B\xACV[\x90Pa\x08\x96\x86\x86\x83\x87\x86a\x0F\xB8V[\x97\x96PPPPPPPV[``a\x03 \x82a\x10\xC5V[```\0\x80`\0a\x08\xBC\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x10\xF1V[``a\x03 \x82a\x112V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tT\x91\x90a%\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x81\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xC6\x91\x90\x81\x01\x90a&\x9DV[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\t\xDFWa\t\xDFa#\xEDV[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\t\xFEWa\t\xFEa#\xEDV[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\nM`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xBE\x91\x90\x81\x01\x90a'xV[\x80` \x01\x90Q\x81\x01\x90a\x03 \x91\x90a(YV[``a\x03y\x84\x84\x84a\x11HV[```\0a\n\xED\x86\x86\x85a\x12\x1CV[\x90P`\0a\n\xFC\x87\x86\x86a\x12\x1CV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[```\x02\x83\x83`@Q` \x01a\x0BN\x93\x92\x91\x90a(\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0Br\x84\x84a\x12)V[\x90P`\0a\x0B\x87a\x0B\x82\x83a\x12pV[a\x12\xD9V[\x84Q\x90\x91Pa\x0B\xA2\x90\x82\x90a\x0B\x9C\x90\x89a\x13\"V[\x90a\x13\"V[\x96\x95PPPPPPV[`\0\x80a\x0B\xC1a\x0B\xBC\x87\x86a\x137V[a\x13LV[\x90P`\0a\x0B\xE9a\x0B\xBCa\x0B\xE2\x86`\0\x01Q\x88a\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x137V[` \x85\x01Q\x90\x91Pa\x0B\xFB\x82\x84a(\xB6V[a\x08\x96\x91\x90a(\xB6V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0CEW[`\0\x81\x12\x15a\x0C@Wa\x0C+\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92Pa\x0C9\x89\x84\x8A\x88a\x0B\xACV[\x90Pa\x0C\x13V[a\x0CrV[`\0\x81\x13\x15a\x0CrWa\x0C]\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x91Pa\x0Ck\x89\x83\x8A\x88a\x0B\xACV[\x90Pa\x0CEV[`\0\x80a\x0C\xAC\x8B\x8B\x85\x8A`@Q` \x01a\x0C\x8F\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x146a\x14cV[P\x91P\x91Pa\x0C\xBD\x8B\x83\x8C\x8Aa\x0B\xACV[`\0\x03a\x0C\xCCW\x81\x95Pa\x0C\xD0V[\x80\x95P[PPPPP\x95\x94PPPPPV[`\0\x80a\x0C\xEE\x83` \x01Qa\x15\x7FV[\x90P`\0a\r\x11a\x0C\xFF\x87\x87a\x137V[a\x0B\xBC\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x90P`\0a\r?\x83a\r0\x87` \x01Q\x85a\x15\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r:\x91\x90a)\0V[a\x15\xD0V[\x85Q\x90\x91Pa\x08\x96\x90\x82a\x13\"V[`\0\x80a\rh\x87\x84`@\x01Qa\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\rw\x87\x86\x86a\x0C\xDEV[\x90Pa\r\xA5\x86a\r\x87\x83\x8Aa\x17yV[a\r\x91\x91\x90a#\xC7V[a\r\x9F\x84a\x0B\x9C\x85\x8Aa\x13\"V[\x90a\x137V[\x98\x97PPPPPPPPV[`\0\x80a\r\xCB\x87\x84`@\x01Qa\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\r\xDA\x87\x86\x86a\x0C\xDEV[\x90Pa\r\xA5\x86a\r\xEA\x83\x8Aa\x17yV[a\r\xF4\x91\x90a#\xC7V[a\r\x9F\x87\x85a\x13\"V[`\0\x80a\x0E\x0E\x83` \x01Qa\x15\x7FV[\x90P`\0a\x0E/a\x0B\xBCa\x0B\xE2\x87\x87`\0\x01Qa\x17y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\r?\x83a\x0EN\x87` \x01Q\x85a\x15\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r:\x91\x90a(\xB6V[`\0\x80a\x0Ee\x84\x84a\x17\x8EV[\x90P`\0a\x0Er\x82a\x12pV[\x90P`\0a\x0E\x7F\x82a\x12\xD9V[\x90Pa\x08\x96a\x0E\x96\x82g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x88\x90a\x13\"V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0E\xEEW[`\0\x81\x12\x15a\x0E\xE9Wa\x0E\xC3\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92P\x87\x83\x11a\x0E\xD2W\x82a\x0E\xD4V[\x87[\x92Pa\x0E\xE2\x83\x8A\x8A\x88a\x0B\xACV[\x90Pa\x0E\xABV[a\x0F,V[`\0\x81\x13\x15a\x0F,Wa\x0F\x06\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x91P\x87\x82\x11a\x0F\x15W\x81a\x0F\x17V[\x87[\x91Pa\x0F%\x82\x8A\x8A\x88a\x0B\xACV[\x90Pa\x0E\xEEV[`\0\x80a\x0Ff\x8B\x8B\x85\x8A`@Q` \x01a\x0FI\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x17\xC6a\x14cV[P\x91P\x91Pa\x0C\xBD\x82\x8C\x8C\x8Aa\x0B\xACV[```\0a\x0F\x86\x86\x86\x86a\x17\xF3V[\x90P`\0a\x0F\x95\x87\x85\x87a\x18\0V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\x0B\x1BV[`\0\x82\x80\x85\x83\x81\x12\x15a\x10MW[`\0\x81\x12\x15a\x10HWa\x0F\xDE\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x85Q\x90\x92P`\0\x90a\x0F\xF1\x90\x8A\x90a\x137V[\x8A\x11a\x10\x15W\x85Qa\x10\x04\x90\x8A\x90a\x137V[a\x10\x10\x90a\x03\xE8a#\xC7V[a\x10!V[a\x10!\x8Aa\x03\xE8a#\xC7V[\x90P\x89\x83\x10a\x100W\x82a\x102V[\x80[\x92Pa\x10@\x8A\x8A\x85\x89a\x0B\xACV[\x91PPa\x0F\xC6V[a\x10zV[`\0\x81\x13\x15a\x10zWa\x10e\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92Pa\x10s\x89\x89\x85\x88a\x0B\xACV[\x90Pa\x10MV[`\0\x80a\x10\xB4\x8B\x8B\x85\x8A`@Q` \x01a\x10\x97\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01`\x80a\x18\ra\x14cV[\x92PP\x91Pa\x0C\xBD\x8B\x8B\x84\x8Aa\x0B\xACV[```\x01\x82`@Q` \x01a\x10\xDB\x92\x91\x90a)'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\0a\x11\0\x86\x86\x86a\x12\x1CV[\x90P`\0a\x11\x0F\x87\x85\x88a\x18\0V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\x0B\x1BV[```\x04\x82`@Q` \x01a\x10\xDB\x92\x91\x90a)BV[```\0a\x11W\x85\x85\x85a\x18:V[\x90P`\0a\x11f\x82\x86\x86a\x0BeV[\x90P`\0a\x11v\x87\x83\x85\x88a\x0B\xACV[\x90Pa\x11\x85\x87\x83\x83\x86\x89a\x0F\xB8V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x95P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x11\xBFWa\x11\xBFa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x01\x81Q\x81\x10a\x11\xDFWa\x11\xDFa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x87`@Q` \x01a\x12\0\x93\x92\x91\x90a)hV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0a\x03y\x82\x85\x85a\x13\xE9V[`\0\x80a\x12:\x84\x84`\0\x01Qa\x18uV[\x90P`\0a\x12K\x84` \x01Qa\x15\x7FV[\x90Pa\x12g\x84` \x01Q\x82\x84a\x12a\x91\x90a)\0V[\x90a\x18\x89V[\x95\x94PPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x12\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x85a)\x90V[a\x12\x98\x91\x90a)\xD6V[\x90P`\0a\x12\xA5\x82a*\x04V[\x90P`\0a\x12\xB2\x82a\x18\xADV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x12\xCFg\r\xE0\xB6\xB3\xA7d\0\0\x83a)\x90V[a\x12g\x91\x90a)\xD6V[`\0\x80\x82\x12\x15a\x13\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x04\xD9V[P\x90V[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xE9V[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x14\x17V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13eWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x13\x8DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x13\xAEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xBB\x83`\x02a)\x90V[\x90P`\0a\x13\xC8\x82a\x1A\x96V[\x90P`\0a\x13\xDEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1D\x0FV[\x90Pa\x12g\x81a*\x04V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x14\x01W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x14/W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x14P\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x83\x86\x84\x84a\x0B\xACV[`\0\x80`\0\x86\x88\x11\x15a\x14\x93W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x04\xD9V[`\0a\x14\xA3\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14\xB5\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14\xC3\x82\x84a)\x90V[\x13\x15a\x14\xECW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x04\xD9V[`\0a\x14\xF8\x8B\x8Ba#\xDAV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x15\x0F\x87\x87a#\xC7V[a\x15\x19\x91\x90a*`V[\x96P`\0a\x15+\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x159\x86\x83a)\x90V[\x13a\x15FW\x87\x96Pa\x15MV[\x87\x95P\x80\x94P[a\x15W\x8D\x8Da#\xDAV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x15kWP\x88\x81\x10[a\x15\x03WPPPP\x96P\x96P\x96\x93PPPPV[`\0a\x03 a\x15\x8E\x83\x80a\x13\"V[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x17yV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x15\xBFW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\xEBWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x162W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04\xD9V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x14\x17V[`\0\x80a\x17\x9F\x84\x84`\0\x01Qa\x18uV[\x90P`\0a\x17\xB0\x84` \x01Qa\x15\x7FV[\x90Pa\x12g\x84` \x01Q\x82\x84a\x12a\x91\x90a(\xB6V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\xE0\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x0B\xACV[`\0a\x03y\x83\x85\x84a\x13\xE9V[`\0a\x03y\x83\x85\x84a\x14\x17V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18'\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x83\x83\x87\x84a\x0B\xACV[`\0\x80a\x18G\x84\x84a\x17\x8EV[\x90P`\0a\x18Wa\x0B\x82\x83a\x12pV[\x90Pa\x0B\xA2a\x18n\x82g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x87\x90a\x1D$V[`\0a\x032a\x18\x84\x84\x84a\x1D$V[a\x1D9V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x18\xA7W`\0\x80\xFD[\x05\x91\x90PV[`\0\x81`\0\x03a\x18\xC6WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x18\xDDWP`\0\x91\x90PV[a\x18\xEEgV\x98\xEE\xF0fp\0\0a*\x04V[\x82\x13a\x19\x03WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x19\x0E\x83a\x1F\x14V[\x90P`\0a\x19Gg\r\xE0\xB6\xB3\xA7d\0\0a\x190\x84g\x1B\xC1mgN\xC8\0\0a\x137V[a\x19B\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\xB6V[a\x1FPV[\x90P`\0\x80\x82a\x19\xA8\x81a\x19\x95\x81a\x19\x83\x81a\x19k\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1D\x0FV[a\x19~\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a(\xB6V[a\x1D\x0FV[a\x19~\x90g\x14\xA8EL\x19\xE1\xAC\0a(\xB6V[a\x19~\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a(\xB6V[a\x19\xBA\x90g\x03\xDE\xBD\x08;\x8C|\0a(\xB6V[\x91P\x83\x90Pa\x1A\"\x81a\x1A\x10\x81a\x19\xFE\x81a\x19\xEC\x81a\x19\xD9\x81\x8Ba\x1D\x0FV[a\x19~\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a(\xB6V[a\x19~\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a(\xB6V[a\x19~\x90g\x051\n\xA7\xD5!0\0a(\xB6V[a\x19~\x90g\r\xE0\xCC=\x15a\0\0a(\xB6V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1A8\x87\x88a\x1D\x0FV[a\x1AD\x90`\0\x19a)\x90V[a\x1AN\x91\x90a)\0V[a\x1AX\x91\x90a(\xB6V[\x92PP`\0a\x1Af\x83a\x15\xD0V[\x90P`\0a\x1At\x85\x83a\x1D\x0FV[\x90P`\0\x88\x12a\x1A\x84W\x80a\r\xA5V[a\r\xA5\x81g\x1B\xC1mgN\xC8\0\0a)\0V[`\0\x80\x82\x12\x80a\x1A\xADWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1A\xCBW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1A\xECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1B\x14W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1B\x1FW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1BGWa\x1BB\x83g\x1B\xC1mgN\xC8\0\0a)\0V[a\x1BIV[\x82[\x90P`\0a\x1B_\x82g\x1B\xC1mgN\xC8\0\0a\x1FPV[\x90P\x80`\0\x03a\x1B\x82W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x8D\x82a\x1D9V[\x90P`\0c;\x9A\xCA\0a\x1B\xB8a\x1B\xB3a\x1B\xADg\x1B\xC1mgN\xC8\0\0a*\x04V[\x85a\x1D\x0FV[a\x1FeV[a\x1B\xC2\x91\x90a)\x90V[\x90P`\0\x80a\x1B\xD9\x83g\x03\xC1f\\z\xAB \0a\x1D\x0FV[a\x1B\xEB\x90g \x05\xFEO&\x8E\xA0\0a(\xB6V[\x90P`\0a\x1C\x16\x84a\x1C\x04\x86f\x9F2u$b\xA0\0a\x1D\x0FV[a\x19~\x90g\r\xC5R\x7Fd, \0a(\xB6V[a\x1C(\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\xB6V[\x90Pa\x1CLg\t\xD0(\xCCo _\xFF\x19\x85a\x1CB\x85\x85a\x1FPV[a\x19~\x91\x90a)\0V[\x92PPP`\0[`\x02\x81\x10\x15a\x1C\xE7W`\0\x86a\x1Ch\x84a\x18\xADV[a\x1Cr\x91\x90a)\0V[\x90P`\0a\x1C\x80\x84\x85a\x1D\x0FV[a\x1C\x89\x90a*\x04V[\x90P`\0a\x1C\x96\x82a\x15\xD0V[\x90P`\0a\x1C\xA4\x86\x85a\x1D\x0FV[a\x1C\xB6g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1D\x0FV[a\x1C\xC0\x91\x90a)\0V[\x90Pa\x1C\xCC\x84\x82a\x1FPV[a\x1C\xD6\x90\x87a(\xB6V[\x95P\x84`\x01\x01\x94PPPPPa\x1CSV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1D\x04Wa\x1C\xFF\x82a*\x04V[a\r\xA5V[P\x96\x95PPPPPPV[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \tV[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xE9V[`\0\x80\x82\x13a\x1DvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\xD9V[`\0``a\x1D\x83\x84a (V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1F:W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13\x1EWP\x19`\x01\x01\x90V[\x91\x90PV[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \tV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F~W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F\x9AW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1F\xB2W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1F\xC8W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a !W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\xD9V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a \xE3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a!\rW\x81\x81\x01Q\x83\x82\x01R` \x01a \xF5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra!.\x81` \x86\x01` \x86\x01a \xF2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x032` \x83\x01\x84a!\x16V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a!kW`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x9CW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!\xC1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xD9W`\0\x80\xFD[\x835\x92P` \x84\x015a!\xEB\x81a!\xB3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0B\xA2`\x80\x83\x01\x84a!\x16V[`\0` \x82\x84\x03\x12\x15a\"5W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\xC1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"cW`\0\x80\xFD[\x815a\x032\x81a\"<V[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x03 \x82\x84a\"nV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xE1Wa\"\xE1a\"\xA8V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xE1Wa\"\xE1a\"\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#3Wa#3a\"\xA8V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a#QW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a#nW`\0\x80\xFD[Pa#wa\"\xBEV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a#\xA0\x81a\"<V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03 Wa\x03 a#\xB1V[\x81\x81\x03\x81\x81\x11\x15a\x03 Wa\x03 a#\xB1V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$4W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a$\x18V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a$\xADW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a$\x8BV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa$\xCB\x81\x86a$\x03V[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa$\xF6a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x08\x96\x81\x85a!\x16V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a%CW`\0\x80\xFD[\x87Qa%N\x81a!\xB3V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[\x80Qa\x1FK\x81a\"<V[`\0` \x82\x84\x03\x12\x15a%\xA2W`\0\x80\xFD[\x81Qa\x032\x81a\"<V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xC7Wa%\xC7a\"\xA8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a%\xE2W`\0\x80\xFD[\x81Q` a%\xF7a%\xF2\x83a%\xADV[a#\nV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a&\x19W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1D\x04W\x80Qa&1\x81a\"<V[\x83R\x91\x83\x01\x91\x83\x01a&\x1EV[`\0\x82`\x1F\x83\x01\x12a&OW`\0\x80\xFD[\x81Q` a&_a%\xF2\x83a%\xADV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a&\x81W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1D\x04W\x80Q\x83R\x91\x83\x01\x91\x83\x01a&\x86V[`\0` \x82\x84\x03\x12\x15a&\xAFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xC7W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a&\xDBW`\0\x80\xFD[a&\xE3a\"\xE7V[a&\xEC\x83a%\x85V[\x81R` \x83\x01Q\x82\x81\x11\x15a'\0W`\0\x80\xFD[a'\x0C\x87\x82\x86\x01a%\xD1V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a'$W`\0\x80\xFD[a'0\x87\x82\x86\x01a&>V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra'L`\x80\x84\x01a%\x85V[`\x80\x82\x01Ra']`\xA0\x84\x01a%\x85V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\x8AW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'\xA2W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a'\xB6W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a'\xC8Wa'\xC8a\"\xA8V[a'\xDB`\x1F\x82\x01`\x1F\x19\x16` \x01a#\nV[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a'\xF2W`\0\x80\xFD[a(\x03\x81` \x84\x01` \x86\x01a \xF2V[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a(\x1EW`\0\x80\xFD[a(&a\"\xBEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa(N\x81a\"<V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a(kW`\0\x80\xFD[a\x032\x83\x83a(\x0CV[`\x05\x81\x10a(\x93WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a(\xA5\x82\x86a(uV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xD6Wa(\xD6a#\xB1V[PP\x92\x91PPV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x12g``\x83\x01\x84a\"nV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a) Wa) a#\xB1V[P\x92\x91PPV[`@\x81\x01a)5\x82\x85a(uV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a)P\x82\x85a(uV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xC0\x81R`\0a){`\xC0\x83\x01\x86a$\x03V[\x90P\x83` \x83\x01Ra\x03y`@\x83\x01\x84a\"nV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)\xACWa)\xACa#\xB1V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03 Wa\x03 a#\xB1V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xE5Wa)\xE5a)\xC0V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xFFWa)\xFFa#\xB1V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a*\x19Wa*\x19a#\xB1V[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a*6W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa*U\x86``\x87\x01a(\x0CV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a*oWa*oa)\xC0V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xA4\xF3\x9A\x0B\t\xE1$\0\xA2\x8B6\x91teY\xE8v\xB0\xCA\x0174\x8A\xBF\x9A\xAF\xEC\r\xA2k\xF4\x9EdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01,W`\x005`\xE0\x1C\x80c\xA8\xC6.v\x11a\0\xADW\x80c\xCE\x15;\xF4\x11a\0qW\x80c\xCE\x15;\xF4\x14a\x02\x8FW\x80c\xDC\x17\x83U\x14a\x02\xBDW\x80c\xDE\xF1_\x92\x14a\x02\xDDW\x80c\xEA\xAE\x17\xBA\x14a\x01ZW\x80c\xF9\xC2\x82\x11\x14a\x02\xF0W`\0\x80\xFD[\x80c\xA8\xC6.v\x14a\x02\x18W\x80c\xAFNC\x7F\x14a\x02CW\x80c\xB0\x9D\x04\xE5\x14a\x02VW\x80c\xC6a\xDB\xF5\x14a\x02iW\x80c\xCB\x1FU2\x14a\x02|W`\0\x80\xFD[\x80c;M\x100\x11a\0\xF4W\x80c;M\x100\x14a\x01\xC4W\x80cN\x81\x7F\xD9\x14a\x01\xD7W\x80c^\xB4\x08\xFC\x14a\x01\xEAW\x80cme\"\x99\x14a\x01\xFDW\x80c\x8C5\x82M\x14a\x02\x05W`\0\x80\xFD[\x80c\x08TQ[\x14a\x011W\x80c\x0F\x85z\xB9\x14a\x01ZW\x80c\x12\x06I\xC5\x14a\x01mW\x80c\x1E\x97\x8C\xB0\x14a\x01\x8EW\x80c9(\xFF\x97\x14a\x01\xA1W[`\0\x80\xFD[a\x01Da\x01?6`\x04a \xD0V[a\x02\xF8V[`@Qa\x01Q\x91\x90a!BV[`@Q\x80\x91\x03\x90\xF3[a\x01Da\x01h6`\x04a \xD0V[a\x03&V[a\x01\x80a\x01{6`\x04a!UV[a\x039V[`@Q\x90\x81R` \x01a\x01QV[a\x01\x80a\x01\x9C6`\x04a!\x87V[a\x03\x81V[a\x01\xB4a\x01\xAF6`\x04a!\xC4V[a\x03\x96V[`@Qa\x01Q\x94\x93\x92\x91\x90a!\xFCV[a\x01\x80a\x01\xD26`\x04a\"#V[a\x07\xD9V[a\x01\x80a\x01\xE56`\x04a!\x87V[a\x07\xFAV[a\x01\x80a\x01\xF86`\x04a!UV[a\x08\x0FV[a\x01\x80`\0\x81V[a\x01Da\x02\x136`\x04a \xD0V[a\x08IV[`\0Ta\x02+\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01QV[a\x01\x80a\x02Q6`\x04a!UV[a\x08kV[a\x01Da\x02d6`\x04a\"#V[a\x08\xA1V[a\x01Da\x02w6`\x04a \xD0V[a\x08\xACV[a\x01Da\x02\x8A6`\x04a\"QV[a\x08\xCEV[a\x02\xA2a\x02\x9D6`\x04a\"#V[a\x08\xD9V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01QV[a\x02\xD0a\x02\xCB6`\x04a\"#V[a\n\x19V[`@Qa\x01Q\x91\x90a\"\x9AV[a\x01Da\x02\xEB6`\x04a#;V[a\n\xD1V[a\x01\x80`x\x81V[```\0\x80`\0a\x03\x08\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\n\xDEV[\x93PPPP[\x92\x91PPV[``a\x032\x83\x83a\x0B6V[\x93\x92PPPV[`\0\x80a\x03E\x86a\n\x19V[\x90P`\0a\x03T\x85\x85\x84a\x0BeV[\x90P`\0a\x03d\x87\x83\x88\x86a\x0B\xACV[\x90Pa\x03s\x87\x87\x83\x85\x87a\x0C\x05V[\x93PPPP[\x94\x93PPPPV[`\0a\x03y\x83\x83a\x03\x91\x87a\n\x19V[a\x0C\xDEV[`\0\x80`\0``a\x03\xC1`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80`\0a\x03\xCF\x8Ba\x08\xD9V[\x92P\x92P\x92P`\0a\x03\xE0\x8Ca\n\x19V[\x90Pa\x04\x06`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x04\x14\x8E\x87\x87\x87a\x08kV[\x90P\x8C\x15a\x04\xF9Wa\x04)\x8C\x87\x87\x87\x87a\rNV[` \x83\x01Ra\x048\x8C\x87a#\xC7V[\x87R` \x82\x01Qa\x04I\x90\x82a#\xC7V[\x87`@\x01\x81\x81RPP`\0a\x04g\x8F\x89`\0\x01Q\x8A`@\x01Qa\x03\x81V[\x90Pa\x04}\x8F\x89`\0\x01Q\x8A`@\x01Q\x84a\x039V[` \x89\x01\x81\x90R\x86\x11a\x04\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: y reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[` \x88\x01Qa\x04\xF1\x90\x87a#\xDAV[\x83RPa\x05\xCBV[a\x05\x06\x8C\x87\x87\x87\x87a\r\xB1V[` \x83\x01Ra\x05\x15\x8C\x86a#\xC7V[` \x80\x89\x01\x91\x90\x91R\x82\x01Qa\x05+\x90\x82a#\xC7V[\x87`@\x01\x81\x81RPP`\0a\x05I\x8F\x89` \x01Q\x8A`@\x01Qa\x07\xFAV[\x90Pa\x05_\x8F\x89` \x01Q\x8A`@\x01Q\x84a\x08\x0FV[\x80\x89R\x87\x11a\x05\xBBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\"`$\x82\x01R\x7Finvalid swap: x reserve increase`D\x82\x01Rad!`\xF0\x1B`d\x82\x01R`\x84\x01a\x04\xD9V[\x87Qa\x05\xC7\x90\x88a#\xDAV[\x83RP[Pa\x06'`@Q\x80`\xE0\x01`@R\x80`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01``\x81R` \x01``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81RP\x90V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x90\x91` \x83\x01\x90\x806\x837PPP`@\x82\x01\x81\x90R\x87Q\x81Q\x90\x91\x90`\0\x90a\x06aWa\x06aa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x86` \x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x06\x89Wa\x06\x89a#\xEDV[` \x90\x81\x02\x91\x90\x91\x01\x01R``\x80\x82\x01\x85\x90R\x8D\x15a\x06\xDAWP\x81Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8F\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x07\x0EV[P\x81Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8F\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x8F\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x84\x87\x87`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07[\x94\x93\x92\x91\x90a$?V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07xW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07\x9C\x91\x90a%(V[PPPPPP\x90P\x80\x85`\0\x01Qa\x07\xBD\x8C`\0\x01Q\x8D`@\x01Q\x8Aa\x0C\xDEV[\x85\x9DP\x9DP\x9DP\x9DPPPPPPPPPPP\x93P\x93P\x93P\x93V[`\0\x80`\0a\x07\xE7\x84a\x08\xD9V[\x92PP\x91Pa\x03y\x82\x82a\x03\x91\x87a\n\x19V[`\0a\x03y\x83\x83a\x08\n\x87a\n\x19V[a\r\xFEV[`\0\x80a\x08\x1B\x86a\n\x19V[\x90P`\0a\x08*\x85\x85\x84a\x0EXV[\x90P`\0a\x08:\x82\x88\x88\x86a\x0B\xACV[\x90Pa\x03s\x87\x87\x83\x85\x87a\x0E\x9DV[```\0\x80`\0a\x08Y\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x0FwV[`\0\x80a\x08w\x86a\n\x19V[\x90P`\0a\x08\x87\x86\x86\x86\x85a\x0B\xACV[\x90Pa\x08\x96\x86\x86\x83\x87\x86a\x0F\xB8V[\x97\x96PPPPPPPV[``a\x03 \x82a\x10\xC5V[```\0\x80`\0a\x08\xBC\x86a\x08\xD9V[\x92P\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x10\xF1V[``a\x03 \x82a\x112V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\tT\x91\x90a%\x90V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\x81\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\x9EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\t\xC6\x91\x90\x81\x01\x90a&\x9DV[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\t\xDFWa\t\xDFa#\xEDV[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\t\xFEWa\t\xFEa#\xEDV[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\nM`@Q\x80`\x80\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\x96W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\xBE\x91\x90\x81\x01\x90a'xV[\x80` \x01\x90Q\x81\x01\x90a\x03 \x91\x90a(YV[``a\x03y\x84\x84\x84a\x11HV[```\0a\n\xED\x86\x86\x85a\x12\x1CV[\x90P`\0a\n\xFC\x87\x86\x86a\x12\x1CV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[```\x02\x83\x83`@Q` \x01a\x0BN\x93\x92\x91\x90a(\x97V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x92\x91PPV[`\0\x80a\x0Br\x84\x84a\x12)V[\x90P`\0a\x0B\x87a\x0B\x82\x83a\x12pV[a\x12\xD9V[\x84Q\x90\x91Pa\x0B\xA2\x90\x82\x90a\x0B\x9C\x90\x89a\x13\"V[\x90a\x13\"V[\x96\x95PPPPPPV[`\0\x80a\x0B\xC1a\x0B\xBC\x87\x86a\x137V[a\x13LV[\x90P`\0a\x0B\xE9a\x0B\xBCa\x0B\xE2\x86`\0\x01Q\x88a\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88\x90a\x137V[` \x85\x01Q\x90\x91Pa\x0B\xFB\x82\x84a(\xB6V[a\x08\x96\x91\x90a(\xB6V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0CEW[`\0\x81\x12\x15a\x0C@Wa\x0C+\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92Pa\x0C9\x89\x84\x8A\x88a\x0B\xACV[\x90Pa\x0C\x13V[a\x0CrV[`\0\x81\x13\x15a\x0CrWa\x0C]\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x91Pa\x0Ck\x89\x83\x8A\x88a\x0B\xACV[\x90Pa\x0CEV[`\0\x80a\x0C\xAC\x8B\x8B\x85\x8A`@Q` \x01a\x0C\x8F\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x146a\x14cV[P\x91P\x91Pa\x0C\xBD\x8B\x83\x8C\x8Aa\x0B\xACV[`\0\x03a\x0C\xCCW\x81\x95Pa\x0C\xD0V[\x80\x95P[PPPPP\x95\x94PPPPPV[`\0\x80a\x0C\xEE\x83` \x01Qa\x15\x7FV[\x90P`\0a\r\x11a\x0C\xFF\x87\x87a\x137V[a\x0B\xBC\x90g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x90P`\0a\r?\x83a\r0\x87` \x01Q\x85a\x15\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r:\x91\x90a)\0V[a\x15\xD0V[\x85Q\x90\x91Pa\x08\x96\x90\x82a\x13\"V[`\0\x80a\rh\x87\x84`@\x01Qa\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\rw\x87\x86\x86a\x0C\xDEV[\x90Pa\r\xA5\x86a\r\x87\x83\x8Aa\x17yV[a\r\x91\x91\x90a#\xC7V[a\r\x9F\x84a\x0B\x9C\x85\x8Aa\x13\"V[\x90a\x137V[\x98\x97PPPPPPPPV[`\0\x80a\r\xCB\x87\x84`@\x01Qa\x13\"\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\r\xDA\x87\x86\x86a\x0C\xDEV[\x90Pa\r\xA5\x86a\r\xEA\x83\x8Aa\x17yV[a\r\xF4\x91\x90a#\xC7V[a\r\x9F\x87\x85a\x13\"V[`\0\x80a\x0E\x0E\x83` \x01Qa\x15\x7FV[\x90P`\0a\x0E/a\x0B\xBCa\x0B\xE2\x87\x87`\0\x01Qa\x17y\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\r?\x83a\x0EN\x87` \x01Q\x85a\x15\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\r:\x91\x90a(\xB6V[`\0\x80a\x0Ee\x84\x84a\x17\x8EV[\x90P`\0a\x0Er\x82a\x12pV[\x90P`\0a\x0E\x7F\x82a\x12\xD9V[\x90Pa\x08\x96a\x0E\x96\x82g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x88\x90a\x13\"V[`\0\x82\x80\x85\x83\x81\x12\x15a\x0E\xEEW[`\0\x81\x12\x15a\x0E\xE9Wa\x0E\xC3\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92P\x87\x83\x11a\x0E\xD2W\x82a\x0E\xD4V[\x87[\x92Pa\x0E\xE2\x83\x8A\x8A\x88a\x0B\xACV[\x90Pa\x0E\xABV[a\x0F,V[`\0\x81\x13\x15a\x0F,Wa\x0F\x06\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x91P\x87\x82\x11a\x0F\x15W\x81a\x0F\x17V[\x87[\x91Pa\x0F%\x82\x8A\x8A\x88a\x0B\xACV[\x90Pa\x0E\xEEV[`\0\x80a\x0Ff\x8B\x8B\x85\x8A`@Q` \x01a\x0FI\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\0`\x80a\x17\xC6a\x14cV[P\x91P\x91Pa\x0C\xBD\x82\x8C\x8C\x8Aa\x0B\xACV[```\0a\x0F\x86\x86\x86\x86a\x17\xF3V[\x90P`\0a\x0F\x95\x87\x85\x87a\x18\0V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\x0B\x1BV[`\0\x82\x80\x85\x83\x81\x12\x15a\x10MW[`\0\x81\x12\x15a\x10HWa\x0F\xDE\x82a\x03\xE7a\x03\xE8a\x14\x17V[\x85Q\x90\x92P`\0\x90a\x0F\xF1\x90\x8A\x90a\x137V[\x8A\x11a\x10\x15W\x85Qa\x10\x04\x90\x8A\x90a\x137V[a\x10\x10\x90a\x03\xE8a#\xC7V[a\x10!V[a\x10!\x8Aa\x03\xE8a#\xC7V[\x90P\x89\x83\x10a\x100W\x82a\x102V[\x80[\x92Pa\x10@\x8A\x8A\x85\x89a\x0B\xACV[\x91PPa\x0F\xC6V[a\x10zV[`\0\x81\x13\x15a\x10zWa\x10e\x83a\x03\xE9a\x03\xE8a\x13\xE9V[\x92Pa\x10s\x89\x89\x85\x88a\x0B\xACV[\x90Pa\x10MV[`\0\x80a\x10\xB4\x8B\x8B\x85\x8A`@Q` \x01a\x10\x97\x94\x93\x92\x91\x90a(\xDEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x85\x87`\x01`\x80a\x18\ra\x14cV[\x92PP\x91Pa\x0C\xBD\x8B\x8B\x84\x8Aa\x0B\xACV[```\x01\x82`@Q` \x01a\x10\xDB\x92\x91\x90a)'V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\0a\x11\0\x86\x86\x86a\x12\x1CV[\x90P`\0a\x11\x0F\x87\x85\x88a\x18\0V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\x0B\x1BV[```\x04\x82`@Q` \x01a\x10\xDB\x92\x91\x90a)BV[```\0a\x11W\x85\x85\x85a\x18:V[\x90P`\0a\x11f\x82\x86\x86a\x0BeV[\x90P`\0a\x11v\x87\x83\x85\x88a\x0B\xACV[\x90Pa\x11\x85\x87\x83\x83\x86\x89a\x0F\xB8V[`@\x80Q`\x02\x80\x82R``\x82\x01\x83R\x92\x95P`\0\x92\x90\x91` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x87\x81`\0\x81Q\x81\x10a\x11\xBFWa\x11\xBFa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x82\x81`\x01\x81Q\x81\x10a\x11\xDFWa\x11\xDFa#\xEDV[` \x02` \x01\x01\x81\x81RPP\x80\x84\x87`@Q` \x01a\x12\0\x93\x92\x91\x90a)hV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[`\0a\x03y\x82\x85\x85a\x13\xE9V[`\0\x80a\x12:\x84\x84`\0\x01Qa\x18uV[\x90P`\0a\x12K\x84` \x01Qa\x15\x7FV[\x90Pa\x12g\x84` \x01Q\x82\x84a\x12a\x91\x90a)\0V[\x90a\x18\x89V[\x95\x94PPPPPV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x12\x8Eg\r\xE0\xB6\xB3\xA7d\0\0\x85a)\x90V[a\x12\x98\x91\x90a)\xD6V[\x90P`\0a\x12\xA5\x82a*\x04V[\x90P`\0a\x12\xB2\x82a\x18\xADV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x12\xCFg\r\xE0\xB6\xB3\xA7d\0\0\x83a)\x90V[a\x12g\x91\x90a)\xD6V[`\0\x80\x82\x12\x15a\x13\x1EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01RotoUint: negative`\x80\x1B`D\x82\x01R`d\x01a\x04\xD9V[P\x90V[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x13\xE9V[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x14\x17V[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x13eWP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x13\x8DW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x13\xAEW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x13\xBB\x83`\x02a)\x90V[\x90P`\0a\x13\xC8\x82a\x1A\x96V[\x90P`\0a\x13\xDEg\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x1D\x0FV[\x90Pa\x12g\x81a*\x04V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x14\x01W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x14/W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x14P\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x83\x86\x84\x84a\x0B\xACV[`\0\x80`\0\x86\x88\x11\x15a\x14\x93W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01a\x04\xD9V[`\0a\x14\xA3\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14\xB5\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x14\xC3\x82\x84a)\x90V[\x13\x15a\x14\xECW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\x04\xD9V[`\0a\x14\xF8\x8B\x8Ba#\xDAV[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\x15\x0F\x87\x87a#\xC7V[a\x15\x19\x91\x90a*`V[\x96P`\0a\x15+\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x159\x86\x83a)\x90V[\x13a\x15FW\x87\x96Pa\x15MV[\x87\x95P\x80\x94P[a\x15W\x8D\x8Da#\xDAV[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\x15kWP\x88\x81\x10[a\x15\x03WPPPP\x96P\x96P\x96\x93PPPPV[`\0a\x03 a\x15\x8E\x83\x80a\x13\"V[g\x06\xF0[Y\xD3\xB2\0\0\x90a\x17yV[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x15\xBFW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x15\xEBWP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x162W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x04\xD9V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x14\x17V[`\0\x80a\x17\x9F\x84\x84`\0\x01Qa\x18uV[\x90P`\0a\x17\xB0\x84` \x01Qa\x15\x7FV[\x90Pa\x12g\x84` \x01Q\x82\x84a\x12a\x91\x90a(\xB6V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x17\xE0\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x85\x84\x84\x84a\x0B\xACV[`\0a\x03y\x83\x85\x84a\x13\xE9V[`\0a\x03y\x83\x85\x84a\x14\x17V[`\0\x80`\0\x80\x85\x80` \x01\x90Q\x81\x01\x90a\x18'\x91\x90a* V[\x93PP\x92P\x92Pa\x03\x1A\x83\x83\x87\x84a\x0B\xACV[`\0\x80a\x18G\x84\x84a\x17\x8EV[\x90P`\0a\x18Wa\x0B\x82\x83a\x12pV[\x90Pa\x0B\xA2a\x18n\x82g\r\xE0\xB6\xB3\xA7d\0\0a#\xDAV[\x87\x90a\x1D$V[`\0a\x032a\x18\x84\x84\x84a\x1D$V[a\x1D9V[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x18\xA7W`\0\x80\xFD[\x05\x91\x90PV[`\0\x81`\0\x03a\x18\xC6WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x18\xDDWP`\0\x91\x90PV[a\x18\xEEgV\x98\xEE\xF0fp\0\0a*\x04V[\x82\x13a\x19\x03WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x19\x0E\x83a\x1F\x14V[\x90P`\0a\x19Gg\r\xE0\xB6\xB3\xA7d\0\0a\x190\x84g\x1B\xC1mgN\xC8\0\0a\x137V[a\x19B\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\xB6V[a\x1FPV[\x90P`\0\x80\x82a\x19\xA8\x81a\x19\x95\x81a\x19\x83\x81a\x19k\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x1D\x0FV[a\x19~\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a(\xB6V[a\x1D\x0FV[a\x19~\x90g\x14\xA8EL\x19\xE1\xAC\0a(\xB6V[a\x19~\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a(\xB6V[a\x19\xBA\x90g\x03\xDE\xBD\x08;\x8C|\0a(\xB6V[\x91P\x83\x90Pa\x1A\"\x81a\x1A\x10\x81a\x19\xFE\x81a\x19\xEC\x81a\x19\xD9\x81\x8Ba\x1D\x0FV[a\x19~\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a(\xB6V[a\x19~\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a(\xB6V[a\x19~\x90g\x051\n\xA7\xD5!0\0a(\xB6V[a\x19~\x90g\r\xE0\xCC=\x15a\0\0a(\xB6V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x1A8\x87\x88a\x1D\x0FV[a\x1AD\x90`\0\x19a)\x90V[a\x1AN\x91\x90a)\0V[a\x1AX\x91\x90a(\xB6V[\x92PP`\0a\x1Af\x83a\x15\xD0V[\x90P`\0a\x1At\x85\x83a\x1D\x0FV[\x90P`\0\x88\x12a\x1A\x84W\x80a\r\xA5V[a\r\xA5\x81g\x1B\xC1mgN\xC8\0\0a)\0V[`\0\x80\x82\x12\x80a\x1A\xADWPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x1A\xCBW`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x1A\xECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x1B\x14W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x1B\x1FW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x1BGWa\x1BB\x83g\x1B\xC1mgN\xC8\0\0a)\0V[a\x1BIV[\x82[\x90P`\0a\x1B_\x82g\x1B\xC1mgN\xC8\0\0a\x1FPV[\x90P\x80`\0\x03a\x1B\x82W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x1B\x8D\x82a\x1D9V[\x90P`\0c;\x9A\xCA\0a\x1B\xB8a\x1B\xB3a\x1B\xADg\x1B\xC1mgN\xC8\0\0a*\x04V[\x85a\x1D\x0FV[a\x1FeV[a\x1B\xC2\x91\x90a)\x90V[\x90P`\0\x80a\x1B\xD9\x83g\x03\xC1f\\z\xAB \0a\x1D\x0FV[a\x1B\xEB\x90g \x05\xFEO&\x8E\xA0\0a(\xB6V[\x90P`\0a\x1C\x16\x84a\x1C\x04\x86f\x9F2u$b\xA0\0a\x1D\x0FV[a\x19~\x90g\r\xC5R\x7Fd, \0a(\xB6V[a\x1C(\x90g\r\xE0\xB6\xB3\xA7d\0\0a(\xB6V[\x90Pa\x1CLg\t\xD0(\xCCo _\xFF\x19\x85a\x1CB\x85\x85a\x1FPV[a\x19~\x91\x90a)\0V[\x92PPP`\0[`\x02\x81\x10\x15a\x1C\xE7W`\0\x86a\x1Ch\x84a\x18\xADV[a\x1Cr\x91\x90a)\0V[\x90P`\0a\x1C\x80\x84\x85a\x1D\x0FV[a\x1C\x89\x90a*\x04V[\x90P`\0a\x1C\x96\x82a\x15\xD0V[\x90P`\0a\x1C\xA4\x86\x85a\x1D\x0FV[a\x1C\xB6g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x1D\x0FV[a\x1C\xC0\x91\x90a)\0V[\x90Pa\x1C\xCC\x84\x82a\x1FPV[a\x1C\xD6\x90\x87a(\xB6V[\x95P\x84`\x01\x01\x94PPPPPa\x1CSV[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x1D\x04Wa\x1C\xFF\x82a*\x04V[a\r\xA5V[P\x96\x95PPPPPPV[`\0a\x032\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \tV[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x13\xE9V[`\0\x80\x82\x13a\x1DvW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\xD9V[`\0``a\x1D\x83\x84a (V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1F:W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13\x1EWP\x19`\x01\x01\x90V[\x91\x90PV[`\0a\x032\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \tV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x1F~W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x1F\x9AW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x1F\xB2W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x1F\xC8W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a !W`\0\x80\xFD[\x05\x92\x91PPV[`\0\x80\x82\x11a eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x04\xD9V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80`@\x83\x85\x03\x12\x15a \xE3W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a!\rW\x81\x81\x01Q\x83\x82\x01R` \x01a \xF5V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra!.\x81` \x86\x01` \x86\x01a \xF2V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x032` \x83\x01\x84a!\x16V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a!kW`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\x9CW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x80\x15\x15\x81\x14a!\xC1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a!\xD9W`\0\x80\xFD[\x835\x92P` \x84\x015a!\xEB\x81a!\xB3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x0B\xA2`\x80\x83\x01\x84a!\x16V[`\0` \x82\x84\x03\x12\x15a\"5W`\0\x80\xFD[P5\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a!\xC1W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\"cW`\0\x80\xFD[\x815a\x032\x81a\"<V[\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x80\x82\x01Q\x90\x83\x01R``\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x80\x81\x01a\x03 \x82\x84a\"nV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xE1Wa\"\xE1a\"\xA8V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\"\xE1Wa\"\xE1a\"\xA8V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a#3Wa#3a\"\xA8V[`@R\x91\x90PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a#QW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a#nW`\0\x80\xFD[Pa#wa\"\xBEV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015a#\xA0\x81a\"<V[``\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x03 Wa\x03 a#\xB1V[\x81\x81\x03\x81\x81\x11\x15a\x03 Wa\x03 a#\xB1V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a$4W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a$\x18V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a$\xADW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a$\x8BV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa$\xCB\x81\x86a$\x03V[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa$\xF6a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x08\x96\x81\x85a!\x16V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a%CW`\0\x80\xFD[\x87Qa%N\x81a!\xB3V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[\x80Qa\x1FK\x81a\"<V[`\0` \x82\x84\x03\x12\x15a%\xA2W`\0\x80\xFD[\x81Qa\x032\x81a\"<V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a%\xC7Wa%\xC7a\"\xA8V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a%\xE2W`\0\x80\xFD[\x81Q` a%\xF7a%\xF2\x83a%\xADV[a#\nV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a&\x19W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1D\x04W\x80Qa&1\x81a\"<V[\x83R\x91\x83\x01\x91\x83\x01a&\x1EV[`\0\x82`\x1F\x83\x01\x12a&OW`\0\x80\xFD[\x81Q` a&_a%\xF2\x83a%\xADV[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a&\x81W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x1D\x04W\x80Q\x83R\x91\x83\x01\x91\x83\x01a&\x86V[`\0` \x82\x84\x03\x12\x15a&\xAFW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a&\xC7W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a&\xDBW`\0\x80\xFD[a&\xE3a\"\xE7V[a&\xEC\x83a%\x85V[\x81R` \x83\x01Q\x82\x81\x11\x15a'\0W`\0\x80\xFD[a'\x0C\x87\x82\x86\x01a%\xD1V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a'$W`\0\x80\xFD[a'0\x87\x82\x86\x01a&>V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra'L`\x80\x84\x01a%\x85V[`\x80\x82\x01Ra']`\xA0\x84\x01a%\x85V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a'\x8AW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a'\xA2W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a'\xB6W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a'\xC8Wa'\xC8a\"\xA8V[a'\xDB`\x1F\x82\x01`\x1F\x19\x16` \x01a#\nV[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a'\xF2W`\0\x80\xFD[a(\x03\x81` \x84\x01` \x86\x01a \xF2V[P\x94\x93PPPPV[`\0`\x80\x82\x84\x03\x12\x15a(\x1EW`\0\x80\xFD[a(&a\"\xBEV[\x90P\x81Q\x81R` \x82\x01Q` \x82\x01R`@\x82\x01Q`@\x82\x01R``\x82\x01Qa(N\x81a\"<V[``\x82\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a(kW`\0\x80\xFD[a\x032\x83\x83a(\x0CV[`\x05\x81\x10a(\x93WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[``\x81\x01a(\xA5\x82\x86a(uV[` \x82\x01\x93\x90\x93R`@\x01R\x91\x90PV[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a(\xD6Wa(\xD6a#\xB1V[PP\x92\x91PPV[\x84\x81R` \x81\x01\x84\x90R`@\x81\x01\x83\x90R`\xE0\x81\x01a\x12g``\x83\x01\x84a\"nV[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a) Wa) a#\xB1V[P\x92\x91PPV[`@\x81\x01a)5\x82\x85a(uV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a)P\x82\x85a(uV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV[`\xC0\x81R`\0a){`\xC0\x83\x01\x86a$\x03V[\x90P\x83` \x83\x01Ra\x03y`@\x83\x01\x84a\"nV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a)\xACWa)\xACa#\xB1V[\x81\x81\x05\x83\x14\x82\x15\x17a\x03 Wa\x03 a#\xB1V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a)\xE5Wa)\xE5a)\xC0V[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a)\xFFWa)\xFFa#\xB1V[P\x05\x90V[`\0`\x01`\xFF\x1B\x82\x01a*\x19Wa*\x19a#\xB1V[P`\0\x03\x90V[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a*6W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa*U\x86``\x87\x01a(\x0CV[\x90P\x92\x95\x91\x94P\x92PV[`\0\x82a*oWa*oa)\xC0V[P\x04\x90V\xFE\xA2dipfsX\"\x12 \xA4\xF3\x9A\x0B\t\xE1$\0\xA2\x8B6\x91teY\xE8v\xB0\xCA\x0174\x8A\xBF\x9A\xAF\xEC\r\xA2k\xF4\x9EdsolcC\0\x08\x16\x003";
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
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetNextReserveX(GetNextReserveXCall),
        GetNextReserveY(GetNextReserveYCall),
        GetPoolParams(GetPoolParamsCall),
        GetPriceGivenXL(GetPriceGivenXLCall),
        GetPriceGivenYL(GetPriceGivenYLCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        InternalPrice(InternalPriceCall),
        PrepareAllocationDeltasGivenDeltaL(PrepareAllocationDeltasGivenDeltaLCall),
        PrepareAllocationDeltasGivenDeltaX(PrepareAllocationDeltasGivenDeltaXCall),
        PrepareAllocationDeltasGivenDeltaY(PrepareAllocationDeltasGivenDeltaYCall),
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
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveX(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextReserveY(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenXL(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPriceGivenYL(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaLCall> for LogNormalSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaLCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaL(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaXCall> for LogNormalSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaXCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaYCall> for LogNormalSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaYCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaY(value)
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

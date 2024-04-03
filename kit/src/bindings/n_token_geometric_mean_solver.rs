pub use n_token_geometric_mean_solver::*;
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
pub mod n_token_geometric_mean_solver {
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
                    ::std::borrow::ToOwned::to_owned("computePriceOfToken"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computePriceOfToken",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("rNumeraire"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("wNumeraire"),
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
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getAllocationDeltasGivenDeltaT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getAllocationDeltasGivenDeltaT",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("indexT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getDeallocationDeltasGivenDeltaT"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDeallocationDeltasGivenDeltaT",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("indexT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deltaT"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("numeraireAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("prices"),
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
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct NTokenGeometricMeanParams",
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
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("poolId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                                ::ethers::core::abi::ethabi::ParamType::Array(
                                    ::std::boxed::Box::new(
                                        ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ),
                                ),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned(
                                    "struct NTokenGeometricMeanParams",
                                ),
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
                    ::std::borrow::ToOwned::to_owned("prepareWeightsUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareWeightsUpdate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("targetWeights"),
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
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static NTOKENGEOMETRICMEANSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1Dr8\x03\x80a\x1Dr\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x1C\xDF\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xB0\x9D\x04\xE5\x11a\0qW\x80c\xB0\x9D\x04\xE5\x14a\x01uW\x80c\xC2\x93\x87\xE5\x14a\x01\x88W\x80c\xCB\x1FU2\x14a\x01\xAAW\x80c\xCE\x15;\xF4\x14a\x01\xBDW\x80c\xDC\x17\x83U\x14a\x01\xD0W\x80c\xEC\xA5D\x1A\x14a\x01\xF0W`\0\x80\xFD[\x80c\x12\xD9\xD9\x99\x14a\0\xB9W\x80c\"W\xB4\xC5\x14a\0\xE2W\x80c.y\xED]\x14a\x01\x03W\x80cL\x83*\xDE\x14a\x01$W\x80ch@\xC8A\x14a\x017W\x80c\xA8\xC6.v\x14a\x01JW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x13\xC1V[a\x02\x03V[`@Qa\0\xD9\x91\x90a\x14\xCDV[`@Q\x80\x91\x03\x90\xF3[a\0\xF5a\0\xF06`\x04a\x14\xE0V[a\x02\x18V[`@Qa\0\xD9\x92\x91\x90a\x15HV[a\x01\x16a\x01\x116`\x04a\x15jV[a\x02FV[`@Q\x90\x81R` \x01a\0\xD9V[a\0\xCCa\x0126`\x04a\x15\x9CV[a\x02xV[a\x01\x16a\x01E6`\x04a\x16\x16V[a\x02\x85V[`\0Ta\x01]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD9V[a\0\xCCa\x01\x836`\x04a\x16\x16V[a\x02\xADV[a\x01\x9Ba\x01\x966`\x04a\x15jV[a\x02\xBEV[`@Qa\0\xD9\x93\x92\x91\x90a\x16/V[a\0\xCCa\x01\xB86`\x04a\x16PV[a\x06\xC6V[a\0\xF5a\x01\xCB6`\x04a\x16\x16V[a\x06\xD1V[a\x01\xE3a\x01\xDE6`\x04a\x16\x16V[a\x07\xD3V[`@Qa\0\xD9\x91\x90a\x16mV[a\0\xF5a\x01\xFE6`\x04a\x14\xE0V[a\x08\x84V[``a\x02\x10\x84\x84\x84a\x08\xA4V[\x94\x93PPPPV[```\0\x80`\0a\x02(\x87a\x06\xD1V[\x91P\x91Pa\x028\x85\x87\x84\x84a\n3V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80a\x02S\x84\x84a\x0BOV[\x90P`\0a\x02a\x86\x88a\x0BOV[\x90Pa\x02m\x82\x82a\x0BdV[\x97\x96PPPPPPPV[``a\x02\x10\x84\x84\x84a\x0ByV[`\0\x80a\x02\x91\x83a\x06\xD1V[P\x90Pa\x02\xA6\x81a\x02\xA1\x85a\x07\xD3V[a\x0B\xABV[\x93\x92PPPV[``a\x02\xB8\x82a\x0C\x15V[\x92\x91PPV[`\0\x80```\0a\x02\xCE\x88a\x07\xD3V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03H\x91\x90a\x16\xC4V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03u\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xBA\x91\x90\x81\x01\x90a\x17\xA8V[\x90Pa\x04\x04`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81`@\x01Q\x89\x81Q\x81\x10a\x04\x1AWa\x04\x1Aa\x18\x82V[` \x02` \x01\x01Q\x81`@\x01\x81\x81RPP\x81`@\x01Q\x88\x81Q\x81\x10a\x04AWa\x04Aa\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01Q``\x82\x01R\x82Q\x80Q\x8A\x90\x81\x10a\x04eWa\x04ea\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x80\x82\x01R\x82Q\x80Q\x89\x90\x81\x10a\x04\x89Wa\x04\x89a\x18\x82V[` \x02` \x01\x01Q\x81`\xA0\x01\x81\x81RPPa\x04\xB7\x87\x82`@\x01Q\x84``\x01Q\x84`\x80\x01Q\x87` \x01Qa\x0CAV[`\xC0\x82\x01\x81\x90R``\x83\x01Q`\0\x91a\x04\xCF\x91a\x18\xAEV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0[\x84`@\x01QQ\x81\x10\x15a\x05fW\x8A\x81\x14\x15\x80\x15a\x04\xFAWP\x8B\x81\x14\x15[\x15a\x05^W`\0a\x05N\x87`\0\x01Q\x83\x81Q\x81\x10a\x05\x1AWa\x05\x1Aa\x18\x82V[` \x02` \x01\x01Q\x87`@\x01Q\x84\x81Q\x81\x10a\x058Wa\x058a\x18\x82V[` \x02` \x01\x01Qa\x0Cl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05Z\x83\x82a\x0C\x9DV[\x92PP[`\x01\x01a\x04\xDDV[P`\0a\x05\x87\x84`\x80\x01Q\x8B\x86`@\x01Qa\x05\x81\x91\x90a\x18\xAEV[\x90a\x0ClV[\x90P`\0a\x05\xC1a\x05\xAD\x86`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xB2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\x81a\x05\xBA\x85\x87a\x0C\x9DV[\x87\x90a\x0C\xB2V[\x90P\x80\x85``\x01Qa\x05\xD3\x91\x90a\x18\xC1V[` \x80\x87\x01\x82\x90R`@Q`\0\x96Pa\x06\x0C\x95P\x8F\x94P\x8E\x93P\x8D\x92\x91\x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x8E\x87\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06e\x94\x93\x92\x91\x90a\x18\xD4V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA6\x91\x90a\x19\xBDV[PPPP` \x95\x90\x95\x01Q\x91\x9E\x91\x9DP\x92\x9BP\x99PPPPPPPPPPV[``a\x02\xB8\x82a\x0C\xC7V[```\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07K\x91\x90a\x16\xC4V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07x\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xBD\x91\x90\x81\x01\x90a\x17\xA8V[\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP\x91P\x91V[a\x08\0`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08q\x91\x90\x81\x01\x90a\x1A\x1FV[\x80` \x01\x90Q\x81\x01\x90a\x02\xB8\x91\x90a\x1A\xB2V[```\0\x80`\0a\x08\x94\x87a\x06\xD1V[\x91P\x91Pa\x028\x85\x87\x84\x84a\x0C\xDDV[```\0\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xC1Wa\x08\xC1a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01\x86Qa\x08\xFE\x91\x90a\x18\xC1V[\x81Q\x81\x10a\t\x0EWa\t\x0Ea\x18\x82V[` \x02` \x01\x01Q\x90P`\0\x84`\0\x01Q`\x01\x86`\0\x01QQa\t1\x91\x90a\x18\xC1V[\x81Q\x81\x10a\tAWa\tAa\x18\x82V[` \x02` \x01\x01Q\x90P`\0[`\x01\x87Qa\t\\\x91\x90a\x18\xC1V[\x81\x10\x15a\t\xB7W`\0a\t\x8E\x89\x85\x89`\0\x01Q\x85\x81Q\x81\x10a\t\x80Wa\t\x80a\x18\x82V[` \x02` \x01\x01Q\x86a\r\xE9V[\x90P\x80\x85\x83\x81Q\x81\x10a\t\xA3Wa\t\xA3a\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\tNV[P\x86\x83`\x01\x88Qa\t\xC8\x91\x90a\x18\xC1V[\x81Q\x81\x10a\t\xD8Wa\t\xD8a\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0a\t\xF0\x84\x87a\x0B\xABV[\x90P\x83\x81\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q`@Q` \x01a\n\x17\x95\x94\x93\x92\x91\x90a\x1B?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[```\0\x80a\nd\x85\x87\x81Q\x81\x10a\nMWa\nMa\x18\x82V[` \x02` \x01\x01Q\x88a\x0BO\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x81Wa\n\x81a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xAAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x87\x81\x88\x81Q\x81\x10a\n\xC0Wa\n\xC0a\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0[\x86Q\x81\x10\x15a\x0B2W\x87\x81\x14a\x0B*Wa\x0B\x0B\x87\x82\x81Q\x81\x10a\n\xF4Wa\n\xF4a\x18\x82V[` \x02` \x01\x01Q\x84a\x0Bd\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a\x0B\x1DWa\x0B\x1Da\x18\x82V[` \x02` \x01\x01\x81\x81RPP[`\x01\x01a\n\xCFV[P`\0a\x0B?\x83\x87a\x0C\x9DV[\x91\x99\x91\x98P\x90\x96PPPPPPPV[`\0a\x02\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0E\nV[`\0a\x02\xA6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\nV[```\x02\x84\x84\x84`@Q` \x01a\x0B\x93\x94\x93\x92\x91\x90a\x1B\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81[\x84Q\x81\x10\x15a\x0C\rW`\0a\x0B\xF6\x85`\0\x01Q\x83\x81Q\x81\x10a\x0B\xDCWa\x0B\xDCa\x18\x82V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x058Wa\x058a\x18\x82V[\x90Pa\x0C\x02\x83\x82a\x0C\x9DV[\x92PP`\x01\x01a\x0B\xB8V[P\x93\x92PPPV[```\x01\x82`@Q` \x01a\x0C+\x92\x91\x90a\x1B\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x0Cba\x0CP\x87\x87a\x0BOV[a\x0C\\\x86\x81\x87\x87a\x0BdV[\x90a\x0BdV[\x96\x95PPPPPPV[`\0a\x02\xA6g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\x84\x86a\x0E)V[a\x0C\x8E\x91\x90a\x1C\x17V[a\x0C\x98\x91\x90a\x1CGV[a\x10\tV[`\0a\x02\xA6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB2V[`\0a\x02\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x11\xB2V[```\x03\x82`@Q` \x01a\x0C+\x92\x91\x90a\x1C\x83V[```\0\x80a\r\x0E\x85\x87\x81Q\x81\x10a\x0C\xF7Wa\x0C\xF7a\x18\x82V[` \x02` \x01\x01Q\x88a\x0C\xB2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r+Wa\r+a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rTW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x87\x81\x88\x81Q\x81\x10a\rjWa\rja\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0[\x86Q\x81\x10\x15a\r\xDCW\x87\x81\x14a\r\xD4Wa\r\xB5\x87\x82\x81Q\x81\x10a\r\x9EWa\r\x9Ea\x18\x82V[` \x02` \x01\x01Q\x84a\x0C\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a\r\xC7Wa\r\xC7a\x18\x82V[` \x02` \x01\x01\x81\x81RPP[`\x01\x01a\ryV[P`\0a\x0B?\x83\x87a\x0BdV[`\0a\x0E\x01\x85a\r\xF9\x84\x87a\x0C\x9DV[\x85\x91\x90a\x0E\nV[\x95\x94PPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0E\"W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\x0EkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x0Ex\x84a\x11\xE0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x10$WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x10kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0EbV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x11\xCAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x12\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0EbV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xBFWa\x12\xBFa\x12\x87V[`@R\x90V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xBFWa\x12\xBFa\x12\x87V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x0FWa\x13\x0Fa\x12\x87V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x130Wa\x130a\x12\x87V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x13KW`\0\x80\xFD[\x815` a\x13`a\x13[\x83a\x13\x17V[a\x12\xE7V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x13\x82W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x805\x83R\x91\x83\x01\x91\x83\x01a\x13\x87V[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\xBEW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xD6W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xF4W`\0\x80\xFD[a\x14\0\x87\x83\x88\x01a\x13:V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x14\x16W`\0\x80\xFD[\x90\x85\x01\x90``\x82\x88\x03\x12\x15a\x14*W`\0\x80\xFD[a\x142a\x12\x9DV[\x825\x82\x81\x11\x15a\x14AW`\0\x80\xFD[a\x14M\x89\x82\x86\x01a\x13:V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015\x92Pa\x14j\x83a\x13\xA9V[\x82`@\x82\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x14\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xB9\x81` \x86\x01` \x86\x01a\x14}V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02\xA6` \x83\x01\x84a\x14\xA1V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xF5W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x15=W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x15!V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x15[`@\x83\x01\x85a\x15\x0CV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x80W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x15\xB1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x15\xC8W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x15\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x15\xEBW`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\0W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16(W`\0\x80\xFD[P5\x91\x90PV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0E\x01``\x83\x01\x84a\x14\xA1V[`\0` \x82\x84\x03\x12\x15a\x16bW`\0\x80\xFD[\x815a\x02\xA6\x81a\x13\xA9V[` \x81R`\0\x82Q``` \x84\x01Ra\x16\x89`\x80\x84\x01\x82a\x15\x0CV[` \x85\x01Q`@\x85\x81\x01\x91\x90\x91R\x90\x94\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[\x80Qa\x16\xBF\x81a\x13\xA9V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16\xD6W`\0\x80\xFD[\x81Qa\x02\xA6\x81a\x13\xA9V[`\0\x82`\x1F\x83\x01\x12a\x16\xF2W`\0\x80\xFD[\x81Q` a\x17\x02a\x13[\x83a\x13\x17V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17$W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x80Qa\x17<\x81a\x13\xA9V[\x83R\x91\x83\x01\x91\x83\x01a\x17)V[`\0\x82`\x1F\x83\x01\x12a\x17ZW`\0\x80\xFD[\x81Q` a\x17ja\x13[\x83a\x13\x17V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17\x8CW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x17\x91V[`\0` \x82\x84\x03\x12\x15a\x17\xBAW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xD1W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x17\xE5W`\0\x80\xFD[a\x17\xEDa\x12\xC5V[a\x17\xF6\x83a\x16\xB4V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x18\nW`\0\x80\xFD[a\x18\x16\x87\x82\x86\x01a\x16\xE1V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x18.W`\0\x80\xFD[a\x18:\x87\x82\x86\x01a\x17IV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x18V`\x80\x84\x01a\x16\xB4V[`\x80\x82\x01Ra\x18g`\xA0\x84\x01a\x16\xB4V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xB8Wa\x02\xB8a\x18\x98V[\x81\x81\x03\x81\x81\x11\x15a\x02\xB8Wa\x02\xB8a\x18\x98V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x19BW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x19 V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x19`\x81\x86a\x15\x0CV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x19\x8Ba\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x02m\x81\x85a\x14\xA1V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x19\xD8W`\0\x80\xFD[\x87Q\x80\x15\x15\x81\x14a\x19\xE8W`\0\x80\xFD[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1A1W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1AHW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x1A\\W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1AnWa\x1Ana\x12\x87V[a\x1A\x81`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\xE7V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x1A\x98W`\0\x80\xFD[a\x1A\xA9\x81` \x84\x01` \x86\x01a\x14}V[P\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1A\xC4W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A\xDBW`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x1A\xEFW`\0\x80\xFD[a\x1A\xF7a\x12\x9DV[\x82Q\x82\x81\x11\x15a\x1B\x06W`\0\x80\xFD[a\x1B\x12\x87\x82\x86\x01a\x17IV[\x82RP` \x83\x01Q` \x82\x01R`@\x83\x01Q\x92Pa\x1B/\x83a\x13\xA9V[`@\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\xA0\x81R`\0a\x1BR`\xA0\x83\x01\x88a\x15\x0CV[\x86` \x84\x01R\x82\x81\x03`@\x84\x01Ra\x1Bj\x81\x87a\x15\x0CV[``\x84\x01\x95\x90\x95RPP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x80\x90\x91\x01R\x93\x92PPPV[`\x04\x81\x10a\x1B\xABWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[a\x1B\xB9\x81\x86a\x1B\x8DV[``` \x82\x01\x81\x90R\x81\x01\x83\x90R`\0`\x01`\x01`\xFB\x1B\x03\x84\x11\x15a\x1B\xDDW`\0\x80\xFD[\x83`\x05\x1B\x80\x86`\x80\x85\x017`@\x83\x01\x93\x90\x93RP\x01`\x80\x01\x93\x92PPPV[`@\x81\x01a\x1C\n\x82\x85a\x1B\x8DV[\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1C3Wa\x1C3a\x18\x98V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xB8Wa\x02\xB8a\x18\x98V[`\0\x82a\x1CdWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1C~Wa\x1C~a\x18\x98V[P\x05\x90V[`@\x81\x01a\x1C\x91\x82\x85a\x1B\x8DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 P\x15^\xE62\xD9.\x1C\x8F\x11\xCCla\x07IA6\xAEpL\x10:m\xE0\x80\xF4\x16h#\xB2z6dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static NTOKENGEOMETRICMEANSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xB4W`\x005`\xE0\x1C\x80c\xB0\x9D\x04\xE5\x11a\0qW\x80c\xB0\x9D\x04\xE5\x14a\x01uW\x80c\xC2\x93\x87\xE5\x14a\x01\x88W\x80c\xCB\x1FU2\x14a\x01\xAAW\x80c\xCE\x15;\xF4\x14a\x01\xBDW\x80c\xDC\x17\x83U\x14a\x01\xD0W\x80c\xEC\xA5D\x1A\x14a\x01\xF0W`\0\x80\xFD[\x80c\x12\xD9\xD9\x99\x14a\0\xB9W\x80c\"W\xB4\xC5\x14a\0\xE2W\x80c.y\xED]\x14a\x01\x03W\x80cL\x83*\xDE\x14a\x01$W\x80ch@\xC8A\x14a\x017W\x80c\xA8\xC6.v\x14a\x01JW[`\0\x80\xFD[a\0\xCCa\0\xC76`\x04a\x13\xC1V[a\x02\x03V[`@Qa\0\xD9\x91\x90a\x14\xCDV[`@Q\x80\x91\x03\x90\xF3[a\0\xF5a\0\xF06`\x04a\x14\xE0V[a\x02\x18V[`@Qa\0\xD9\x92\x91\x90a\x15HV[a\x01\x16a\x01\x116`\x04a\x15jV[a\x02FV[`@Q\x90\x81R` \x01a\0\xD9V[a\0\xCCa\x0126`\x04a\x15\x9CV[a\x02xV[a\x01\x16a\x01E6`\x04a\x16\x16V[a\x02\x85V[`\0Ta\x01]\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xD9V[a\0\xCCa\x01\x836`\x04a\x16\x16V[a\x02\xADV[a\x01\x9Ba\x01\x966`\x04a\x15jV[a\x02\xBEV[`@Qa\0\xD9\x93\x92\x91\x90a\x16/V[a\0\xCCa\x01\xB86`\x04a\x16PV[a\x06\xC6V[a\0\xF5a\x01\xCB6`\x04a\x16\x16V[a\x06\xD1V[a\x01\xE3a\x01\xDE6`\x04a\x16\x16V[a\x07\xD3V[`@Qa\0\xD9\x91\x90a\x16mV[a\0\xF5a\x01\xFE6`\x04a\x14\xE0V[a\x08\x84V[``a\x02\x10\x84\x84\x84a\x08\xA4V[\x94\x93PPPPV[```\0\x80`\0a\x02(\x87a\x06\xD1V[\x91P\x91Pa\x028\x85\x87\x84\x84a\n3V[\x93P\x93PPP\x93P\x93\x91PPV[`\0\x80a\x02S\x84\x84a\x0BOV[\x90P`\0a\x02a\x86\x88a\x0BOV[\x90Pa\x02m\x82\x82a\x0BdV[\x97\x96PPPPPPPV[``a\x02\x10\x84\x84\x84a\x0ByV[`\0\x80a\x02\x91\x83a\x06\xD1V[P\x90Pa\x02\xA6\x81a\x02\xA1\x85a\x07\xD3V[a\x0B\xABV[\x93\x92PPPV[``a\x02\xB8\x82a\x0C\x15V[\x92\x91PPV[`\0\x80```\0a\x02\xCE\x88a\x07\xD3V[\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03$W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03H\x91\x90a\x16\xC4V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x8A`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03u\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x92W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xBA\x91\x90\x81\x01\x90a\x17\xA8V[\x90Pa\x04\x04`@Q\x80a\x01\0\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[\x81`@\x01Q\x89\x81Q\x81\x10a\x04\x1AWa\x04\x1Aa\x18\x82V[` \x02` \x01\x01Q\x81`@\x01\x81\x81RPP\x81`@\x01Q\x88\x81Q\x81\x10a\x04AWa\x04Aa\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01Q``\x82\x01R\x82Q\x80Q\x8A\x90\x81\x10a\x04eWa\x04ea\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01Q`\x80\x82\x01R\x82Q\x80Q\x89\x90\x81\x10a\x04\x89Wa\x04\x89a\x18\x82V[` \x02` \x01\x01Q\x81`\xA0\x01\x81\x81RPPa\x04\xB7\x87\x82`@\x01Q\x84``\x01Q\x84`\x80\x01Q\x87` \x01Qa\x0CAV[`\xC0\x82\x01\x81\x90R``\x83\x01Q`\0\x91a\x04\xCF\x91a\x18\xAEV[\x90Pg\r\xE0\xB6\xB3\xA7d\0\0`\0[\x84`@\x01QQ\x81\x10\x15a\x05fW\x8A\x81\x14\x15\x80\x15a\x04\xFAWP\x8B\x81\x14\x15[\x15a\x05^W`\0a\x05N\x87`\0\x01Q\x83\x81Q\x81\x10a\x05\x1AWa\x05\x1Aa\x18\x82V[` \x02` \x01\x01Q\x87`@\x01Q\x84\x81Q\x81\x10a\x058Wa\x058a\x18\x82V[` \x02` \x01\x01Qa\x0Cl\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x05Z\x83\x82a\x0C\x9DV[\x92PP[`\x01\x01a\x04\xDDV[P`\0a\x05\x87\x84`\x80\x01Q\x8B\x86`@\x01Qa\x05\x81\x91\x90a\x18\xAEV[\x90a\x0ClV[\x90P`\0a\x05\xC1a\x05\xAD\x86`\xA0\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\xB2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x05\x81a\x05\xBA\x85\x87a\x0C\x9DV[\x87\x90a\x0C\xB2V[\x90P\x80\x85``\x01Qa\x05\xD3\x91\x90a\x18\xC1V[` \x80\x87\x01\x82\x90R`@Q`\0\x96Pa\x06\x0C\x95P\x8F\x94P\x8E\x93P\x8D\x92\x91\x01\x93\x84R` \x84\x01\x92\x90\x92R`@\x83\x01R``\x82\x01R`\x80\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16cu\xE6D\x0F0\x8E\x87\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06e\x94\x93\x92\x91\x90a\x18\xD4V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x82W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA6\x91\x90a\x19\xBDV[PPPP` \x95\x90\x95\x01Q\x91\x9E\x91\x9DP\x92\x9BP\x99PPPPPPPPPPV[``a\x02\xB8\x82a\x0C\xC7V[```\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07'W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07K\x91\x90a\x16\xC4V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x85`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07x\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x95W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xBD\x91\x90\x81\x01\x90a\x17\xA8V[\x90P\x80`@\x01Q\x81``\x01Q\x92P\x92PP\x91P\x91V[a\x08\0`@Q\x80``\x01`@R\x80``\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08IW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08q\x91\x90\x81\x01\x90a\x1A\x1FV[\x80` \x01\x90Q\x81\x01\x90a\x02\xB8\x91\x90a\x1A\xB2V[```\0\x80`\0a\x08\x94\x87a\x06\xD1V[\x91P\x91Pa\x028\x85\x87\x84\x84a\x0C\xDDV[```\0\x83Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\x08\xC1Wa\x08\xC1a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x08\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01\x86Qa\x08\xFE\x91\x90a\x18\xC1V[\x81Q\x81\x10a\t\x0EWa\t\x0Ea\x18\x82V[` \x02` \x01\x01Q\x90P`\0\x84`\0\x01Q`\x01\x86`\0\x01QQa\t1\x91\x90a\x18\xC1V[\x81Q\x81\x10a\tAWa\tAa\x18\x82V[` \x02` \x01\x01Q\x90P`\0[`\x01\x87Qa\t\\\x91\x90a\x18\xC1V[\x81\x10\x15a\t\xB7W`\0a\t\x8E\x89\x85\x89`\0\x01Q\x85\x81Q\x81\x10a\t\x80Wa\t\x80a\x18\x82V[` \x02` \x01\x01Q\x86a\r\xE9V[\x90P\x80\x85\x83\x81Q\x81\x10a\t\xA3Wa\t\xA3a\x18\x82V[` \x90\x81\x02\x91\x90\x91\x01\x01RP`\x01\x01a\tNV[P\x86\x83`\x01\x88Qa\t\xC8\x91\x90a\x18\xC1V[\x81Q\x81\x10a\t\xD8Wa\t\xD8a\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0a\t\xF0\x84\x87a\x0B\xABV[\x90P\x83\x81\x87`\0\x01Q\x88` \x01Q\x89`@\x01Q`@Q` \x01a\n\x17\x95\x94\x93\x92\x91\x90a\x1B?V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x94PPPPP\x93\x92PPPV[```\0\x80a\nd\x85\x87\x81Q\x81\x10a\nMWa\nMa\x18\x82V[` \x02` \x01\x01Q\x88a\x0BO\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x81Wa\n\x81a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xAAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x87\x81\x88\x81Q\x81\x10a\n\xC0Wa\n\xC0a\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0[\x86Q\x81\x10\x15a\x0B2W\x87\x81\x14a\x0B*Wa\x0B\x0B\x87\x82\x81Q\x81\x10a\n\xF4Wa\n\xF4a\x18\x82V[` \x02` \x01\x01Q\x84a\x0Bd\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a\x0B\x1DWa\x0B\x1Da\x18\x82V[` \x02` \x01\x01\x81\x81RPP[`\x01\x01a\n\xCFV[P`\0a\x0B?\x83\x87a\x0C\x9DV[\x91\x99\x91\x98P\x90\x96PPPPPPPV[`\0a\x02\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0E\nV[`\0a\x02\xA6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0E\nV[```\x02\x84\x84\x84`@Q` \x01a\x0B\x93\x94\x93\x92\x91\x90a\x1B\xAFV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x81[\x84Q\x81\x10\x15a\x0C\rW`\0a\x0B\xF6\x85`\0\x01Q\x83\x81Q\x81\x10a\x0B\xDCWa\x0B\xDCa\x18\x82V[` \x02` \x01\x01Q\x87\x84\x81Q\x81\x10a\x058Wa\x058a\x18\x82V[\x90Pa\x0C\x02\x83\x82a\x0C\x9DV[\x92PP`\x01\x01a\x0B\xB8V[P\x93\x92PPPV[```\x01\x82`@Q` \x01a\x0C+\x92\x91\x90a\x1B\xFCV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[`\0a\x0Cba\x0CP\x87\x87a\x0BOV[a\x0C\\\x86\x81\x87\x87a\x0BdV[\x90a\x0BdV[\x96\x95PPPPPPV[`\0a\x02\xA6g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x0C\x84\x86a\x0E)V[a\x0C\x8E\x91\x90a\x1C\x17V[a\x0C\x98\x91\x90a\x1CGV[a\x10\tV[`\0a\x02\xA6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x11\xB2V[`\0a\x02\xA6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x11\xB2V[```\x03\x82`@Q` \x01a\x0C+\x92\x91\x90a\x1C\x83V[```\0\x80a\r\x0E\x85\x87\x81Q\x81\x10a\x0C\xF7Wa\x0C\xF7a\x18\x82V[` \x02` \x01\x01Q\x88a\x0C\xB2\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0\x85Q`\x01`\x01`@\x1B\x03\x81\x11\x15a\r+Wa\r+a\x12\x87V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\rTW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P\x87\x81\x88\x81Q\x81\x10a\rjWa\rja\x18\x82V[` \x02` \x01\x01\x81\x81RPP`\0[\x86Q\x81\x10\x15a\r\xDCW\x87\x81\x14a\r\xD4Wa\r\xB5\x87\x82\x81Q\x81\x10a\r\x9EWa\r\x9Ea\x18\x82V[` \x02` \x01\x01Q\x84a\x0C\x9D\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x82\x82\x81Q\x81\x10a\r\xC7Wa\r\xC7a\x18\x82V[` \x02` \x01\x01\x81\x81RPP[`\x01\x01a\ryV[P`\0a\x0B?\x83\x87a\x0BdV[`\0a\x0E\x01\x85a\r\xF9\x84\x87a\x0C\x9DV[\x85\x91\x90a\x0E\nV[\x95\x94PPPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0E\"W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80\x82\x13a\x0EkW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x0Ex\x84a\x11\xE0V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x10$WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x10kW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x0EbV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x11\xCAW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x12\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x0EbV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1C`\x01`\x01`@\x1B\x03\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xBFWa\x12\xBFa\x12\x87V[`@R\x90V[`@Q`\xE0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x12\xBFWa\x12\xBFa\x12\x87V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a\x13\x0FWa\x13\x0Fa\x12\x87V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a\x130Wa\x130a\x12\x87V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x13KW`\0\x80\xFD[\x815` a\x13`a\x13[\x83a\x13\x17V[a\x12\xE7V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x13\x82W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x805\x83R\x91\x83\x01\x91\x83\x01a\x13\x87V[P\x96\x95PPPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\xBEW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x13\xD6W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x13\xF4W`\0\x80\xFD[a\x14\0\x87\x83\x88\x01a\x13:V[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a\x14\x16W`\0\x80\xFD[\x90\x85\x01\x90``\x82\x88\x03\x12\x15a\x14*W`\0\x80\xFD[a\x142a\x12\x9DV[\x825\x82\x81\x11\x15a\x14AW`\0\x80\xFD[a\x14M\x89\x82\x86\x01a\x13:V[\x82RP` \x83\x015` \x82\x01R`@\x83\x015\x92Pa\x14j\x83a\x13\xA9V[\x82`@\x82\x01R\x80\x93PPPP\x92P\x92P\x92V[`\0[\x83\x81\x10\x15a\x14\x98W\x81\x81\x01Q\x83\x82\x01R` \x01a\x14\x80V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x14\xB9\x81` \x86\x01` \x86\x01a\x14}V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x02\xA6` \x83\x01\x84a\x14\xA1V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x14\xF5W`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x15=W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x15!V[P\x94\x95\x94PPPPPV[`@\x81R`\0a\x15[`@\x83\x01\x85a\x15\x0CV[\x90P\x82` \x83\x01R\x93\x92PPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x15\x80W`\0\x80\xFD[PP\x825\x94` \x84\x015\x94P`@\x84\x015\x93``\x015\x92P\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x15\xB1W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x15\xC8W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x15\xDCW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x15\xEBW`\0\x80\xFD[\x87` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x16\0W`\0\x80\xFD[` \x92\x83\x01\x98\x90\x97P\x95\x90\x91\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x16(W`\0\x80\xFD[P5\x91\x90PV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0E\x01``\x83\x01\x84a\x14\xA1V[`\0` \x82\x84\x03\x12\x15a\x16bW`\0\x80\xFD[\x815a\x02\xA6\x81a\x13\xA9V[` \x81R`\0\x82Q``` \x84\x01Ra\x16\x89`\x80\x84\x01\x82a\x15\x0CV[` \x85\x01Q`@\x85\x81\x01\x91\x90\x91R\x90\x94\x01Q`\x01`\x01`\xA0\x1B\x03\x16``\x90\x93\x01\x92\x90\x92RP\x90\x91\x90PV[\x80Qa\x16\xBF\x81a\x13\xA9V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x16\xD6W`\0\x80\xFD[\x81Qa\x02\xA6\x81a\x13\xA9V[`\0\x82`\x1F\x83\x01\x12a\x16\xF2W`\0\x80\xFD[\x81Q` a\x17\x02a\x13[\x83a\x13\x17V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17$W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x80Qa\x17<\x81a\x13\xA9V[\x83R\x91\x83\x01\x91\x83\x01a\x17)V[`\0\x82`\x1F\x83\x01\x12a\x17ZW`\0\x80\xFD[\x81Q` a\x17ja\x13[\x83a\x13\x17V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x17\x8CW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x13\x9EW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x17\x91V[`\0` \x82\x84\x03\x12\x15a\x17\xBAW`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x17\xD1W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x17\xE5W`\0\x80\xFD[a\x17\xEDa\x12\xC5V[a\x17\xF6\x83a\x16\xB4V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x18\nW`\0\x80\xFD[a\x18\x16\x87\x82\x86\x01a\x16\xE1V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x18.W`\0\x80\xFD[a\x18:\x87\x82\x86\x01a\x17IV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x18V`\x80\x84\x01a\x16\xB4V[`\x80\x82\x01Ra\x18g`\xA0\x84\x01a\x16\xB4V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x01\x80\x82\x11\x15a\x02\xB8Wa\x02\xB8a\x18\x98V[\x81\x81\x03\x81\x81\x11\x15a\x02\xB8Wa\x02\xB8a\x18\x98V[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x19BW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x19 V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x19`\x81\x86a\x15\x0CV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x19\x8Ba\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x02m\x81\x85a\x14\xA1V[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x19\xD8W`\0\x80\xFD[\x87Q\x80\x15\x15\x81\x14a\x19\xE8W`\0\x80\xFD[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\0` \x82\x84\x03\x12\x15a\x1A1W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1AHW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x1A\\W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1AnWa\x1Ana\x12\x87V[a\x1A\x81`\x1F\x82\x01`\x1F\x19\x16` \x01a\x12\xE7V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x1A\x98W`\0\x80\xFD[a\x1A\xA9\x81` \x84\x01` \x86\x01a\x14}V[P\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a\x1A\xC4W`\0\x80\xFD[\x81Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x1A\xDBW`\0\x80\xFD[\x90\x83\x01\x90``\x82\x86\x03\x12\x15a\x1A\xEFW`\0\x80\xFD[a\x1A\xF7a\x12\x9DV[\x82Q\x82\x81\x11\x15a\x1B\x06W`\0\x80\xFD[a\x1B\x12\x87\x82\x86\x01a\x17IV[\x82RP` \x83\x01Q` \x82\x01R`@\x83\x01Q\x92Pa\x1B/\x83a\x13\xA9V[`@\x81\x01\x92\x90\x92RP\x93\x92PPPV[`\xA0\x81R`\0a\x1BR`\xA0\x83\x01\x88a\x15\x0CV[\x86` \x84\x01R\x82\x81\x03`@\x84\x01Ra\x1Bj\x81\x87a\x15\x0CV[``\x84\x01\x95\x90\x95RPP`\x01`\x01`\xA0\x1B\x03\x91\x90\x91\x16`\x80\x90\x91\x01R\x93\x92PPPV[`\x04\x81\x10a\x1B\xABWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[a\x1B\xB9\x81\x86a\x1B\x8DV[``` \x82\x01\x81\x90R\x81\x01\x83\x90R`\0`\x01`\x01`\xFB\x1B\x03\x84\x11\x15a\x1B\xDDW`\0\x80\xFD[\x83`\x05\x1B\x80\x86`\x80\x85\x017`@\x83\x01\x93\x90\x93RP\x01`\x80\x01\x93\x92PPPV[`@\x81\x01a\x1C\n\x82\x85a\x1B\x8DV[\x82` \x83\x01R\x93\x92PPPV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1C3Wa\x1C3a\x18\x98V[\x81\x81\x05\x83\x14\x82\x15\x17a\x02\xB8Wa\x02\xB8a\x18\x98V[`\0\x82a\x1CdWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1C~Wa\x1C~a\x18\x98V[P\x05\x90V[`@\x81\x01a\x1C\x91\x82\x85a\x1B\x8DV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 P\x15^\xE62\xD9.\x1C\x8F\x11\xCCla\x07IA6\xAEpL\x10:m\xE0\x80\xF4\x16h#\xB2z6dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static NTOKENGEOMETRICMEANSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct NTokenGeometricMeanSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for NTokenGeometricMeanSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for NTokenGeometricMeanSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for NTokenGeometricMeanSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for NTokenGeometricMeanSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(NTokenGeometricMeanSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> NTokenGeometricMeanSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                NTOKENGEOMETRICMEANSOLVER_ABI.clone(),
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
                NTOKENGEOMETRICMEANSOLVER_ABI.clone(),
                NTOKENGEOMETRICMEANSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `computePriceOfToken` (0x2e79ed5d) function
        pub fn compute_price_of_token(
            &self,
            r_t: ::ethers::core::types::U256,
            r_numeraire: ::ethers::core::types::U256,
            w_t: ::ethers::core::types::U256,
            w_numeraire: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([46, 121, 237, 93], (r_t, r_numeraire, w_t, w_numeraire))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getAllocationDeltasGivenDeltaT` (0xeca5441a)
        /// function
        pub fn get_allocation_deltas_given_delta_t(
            &self,
            pool_id: ::ethers::core::types::U256,
            index_t: ::ethers::core::types::U256,
            delta_t: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([236, 165, 68, 26], (pool_id, index_t, delta_t))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getDeallocationDeltasGivenDeltaT` (0x2257b4c5)
        /// function
        pub fn get_deallocation_deltas_given_delta_t(
            &self,
            pool_id: ::ethers::core::types::U256,
            index_t: ::ethers::core::types::U256,
            delta_t: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([34, 87, 180, 197], (pool_id, index_t, delta_t))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getInitialPoolData` (0x12d9d999) function
        pub fn get_initial_pool_data(
            &self,
            numeraire_amount: ::ethers::core::types::U256,
            prices: ::std::vec::Vec<::ethers::core::types::U256>,
            params: NtokenGeometricMeanParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([18, 217, 217, 153], (numeraire_amount, prices, params))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getNextLiquidity` (0x6840c841) function
        pub fn get_next_liquidity(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([104, 64, 200, 65], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, NtokenGeometricMeanParams> {
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
                ::std::vec::Vec<::ethers::core::types::U256>,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([206, 21, 59, 244], pool_id)
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
        /// Calls the contract's `prepareWeightsUpdate` (0x4c832ade) function
        pub fn prepare_weights_update(
            &self,
            target_weights: ::std::vec::Vec<::ethers::core::types::U256>,
            target_timestamp: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([76, 131, 42, 222], (target_weights, target_timestamp))
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
        for NTokenGeometricMeanSolver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Container type for all input parameters for the `computePriceOfToken`
    /// function with signature
    /// `computePriceOfToken(uint256,uint256,uint256,uint256)` and selector
    /// `0x2e79ed5d`
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
        name = "computePriceOfToken",
        abi = "computePriceOfToken(uint256,uint256,uint256,uint256)"
    )]
    pub struct ComputePriceOfTokenCall {
        pub r_t: ::ethers::core::types::U256,
        pub r_numeraire: ::ethers::core::types::U256,
        pub w_t: ::ethers::core::types::U256,
        pub w_numeraire: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `getAllocationDeltasGivenDeltaT` function with signature
    /// `getAllocationDeltasGivenDeltaT(uint256,uint256,uint256)` and selector
    /// `0xeca5441a`
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
        name = "getAllocationDeltasGivenDeltaT",
        abi = "getAllocationDeltasGivenDeltaT(uint256,uint256,uint256)"
    )]
    pub struct GetAllocationDeltasGivenDeltaTCall {
        pub pool_id: ::ethers::core::types::U256,
        pub index_t: ::ethers::core::types::U256,
        pub delta_t: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `getDeallocationDeltasGivenDeltaT` function with signature
    /// `getDeallocationDeltasGivenDeltaT(uint256,uint256,uint256)` and selector
    /// `0x2257b4c5`
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
        name = "getDeallocationDeltasGivenDeltaT",
        abi = "getDeallocationDeltasGivenDeltaT(uint256,uint256,uint256)"
    )]
    pub struct GetDeallocationDeltasGivenDeltaTCall {
        pub pool_id: ::ethers::core::types::U256,
        pub index_t: ::ethers::core::types::U256,
        pub delta_t: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256[],(uint256[],uint256,address))` and
    /// selector `0x12d9d999`
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
        abi = "getInitialPoolData(uint256,uint256[],(uint256[],uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub numeraire_amount: ::ethers::core::types::U256,
        pub prices: ::std::vec::Vec<::ethers::core::types::U256>,
        pub params: NtokenGeometricMeanParams,
    }
    /// Container type for all input parameters for the `getNextLiquidity`
    /// function with signature `getNextLiquidity(uint256)` and selector
    /// `0x6840c841`
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
    #[ethcall(name = "getNextLiquidity", abi = "getNextLiquidity(uint256)")]
    pub struct GetNextLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
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
    /// Container type for all input parameters for the `prepareWeightsUpdate`
    /// function with signature `prepareWeightsUpdate(uint256[],uint256)` and
    /// selector `0x4c832ade`
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
        name = "prepareWeightsUpdate",
        abi = "prepareWeightsUpdate(uint256[],uint256)"
    )]
    pub struct PrepareWeightsUpdateCall {
        pub target_weights: ::std::vec::Vec<::ethers::core::types::U256>,
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
    pub enum NTokenGeometricMeanSolverCalls {
        ComputePriceOfToken(ComputePriceOfTokenCall),
        GetAllocationDeltasGivenDeltaT(GetAllocationDeltasGivenDeltaTCall),
        GetDeallocationDeltasGivenDeltaT(GetDeallocationDeltasGivenDeltaTCall),
        GetInitialPoolData(GetInitialPoolDataCall),
        GetNextLiquidity(GetNextLiquidityCall),
        GetPoolParams(GetPoolParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PrepareFeeUpdate(PrepareFeeUpdateCall),
        PrepareWeightsUpdate(PrepareWeightsUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for NTokenGeometricMeanSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ComputePriceOfTokenCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputePriceOfToken(decoded));
            }
            if let Ok(decoded) =
                <GetAllocationDeltasGivenDeltaTCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetAllocationDeltasGivenDeltaT(decoded));
            }
            if let Ok(decoded) =
                <GetDeallocationDeltasGivenDeltaTCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::GetDeallocationDeltasGivenDeltaT(decoded));
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
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) =
                <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetReservesAndLiquidity(decoded));
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
                <PrepareWeightsUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareWeightsUpdate(decoded));
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
    impl ::ethers::core::abi::AbiEncode for NTokenGeometricMeanSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputePriceOfToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAllocationDeltasGivenDeltaT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetDeallocationDeltasGivenDeltaT(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetNextLiquidity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareControllerUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareFeeUpdate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PrepareWeightsUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for NTokenGeometricMeanSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputePriceOfToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllocationDeltasGivenDeltaT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetDeallocationDeltasGivenDeltaT(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNextLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareControllerUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareWeightsUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputePriceOfTokenCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: ComputePriceOfTokenCall) -> Self {
            Self::ComputePriceOfToken(value)
        }
    }
    impl ::core::convert::From<GetAllocationDeltasGivenDeltaTCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: GetAllocationDeltasGivenDeltaTCall) -> Self {
            Self::GetAllocationDeltasGivenDeltaT(value)
        }
    }
    impl ::core::convert::From<GetDeallocationDeltasGivenDeltaTCall>
        for NTokenGeometricMeanSolverCalls
    {
        fn from(value: GetDeallocationDeltasGivenDeltaTCall) -> Self {
            Self::GetDeallocationDeltasGivenDeltaT(value)
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetNextLiquidityCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: GetNextLiquidityCall) -> Self {
            Self::GetNextLiquidity(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareFeeUpdateCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: PrepareFeeUpdateCall) -> Self {
            Self::PrepareFeeUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareWeightsUpdateCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: PrepareWeightsUpdateCall) -> Self {
            Self::PrepareWeightsUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for NTokenGeometricMeanSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    /// Container type for all return fields from the `computePriceOfToken`
    /// function with signature
    /// `computePriceOfToken(uint256,uint256,uint256,uint256)` and selector
    /// `0x2e79ed5d`
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
    pub struct ComputePriceOfTokenReturn {
        pub price: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the
    /// `getAllocationDeltasGivenDeltaT` function with signature
    /// `getAllocationDeltasGivenDeltaT(uint256,uint256,uint256)` and selector
    /// `0xeca5441a`
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
    pub struct GetAllocationDeltasGivenDeltaTReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the
    /// `getDeallocationDeltasGivenDeltaT` function with signature
    /// `getDeallocationDeltasGivenDeltaT(uint256,uint256,uint256)` and selector
    /// `0x2257b4c5`
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
    pub struct GetDeallocationDeltasGivenDeltaTReturn(
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256[],(uint256[],uint256,address))` and
    /// selector `0x12d9d999`
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
    /// function with signature `getNextLiquidity(uint256)` and selector
    /// `0x6840c841`
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
    pub struct GetPoolParamsReturn(pub NtokenGeometricMeanParams);
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
    /// Container type for all return fields from the `prepareWeightsUpdate`
    /// function with signature `prepareWeightsUpdate(uint256[],uint256)` and
    /// selector `0x4c832ade`
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
    pub struct PrepareWeightsUpdateReturn(pub ::ethers::core::types::Bytes);
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
    /// `NtokenGeometricMeanParams(uint256[],uint256,address)`
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
    pub struct NtokenGeometricMeanParams {
        pub weights: ::std::vec::Vec<::ethers::core::types::U256>,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}

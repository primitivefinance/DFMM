pub use constant_sum_solver::*;
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
pub mod constant_sum_solver {
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
                    ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getInitialPoolData"),
                        inputs: ::std::vec![
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
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct ConstantSumParams"),
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
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct ConstantSumParams"),
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
                    ::std::borrow::ToOwned::to_owned("prepareAllocationDeltaGivenDeltaX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareAllocationDeltaGivenDeltaX",),
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
                    ::std::borrow::ToOwned::to_owned("prepareAllocationDeltaGivenDeltaY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareAllocationDeltaGivenDeltaY",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("deltaY"),
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
                            name: ::std::borrow::ToOwned::to_owned("newController"),
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
                    ::std::borrow::ToOwned::to_owned("preparePriceUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("preparePriceUpdate"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newPrice"),
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
                    ::std::borrow::ToOwned::to_owned("prepareSwapFeeUpdate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("prepareSwapFeeUpdate",),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("newSwapFee"),
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
            errors: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
                ::std::vec![::ethers::core::abi::ethabi::AbiError {
                    name: ::std::borrow::ToOwned::to_owned("NotEnoughLiquidity"),
                    inputs: ::std::vec![],
                },],
            )]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static CONSTANTSUMSOLVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x13\x838\x03\x80a\x13\x83\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x12\xF0\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x91]?\xB9\x11a\0\x8CW\x80c\xC6a\xDB\xF5\x11a\0fW\x80c\xC6a\xDB\xF5\x14a\x01\xBCW\x80c\xCB\x1FU2\x14a\x01\xCFW\x80c\xCE\x15;\xF4\x14a\x01\xE2W\x80c\xDC\x17\x83U\x14a\x02\x10W`\0\x80\xFD[\x80c\x91]?\xB9\x14a\x01kW\x80c\xA4##\x87\x14a\x01~W\x80c\xA8\xC6.v\x14a\x01\x91W`\0\x80\xFD[\x80c\x08TQ[\x14a\0\xD4W\x80c#\x03\x96O\x14a\0\xFDW\x80c9(\xFF\x97\x14a\x01\x10W\x80cC\xC8?v\x14a\x012W\x80c\x89\xEA\x85Y\x14a\x01EW\x80c\x8C5\x82M\x14a\x01XW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x0B\x0FV[a\x020V[`@Qa\0\xF4\x91\x90a\x0B\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x01\x0B6`\x04a\x0B\x0FV[a\x02^V[a\x01#a\x01\x1E6`\x04a\x0B\xA5V[a\x02\xB4V[`@Qa\0\xF4\x93\x92\x91\x90a\x0B\xDDV[a\0\xE7a\x01@6`\x04a\x0C\x07V[a\x06#V[a\0\xE7a\x01S6`\x04a\x0C\xC8V[a\x06WV[a\0\xE7a\x01f6`\x04a\x0B\x0FV[a\x06nV[a\0\xE7a\x01y6`\x04a\x0C\x07V[a\x06\x90V[a\0\xE7a\x01\x8C6`\x04a\x0C\x07V[a\x06\x9BV[`\0Ta\x01\xA4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF4V[a\0\xE7a\x01\xCA6`\x04a\x0B\x0FV[a\x06\xA6V[a\0\xE7a\x01\xDD6`\x04a\r4V[a\x06\xC8V[a\x01\xF5a\x01\xF06`\x04a\x0C\x07V[a\x06\xD3V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xF4V[a\x02#a\x02\x1E6`\x04a\x0C\x07V[a\x08\x13V[`@Qa\0\xF4\x91\x90a\rQV[```\0\x80`\0a\x02@\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\x08\xC4V[\x93PPPP[\x92\x91PPV[```\0a\x02k\x84a\x08\x13V[\x80Q\x90\x91P`\0\x90a\x02~\x90\x85\x90a\t\x1CV[`@\x80Q` \x81\x01\x96\x90\x96R`\0\x86\x82\x01R``\x80\x87\x01\x92\x90\x92R\x80Q\x80\x87\x03\x90\x92\x01\x82R`\x80\x90\x95\x01\x90\x94RP\x91\x93\x92PPPV[`\0\x80```\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x031\x91\x90a\r\x8BV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03^\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xA3\x91\x90\x81\x01\x90a\x0E\xA3V[`\0\x80T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x1A\x91\x90\x81\x01\x90a\x0F~V[\x80` \x01\x90Q\x81\x01\x90a\x04-\x91\x90a\x10\x12V[\x90P`\0\x87\x15a\x04\xABWa\x04e\x82` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04R\x91\x90a\x10TV[\x83Qa\x04_\x90\x8A\x90a\t\x1CV[\x90a\t\x1CV[\x90P\x80\x83`@\x01Q`\x01\x81Q\x81\x10a\x04\x7FWa\x04\x7Fa\x10uV[` \x02` \x01\x01Q\x10\x15a\x04\xA6W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x16V[a\x04\xD5\x82`\0\x01Qa\x04\xCF\x89\x85` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04_\x91\x90a\x10TV[\x90a\t1V[\x90P\x80\x83`@\x01Q`\0\x81Q\x81\x10a\x04\xEFWa\x04\xEFa\x10uV[` \x02` \x01\x01Q\x10\x15a\x05\x16W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x88\x15a\x05YW`@\x80Q`\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x05\x90V[`@\x80Q`\x01` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0\x80T`@Qcu\xE6D\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cu\xE6D\x0F\x90a\x05\xC7\x900\x90\x8F\x90\x8A\x90\x88\x90`\x04\x01a\x10\xC7V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x08\x91\x90a\x11\xBBV[P\x94\x9CP\x96\x9AP\x94\x98PPPPPPPPP\x93P\x93P\x93\x90PV[`@\x80Q`\0` \x82\x01R\x90\x81\x01\x82\x90R``\x81\x81\x01\x83\x90R\x90`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``a\x06d\x84\x84\x84a\tFV[\x90P[\x93\x92PPPV[```\0\x80`\0a\x06~\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\t\xD7V[``a\x02X\x82a\n\x18V[``a\x02X\x82a\n.V[```\0\x80`\0a\x06\xB6\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\nDV[``a\x02X\x82a\n\x85V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07N\x91\x90a\r\x8BV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07{\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xC0\x91\x90\x81\x01\x90a\x0E\xA3V[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\x07\xD9Wa\x07\xD9a\x10uV[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x07\xF8Wa\x07\xF8a\x10uV[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\x08@`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xB1\x91\x90\x81\x01\x90a\x0F~V[\x80` \x01\x90Q\x81\x01\x90a\x02X\x91\x90a\x10\x12V[```\0a\x08\xD3\x86\x86\x85a\n\x9BV[\x90P`\0a\x08\xE2\x87\x86\x86a\n\x9BV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[`\0a\x06g\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\n\xA8V[`\0a\x06g\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xA8V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\t\x7FWa\t\x7Fa\x10uV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\t\x9FWa\t\x9Fa\x10uV[` \x02` \x01\x01\x81\x81RPP\x80\x83`@Q` \x01a\t\xBE\x92\x91\x90a\x12\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[```\0a\t\xE6\x86\x86\x86a\n\xC7V[\x90P`\0a\t\xF5\x87\x85\x87a\n\xD4V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\t\x01V[```\x01\x82`@Q` \x01a\x06A\x92\x91\x90a\x12yV[```\x02\x82`@Q` \x01a\x06A\x92\x91\x90a\x12yV[```\0a\nS\x86\x86\x86a\n\x9BV[\x90P`\0a\nb\x87\x85\x88a\n\xD4V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\t\x01V[```\x03\x82`@Q` \x01a\x06A\x92\x91\x90a\x12\x94V[`\0a\x06d\x82\x85\x85a\n\xE1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\xC0W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x06d\x83\x85\x84a\n\xE1V[`\0a\x06d\x83\x85\x84a\n\xA8V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\xF9W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\"W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x0BLW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0Bm\x81` \x86\x01` \x86\x01a\x0B1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06g` \x83\x01\x84a\x0BUV[\x80\x15\x15\x81\x14a\x0B\xA2W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xBAW`\0\x80\xFD[\x835\x92P` \x84\x015a\x0B\xCC\x81a\x0B\x94V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0B\xFE``\x83\x01\x84a\x0BUV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x19W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\xABWa\x0C\xABa\x0C V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xA2W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0C\xDEW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0C\xFBW`\0\x80\xFD[Pa\r\x04a\x0C6V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\r#\x81a\x0C\xB3V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\rFW`\0\x80\xFD[\x815a\x06g\x81a\x0C\xB3V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x02XV[\x80Qa\r\x86\x81a\x0C\xB3V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\x9DW`\0\x80\xFD[\x81Qa\x06g\x81a\x0C\xB3V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC2Wa\r\xC2a\x0C V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\r\xDDW`\0\x80\xFD[\x81Q` a\r\xF2a\r\xED\x83a\r\xA8V[a\x0C\x82V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\x14W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E9W\x80Qa\x0E,\x81a\x0C\xB3V[\x83R\x91\x83\x01\x91\x83\x01a\x0E\x19V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0EUW`\0\x80\xFD[\x81Q` a\x0Eea\r\xED\x83a\r\xA8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\x87W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E9W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x0E\x8CV[`\0` \x82\x84\x03\x12\x15a\x0E\xB5W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xCDW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0E\xE1W`\0\x80\xFD[a\x0E\xE9a\x0C_V[a\x0E\xF2\x83a\r{V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x0F\x06W`\0\x80\xFD[a\x0F\x12\x87\x82\x86\x01a\r\xCCV[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x0F*W`\0\x80\xFD[a\x0F6\x87\x82\x86\x01a\x0EDV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x0FR`\x80\x84\x01a\r{V[`\x80\x82\x01Ra\x0Fc`\xA0\x84\x01a\r{V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\x90W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xA8W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0F\xBCW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F\xCEWa\x0F\xCEa\x0C V[a\x0F\xE1`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0C\x82V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\t\x81` \x84\x01` \x86\x01a\x0B1V[P\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a\x10$W`\0\x80\xFD[a\x10,a\x0C6V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x10H\x81a\x0C\xB3V[`@\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x02XWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x10\xBCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x10\xA0V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x115W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x11\x13V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x11S\x81\x86a\x10\x8BV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x11~a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x11\xB0\x81\x85a\x0BUV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x11\xD6W`\0\x80\xFD[\x87Qa\x11\xE1\x81a\x0B\x94V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x80\x81R`\0a\x12+`\x80\x83\x01\x85a\x10\x8BV[\x90Pa\x06g` \x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x04\x81\x10a\x12uWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01a\x12\x87\x82\x85a\x12WV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a\x12\xA2\x82\x85a\x12WV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA2\x84\xD5\n\xB4h\x15U\xFBj/Jt\xC1C\0\xC0\xCF|\x89\xA5\x06\xFC;UgsO\xE8\xAC\x85\rdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xCFW`\x005`\xE0\x1C\x80c\x91]?\xB9\x11a\0\x8CW\x80c\xC6a\xDB\xF5\x11a\0fW\x80c\xC6a\xDB\xF5\x14a\x01\xBCW\x80c\xCB\x1FU2\x14a\x01\xCFW\x80c\xCE\x15;\xF4\x14a\x01\xE2W\x80c\xDC\x17\x83U\x14a\x02\x10W`\0\x80\xFD[\x80c\x91]?\xB9\x14a\x01kW\x80c\xA4##\x87\x14a\x01~W\x80c\xA8\xC6.v\x14a\x01\x91W`\0\x80\xFD[\x80c\x08TQ[\x14a\0\xD4W\x80c#\x03\x96O\x14a\0\xFDW\x80c9(\xFF\x97\x14a\x01\x10W\x80cC\xC8?v\x14a\x012W\x80c\x89\xEA\x85Y\x14a\x01EW\x80c\x8C5\x82M\x14a\x01XW[`\0\x80\xFD[a\0\xE7a\0\xE26`\x04a\x0B\x0FV[a\x020V[`@Qa\0\xF4\x91\x90a\x0B\x81V[`@Q\x80\x91\x03\x90\xF3[a\0\xE7a\x01\x0B6`\x04a\x0B\x0FV[a\x02^V[a\x01#a\x01\x1E6`\x04a\x0B\xA5V[a\x02\xB4V[`@Qa\0\xF4\x93\x92\x91\x90a\x0B\xDDV[a\0\xE7a\x01@6`\x04a\x0C\x07V[a\x06#V[a\0\xE7a\x01S6`\x04a\x0C\xC8V[a\x06WV[a\0\xE7a\x01f6`\x04a\x0B\x0FV[a\x06nV[a\0\xE7a\x01y6`\x04a\x0C\x07V[a\x06\x90V[a\0\xE7a\x01\x8C6`\x04a\x0C\x07V[a\x06\x9BV[`\0Ta\x01\xA4\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF4V[a\0\xE7a\x01\xCA6`\x04a\x0B\x0FV[a\x06\xA6V[a\0\xE7a\x01\xDD6`\x04a\r4V[a\x06\xC8V[a\x01\xF5a\x01\xF06`\x04a\x0C\x07V[a\x06\xD3V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\0\xF4V[a\x02#a\x02\x1E6`\x04a\x0C\x07V[a\x08\x13V[`@Qa\0\xF4\x91\x90a\rQV[```\0\x80`\0a\x02@\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\x08\xC4V[\x93PPPP[\x92\x91PPV[```\0a\x02k\x84a\x08\x13V[\x80Q\x90\x91P`\0\x90a\x02~\x90\x85\x90a\t\x1CV[`@\x80Q` \x81\x01\x96\x90\x96R`\0\x86\x82\x01R``\x80\x87\x01\x92\x90\x92R\x80Q\x80\x87\x03\x90\x92\x01\x82R`\x80\x90\x95\x01\x90\x94RP\x91\x93\x92PPPV[`\0\x80```\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\rW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x031\x91\x90a\r\x8BV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03^\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03{W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xA3\x91\x90\x81\x01\x90a\x0E\xA3V[`\0\x80T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x04\x1A\x91\x90\x81\x01\x90a\x0F~V[\x80` \x01\x90Q\x81\x01\x90a\x04-\x91\x90a\x10\x12V[\x90P`\0\x87\x15a\x04\xABWa\x04e\x82` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04R\x91\x90a\x10TV[\x83Qa\x04_\x90\x8A\x90a\t\x1CV[\x90a\t\x1CV[\x90P\x80\x83`@\x01Q`\x01\x81Q\x81\x10a\x04\x7FWa\x04\x7Fa\x10uV[` \x02` \x01\x01Q\x10\x15a\x04\xA6W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x05\x16V[a\x04\xD5\x82`\0\x01Qa\x04\xCF\x89\x85` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x04_\x91\x90a\x10TV[\x90a\t1V[\x90P\x80\x83`@\x01Q`\0\x81Q\x81\x10a\x04\xEFWa\x04\xEFa\x10uV[` \x02` \x01\x01Q\x10\x15a\x05\x16W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x88\x15a\x05YW`@\x80Q`\0` \x82\x01R`\x01\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90Pa\x05\x90V[`@\x80Q`\x01` \x82\x01R`\0\x91\x81\x01\x91\x90\x91R``\x81\x01\x89\x90R`\x80\x81\x01\x83\x90R`\xA0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P[`\0\x80T`@Qcu\xE6D\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cu\xE6D\x0F\x90a\x05\xC7\x900\x90\x8F\x90\x8A\x90\x88\x90`\x04\x01a\x10\xC7V[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xE4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x08\x91\x90a\x11\xBBV[P\x94\x9CP\x96\x9AP\x94\x98PPPPPPPPP\x93P\x93P\x93\x90PV[`@\x80Q`\0` \x82\x01R\x90\x81\x01\x82\x90R``\x81\x81\x01\x83\x90R\x90`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[``a\x06d\x84\x84\x84a\tFV[\x90P[\x93\x92PPPV[```\0\x80`\0a\x06~\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\t\xD7V[``a\x02X\x82a\n\x18V[``a\x02X\x82a\n.V[```\0\x80`\0a\x06\xB6\x86a\x06\xD3V[\x92P\x92P\x92Pa\x02R\x85\x84\x84\x84a\nDV[``a\x02X\x82a\n\x85V[`\0\x80`\0\x80`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07*W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07N\x91\x90a\r\x8BV[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x86`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x07{\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x98W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\xC0\x91\x90\x81\x01\x90a\x0E\xA3V[\x90P\x80`@\x01Q`\0\x81Q\x81\x10a\x07\xD9Wa\x07\xD9a\x10uV[` \x02` \x01\x01Q\x81`@\x01Q`\x01\x81Q\x81\x10a\x07\xF8Wa\x07\xF8a\x10uV[` \x02` \x01\x01Q\x82``\x01Q\x93P\x93P\x93PP\x91\x93\x90\x92PV[a\x08@`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90V[`\0T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x84\x90R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x89W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x08\xB1\x91\x90\x81\x01\x90a\x0F~V[\x80` \x01\x90Q\x81\x01\x90a\x02X\x91\x90a\x10\x12V[```\0a\x08\xD3\x86\x86\x85a\n\x9BV[\x90P`\0a\x08\xE2\x87\x86\x86a\n\x9BV[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x82\x90R``\x81\x01\x89\x90R\x90\x91P`\x80\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x92PPP\x94\x93PPPPV[`\0a\x06g\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\n\xA8V[`\0a\x06g\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\n\xA8V[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\t\x7FWa\t\x7Fa\x10uV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\t\x9FWa\t\x9Fa\x10uV[` \x02` \x01\x01\x81\x81RPP\x80\x83`@Q` \x01a\t\xBE\x92\x91\x90a\x12\x18V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[```\0a\t\xE6\x86\x86\x86a\n\xC7V[\x90P`\0a\t\xF5\x87\x85\x87a\n\xD4V[`@\x80Q` \x81\x01\x85\x90R\x90\x81\x01\x89\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\t\x01V[```\x01\x82`@Q` \x01a\x06A\x92\x91\x90a\x12yV[```\x02\x82`@Q` \x01a\x06A\x92\x91\x90a\x12yV[```\0a\nS\x86\x86\x86a\n\x9BV[\x90P`\0a\nb\x87\x85\x88a\n\xD4V[`@\x80Q` \x81\x01\x8A\x90R\x90\x81\x01\x84\x90R``\x81\x01\x82\x90R\x90\x91P`\x80\x01a\t\x01V[```\x03\x82`@Q` \x01a\x06A\x92\x91\x90a\x12\x94V[`\0a\x06d\x82\x85\x85a\n\xE1V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\xC0W`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x06d\x83\x85\x84a\n\xE1V[`\0a\x06d\x83\x85\x84a\n\xA8V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\n\xF9W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B\"W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0[\x83\x81\x10\x15a\x0BLW\x81\x81\x01Q\x83\x82\x01R` \x01a\x0B4V[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x0Bm\x81` \x86\x01` \x86\x01a\x0B1V[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x06g` \x83\x01\x84a\x0BUV[\x80\x15\x15\x81\x14a\x0B\xA2W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0B\xBAW`\0\x80\xFD[\x835\x92P` \x84\x015a\x0B\xCC\x81a\x0B\x94V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x0B\xFE``\x83\x01\x84a\x0BUV[\x95\x94PPPPPV[`\0` \x82\x84\x03\x12\x15a\x0C\x19W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C V[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CYWa\x0CYa\x0C V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0C\xABWa\x0C\xABa\x0C V[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0B\xA2W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0C\xDEW`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0C\xFBW`\0\x80\xFD[Pa\r\x04a\x0C6V[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\r#\x81a\x0C\xB3V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[`\0` \x82\x84\x03\x12\x15a\rFW`\0\x80\xFD[\x815a\x06g\x81a\x0C\xB3V[\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90\x82\x01R``\x81\x01a\x02XV[\x80Qa\r\x86\x81a\x0C\xB3V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\r\x9DW`\0\x80\xFD[\x81Qa\x06g\x81a\x0C\xB3V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\r\xC2Wa\r\xC2a\x0C V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\r\xDDW`\0\x80\xFD[\x81Q` a\r\xF2a\r\xED\x83a\r\xA8V[a\x0C\x82V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\x14W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E9W\x80Qa\x0E,\x81a\x0C\xB3V[\x83R\x91\x83\x01\x91\x83\x01a\x0E\x19V[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\x0EUW`\0\x80\xFD[\x81Q` a\x0Eea\r\xED\x83a\r\xA8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\x0E\x87W`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\x0E9W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x0E\x8CV[`\0` \x82\x84\x03\x12\x15a\x0E\xB5W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0E\xCDW`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\x0E\xE1W`\0\x80\xFD[a\x0E\xE9a\x0C_V[a\x0E\xF2\x83a\r{V[\x81R` \x83\x01Q\x82\x81\x11\x15a\x0F\x06W`\0\x80\xFD[a\x0F\x12\x87\x82\x86\x01a\r\xCCV[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\x0F*W`\0\x80\xFD[a\x0F6\x87\x82\x86\x01a\x0EDV[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\x0FR`\x80\x84\x01a\r{V[`\x80\x82\x01Ra\x0Fc`\xA0\x84\x01a\r{V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0F\x90W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F\xA8W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0F\xBCW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F\xCEWa\x0F\xCEa\x0C V[a\x0F\xE1`\x1F\x82\x01`\x1F\x19\x16` \x01a\x0C\x82V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x0F\xF8W`\0\x80\xFD[a\x10\t\x81` \x84\x01` \x86\x01a\x0B1V[P\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a\x10$W`\0\x80\xFD[a\x10,a\x0C6V[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x10H\x81a\x0C\xB3V[`@\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x02XWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x10\xBCW\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x10\xA0V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x115W\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x11\x13V[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x11S\x81\x86a\x10\x8BV[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x11~a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x11\xB0\x81\x85a\x0BUV[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x11\xD6W`\0\x80\xFD[\x87Qa\x11\xE1\x81a\x0B\x94V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x80\x81R`\0a\x12+`\x80\x83\x01\x85a\x10\x8BV[\x90Pa\x06g` \x83\x01\x84\x80Q\x82R` \x80\x82\x01Q\x90\x83\x01R`@\x90\x81\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[`\x04\x81\x10a\x12uWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01a\x12\x87\x82\x85a\x12WV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a\x12\xA2\x82\x85a\x12WV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 \xA2\x84\xD5\n\xB4h\x15U\xFBj/Jt\xC1C\0\xC0\xCF|\x89\xA5\x06\xFC;UgsO\xE8\xAC\x85\rdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static CONSTANTSUMSOLVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ConstantSumSolver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConstantSumSolver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConstantSumSolver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConstantSumSolver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConstantSumSolver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConstantSumSolver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConstantSumSolver<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                CONSTANTSUMSOLVER_ABI.clone(),
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
                CONSTANTSUMSOLVER_ABI.clone(),
                CONSTANTSUMSOLVER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `getInitialPoolData` (0x89ea8559) function
        pub fn get_initial_pool_data(
            &self,
            reserve_x: ::ethers::core::types::U256,
            reserve_y: ::ethers::core::types::U256,
            params: ConstantSumParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([137, 234, 133, 89], (reserve_x, reserve_y, params))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ConstantSumParams> {
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
        /// Calls the contract's `prepareAllocationDeltaGivenDeltaX`
        /// (0x2303964f) function
        pub fn prepare_allocation_delta_given_delta_x(
            &self,
            pool_id: ::ethers::core::types::U256,
            delta_x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([35, 3, 150, 79], (pool_id, delta_x))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareAllocationDeltaGivenDeltaY`
        /// (0x43c83f76) function
        pub fn prepare_allocation_delta_given_delta_y(
            &self,
            delta_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([67, 200, 63, 118], delta_y)
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
            new_controller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([203, 31, 85, 50], new_controller)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `preparePriceUpdate` (0xa4232387) function
        pub fn prepare_price_update(
            &self,
            new_price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([164, 35, 35, 135], new_price)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `prepareSwapFeeUpdate` (0x915d3fb9) function
        pub fn prepare_swap_fee_update(
            &self,
            new_swap_fee: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([145, 93, 63, 185], new_swap_fee)
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
        for ConstantSumSolver<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `NotEnoughLiquidity` with signature
    /// `NotEnoughLiquidity()` and selector `0x4323a555`
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
    #[etherror(name = "NotEnoughLiquidity", abi = "NotEnoughLiquidity()")]
    pub struct NotEnoughLiquidity;
    /// Container type for all input parameters for the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and
    /// selector `0x89ea8559`
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
        abi = "getInitialPoolData(uint256,uint256,(uint256,uint256,address))"
    )]
    pub struct GetInitialPoolDataCall {
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub params: ConstantSumParams,
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
    /// `prepareAllocationDeltaGivenDeltaX` function with signature
    /// `prepareAllocationDeltaGivenDeltaX(uint256,uint256)` and selector
    /// `0x2303964f`
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
        name = "prepareAllocationDeltaGivenDeltaX",
        abi = "prepareAllocationDeltaGivenDeltaX(uint256,uint256)"
    )]
    pub struct PrepareAllocationDeltaGivenDeltaXCall {
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the
    /// `prepareAllocationDeltaGivenDeltaY` function with signature
    /// `prepareAllocationDeltaGivenDeltaY(uint256)` and selector `0x43c83f76`
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
        name = "prepareAllocationDeltaGivenDeltaY",
        abi = "prepareAllocationDeltaGivenDeltaY(uint256)"
    )]
    pub struct PrepareAllocationDeltaGivenDeltaYCall {
        pub delta_y: ::ethers::core::types::U256,
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
        pub new_controller: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `preparePriceUpdate`
    /// function with signature `preparePriceUpdate(uint256)` and selector
    /// `0xa4232387`
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
    #[ethcall(name = "preparePriceUpdate", abi = "preparePriceUpdate(uint256)")]
    pub struct PreparePriceUpdateCall {
        pub new_price: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `prepareSwapFeeUpdate`
    /// function with signature `prepareSwapFeeUpdate(uint256)` and selector
    /// `0x915d3fb9`
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
    #[ethcall(name = "prepareSwapFeeUpdate", abi = "prepareSwapFeeUpdate(uint256)")]
    pub struct PrepareSwapFeeUpdateCall {
        pub new_swap_fee: ::ethers::core::types::U256,
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
    pub enum ConstantSumSolverCalls {
        GetInitialPoolData(GetInitialPoolDataCall),
        GetPoolParams(GetPoolParamsCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        PrepareAllocationDeltaGivenDeltaX(PrepareAllocationDeltaGivenDeltaXCall),
        PrepareAllocationDeltaGivenDeltaY(PrepareAllocationDeltaGivenDeltaYCall),
        PrepareAllocationDeltasGivenDeltaL(PrepareAllocationDeltasGivenDeltaLCall),
        PrepareAllocationDeltasGivenDeltaX(PrepareAllocationDeltasGivenDeltaXCall),
        PrepareAllocationDeltasGivenDeltaY(PrepareAllocationDeltasGivenDeltaYCall),
        PrepareControllerUpdate(PrepareControllerUpdateCall),
        PreparePriceUpdate(PreparePriceUpdateCall),
        PrepareSwapFeeUpdate(PrepareSwapFeeUpdateCall),
        SimulateSwap(SimulateSwapCall),
        Strategy(StrategyCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumSolverCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <GetInitialPoolDataCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetInitialPoolData(decoded));
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
                <PrepareAllocationDeltaGivenDeltaXCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PrepareAllocationDeltaGivenDeltaX(decoded));
            }
            if let Ok(decoded) =
                <PrepareAllocationDeltaGivenDeltaYCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::PrepareAllocationDeltaGivenDeltaY(decoded));
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
                <PreparePriceUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreparePriceUpdate(decoded));
            }
            if let Ok(decoded) =
                <PrepareSwapFeeUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PrepareSwapFeeUpdate(decoded));
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
    impl ::ethers::core::abi::AbiEncode for ConstantSumSolverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetInitialPoolData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareAllocationDeltaGivenDeltaX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareAllocationDeltaGivenDeltaY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
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
                Self::PreparePriceUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PrepareSwapFeeUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Strategy(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumSolverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetInitialPoolData(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareAllocationDeltaGivenDeltaX(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PrepareAllocationDeltaGivenDeltaY(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::PreparePriceUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::PrepareSwapFeeUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Strategy(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetInitialPoolDataCall> for ConstantSumSolverCalls {
        fn from(value: GetInitialPoolDataCall) -> Self {
            Self::GetInitialPoolData(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for ConstantSumSolverCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for ConstantSumSolverCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltaGivenDeltaXCall> for ConstantSumSolverCalls {
        fn from(value: PrepareAllocationDeltaGivenDeltaXCall) -> Self {
            Self::PrepareAllocationDeltaGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltaGivenDeltaYCall> for ConstantSumSolverCalls {
        fn from(value: PrepareAllocationDeltaGivenDeltaYCall) -> Self {
            Self::PrepareAllocationDeltaGivenDeltaY(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaLCall> for ConstantSumSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaLCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaL(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaXCall> for ConstantSumSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaXCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaX(value)
        }
    }
    impl ::core::convert::From<PrepareAllocationDeltasGivenDeltaYCall> for ConstantSumSolverCalls {
        fn from(value: PrepareAllocationDeltasGivenDeltaYCall) -> Self {
            Self::PrepareAllocationDeltasGivenDeltaY(value)
        }
    }
    impl ::core::convert::From<PrepareControllerUpdateCall> for ConstantSumSolverCalls {
        fn from(value: PrepareControllerUpdateCall) -> Self {
            Self::PrepareControllerUpdate(value)
        }
    }
    impl ::core::convert::From<PreparePriceUpdateCall> for ConstantSumSolverCalls {
        fn from(value: PreparePriceUpdateCall) -> Self {
            Self::PreparePriceUpdate(value)
        }
    }
    impl ::core::convert::From<PrepareSwapFeeUpdateCall> for ConstantSumSolverCalls {
        fn from(value: PrepareSwapFeeUpdateCall) -> Self {
            Self::PrepareSwapFeeUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for ConstantSumSolverCalls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<StrategyCall> for ConstantSumSolverCalls {
        fn from(value: StrategyCall) -> Self {
            Self::Strategy(value)
        }
    }
    /// Container type for all return fields from the `getInitialPoolData`
    /// function with signature
    /// `getInitialPoolData(uint256,uint256,(uint256,uint256,address))` and
    /// selector `0x89ea8559`
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
    pub struct GetPoolParamsReturn(pub ConstantSumParams);
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
    /// Container type for all return fields from the
    /// `prepareAllocationDeltaGivenDeltaX` function with signature
    /// `prepareAllocationDeltaGivenDeltaX(uint256,uint256)` and selector
    /// `0x2303964f`
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
    pub struct PrepareAllocationDeltaGivenDeltaXReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the
    /// `prepareAllocationDeltaGivenDeltaY` function with signature
    /// `prepareAllocationDeltaGivenDeltaY(uint256)` and selector `0x43c83f76`
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
    pub struct PrepareAllocationDeltaGivenDeltaYReturn(pub ::ethers::core::types::Bytes);
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
    /// Container type for all return fields from the `preparePriceUpdate`
    /// function with signature `preparePriceUpdate(uint256)` and selector
    /// `0xa4232387`
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
    pub struct PreparePriceUpdateReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `prepareSwapFeeUpdate`
    /// function with signature `prepareSwapFeeUpdate(uint256)` and selector
    /// `0x915d3fb9`
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
    pub struct PrepareSwapFeeUpdateReturn(pub ::ethers::core::types::Bytes);
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
    /// `ConstantSumParams(uint256,uint256,address)`
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
    pub struct ConstantSumParams {
        pub price: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
}

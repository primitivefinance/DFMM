pub use log_normal_arbitrage::*;
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
pub mod log_normal_arbitrage {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("solver_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
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
                    ::std::borrow::ToOwned::to_owned("getDxGivenS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDxGivenS"),
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
                    ::std::borrow::ToOwned::to_owned("getDyGivenS"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getDyGivenS"),
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
    pub static LOGNORMALARBITRAGE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x1E\xE58\x03\x80a\x1E\xE5\x839\x81\x01`@\x81\x90Ra\0/\x91a\0RV[P`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x81\x16`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x17\x90Ua\0\x82V[`\0` \x82\x84\x03\x12\x15a\0dW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0{W`\0\x80\xFD[\x93\x92PPPV[a\x1ET\x80a\0\x91`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c0m\xB4k\x14a\0gW\x80c3\"f\xF3\x14a\0\x8CW\x80cO\xD6|X\x14a\0\x9FW\x80cU\xF0\x11\xC6\x14a\0\xB2W\x80c\x90.\xCA\xA2\x14a\0\xC5W\x80c\xE4]Y<\x14a\0\xD8W[`\0\x80\xFD[a\0za\0u6`\x04a\x1B.V[a\0\xEBV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0za\0\x9A6`\x04a\x1B.V[a\x01\xE9V[a\0za\0\xAD6`\x04a\x1B.V[a\x02\xDCV[a\0za\0\xC06`\x04a\x1BZV[a\x03\xCFV[a\0za\0\xD36`\x04a\x1B.V[a\x04\xCDV[a\0za\0\xE66`\x04a\x1BZV[a\x05\xC0V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x015W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01Y\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCC\x91\x90a\x1C\"V[\x92PP\x91Pa\x01\xDE\x86\x83\x83\x88\x87a\x06\xB4V[\x97\x96PPPPPPPV[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x023W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02W\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x1C\"V[\x92PP\x91Pa\x01\xDE\x86\x83\x83\x88\x87a\x07(V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03J\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xBD\x91\x90a\x1C\"V[\x92P\x92PPa\x01\xDE\x86\x83\x83\x88\x87a\x07\xA3V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04=\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB0\x91\x90a\x1C\"V[\x92PP\x91Pa\x04\xC1\x85\x83\x83\x86a\x08\x05V[\x93PPPP[\x92\x91PPV[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05;\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\x1C\"V[\x92P\x92PPa\x01\xDE\x86\x83\x83\x88\x87a\x08\x99V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06.\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA1\x91\x90a\x1C\"V[\x92P\x92PP`\0a\x01\xDE\x86\x84\x84\x87a\x08\xF5V[`\0\x82`\x01\x82a\x06\xC7\x89\x89\x89\x85\x89a\x07(V[\x90P`\0\x81\x12\x15a\x06\xDEW`\0\x93PPPPa\x07\x1FV[a\x07\x16\x89\x89\x89\x88`@Q` \x01a\x06\xF8\x94\x93\x92\x91\x90a\x1CPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\t\x92a\t\xC3V[P\x90\x94PPPPP[\x95\x94PPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x07C\x91\x90a\x1C\xAEV[\x90P`\0a\x07U\x88\x88\x88\x85\x89\x89a\n\xE4V[\x90P`\0a\x07b\x82a\x0C\x04V[\x90P`\0a\x07o\x83a\r&V[\x90P\x80\x82\x84`\xE0\x01Qa\x07\x81\x90a\x1C\xD5V[a\x07\x8B\x91\x90a\x1C\xF1V[a\x07\x95\x91\x90a\x1C\xF1V[\x9A\x99PPPPPPPPPPV[`\0\x82`\x01\x82a\x07\xB6\x89\x89\x89\x85\x89a\x08\x99V[\x90P`\0\x81\x12\x15a\x07\xCDW`\0\x93PPPPa\x07\x1FV[a\x07\x16\x89\x89\x89\x88`@Q` \x01a\x07\xE7\x94\x93\x92\x91\x90a\x1CPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\r\xB1a\t\xC3V[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x08 \x91\x90a\x1C\xAEV[\x90P`\0\x83` \x01Q\x90P`\0a\x08;\x88\x86`\0\x01Qa\r\xE2V[\x90P`\0a\x08ma\x08T\x84g\x1B\xC1mgN\xC8\0\0a\r\xFDV[a\x08^\x84\x86a\r\xFDV[a\x08h\x91\x90a\x1C\xF1V[a\x0E!V[\x90P`\0a\x08\x8Da\x08\x86\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xAEV[\x89\x90a\x0E\x8AV[\x90Pa\x07\x95\x89\x82a\x1C\xAEV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x08\xB4\x91\x90a\x1C\xAEV[\x90P`\0a\x08\xC6\x88\x88\x88\x85\x89\x89a\x0E\xBDV[\x90P`\0a\x08\xD3\x82a\x0FAV[\x90P`\0a\x08\xE0\x83a\x10UV[\x90P\x80\x82a\x07\x81g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xD5V[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\t\x10\x91\x90a\x1C\xAEV[\x83Q` \x85\x01Q\x91\x92P\x90`\0a\t'\x89\x84a\r\xE2V[\x90P`\0a\t=\x83g\x1B\xC1mgN\xC8\0\0a\r\xFDV[a\tG\x83\x85a\r\xFDV[a\tQ\x91\x90a\x1C\xAEV[\x90P`\0a\t^\x82a\x0E!V[\x90P`\0a\tv\x82a\tp\x8C\x89a\x0E\x8AV[\x90a\x0E\x8AV[\x90Pa\t\x82\x8B\x82a\x1C\xAEV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\t\xAE\x91\x90a\x1D\x19V[\x93P\x93P\x93P\x93Pa\x01\xDE\x84\x84\x84\x89\x85a\x07(V[`\0\x80`\0\x86\x88\x11\x15a\t\xF8W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\n\x08\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n\x1A\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n(\x82\x84a\x1DYV[\x13\x15a\nQW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\t\xEFV[`\0a\n]\x8B\x8Ba\x1D\x89V[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\nt\x87\x87a\x1D\x9CV[a\n~\x91\x90a\x1D\xC5V[\x96P`\0a\n\x90\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n\x9E\x86\x83a\x1DYV[\x13a\n\xABW\x87\x96Pa\n\xB2V[\x87\x95P\x80\x94P[a\n\xBC\x8D\x8Da\x1D\x89V[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\n\xD0WP\x88\x81\x10[a\nhWPPPP\x96P\x96P\x96\x93PPPPV[a\x0B3`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x0BQa\x0BB\x88\x86a\x1C\xF1V[g\x1B\xC1mgN\xC8\0\0\x90a\x0E\x8AV[\x90P`\0a\x0B_\x85\x87a\x0E\x8AV[a\x0Bi\x86\x89a\x1C\xF1V[a\x0Bs\x91\x90a\x1C\xAEV[\x90P`\0a\x0B\x89a\x0B\x84\x84\x84a\r\xFDV[a\x11\x01V[\x90P`\0a\x0B\x9Eg\x1B\xC1mgN\xC8\0\0a\x13\x7FV[a\x0B\xAC\x90c;\x9A\xCA\0a\x1D\xD9V[`@\x80Qa\x01 \x81\x01\x82R\x93\x84R\x87Q` \x80\x86\x01\x91\x90\x91R\x88\x01Q\x90\x84\x01R``\x83\x01\x89\x90R`\x80\x83\x01\x8B\x90R`\xA0\x83\x01\x8A\x90R`\xC0\x83\x01\x88\x90R`\xE0\x83\x01\x8C\x90Ra\x01\0\x83\x01RP\x92PPP\x96\x95PPPPPPV[`@\x81\x01Q`\0\x90\x81\x90a\x0C,\x90g\x1B\xC1mgN\xC8\0\0\x90a\x0C&\x90\x80a\x0E\x8AV[\x90a\r\xFDV[a\x0C5\x90a\x1C\xD5V[\x90P`\0a\x0C]\x84`\0\x01Qa\tp\x86`@\x01Q\x87a\x01\0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0Csa\x0Cn\x83\x85a\x1C\xF1V[a\x14#V[\x90P`\0a\x0C\xC3a\x0C\xA9\x87``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x94\x90a\x1C\xD5V[a\x0C\x9E\x91\x90a\x1C\xF1V[`\x80\x89\x01Q\x90a\x0E\x8AV[\x87`\xA0\x01Qa\x0C\xB8\x91\x90a\x1C\xF1V[` \x88\x01Q\x90a\x0E\x8AV[\x90P`\0a\x0C\xD1\x83\x83a\x0E\x8AV[\x90P`\0a\x0C\xF0\x88``\x01Q\x89`\xC0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xC0\x01Q\x89`\xA0\x01Qa\r\x04\x91\x90a\x1C\xF1V[a\r\x0E\x91\x90a\x1C\xAEV[\x90Pa\r\x1A\x82\x82a\r\xFDV[\x98\x97PPPPPPPPV[`\0\x80a\re\x83``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\rC\x90a\x1C\xD5V[a\rM\x91\x90a\x1C\xF1V[` \x85\x01Qa\tp\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a\x0E\x8AV[\x90P`\0a\r\x85\x84a\x01\0\x01Q\x85`@\x01Qa\r\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\r\xA9a\r\xA2\x85`\0\x01Q\x83a\r\x9D\x91\x90a\x1C\xAEV[a\x15\xCCV[\x83\x90a\x0E\x8AV[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\r\xCD\x91\x90a\x1D\x19V[\x93P\x93P\x93P\x93Pa\x01\xDE\x84\x84\x84\x89\x85a\x08\x99V[`\0a\r\xF6a\r\xF1\x84\x84a\x17\xB0V[a\x17\xC5V[\x93\x92PPPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x0E\x1BW`\0\x80\xFD[\x05\x91\x90PV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0E?g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1DYV[a\x0EI\x91\x90a\x1D\xF0V[\x90P`\0a\x0EV\x82a\x1C\xD5V[\x90P`\0a\x0Ec\x82a\x15\xCCV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x0E\x80g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DYV[a\x07\x1F\x91\x90a\x1D\xF0V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x0E\xACW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[a\x0F\x0C`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x0F\x1Ba\x0BB\x88\x86a\x1C\xF1V[\x90P`\0a\x0F)\x85\x87a\x0E\x8AV[\x84Q\x86\x90a\x0F7\x90\x8Aa\x0E\x8AV[a\x0Bi\x91\x90a\x1C\xF1V[`@\x81\x01Q`\0\x90\x81\x90a\x0Fc\x90g\x1B\xC1mgN\xC8\0\0\x90a\x0C&\x90\x80a\x0E\x8AV[a\x0Fl\x90a\x1C\xD5V[\x90P`\0a\x0F\x94\x84`\0\x01Qa\tp\x86`@\x01Q\x87a\x01\0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xA5a\x0Cn\x83\x85a\x1C\xF1V[\x90P`\0a\x0F\xEEa\x0F\xC6\x87``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x94\x90a\x1C\xD5V[`\xA0\x88\x01Q` \x89\x01Qa\x0F\xD9\x91a\x0E\x8AV[a\x0F\xE3\x91\x90a\x1C\xF1V[`\xE0\x88\x01Q\x90a\x0E\x8AV[\x90P`\0a\x0F\xFC\x83\x83a\x0E\x8AV[\x90P`\0a\r\x0Ea\x10\x1E\x89``\x01Q\x8A`\xC0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8A\x01Q`\xA0\x8B\x01Q` \x8C\x01Qa\x106\x91a\x0E\x8AV[a\x10@\x91\x90a\x1C\xF1V[a\x10J\x91\x90a\x1C\xAEV[` \x8A\x01Q\x90a\x0E\x8AV[`\0\x80a\x10\x87\x83``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x10r\x90a\x1C\xD5V[a\x10|\x91\x90a\x1C\xF1V[`\xE0\x85\x01Q\x90a\x0E\x8AV[\x90P`\0a\x10\xA7\x84a\x01\0\x01Q\x85`@\x01Qa\r\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\xC8a\x10\xC1\x86`\0\x01Q\x84a\r\x9D\x91\x90a\x1C\xAEV[\x84\x90a\x0E\x8AV[\x90P`\0a\x10\xEB\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\xF7\x82\x82a\r\xFDV[\x96\x95PPPPPPV[`\0\x80\x82\x12\x80a\x11\x18WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x116W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11WW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x11\x7FW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x11\x8AW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x11\xB2Wa\x11\xAD\x83g\x1B\xC1mgN\xC8\0\0a\x1C\xAEV[a\x11\xB4V[\x82[\x90P`\0a\x11\xCA\x82g\x1B\xC1mgN\xC8\0\0a\x19\xA0V[\x90P\x80`\0\x03a\x11\xEDW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF8\x82a\x17\xC5V[\x90P`\0c;\x9A\xCA\0a\x12#a\x12\x1Ea\x12\x18g\x1B\xC1mgN\xC8\0\0a\x1C\xD5V[\x85a\x19\xB5V[a\x13\x7FV[a\x12-\x91\x90a\x1DYV[\x90P`\0\x80a\x12D\x83g\x03\xC1f\\z\xAB \0a\x19\xB5V[a\x12V\x90g \x05\xFEO&\x8E\xA0\0a\x1C\xF1V[\x90P`\0a\x12\x86\x84a\x12o\x86f\x9F2u$b\xA0\0a\x19\xB5V[a\x12\x81\x90g\r\xC5R\x7Fd, \0a\x1C\xF1V[a\x19\xB5V[a\x12\x98\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF1V[\x90Pa\x12\xBCg\t\xD0(\xCCo _\xFF\x19\x85a\x12\xB2\x85\x85a\x19\xA0V[a\x12\x81\x91\x90a\x1C\xAEV[\x92PPP`\0[`\x02\x81\x10\x15a\x13WW`\0\x86a\x12\xD8\x84a\x15\xCCV[a\x12\xE2\x91\x90a\x1C\xAEV[\x90P`\0a\x12\xF0\x84\x85a\x19\xB5V[a\x12\xF9\x90a\x1C\xD5V[\x90P`\0a\x13\x06\x82a\x14#V[\x90P`\0a\x13\x14\x86\x85a\x19\xB5V[a\x13&g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x19\xB5V[a\x130\x91\x90a\x1C\xAEV[\x90Pa\x13<\x84\x82a\x19\xA0V[a\x13F\x90\x87a\x1C\xF1V[\x95P\x84`\x01\x01\x94PPPPPa\x12\xC3V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x13tWa\x13o\x82a\x1C\xD5V[a\r\x1AV[P\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x13\x98W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x13\xB4W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x13\xCCW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x13\xE2W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14>WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\t\xEFV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x81`\0\x03a\x15\xE5WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x15\xFCWP`\0\x91\x90PV[a\x16\rgV\x98\xEE\xF0fp\0\0a\x1C\xD5V[\x82\x13a\x16\"WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16-\x83a\x19\xCAV[\x90P`\0a\x16fg\r\xE0\xB6\xB3\xA7d\0\0a\x16O\x84g\x1B\xC1mgN\xC8\0\0a\x1A\x05V[a\x16a\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF1V[a\x19\xA0V[\x90P`\0\x80\x82a\x16\xC2\x81a\x16\xAF\x81a\x16\x9D\x81a\x16\x8A\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x19\xB5V[a\x12\x81\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1C\xF1V[a\x12\x81\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1C\xF1V[a\x12\x81\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1C\xF1V[a\x16\xD4\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1C\xF1V[\x91P\x83\x90Pa\x17<\x81a\x17*\x81a\x17\x18\x81a\x17\x06\x81a\x16\xF3\x81\x8Ba\x19\xB5V[a\x12\x81\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1C\xF1V[a\x12\x81\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1C\xF1V[a\x12\x81\x90g\x051\n\xA7\xD5!0\0a\x1C\xF1V[a\x12\x81\x90g\r\xE0\xCC=\x15a\0\0a\x1C\xF1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17R\x87\x88a\x19\xB5V[a\x17^\x90`\0\x19a\x1DYV[a\x17h\x91\x90a\x1C\xAEV[a\x17r\x91\x90a\x1C\xF1V[\x92PP`\0a\x17\x80\x83a\x14#V[\x90P`\0a\x17\x8E\x85\x83a\x19\xB5V[\x90P`\0\x88\x12a\x17\x9EW\x80a\r\x1AV[a\r\x1A\x81g\x1B\xC1mgN\xC8\0\0a\x1C\xAEV[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\x1AV[`\0\x80\x82\x13a\x18\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\xEFV[`\0``a\x18\x0F\x84a\x1AHV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\xF0V[`\0a\r\xF6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF0V[`\0`\x01`\xFF\x1B\x82\x03a\x19\xF0W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1A\x01WP\x19`\x01\x01\x90V[P\x90V[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\x0FV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1A2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x1A\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\xEFV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\x08W`\0\x80\xFD[\x05\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B'W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BCW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1BmW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0`\x80\x82\x84\x03\x12\x15a\x1B\x8EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\xBFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x90\x81R\x83Q\x82R` \x80\x85\x01Q\x90\x83\x01R\x83\x81\x01Q\x90\x82\x01R``\x83\x01Q\x90\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF9W`\0\x80\xFD[``\x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x18W`\0\x80\xFD[a\r\xF6\x83\x83a\x1B|V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C7W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x93\x84R` \x80\x85\x01\x93\x90\x93R`@\x80\x85\x01\x92\x90\x92R\x80Q``\x80\x86\x01\x91\x90\x91R\x92\x81\x01Q`\x80\x85\x01R\x90\x81\x01Q`\xA0\x84\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C\xCEWa\x1C\xCEa\x1C\x98V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a\x1C\xEAWa\x1C\xEAa\x1C\x98V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1D\x11Wa\x1D\x11a\x1C\x98V[PP\x92\x91PPV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x1D/W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa\x1DN\x86``\x87\x01a\x1B|V[\x90P\x92\x95\x91\x94P\x92PV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DuWa\x1Dua\x1C\x98V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\xC7Wa\x04\xC7a\x1C\x98V[\x81\x81\x03\x81\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x1C\x98V[\x80\x82\x01\x80\x82\x11\x15a\x04\xC7Wa\x04\xC7a\x1C\x98V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1D\xD4Wa\x1D\xD4a\x1D\xAFV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xC7Wa\x04\xC7a\x1C\x98V[`\0\x82a\x1D\xFFWa\x1D\xFFa\x1D\xAFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1E\x19Wa\x1E\x19a\x1C\x98V[P\x05\x90V\xFE\xA2dipfsX\"\x12 s\x0F\xB8\xED)\x80v\xDD]T\xFC1u\xA5%Z\xFFL\x94\x01\xF9C\0X\xF7R\x967\xF9\x9D\xEB\rdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMALARBITRAGE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c0m\xB4k\x14a\0gW\x80c3\"f\xF3\x14a\0\x8CW\x80cO\xD6|X\x14a\0\x9FW\x80cU\xF0\x11\xC6\x14a\0\xB2W\x80c\x90.\xCA\xA2\x14a\0\xC5W\x80c\xE4]Y<\x14a\0\xD8W[`\0\x80\xFD[a\0za\0u6`\x04a\x1B.V[a\0\xEBV[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0za\0\x9A6`\x04a\x1B.V[a\x01\xE9V[a\0za\0\xAD6`\x04a\x1B.V[a\x02\xDCV[a\0za\0\xC06`\x04a\x1BZV[a\x03\xCFV[a\0za\0\xD36`\x04a\x1B.V[a\x04\xCDV[a\0za\0\xE66`\x04a\x1BZV[a\x05\xC0V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x015W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01Y\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xA8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xCC\x91\x90a\x1C\"V[\x92PP\x91Pa\x01\xDE\x86\x83\x83\x88\x87a\x06\xB4V[\x97\x96PPPPPPPV[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x023W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02W\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xA6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xCA\x91\x90a\x1C\"V[\x92PP\x91Pa\x01\xDE\x86\x83\x83\x88\x87a\x07(V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03J\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\x99W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03\xBD\x91\x90a\x1C\"V[\x92P\x92PPa\x01\xDE\x86\x83\x83\x88\x87a\x07\xA3V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04=\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04\x8CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\xB0\x91\x90a\x1C\"V[\x92PP\x91Pa\x04\xC1\x85\x83\x83\x86a\x08\x05V[\x93PPPP[\x92\x91PPV[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x86\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x17W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05;\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x89\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x8AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xAE\x91\x90a\x1C\"V[\x92P\x92PPa\x01\xDE\x86\x83\x83\x88\x87a\x08\x99V[`\0\x80T`@Qc@\xDA\xFDa`\xE1\x1B\x81R`\x04\x81\x01\x85\x90R\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x81\xB5\xFA\xC2\x90`$\x01`\x80`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06.\x91\x90a\x1C\x06V[`\0\x80T`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x88\x90R\x92\x93P\x90\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xCE\x15;\xF4\x90`$\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06}W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA1\x91\x90a\x1C\"V[\x92P\x92PP`\0a\x01\xDE\x86\x84\x84\x87a\x08\xF5V[`\0\x82`\x01\x82a\x06\xC7\x89\x89\x89\x85\x89a\x07(V[\x90P`\0\x81\x12\x15a\x06\xDEW`\0\x93PPPPa\x07\x1FV[a\x07\x16\x89\x89\x89\x88`@Q` \x01a\x06\xF8\x94\x93\x92\x91\x90a\x1CPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\t\x92a\t\xC3V[P\x90\x94PPPPP[\x95\x94PPPPPV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x07C\x91\x90a\x1C\xAEV[\x90P`\0a\x07U\x88\x88\x88\x85\x89\x89a\n\xE4V[\x90P`\0a\x07b\x82a\x0C\x04V[\x90P`\0a\x07o\x83a\r&V[\x90P\x80\x82\x84`\xE0\x01Qa\x07\x81\x90a\x1C\xD5V[a\x07\x8B\x91\x90a\x1C\xF1V[a\x07\x95\x91\x90a\x1C\xF1V[\x9A\x99PPPPPPPPPPV[`\0\x82`\x01\x82a\x07\xB6\x89\x89\x89\x85\x89a\x08\x99V[\x90P`\0\x81\x12\x15a\x07\xCDW`\0\x93PPPPa\x07\x1FV[a\x07\x16\x89\x89\x89\x88`@Q` \x01a\x07\xE7\x94\x93\x92\x91\x90a\x1CPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x83\x85`\x01a\x01\0a\r\xB1a\t\xC3V[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x08 \x91\x90a\x1C\xAEV[\x90P`\0\x83` \x01Q\x90P`\0a\x08;\x88\x86`\0\x01Qa\r\xE2V[\x90P`\0a\x08ma\x08T\x84g\x1B\xC1mgN\xC8\0\0a\r\xFDV[a\x08^\x84\x86a\r\xFDV[a\x08h\x91\x90a\x1C\xF1V[a\x0E!V[\x90P`\0a\x08\x8Da\x08\x86\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xAEV[\x89\x90a\x0E\x8AV[\x90Pa\x07\x95\x89\x82a\x1C\xAEV[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x08\xB4\x91\x90a\x1C\xAEV[\x90P`\0a\x08\xC6\x88\x88\x88\x85\x89\x89a\x0E\xBDV[\x90P`\0a\x08\xD3\x82a\x0FAV[\x90P`\0a\x08\xE0\x83a\x10UV[\x90P\x80\x82a\x07\x81g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xD5V[`\0\x80\x82`@\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\t\x10\x91\x90a\x1C\xAEV[\x83Q` \x85\x01Q\x91\x92P\x90`\0a\t'\x89\x84a\r\xE2V[\x90P`\0a\t=\x83g\x1B\xC1mgN\xC8\0\0a\r\xFDV[a\tG\x83\x85a\r\xFDV[a\tQ\x91\x90a\x1C\xAEV[\x90P`\0a\t^\x82a\x0E!V[\x90P`\0a\tv\x82a\tp\x8C\x89a\x0E\x8AV[\x90a\x0E\x8AV[\x90Pa\t\x82\x8B\x82a\x1C\xAEV[\x9C\x9BPPPPPPPPPPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\t\xAE\x91\x90a\x1D\x19V[\x93P\x93P\x93P\x93Pa\x01\xDE\x84\x84\x84\x89\x85a\x07(V[`\0\x80`\0\x86\x88\x11\x15a\t\xF8W`@Qc0\x82\xDF\xDB`\xE1\x1B\x81R`\x04\x81\x01\x89\x90R`$\x81\x01\x88\x90R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\n\x08\x8A\x8A\x87c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n\x1A\x8B\x8A\x88c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n(\x82\x84a\x1DYV[\x13\x15a\nQW`@Qc\x06\xF1\xBE]`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R`$\x81\x01\x82\x90R`D\x01a\t\xEFV[`\0a\n]\x8B\x8Ba\x1D\x89V[\x90P\x89\x94P\x8A\x93P`\0[`\x02a\nt\x87\x87a\x1D\x9CV[a\n~\x91\x90a\x1D\xC5V[\x96P`\0a\n\x90\x8E\x89\x8Bc\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\n\x9E\x86\x83a\x1DYV[\x13a\n\xABW\x87\x96Pa\n\xB2V[\x87\x95P\x80\x94P[a\n\xBC\x8D\x8Da\x1D\x89V[\x92PP`\x01\x01\x89\x82\x11\x80\x15a\n\xD0WP\x88\x81\x10[a\nhWPPPP\x96P\x96P\x96\x93PPPPV[a\x0B3`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x0BQa\x0BB\x88\x86a\x1C\xF1V[g\x1B\xC1mgN\xC8\0\0\x90a\x0E\x8AV[\x90P`\0a\x0B_\x85\x87a\x0E\x8AV[a\x0Bi\x86\x89a\x1C\xF1V[a\x0Bs\x91\x90a\x1C\xAEV[\x90P`\0a\x0B\x89a\x0B\x84\x84\x84a\r\xFDV[a\x11\x01V[\x90P`\0a\x0B\x9Eg\x1B\xC1mgN\xC8\0\0a\x13\x7FV[a\x0B\xAC\x90c;\x9A\xCA\0a\x1D\xD9V[`@\x80Qa\x01 \x81\x01\x82R\x93\x84R\x87Q` \x80\x86\x01\x91\x90\x91R\x88\x01Q\x90\x84\x01R``\x83\x01\x89\x90R`\x80\x83\x01\x8B\x90R`\xA0\x83\x01\x8A\x90R`\xC0\x83\x01\x88\x90R`\xE0\x83\x01\x8C\x90Ra\x01\0\x83\x01RP\x92PPP\x96\x95PPPPPPV[`@\x81\x01Q`\0\x90\x81\x90a\x0C,\x90g\x1B\xC1mgN\xC8\0\0\x90a\x0C&\x90\x80a\x0E\x8AV[\x90a\r\xFDV[a\x0C5\x90a\x1C\xD5V[\x90P`\0a\x0C]\x84`\0\x01Qa\tp\x86`@\x01Q\x87a\x01\0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0Csa\x0Cn\x83\x85a\x1C\xF1V[a\x14#V[\x90P`\0a\x0C\xC3a\x0C\xA9\x87``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x94\x90a\x1C\xD5V[a\x0C\x9E\x91\x90a\x1C\xF1V[`\x80\x89\x01Q\x90a\x0E\x8AV[\x87`\xA0\x01Qa\x0C\xB8\x91\x90a\x1C\xF1V[` \x88\x01Q\x90a\x0E\x8AV[\x90P`\0a\x0C\xD1\x83\x83a\x0E\x8AV[\x90P`\0a\x0C\xF0\x88``\x01Q\x89`\xC0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x88`\xC0\x01Q\x89`\xA0\x01Qa\r\x04\x91\x90a\x1C\xF1V[a\r\x0E\x91\x90a\x1C\xAEV[\x90Pa\r\x1A\x82\x82a\r\xFDV[\x98\x97PPPPPPPPV[`\0\x80a\re\x83``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\rC\x90a\x1C\xD5V[a\rM\x91\x90a\x1C\xF1V[` \x85\x01Qa\tp\x90g\x06\xF0[Y\xD3\xB2\0\0\x90a\x0E\x8AV[\x90P`\0a\r\x85\x84a\x01\0\x01Q\x85`@\x01Qa\r\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\r\xA9a\r\xA2\x85`\0\x01Q\x83a\r\x9D\x91\x90a\x1C\xAEV[a\x15\xCCV[\x83\x90a\x0E\x8AV[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x86\x80` \x01\x90Q\x81\x01\x90a\r\xCD\x91\x90a\x1D\x19V[\x93P\x93P\x93P\x93Pa\x01\xDE\x84\x84\x84\x89\x85a\x08\x99V[`\0a\r\xF6a\r\xF1\x84\x84a\x17\xB0V[a\x17\xC5V[\x93\x92PPPV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x81\x02\x90\x81\x05\x83\x14\x82\x15\x15\x16a\x0E\x1BW`\0\x80\xFD[\x05\x91\x90PV[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x0E?g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1DYV[a\x0EI\x91\x90a\x1D\xF0V[\x90P`\0a\x0EV\x82a\x1C\xD5V[\x90P`\0a\x0Ec\x82a\x15\xCCV[\x90Pg\x1B\xC1mgN\xC8\0\0a\x0E\x80g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DYV[a\x07\x1F\x91\x90a\x1D\xF0V[\x81\x81\x02\x82\x15\x83\x82\x05\x83\x14\x17`\0\x19\x84\x10`\x01`\xFF\x1B\x84\x13\x17\x16a\x0E\xACW`\0\x80\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x92\x91PPV[a\x0F\x0C`@Q\x80a\x01 \x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0a\x0F\x1Ba\x0BB\x88\x86a\x1C\xF1V[\x90P`\0a\x0F)\x85\x87a\x0E\x8AV[\x84Q\x86\x90a\x0F7\x90\x8Aa\x0E\x8AV[a\x0Bi\x91\x90a\x1C\xF1V[`@\x81\x01Q`\0\x90\x81\x90a\x0Fc\x90g\x1B\xC1mgN\xC8\0\0\x90a\x0C&\x90\x80a\x0E\x8AV[a\x0Fl\x90a\x1C\xD5V[\x90P`\0a\x0F\x94\x84`\0\x01Qa\tp\x86`@\x01Q\x87a\x01\0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x0F\xA5a\x0Cn\x83\x85a\x1C\xF1V[\x90P`\0a\x0F\xEEa\x0F\xC6\x87``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x0C\x94\x90a\x1C\xD5V[`\xA0\x88\x01Q` \x89\x01Qa\x0F\xD9\x91a\x0E\x8AV[a\x0F\xE3\x91\x90a\x1C\xF1V[`\xE0\x88\x01Q\x90a\x0E\x8AV[\x90P`\0a\x0F\xFC\x83\x83a\x0E\x8AV[\x90P`\0a\r\x0Ea\x10\x1E\x89``\x01Q\x8A`\xC0\x01Qa\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[`\xC0\x8A\x01Q`\xA0\x8B\x01Q` \x8C\x01Qa\x106\x91a\x0E\x8AV[a\x10@\x91\x90a\x1C\xF1V[a\x10J\x91\x90a\x1C\xAEV[` \x8A\x01Q\x90a\x0E\x8AV[`\0\x80a\x10\x87\x83``\x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x10r\x90a\x1C\xD5V[a\x10|\x91\x90a\x1C\xF1V[`\xE0\x85\x01Q\x90a\x0E\x8AV[\x90P`\0a\x10\xA7\x84a\x01\0\x01Q\x85`@\x01Qa\r\xFD\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90P`\0a\x10\xC8a\x10\xC1\x86`\0\x01Q\x84a\r\x9D\x91\x90a\x1C\xAEV[\x84\x90a\x0E\x8AV[\x90P`\0a\x10\xEB\x86` \x01Qg\x1B\xC1mgN\xC8\0\0a\x0E\x8A\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x10\xF7\x82\x82a\r\xFDV[\x96\x95PPPPPPV[`\0\x80\x82\x12\x80a\x11\x18WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x116W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x11WW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x11\x7FW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x11\x8AW\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x11\xB2Wa\x11\xAD\x83g\x1B\xC1mgN\xC8\0\0a\x1C\xAEV[a\x11\xB4V[\x82[\x90P`\0a\x11\xCA\x82g\x1B\xC1mgN\xC8\0\0a\x19\xA0V[\x90P\x80`\0\x03a\x11\xEDW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x11\xF8\x82a\x17\xC5V[\x90P`\0c;\x9A\xCA\0a\x12#a\x12\x1Ea\x12\x18g\x1B\xC1mgN\xC8\0\0a\x1C\xD5V[\x85a\x19\xB5V[a\x13\x7FV[a\x12-\x91\x90a\x1DYV[\x90P`\0\x80a\x12D\x83g\x03\xC1f\\z\xAB \0a\x19\xB5V[a\x12V\x90g \x05\xFEO&\x8E\xA0\0a\x1C\xF1V[\x90P`\0a\x12\x86\x84a\x12o\x86f\x9F2u$b\xA0\0a\x19\xB5V[a\x12\x81\x90g\r\xC5R\x7Fd, \0a\x1C\xF1V[a\x19\xB5V[a\x12\x98\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF1V[\x90Pa\x12\xBCg\t\xD0(\xCCo _\xFF\x19\x85a\x12\xB2\x85\x85a\x19\xA0V[a\x12\x81\x91\x90a\x1C\xAEV[\x92PPP`\0[`\x02\x81\x10\x15a\x13WW`\0\x86a\x12\xD8\x84a\x15\xCCV[a\x12\xE2\x91\x90a\x1C\xAEV[\x90P`\0a\x12\xF0\x84\x85a\x19\xB5V[a\x12\xF9\x90a\x1C\xD5V[\x90P`\0a\x13\x06\x82a\x14#V[\x90P`\0a\x13\x14\x86\x85a\x19\xB5V[a\x13&g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x19\xB5V[a\x130\x91\x90a\x1C\xAEV[\x90Pa\x13<\x84\x82a\x19\xA0V[a\x13F\x90\x87a\x1C\xF1V[\x95P\x84`\x01\x01\x94PPPPPa\x12\xC3V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x13tWa\x13o\x82a\x1C\xD5V[a\r\x1AV[P\x96\x95PPPPPPV[`\xB5\x81`\x01`\x88\x1B\x81\x10a\x13\x98W`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\x13\xB4W` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\x13\xCCW`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\x13\xE2W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x14>WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x14\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\t\xEFV[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0\x81`\0\x03a\x15\xE5WPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x15\xFCWP`\0\x91\x90PV[a\x16\rgV\x98\xEE\xF0fp\0\0a\x1C\xD5V[\x82\x13a\x16\"WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x16-\x83a\x19\xCAV[\x90P`\0a\x16fg\r\xE0\xB6\xB3\xA7d\0\0a\x16O\x84g\x1B\xC1mgN\xC8\0\0a\x1A\x05V[a\x16a\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1C\xF1V[a\x19\xA0V[\x90P`\0\x80\x82a\x16\xC2\x81a\x16\xAF\x81a\x16\x9D\x81a\x16\x8A\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x19\xB5V[a\x12\x81\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1C\xF1V[a\x12\x81\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1C\xF1V[a\x12\x81\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1C\xF1V[a\x16\xD4\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1C\xF1V[\x91P\x83\x90Pa\x17<\x81a\x17*\x81a\x17\x18\x81a\x17\x06\x81a\x16\xF3\x81\x8Ba\x19\xB5V[a\x12\x81\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1C\xF1V[a\x12\x81\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1C\xF1V[a\x12\x81\x90g\x051\n\xA7\xD5!0\0a\x1C\xF1V[a\x12\x81\x90g\r\xE0\xCC=\x15a\0\0a\x1C\xF1V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x17R\x87\x88a\x19\xB5V[a\x17^\x90`\0\x19a\x1DYV[a\x17h\x91\x90a\x1C\xAEV[a\x17r\x91\x90a\x1C\xF1V[\x92PP`\0a\x17\x80\x83a\x14#V[\x90P`\0a\x17\x8E\x85\x83a\x19\xB5V[\x90P`\0\x88\x12a\x17\x9EW\x80a\r\x1AV[a\r\x1A\x81g\x1B\xC1mgN\xC8\0\0a\x1C\xAEV[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\x1AV[`\0\x80\x82\x13a\x18\x02W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\xEFV[`\0``a\x18\x0F\x84a\x1AHV[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1A\xF0V[`\0a\r\xF6\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xF0V[`\0`\x01`\xFF\x1B\x82\x03a\x19\xF0W`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1A\x01WP\x19`\x01\x01\x90V[P\x90V[`\0a\r\xF6\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x1B\x0FV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1A2W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80\x82\x11a\x1A\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\t\xEFV[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\x08W`\0\x80\xFD[\x05\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x1B'W`\0\x80\xFD[\x04\x92\x91PPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1BCW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1BmW`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[`\0`\x80\x82\x84\x03\x12\x15a\x1B\x8EW`\0\x80\xFD[`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x1B\xBFWcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x90\x81R\x83Q\x82R` \x80\x85\x01Q\x90\x83\x01R\x83\x81\x01Q\x90\x82\x01R``\x83\x01Q\x90\x91P\x81\x90`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\xF9W`\0\x80\xFD[``\x91\x90\x91\x01R\x92\x91PPV[`\0`\x80\x82\x84\x03\x12\x15a\x1C\x18W`\0\x80\xFD[a\r\xF6\x83\x83a\x1B|V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1C7W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[\x93\x84R` \x80\x85\x01\x93\x90\x93R`@\x80\x85\x01\x92\x90\x92R\x80Q``\x80\x86\x01\x91\x90\x91R\x92\x81\x01Q`\x80\x85\x01R\x90\x81\x01Q`\xA0\x84\x01R\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R`\xE0\x01\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1C\xCEWa\x1C\xCEa\x1C\x98V[P\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x01a\x1C\xEAWa\x1C\xEAa\x1C\x98V[P`\0\x03\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1D\x11Wa\x1D\x11a\x1C\x98V[PP\x92\x91PPV[`\0\x80`\0\x80`\xE0\x85\x87\x03\x12\x15a\x1D/W`\0\x80\xFD[\x84Q\x93P` \x85\x01Q\x92P`@\x85\x01Q\x91Pa\x1DN\x86``\x87\x01a\x1B|V[\x90P\x92\x95\x91\x94P\x92PV[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1DuWa\x1Dua\x1C\x98V[\x81\x81\x05\x83\x14\x82\x15\x17a\x04\xC7Wa\x04\xC7a\x1C\x98V[\x81\x81\x03\x81\x81\x11\x15a\x04\xC7Wa\x04\xC7a\x1C\x98V[\x80\x82\x01\x80\x82\x11\x15a\x04\xC7Wa\x04\xC7a\x1C\x98V[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\0\x82a\x1D\xD4Wa\x1D\xD4a\x1D\xAFV[P\x04\x90V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x04\xC7Wa\x04\xC7a\x1C\x98V[`\0\x82a\x1D\xFFWa\x1D\xFFa\x1D\xAFV[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1E\x19Wa\x1E\x19a\x1C\x98V[P\x05\x90V\xFE\xA2dipfsX\"\x12 s\x0F\xB8\xED)\x80v\xDD]T\xFC1u\xA5%Z\xFFL\x94\x01\xF9C\0X\xF7R\x967\xF9\x9D\xEB\rdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMALARBITRAGE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormalArbitrage<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormalArbitrage<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormalArbitrage<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormalArbitrage<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormalArbitrage<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormalArbitrage))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormalArbitrage<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMALARBITRAGE_ABI.clone(),
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
                LOGNORMALARBITRAGE_ABI.clone(),
                LOGNORMALARBITRAGE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
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
        /// Calls the contract's `getDxGivenS` (0x55f011c6) function
        pub fn get_dx_given_s(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([85, 240, 17, 198], (pool_id, s))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getDyGivenS` (0xe45d593c) function
        pub fn get_dy_given_s(
            &self,
            pool_id: ::ethers::core::types::U256,
            s: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([228, 93, 89, 60], (pool_id, s))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for LogNormalArbitrage<M>
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
    pub enum LogNormalArbitrageErrors {
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
    impl ::ethers::core::abi::AbiDecode for LogNormalArbitrageErrors {
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
    impl ::ethers::core::abi::AbiEncode for LogNormalArbitrageErrors {
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
    impl ::ethers::contract::ContractRevert for LogNormalArbitrageErrors {
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
    impl ::core::fmt::Display for LogNormalArbitrageErrors {
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
    impl ::core::convert::From<::std::string::String> for LogNormalArbitrageErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BisectionLib_InvalidBounds> for LogNormalArbitrageErrors {
        fn from(value: BisectionLib_InvalidBounds) -> Self {
            Self::BisectionLib_InvalidBounds(value)
        }
    }
    impl ::core::convert::From<BisectionLib_RootOutsideBounds> for LogNormalArbitrageErrors {
        fn from(value: BisectionLib_RootOutsideBounds) -> Self {
            Self::BisectionLib_RootOutsideBounds(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalArbitrageErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalArbitrageErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalArbitrageErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalArbitrageErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
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
    /// Container type for all input parameters for the `getDxGivenS` function
    /// with signature `getDxGivenS(uint256,uint256)` and selector `0x55f011c6`
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
    #[ethcall(name = "getDxGivenS", abi = "getDxGivenS(uint256,uint256)")]
    pub struct GetDxGivenSCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `getDyGivenS` function
    /// with signature `getDyGivenS(uint256,uint256)` and selector `0xe45d593c`
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
    #[ethcall(name = "getDyGivenS", abi = "getDyGivenS(uint256,uint256)")]
    pub struct GetDyGivenSCall {
        pub pool_id: ::ethers::core::types::U256,
        pub s: ::ethers::core::types::U256,
    }
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
    pub enum LogNormalArbitrageCalls {
        CalculateDiffLower(CalculateDiffLowerCall),
        CalculateDiffRaise(CalculateDiffRaiseCall),
        ComputeOptimalArbLowerPrice(ComputeOptimalArbLowerPriceCall),
        ComputeOptimalArbRaisePrice(ComputeOptimalArbRaisePriceCall),
        GetDxGivenS(GetDxGivenSCall),
        GetDyGivenS(GetDyGivenSCall),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalArbitrageCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
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
            if let Ok(decoded) = <GetDxGivenSCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDxGivenS(decoded));
            }
            if let Ok(decoded) = <GetDyGivenSCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetDyGivenS(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalArbitrageCalls {
        fn encode(self) -> Vec<u8> {
            match self {
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
                Self::GetDxGivenS(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetDyGivenS(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalArbitrageCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CalculateDiffLower(element) => ::core::fmt::Display::fmt(element, f),
                Self::CalculateDiffRaise(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbLowerPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeOptimalArbRaisePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDxGivenS(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDyGivenS(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CalculateDiffLowerCall> for LogNormalArbitrageCalls {
        fn from(value: CalculateDiffLowerCall) -> Self {
            Self::CalculateDiffLower(value)
        }
    }
    impl ::core::convert::From<CalculateDiffRaiseCall> for LogNormalArbitrageCalls {
        fn from(value: CalculateDiffRaiseCall) -> Self {
            Self::CalculateDiffRaise(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbLowerPriceCall> for LogNormalArbitrageCalls {
        fn from(value: ComputeOptimalArbLowerPriceCall) -> Self {
            Self::ComputeOptimalArbLowerPrice(value)
        }
    }
    impl ::core::convert::From<ComputeOptimalArbRaisePriceCall> for LogNormalArbitrageCalls {
        fn from(value: ComputeOptimalArbRaisePriceCall) -> Self {
            Self::ComputeOptimalArbRaisePrice(value)
        }
    }
    impl ::core::convert::From<GetDxGivenSCall> for LogNormalArbitrageCalls {
        fn from(value: GetDxGivenSCall) -> Self {
            Self::GetDxGivenS(value)
        }
    }
    impl ::core::convert::From<GetDyGivenSCall> for LogNormalArbitrageCalls {
        fn from(value: GetDyGivenSCall) -> Self {
            Self::GetDyGivenS(value)
        }
    }
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
    /// Container type for all return fields from the `getDxGivenS` function
    /// with signature `getDxGivenS(uint256,uint256)` and selector `0x55f011c6`
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
    pub struct GetDxGivenSReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the `getDyGivenS` function
    /// with signature `getDyGivenS(uint256,uint256)` and selector `0xe45d593c`
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
    pub struct GetDyGivenSReturn(pub ::ethers::core::types::I256);
}

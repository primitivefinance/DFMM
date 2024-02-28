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
                                name: ::std::borrow::ToOwned::to_owned("params"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Address,
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ConstantSum.ConstantSumParams",
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
                    ::std::borrow::ToOwned::to_owned("simulateAllocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateAllocate"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
                    ::std::borrow::ToOwned::to_owned("simulateDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateDeallocate"),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x12?8\x03\x80a\x12?\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x11\xAC\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0gW\x80c\x83\x9D\xFA+\x14a\0\x92W\x80c\x89\xEA\x85Y\x14a\0\xB3W\x80c\xA4##\x87\x14a\0\xD3W\x80c\xA8\xC6.v\x14a\0\xE6W\x80c\xC2\xB0\xBBj\x14a\x01\x11W[`\0\x80\xFD[a\0za\0u6`\x04a\x0C\xD4V[a\x01$V[`@Qa\0\x89\x93\x92\x91\x90a\r\\V[`@Q\x80\x91\x03\x90\xF3[a\0\xA5a\0\xA06`\x04a\r\x86V[a\x05UV[`@Qa\0\x89\x92\x91\x90a\r\xB2V[a\0\xC6a\0\xC16`\x04a\x0E)V[a\x08SV[`@Qa\0\x89\x91\x90a\x0E\x95V[a\0\xC6a\0\xE16`\x04a\x0E\xA8V[a\x08\xC9V[`\0Ta\0\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x89V[a\0\xA5a\x01\x1F6`\x04a\r\x86V[a\x08\xDAV[`\0\x80``a\x01M`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x01q`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE6\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x13\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x020W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02T\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xD9\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\x02\xEC\x91\x90a\x0F\xB9V[\x90P`\0\x88\x15a\x03\xA6W`\0a\x03\x0F\x83` \x01Q\x8Aa\x0B\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x03?\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x03,\x91\x90a\x10\x11V[\x84Qa\x039\x90\x8C\x90a\x0B\xCFV[\x90a\x0B\xCFV[\x85Q\x90\x92Pa\x03O\x90\x8A\x90a\x10$V[\x84R`@\x85\x01Qa\x03a\x90\x82\x90a\x10$V[`@\x85\x01R` \x85\x01Q\x82\x11\x15a\x03\x8BW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x85` \x01Qa\x03\x9B\x91\x90a\x10\x11V[` \x85\x01RPa\x04PV[\x81Q` \x83\x01Q`\0\x91a\x03\xC5\x91a\x03\xBF\x90\x8C\x90a\x0B\xB3V[\x90a\x0B\xE4V[\x90Pa\x03\xF1\x83`\0\x01Qa\x03\xEB\x8B\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x039\x91\x90a\x10\x11V[\x90a\x0B\xF9V[\x91P\x88\x85` \x01Qa\x04\x03\x91\x90a\x10$V[` \x85\x01R`@\x85\x01Qa\x04\x18\x90\x82\x90a\x10$V[`@\x85\x01R\x84Q\x82\x11\x15a\x04?W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa\x04L\x90\x83\x90a\x10\x11V[\x84RP[a\x04Xa\x0C\x87V[\x84Q``\x82\x01R\x84Q`\x80\x82\x01R`@\x80\x86\x01Q`\xA0\x83\x01RQ`\0\x90a\x04\x9F\x90\x86\x90` \x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ce\xC9\xFF\xC20\x8F\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF8\x94\x93\x92\x91\x90a\x107V[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x059\x91\x90a\x10\xAEV[P\x93\x9DP\x96\x9BP\x93\x99PPPPPPPPPP\x93P\x93P\x93\x90PV[`\0``a\x05}`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xA1`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x16\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06C\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x84\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\t\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\x07\x1C\x91\x90a\x0F\xB9V[\x83Q\x90\x91Pa\x07,\x90\x88\x90a\x10$V[\x82R` \x83\x01Qa\x07>\x90\x87\x90a\x10$V[` \x83\x01\x81\x90R\x81Qa\x07Q\x91\x90a\x0B\xE4V[\x82Qa\x07]\x91\x90a\x10$V[`@\x83\x01Ra\x07ja\x0C\x87V[\x83Q``\x82\x01R` \x80\x85\x01Q`\x80\x83\x01R`@\x80\x86\x01Q`\xA0\x84\x01RQ`\0\x91a\x07\xB3\x91\x86\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x01%Q\x03`\xE1\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02J\xA2\x06\x90a\x07\xFC\x900\x90\x8F\x90\x88\x90\x88\x90`\x04\x01a\x107V[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08=\x91\x90a\x11\x01V[P\x92\x9E\x94\x9DP\x93\x9BPPPPPPPPPPPPV[\x80Q``\x90`\0\x90a\x08f\x90\x85\x90a\x0B\xE4V[a\x08p\x90\x86a\x10$V[`@\x80Q` \x80\x82\x01\x89\x90R\x81\x83\x01\x88\x90R``\x82\x01\x84\x90R\x86Q`\x80\x83\x01R\x86\x01Q`\xA0\x82\x01R\x90\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[``a\x08\xD4\x82a\x0C\x0EV[\x92\x91PPV[`\0``a\t\x02`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t&`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9B\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC8\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\t\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x8E\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\n\xA1\x91\x90a\x0F\xB9V[\x90P\x86\x83`\0\x01Q\x10\x80a\n\xB8WP\x85\x83` \x01Q\x10[\x15a\n\xD6W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Qa\n\xE3\x90\x88\x90a\x10\x11V[\x82R` \x83\x01Qa\n\xF5\x90\x87\x90a\x10\x11V[` \x83\x01\x81\x90R\x81Qa\x0B\x08\x91\x90a\x0B\xE4V[\x82Qa\x0B\x14\x91\x90a\x10$V[`@\x83\x01Ra\x0B!a\x0C\x87V[\x83Q``\x82\x01R` \x80\x85\x01Q`\x80\x83\x01R`@\x80\x86\x01Q`\xA0\x84\x01RQ`\0\x91a\x0Bj\x91\x86\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x0E[\x8F\x9F`\xE2\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9n>|\x90a\x07\xFC\x900\x90\x8F\x90\x88\x90\x88\x90`\x04\x01a\x107V[`\0a\x0B\xC8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C:V[\x93\x92PPPV[`\0a\x0B\xC8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0ChV[`\0a\x0B\xC8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C:V[`\0a\x0B\xC8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0ChV[```\x02\x82`@Q` \x01a\x0C$\x92\x91\x90a\x11JV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CRW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x80W`\0\x80\xFD[\x04\x92\x91PPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x90V[\x80\x15\x15\x81\x14a\x0C\xD1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xE9W`\0\x80\xFD[\x835\x92P` \x84\x015a\x0C\xFB\x81a\x0C\xC3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\r'W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x0FV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rH\x81` \x86\x01` \x86\x01a\r\x0CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r}``\x83\x01\x84a\r0V[\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r\x9BW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\r\xCD`@\x83\x01\x84a\r0V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x0EWa\x0E\x0Ea\r\xD5V[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xD1W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0E?W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0E\\W`\0\x80\xFD[Pa\x0Eea\r\xEBV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x0E\x84\x81a\x0E\x14V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x0B\xC8` \x83\x01\x84a\r0V[`\0` \x82\x84\x03\x12\x15a\x0E\xBAW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xD3W`\0\x80\xFD[\x81Qa\x0B\xC8\x81a\x0E\x14V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xF3W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\x1EW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F6W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0FJW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F\\Wa\x0F\\a\r\xD5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0F\x84Wa\x0F\x84a\r\xD5V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x0F\x9DW`\0\x80\xFD[a\x0F\xAE\x83` \x83\x01` \x88\x01a\r\x0CV[\x97\x96PPPPPPPV[`\0``\x82\x84\x03\x12\x15a\x0F\xCBW`\0\x80\xFD[a\x0F\xD3a\r\xEBV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0F\xEF\x81a\x0E\x14V[`@\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x08\xD4Wa\x08\xD4a\x0F\xFBV[\x80\x82\x01\x80\x82\x11\x15a\x08\xD4Wa\x08\xD4a\x0F\xFBV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x0F\xAE\x81\x84\x01\x85a\r0V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x10\xC7W`\0\x80\xFD[\x86Qa\x10\xD2\x81a\x0C\xC3V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x11\x19W`\0\x80\xFD[\x85Qa\x11$\x81a\x0C\xC3V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`@\x81\x01`\x04\x84\x10a\x11lWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V\xFE\xA2dipfsX\"\x12 d\x85_\xAA{\xAC,\x01\xCF\xCAcJ\xC4\xC5S\xA8\x06\x17|\nw,tD\x83_\xAE\xD45\xE4\x98\x1BdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0gW\x80c\x83\x9D\xFA+\x14a\0\x92W\x80c\x89\xEA\x85Y\x14a\0\xB3W\x80c\xA4##\x87\x14a\0\xD3W\x80c\xA8\xC6.v\x14a\0\xE6W\x80c\xC2\xB0\xBBj\x14a\x01\x11W[`\0\x80\xFD[a\0za\0u6`\x04a\x0C\xD4V[a\x01$V[`@Qa\0\x89\x93\x92\x91\x90a\r\\V[`@Q\x80\x91\x03\x90\xF3[a\0\xA5a\0\xA06`\x04a\r\x86V[a\x05UV[`@Qa\0\x89\x92\x91\x90a\r\xB2V[a\0\xC6a\0\xC16`\x04a\x0E)V[a\x08SV[`@Qa\0\x89\x91\x90a\x0E\x95V[a\0\xC6a\0\xE16`\x04a\x0E\xA8V[a\x08\xC9V[`\0Ta\0\xF9\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x89V[a\0\xA5a\x01\x1F6`\x04a\r\x86V[a\x08\xDAV[`\0\x80``a\x01M`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x01q`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xC2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xE6\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x89`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x02\x13\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x020W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02T\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02\xB1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\xD9\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\x02\xEC\x91\x90a\x0F\xB9V[\x90P`\0\x88\x15a\x03\xA6W`\0a\x03\x0F\x83` \x01Q\x8Aa\x0B\xB3\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x90Pa\x03?\x83` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x03,\x91\x90a\x10\x11V[\x84Qa\x039\x90\x8C\x90a\x0B\xCFV[\x90a\x0B\xCFV[\x85Q\x90\x92Pa\x03O\x90\x8A\x90a\x10$V[\x84R`@\x85\x01Qa\x03a\x90\x82\x90a\x10$V[`@\x85\x01R` \x85\x01Q\x82\x11\x15a\x03\x8BW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x85` \x01Qa\x03\x9B\x91\x90a\x10\x11V[` \x85\x01RPa\x04PV[\x81Q` \x83\x01Q`\0\x91a\x03\xC5\x91a\x03\xBF\x90\x8C\x90a\x0B\xB3V[\x90a\x0B\xE4V[\x90Pa\x03\xF1\x83`\0\x01Qa\x03\xEB\x8B\x86` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x039\x91\x90a\x10\x11V[\x90a\x0B\xF9V[\x91P\x88\x85` \x01Qa\x04\x03\x91\x90a\x10$V[` \x85\x01R`@\x85\x01Qa\x04\x18\x90\x82\x90a\x10$V[`@\x85\x01R\x84Q\x82\x11\x15a\x04?W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84Qa\x04L\x90\x83\x90a\x10\x11V[\x84RP[a\x04Xa\x0C\x87V[\x84Q``\x82\x01R\x84Q`\x80\x82\x01R`@\x80\x86\x01Q`\xA0\x83\x01RQ`\0\x90a\x04\x9F\x90\x86\x90` \x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16ce\xC9\xFF\xC20\x8F\x86\x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x04\xF8\x94\x93\x92\x91\x90a\x107V[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x059\x91\x90a\x10\xAEV[P\x93\x9DP\x96\x9BP\x93\x99PPPPPPPPPP\x93P\x93P\x93\x90PV[`\0``a\x05}`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\x05\xA1`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xF2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x16\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06C\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06`W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x84\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xE1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x07\t\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\x07\x1C\x91\x90a\x0F\xB9V[\x83Q\x90\x91Pa\x07,\x90\x88\x90a\x10$V[\x82R` \x83\x01Qa\x07>\x90\x87\x90a\x10$V[` \x83\x01\x81\x90R\x81Qa\x07Q\x91\x90a\x0B\xE4V[\x82Qa\x07]\x91\x90a\x10$V[`@\x83\x01Ra\x07ja\x0C\x87V[\x83Q``\x82\x01R` \x80\x85\x01Q`\x80\x83\x01R`@\x80\x86\x01Q`\xA0\x84\x01RQ`\0\x91a\x07\xB3\x91\x86\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x01%Q\x03`\xE1\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\x02J\xA2\x06\x90a\x07\xFC\x900\x90\x8F\x90\x88\x90\x88\x90`\x04\x01a\x107V[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x08\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08=\x91\x90a\x11\x01V[P\x92\x9E\x94\x9DP\x93\x9BPPPPPPPPPPPPV[\x80Q``\x90`\0\x90a\x08f\x90\x85\x90a\x0B\xE4V[a\x08p\x90\x86a\x10$V[`@\x80Q` \x80\x82\x01\x89\x90R\x81\x83\x01\x88\x90R``\x82\x01\x84\x90R\x86Q`\x80\x83\x01R\x86\x01Q`\xA0\x82\x01R\x90\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\xC0\x82\x01R\x90\x91P`\xE0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[``a\x08\xD4\x82a\x0C\x0EV[\x92\x91PPV[`\0``a\t\x02`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[a\t&`@Q\x80``\x01`@R\x80`\0\x81R` \x01`\0\x81R` \x01`\0\x81RP\x90V[`\0\x80T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\twW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\x9B\x91\x90a\x0E\xC1V[`\x01`\x01`\xA0\x1B\x03\x16c\xCE\x15;\xF4\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\t\xC8\x91\x81R` \x01\x90V[```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xE5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n\t\x91\x90a\x0E\xDEV[`@\x85\x81\x01\x91\x90\x91R` \x85\x01\x91\x90\x91R\x90\x83R`\0\x80T\x91Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8A\x90R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nfW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\n\x8E\x91\x90\x81\x01\x90a\x0F\x0CV[\x80` \x01\x90Q\x81\x01\x90a\n\xA1\x91\x90a\x0F\xB9V[\x90P\x86\x83`\0\x01Q\x10\x80a\n\xB8WP\x85\x83` \x01Q\x10[\x15a\n\xD6W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Qa\n\xE3\x90\x88\x90a\x10\x11V[\x82R` \x83\x01Qa\n\xF5\x90\x87\x90a\x10\x11V[` \x83\x01\x81\x90R\x81Qa\x0B\x08\x91\x90a\x0B\xE4V[\x82Qa\x0B\x14\x91\x90a\x10$V[`@\x83\x01Ra\x0B!a\x0C\x87V[\x83Q``\x82\x01R` \x80\x85\x01Q`\x80\x83\x01R`@\x80\x86\x01Q`\xA0\x84\x01RQ`\0\x91a\x0Bj\x91\x86\x91\x01\x81Q\x81R` \x80\x83\x01Q\x90\x82\x01R`@\x91\x82\x01Q\x91\x81\x01\x91\x90\x91R``\x01\x90V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x90\x82\x90R`\0\x80Tc\x0E[\x8F\x9F`\xE2\x1B\x84R\x91\x93P\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9n>|\x90a\x07\xFC\x900\x90\x8F\x90\x88\x90\x88\x90`\x04\x01a\x107V[`\0a\x0B\xC8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0C:V[\x93\x92PPPV[`\0a\x0B\xC8\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x0ChV[`\0a\x0B\xC8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0C:V[`\0a\x0B\xC8\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x0ChV[```\x02\x82`@Q` \x01a\x0C$\x92\x91\x90a\x11JV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0CRW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x0C\x80W`\0\x80\xFD[\x04\x92\x91PPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R\x90V[\x80\x15\x15\x81\x14a\x0C\xD1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0C\xE9W`\0\x80\xFD[\x835\x92P` \x84\x015a\x0C\xFB\x81a\x0C\xC3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\r'W\x81\x81\x01Q\x83\x82\x01R` \x01a\r\x0FV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\rH\x81` \x86\x01` \x86\x01a\r\x0CV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\r}``\x83\x01\x84a\r0V[\x95\x94PPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\r\x9BW`\0\x80\xFD[PP\x815\x93` \x83\x015\x93P`@\x90\x92\x015\x91\x90PV[\x82\x15\x15\x81R`@` \x82\x01R`\0a\r\xCD`@\x83\x01\x84a\r0V[\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0E\x0EWa\x0E\x0Ea\r\xD5V[`@R\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x0C\xD1W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x0E?W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x0E\\W`\0\x80\xFD[Pa\x0Eea\r\xEBV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x0E\x84\x81a\x0E\x14V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x0B\xC8` \x83\x01\x84a\r0V[`\0` \x82\x84\x03\x12\x15a\x0E\xBAW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x0E\xD3W`\0\x80\xFD[\x81Qa\x0B\xC8\x81a\x0E\x14V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x0E\xF3W`\0\x80\xFD[\x83Q\x92P` \x84\x01Q\x91P`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a\x0F\x1EW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0F6W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\x0FJW`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0F\\Wa\x0F\\a\r\xD5V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x0F\x84Wa\x0F\x84a\r\xD5V[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a\x0F\x9DW`\0\x80\xFD[a\x0F\xAE\x83` \x83\x01` \x88\x01a\r\x0CV[\x97\x96PPPPPPPV[`\0``\x82\x84\x03\x12\x15a\x0F\xCBW`\0\x80\xFD[a\x0F\xD3a\r\xEBV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0F\xEF\x81a\x0E\x14V[`@\x82\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x08\xD4Wa\x08\xD4a\x0F\xFBV[\x80\x82\x01\x80\x82\x11\x15a\x08\xD4Wa\x08\xD4a\x0F\xFBV[`\0a\x01@`\x01\x80`\xA0\x1B\x03\x80\x88\x16\x84R\x86` \x85\x01R\x80\x86Q\x16`@\x85\x01R\x80` \x87\x01Q\x16``\x85\x01R\x80`@\x87\x01Q\x16`\x80\x85\x01R``\x86\x01Q`\xA0\x85\x01R`\x80\x86\x01Q`\xC0\x85\x01R`\xA0\x86\x01Q`\xE0\x85\x01R\x80`\xC0\x87\x01Q\x16a\x01\0\x85\x01RP\x80a\x01 \x84\x01Ra\x0F\xAE\x81\x84\x01\x85a\r0V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a\x10\xC7W`\0\x80\xFD[\x86Qa\x10\xD2\x81a\x0C\xC3V[` \x88\x01Q`@\x89\x01Q``\x8A\x01Q`\x80\x8B\x01Q`\xA0\x90\x9B\x01Q\x93\x9C\x92\x9BP\x90\x99\x90\x98P\x96P\x90\x94P\x92PPPV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a\x11\x19W`\0\x80\xFD[\x85Qa\x11$\x81a\x0C\xC3V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`@\x81\x01`\x04\x84\x10a\x11lWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x92\x81R` \x01R\x90V\xFE\xA2dipfsX\"\x12 d\x85_\xAA{\xAC,\x01\xCF\xCAcJ\xC4\xC5S\xA8\x06\x17|\nw,tD\x83_\xAE\xD45\xE4\x98\x1BdsolcC\0\x08\x16\x003";
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
            rx: ::ethers::core::types::U256,
            ry: ::ethers::core::types::U256,
            params: ConstantSumParams,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([137, 234, 133, 89], (rx, ry, params))
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
        /// Calls the contract's `simulateAllocate` (0x839dfa2b) function
        pub fn simulate_allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::Bytes)>
        {
            self.0
                .method_hash([131, 157, 250, 43], (pool_id, amount_x, amount_y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `simulateDeallocate` (0xc2b0bb6a) function
        pub fn simulate_deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::Bytes)>
        {
            self.0
                .method_hash([194, 176, 187, 106], (pool_id, amount_x, amount_y))
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
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub params: ConstantSumParams,
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
    /// Container type for all input parameters for the `simulateAllocate`
    /// function with signature `simulateAllocate(uint256,uint256,uint256)` and
    /// selector `0x839dfa2b`
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
        name = "simulateAllocate",
        abi = "simulateAllocate(uint256,uint256,uint256)"
    )]
    pub struct SimulateAllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `simulateDeallocate`
    /// function with signature `simulateDeallocate(uint256,uint256,uint256)`
    /// and selector `0xc2b0bb6a`
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
        name = "simulateDeallocate",
        abi = "simulateDeallocate(uint256,uint256,uint256)"
    )]
    pub struct SimulateDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub amount_x: ::ethers::core::types::U256,
        pub amount_y: ::ethers::core::types::U256,
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
        PreparePriceUpdate(PreparePriceUpdateCall),
        SimulateAllocate(SimulateAllocateCall),
        SimulateDeallocate(SimulateDeallocateCall),
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
            if let Ok(decoded) =
                <PreparePriceUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PreparePriceUpdate(decoded));
            }
            if let Ok(decoded) =
                <SimulateAllocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateAllocate(decoded));
            }
            if let Ok(decoded) =
                <SimulateDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateDeallocate(decoded));
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
                Self::PreparePriceUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateAllocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulateDeallocate(element) => {
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
                Self::PreparePriceUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateAllocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateDeallocate(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<PreparePriceUpdateCall> for ConstantSumSolverCalls {
        fn from(value: PreparePriceUpdateCall) -> Self {
            Self::PreparePriceUpdate(value)
        }
    }
    impl ::core::convert::From<SimulateAllocateCall> for ConstantSumSolverCalls {
        fn from(value: SimulateAllocateCall) -> Self {
            Self::SimulateAllocate(value)
        }
    }
    impl ::core::convert::From<SimulateDeallocateCall> for ConstantSumSolverCalls {
        fn from(value: SimulateDeallocateCall) -> Self {
            Self::SimulateDeallocate(value)
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
    /// Container type for all return fields from the `simulateAllocate`
    /// function with signature `simulateAllocate(uint256,uint256,uint256)` and
    /// selector `0x839dfa2b`
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
    pub struct SimulateAllocateReturn(pub bool, pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `simulateDeallocate`
    /// function with signature `simulateDeallocate(uint256,uint256,uint256)`
    /// and selector `0xc2b0bb6a`
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
    pub struct SimulateDeallocateReturn(pub bool, pub ::ethers::core::types::Bytes);
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

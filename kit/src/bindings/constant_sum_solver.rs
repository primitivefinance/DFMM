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
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x0E\xBB8\x03\x80a\x0E\xBB\x839\x81\x01`@\x81\x90Ra\0/\x91a\0TV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\0\x84V[`\0` \x82\x84\x03\x12\x15a\0fW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0}W`\0\x80\xFD[\x93\x92PPPV[a\x0E(\x80a\0\x93`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0gW\x80c\x89\xEA\x85Y\x14a\0\x92W\x80c\x91]?\xB9\x14a\0\xB2W\x80c\xA4##\x87\x14a\0\xC5W\x80c\xA8\xC6.v\x14a\0\xD8W\x80c\xCB\x1FU2\x14a\x01\x03W[`\0\x80\xFD[a\0za\0u6`\x04a\x06\xA4V[a\x01\x16V[`@Qa\0\x89\x93\x92\x91\x90a\x07,V[`@Q\x80\x91\x03\x90\xF3[a\0\xA5a\0\xA06`\x04a\x07\xFEV[a\x04\xB4V[`@Qa\0\x89\x91\x90a\x08jV[a\0\xA5a\0\xC06`\x04a\x08}V[a\x04\xCBV[a\0\xA5a\0\xD36`\x04a\x08}V[a\x04\xDCV[`\0Ta\0\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x89V[a\0\xA5a\x01\x116`\x04a\x08\x96V[a\x04\xE7V[`\0\x80```\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x93\x91\x90a\x08\xC3V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xC0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x05\x91\x90\x81\x01\x90a\t\xDBV[`\0\x80T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02|\x91\x90\x81\x01\x90a\n\xB6V[\x80` \x01\x90Q\x81\x01\x90a\x02\x8F\x91\x90a\x0BJV[\x90Pa\x02\xAE`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x87\x15a\x036Wa\x02\xC0\x87\x83`\x01a\x04\xF2V[Pa\x02\xEF\x82` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x02\xDC\x91\x90a\x0B\x8CV[\x83Qa\x02\xE9\x90\x8A\x90a\x05\"V[\x90a\x05\"V[\x80\x82R`@\x84\x01Q\x80Q`\x01\x90\x81\x10a\x03\nWa\x03\na\x0B\xADV[` \x02` \x01\x01Q\x10\x15a\x031W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xADV[a\x03B\x87\x83`\0a\x04\xF2V[Pa\x03m\x82`\0\x01Qa\x03g\x89\x85` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x02\xE9\x91\x90a\x0B\x8CV[\x90a\x057V[\x80\x82R`@\x84\x01Q\x80Q`\0\x90a\x03\x86Wa\x03\x86a\x0B\xADV[` \x02` \x01\x01Q\x10\x15a\x03\xADW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x88\x15a\x03\xEDWP\x80Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8A\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x04!V[P\x80Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8A\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x80T`@Qcu\xE6D\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cu\xE6D\x0F\x90a\x04X\x900\x90\x8F\x90\x8A\x90\x88\x90`\x04\x01a\x0B\xFFV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x99\x91\x90a\x0C\xF3V[PP\x96Q\x93\x9F\x93\x9EP\x94\x9CP\x91\x9APPPPPPPPPPPV[``a\x04\xC1\x84\x84\x84a\x05LV[\x90P[\x93\x92PPPV[``a\x04\xD6\x82a\x05\xDDV[\x92\x91PPV[``a\x04\xD6\x82a\x06\tV[``a\x04\xD6\x82a\x06\x1FV[`\0\x81\x15a\x05\x10W` \x83\x01Qa\x05\t\x90\x85a\x065V[\x90Pa\x04\xC4V[\x82Q` \x84\x01Qa\x05\t\x91\x86\x90a\x06FV[`\0a\x04\xC4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x06tV[`\0a\x04\xC4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x06tV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x05\x85Wa\x05\x85a\x0B\xADV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\x05\xA5Wa\x05\xA5a\x0B\xADV[` \x02` \x01\x01\x81\x81RPP\x80\x83`@Q` \x01a\x05\xC4\x92\x91\x90a\rPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[```\x01\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x02\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xB1V[```\x03\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xCCV[`\0a\x04\xC4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x06^W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x06\x8CW`\0\x80\xFD[\x04\x92\x91PPV[\x80\x15\x15\x81\x14a\x06\xA1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\xB9W`\0\x80\xFD[\x835\x92P` \x84\x015a\x06\xCB\x81a\x06\x93V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\x06\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x18\x81` \x86\x01` \x86\x01a\x06\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x07M``\x83\x01\x84a\x07\0V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8FWa\x07\x8Fa\x07VV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8FWa\x07\x8Fa\x07VV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xE1Wa\x07\xE1a\x07VV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xA1W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x08\x14W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x081W`\0\x80\xFD[Pa\x08:a\x07lV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x08Y\x81a\x07\xE9V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x04\xC4` \x83\x01\x84a\x07\0V[`\0` \x82\x84\x03\x12\x15a\x08\x8FW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xA8W`\0\x80\xFD[\x815a\x04\xC4\x81a\x07\xE9V[\x80Qa\x08\xBE\x81a\x07\xE9V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xD5W`\0\x80\xFD[\x81Qa\x04\xC4\x81a\x07\xE9V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xFAWa\x08\xFAa\x07VV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\t\x15W`\0\x80\xFD[\x81Q` a\t*a\t%\x83a\x08\xE0V[a\x07\xB8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\tLW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\tqW\x80Qa\td\x81a\x07\xE9V[\x83R\x91\x83\x01\x91\x83\x01a\tQV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\t\x8DW`\0\x80\xFD[\x81Q` a\t\x9Da\t%\x83a\x08\xE0V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\t\xBFW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\tqW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\t\xC4V[`\0` \x82\x84\x03\x12\x15a\t\xEDW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\x05W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\n\x19W`\0\x80\xFD[a\n!a\x07\x95V[a\n*\x83a\x08\xB3V[\x81R` \x83\x01Q\x82\x81\x11\x15a\n>W`\0\x80\xFD[a\nJ\x87\x82\x86\x01a\t\x04V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\nbW`\0\x80\xFD[a\nn\x87\x82\x86\x01a\t|V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\n\x8A`\x80\x84\x01a\x08\xB3V[`\x80\x82\x01Ra\n\x9B`\xA0\x84\x01a\x08\xB3V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xC8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE0W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\n\xF4W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0B\x06Wa\x0B\x06a\x07VV[a\x0B\x19`\x1F\x82\x01`\x1F\x19\x16` \x01a\x07\xB8V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x0B0W`\0\x80\xFD[a\x0BA\x81` \x84\x01` \x86\x01a\x06\xDCV[P\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a\x0B\\W`\0\x80\xFD[a\x0Bda\x07lV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0B\x80\x81a\x07\xE9V[`@\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x04\xD6WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x0B\xF4W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0B\xD8V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x0CmW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x0CKV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x0C\x8B\x81\x86a\x0B\xC3V[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x0C\xB6a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x0C\xE8\x81\x85a\x07\0V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\x0EW`\0\x80\xFD[\x87Qa\r\x19\x81a\x06\x93V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x80\x81R`\0a\rc`\x80\x83\x01\x85a\x0B\xC3V[\x90P\x82Q` \x83\x01R` \x83\x01Q`@\x83\x01R`\x01\x80`\xA0\x1B\x03`@\x84\x01Q\x16``\x83\x01R\x93\x92PPPV[`\x04\x81\x10a\r\xADWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01a\r\xBF\x82\x85a\r\x8FV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a\r\xDA\x82\x85a\r\x8FV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 \xFB\x1B^8\x88\x96fL$\xFE\xBD\xA9\xF9\xEFJ1\x06n\xEA\xF6c56m:\xB3\xD3\x14;\xBEG\xB5dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0bW`\x005`\xE0\x1C\x80c9(\xFF\x97\x14a\0gW\x80c\x89\xEA\x85Y\x14a\0\x92W\x80c\x91]?\xB9\x14a\0\xB2W\x80c\xA4##\x87\x14a\0\xC5W\x80c\xA8\xC6.v\x14a\0\xD8W\x80c\xCB\x1FU2\x14a\x01\x03W[`\0\x80\xFD[a\0za\0u6`\x04a\x06\xA4V[a\x01\x16V[`@Qa\0\x89\x93\x92\x91\x90a\x07,V[`@Q\x80\x91\x03\x90\xF3[a\0\xA5a\0\xA06`\x04a\x07\xFEV[a\x04\xB4V[`@Qa\0\x89\x91\x90a\x08jV[a\0\xA5a\0\xC06`\x04a\x08}V[a\x04\xCBV[a\0\xA5a\0\xD36`\x04a\x08}V[a\x04\xDCV[`\0Ta\0\xEB\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\x89V[a\0\xA5a\x01\x116`\x04a\x08\x96V[a\x04\xE7V[`\0\x80```\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xAF\xBA\x13\xC4`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01oW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\x93\x91\x90a\x08\xC3V[`\x01`\x01`\xA0\x1B\x03\x16c\xACJ\xFA8\x88`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xC0\x91\x81R` \x01\x90V[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01\xDDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02\x05\x91\x90\x81\x01\x90a\t\xDBV[`\0\x80T`@Qc\xDC\x17\x83U`\xE0\x1B\x81R`\x04\x81\x01\x8B\x90R\x92\x93P\x90\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDC\x17\x83U\x90`$\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x02TW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x02|\x91\x90\x81\x01\x90a\n\xB6V[\x80` \x01\x90Q\x81\x01\x90a\x02\x8F\x91\x90a\x0BJV[\x90Pa\x02\xAE`@Q\x80`@\x01`@R\x80`\0\x81R` \x01`\0\x81RP\x90V[\x87\x15a\x036Wa\x02\xC0\x87\x83`\x01a\x04\xF2V[Pa\x02\xEF\x82` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x02\xDC\x91\x90a\x0B\x8CV[\x83Qa\x02\xE9\x90\x8A\x90a\x05\"V[\x90a\x05\"V[\x80\x82R`@\x84\x01Q\x80Q`\x01\x90\x81\x10a\x03\nWa\x03\na\x0B\xADV[` \x02` \x01\x01Q\x10\x15a\x031W`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xADV[a\x03B\x87\x83`\0a\x04\xF2V[Pa\x03m\x82`\0\x01Qa\x03g\x89\x85` \x01Qg\r\xE0\xB6\xB3\xA7d\0\0a\x02\xE9\x91\x90a\x0B\x8CV[\x90a\x057V[\x80\x82R`@\x84\x01Q\x80Q`\0\x90a\x03\x86Wa\x03\x86a\x0B\xADV[` \x02` \x01\x01Q\x10\x15a\x03\xADW`@QcC#\xA5U`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x88\x15a\x03\xEDWP\x80Q`@\x80Q`\0` \x82\x01R`\x01\x81\x83\x01R``\x81\x01\x8A\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90Ra\x04!V[P\x80Q`@\x80Q`\x01` \x82\x01R`\0\x81\x83\x01R``\x81\x01\x8A\x90R`\x80\x80\x82\x01\x93\x90\x93R\x81Q\x80\x82\x03\x90\x93\x01\x83R`\xA0\x01\x90R[`\0\x80T`@Qcu\xE6D\x0F`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cu\xE6D\x0F\x90a\x04X\x900\x90\x8F\x90\x8A\x90\x88\x90`\x04\x01a\x0B\xFFV[`\xE0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04uW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04\x99\x91\x90a\x0C\xF3V[PP\x96Q\x93\x9F\x93\x9EP\x94\x9CP\x91\x9APPPPPPPPPPPV[``a\x04\xC1\x84\x84\x84a\x05LV[\x90P[\x93\x92PPPV[``a\x04\xD6\x82a\x05\xDDV[\x92\x91PPV[``a\x04\xD6\x82a\x06\tV[``a\x04\xD6\x82a\x06\x1FV[`\0\x81\x15a\x05\x10W` \x83\x01Qa\x05\t\x90\x85a\x065V[\x90Pa\x04\xC4V[\x82Q` \x84\x01Qa\x05\t\x91\x86\x90a\x06FV[`\0a\x04\xC4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x06tV[`\0a\x04\xC4\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x06tV[`@\x80Q`\x02\x80\x82R``\x80\x83\x01\x84R\x92`\0\x92\x91\x90` \x83\x01\x90\x806\x837\x01\x90PP\x90P\x84\x81`\0\x81Q\x81\x10a\x05\x85Wa\x05\x85a\x0B\xADV[` \x02` \x01\x01\x81\x81RPP\x83\x81`\x01\x81Q\x81\x10a\x05\xA5Wa\x05\xA5a\x0B\xADV[` \x02` \x01\x01\x81\x81RPP\x80\x83`@Q` \x01a\x05\xC4\x92\x91\x90a\rPV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x91PP\x93\x92PPPV[```\x01\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xB1V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x91\x90PV[```\x02\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xB1V[```\x03\x82`@Q` \x01a\x05\xF3\x92\x91\x90a\r\xCCV[`\0a\x04\xC4\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x06^W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x06\x8CW`\0\x80\xFD[\x04\x92\x91PPV[\x80\x15\x15\x81\x14a\x06\xA1W`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x06\xB9W`\0\x80\xFD[\x835\x92P` \x84\x015a\x06\xCB\x81a\x06\x93V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\x06\xF7W\x81\x81\x01Q\x83\x82\x01R` \x01a\x06\xDFV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x07\x18\x81` \x86\x01` \x86\x01a\x06\xDCV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x83\x15\x15\x81R\x82` \x82\x01R```@\x82\x01R`\0a\x07M``\x83\x01\x84a\x07\0V[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8FWa\x07\x8Fa\x07VV[`@R\x90V[`@Q`\xE0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\x8FWa\x07\x8Fa\x07VV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x07\xE1Wa\x07\xE1a\x07VV[`@R\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06\xA1W`\0\x80\xFD[`\0\x80`\0\x83\x85\x03`\xA0\x81\x12\x15a\x08\x14W`\0\x80\xFD[\x845\x93P` \x85\x015\x92P```?\x19\x82\x01\x12\x15a\x081W`\0\x80\xFD[Pa\x08:a\x07lV[`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015a\x08Y\x81a\x07\xE9V[`@\x82\x01R\x92\x95\x91\x94P\x91\x92P\x90PV[` \x81R`\0a\x04\xC4` \x83\x01\x84a\x07\0V[`\0` \x82\x84\x03\x12\x15a\x08\x8FW`\0\x80\xFD[P5\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xA8W`\0\x80\xFD[\x815a\x04\xC4\x81a\x07\xE9V[\x80Qa\x08\xBE\x81a\x07\xE9V[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x08\xD5W`\0\x80\xFD[\x81Qa\x04\xC4\x81a\x07\xE9V[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x08\xFAWa\x08\xFAa\x07VV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\t\x15W`\0\x80\xFD[\x81Q` a\t*a\t%\x83a\x08\xE0V[a\x07\xB8V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\tLW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\tqW\x80Qa\td\x81a\x07\xE9V[\x83R\x91\x83\x01\x91\x83\x01a\tQV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12a\t\x8DW`\0\x80\xFD[\x81Q` a\t\x9Da\t%\x83a\x08\xE0V[\x80\x83\x82R` \x82\x01\x91P` \x84`\x05\x1B\x87\x01\x01\x93P\x86\x84\x11\x15a\t\xBFW`\0\x80\xFD[` \x86\x01[\x84\x81\x10\x15a\tqW\x80Q\x83R\x91\x83\x01\x91\x83\x01a\t\xC4V[`\0` \x82\x84\x03\x12\x15a\t\xEDW`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\x05W`\0\x80\xFD[\x90\x83\x01\x90`\xE0\x82\x86\x03\x12\x15a\n\x19W`\0\x80\xFD[a\n!a\x07\x95V[a\n*\x83a\x08\xB3V[\x81R` \x83\x01Q\x82\x81\x11\x15a\n>W`\0\x80\xFD[a\nJ\x87\x82\x86\x01a\t\x04V[` \x83\x01RP`@\x83\x01Q\x82\x81\x11\x15a\nbW`\0\x80\xFD[a\nn\x87\x82\x86\x01a\t|V[`@\x83\x01RP``\x83\x01Q``\x82\x01Ra\n\x8A`\x80\x84\x01a\x08\xB3V[`\x80\x82\x01Ra\n\x9B`\xA0\x84\x01a\x08\xB3V[`\xA0\x82\x01R`\xC0\x83\x01Q`\xC0\x82\x01R\x80\x93PPPP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xC8W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xE0W`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a\n\xF4W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x0B\x06Wa\x0B\x06a\x07VV[a\x0B\x19`\x1F\x82\x01`\x1F\x19\x16` \x01a\x07\xB8V[\x91P\x80\x82R\x85` \x82\x85\x01\x01\x11\x15a\x0B0W`\0\x80\xFD[a\x0BA\x81` \x84\x01` \x86\x01a\x06\xDCV[P\x94\x93PPPPV[`\0``\x82\x84\x03\x12\x15a\x0B\\W`\0\x80\xFD[a\x0Bda\x07lV[\x82Q\x81R` \x83\x01Q` \x82\x01R`@\x83\x01Qa\x0B\x80\x81a\x07\xE9V[`@\x82\x01R\x93\x92PPPV[\x81\x81\x03\x81\x81\x11\x15a\x04\xD6WcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P` \x84\x01`\0[\x83\x81\x10\x15a\x0B\xF4W\x81Q\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a\x0B\xD8V[P\x94\x95\x94PPPPPV[`\x01`\x01`\xA0\x1B\x03\x85\x81\x16\x82R` \x80\x83\x01\x86\x90R`\x80`@\x84\x01\x81\x90R\x85Q\x83\x16\x90\x84\x01R\x84\x81\x01Q`\xE0`\xA0\x85\x01R\x80Qa\x01`\x85\x01\x81\x90R`\0\x93\x92\x91\x82\x01\x90\x84\x90a\x01\x80\x87\x01\x90[\x80\x83\x10\x15a\x0CmW\x83Q\x86\x16\x82R\x92\x84\x01\x92`\x01\x92\x90\x92\x01\x91\x90\x84\x01\x90a\x0CKV[P`@\x89\x01Q\x87\x82\x03`\x7F\x19\x01`\xC0\x89\x01R\x94Pa\x0C\x8B\x81\x86a\x0B\xC3V[\x94PPPPP``\x85\x01Q`\xE0\x84\x01R`\x80\x85\x01Qa\x0C\xB6a\x01\0\x85\x01\x82`\x01`\x01`\xA0\x1B\x03\x16\x90RV[P`\xA0\x85\x01Q`\x01`\x01`\xA0\x1B\x03\x16a\x01 \x84\x01R`\xC0\x85\x01Qa\x01@\x84\x01R\x82\x81\x03``\x84\x01Ra\x0C\xE8\x81\x85a\x07\0V[\x97\x96PPPPPPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\r\x0EW`\0\x80\xFD[\x87Qa\r\x19\x81a\x06\x93V[` \x89\x01Q`@\x8A\x01Q``\x8B\x01Q`\x80\x8C\x01Q`\xA0\x8D\x01Q`\xC0\x90\x9D\x01Q\x94\x9E\x93\x9DP\x91\x9B\x90\x9AP\x90\x98P\x96P\x90\x94P\x92PPPV[`\x80\x81R`\0a\rc`\x80\x83\x01\x85a\x0B\xC3V[\x90P\x82Q` \x83\x01R` \x83\x01Q`@\x83\x01R`\x01\x80`\xA0\x1B\x03`@\x84\x01Q\x16``\x83\x01R\x93\x92PPPV[`\x04\x81\x10a\r\xADWcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[`@\x81\x01a\r\xBF\x82\x85a\r\x8FV[\x82` \x83\x01R\x93\x92PPPV[`@\x81\x01a\r\xDA\x82\x85a\r\x8FV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16` \x91\x90\x91\x01R\x91\x90PV\xFE\xA2dipfsX\"\x12 \xFB\x1B^8\x88\x96fL$\xFE\xBD\xA9\xF9\xEFJ1\x06n\xEA\xF6c56m:\xB3\xD3\x14;\xBEG\xB5dsolcC\0\x08\x16\x003";
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
        pub rx: ::ethers::core::types::U256,
        pub ry: ::ethers::core::types::U256,
        pub params: ConstantSumParams,
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

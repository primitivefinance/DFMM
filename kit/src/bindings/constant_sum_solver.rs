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
                    ::std::borrow::ToOwned::to_owned("simulateAllocateOrDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("simulateAllocateOrDeallocate",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("IsAllocate"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
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
    const __BYTECODE: &[u8] = b"`\x804a\0tW`\x1Fa\x0C\x058\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0yW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0tWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0tW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x0Bu\x90\x81a\0\x90\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x92\x83c9(\xFF\x97\x14a\0oWPPP\x80c\x89\xEA\x85Y\x14a\0jW\x80c\x8A\x1A \xDE\x14a\0eW\x80c\xA4##\x87\x14a\0`Wc\xA8\xC6.v\x14a\0[W`\0\x80\xFD[a\x08\xA8V[a\x08aV[a\x05\x9FV[a\x04\xD0V[4a\x03\xBEW``6`\x03\x19\x01\x12a\x03\xBEW\x825`$5a\0\x8E\x81a\x03\xC1V[`D5\x91a\0\x9Aa\x08\xD1V[\x95a\0\xA3a\x08\xD1V[\x85T\x90\x94\x90a\0\xC8\x90a\0\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x87Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x90\x94` \x92\x83\x83\x86\x81\x8AZ\xFA\x92\x83\x15a\x02\xA7W\x89\x93a\x03\x8FW[P``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x01\x12\x8B\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02\xA7W\x8B\x93\x8A\x90\x8B\x90\x8C\x94a\x03YW[P\x85\x8D\x01\x93\x84R\x94\x86\x01\x94\x85R\x8CR\x8AQc\xDC\x17\x83U`\xE0\x1B\x81R\x86\x81\x01\x88\x81R\x8B\x90\x82\x90\x81\x90` \x01\x03\x81\x8CZ\xFA\x80\x15a\x02\xA7Wa\x01\x82\x91\x8C\x91a\x037W[P\x86\x80\x82Q\x83\x01\x01\x91\x01a\t\x95V[\x92\x15a\x02\xBCW\x90a\x01\xC3\x83\x92a\x01\xBC\x87a\x01\xCB\x96\x01Qa\x01\xB6a\x01\xB0a\x01\xA8\x83\x86a\n\xD0V[\x97Q\x85a\n\xFCV[\x91a\t\xE5V[\x90a\n\xFCV[\x9DQa\n\x08V[\x89RQa\n\x08V[\x88\x87\x01R\x88\x81Q\x10a\x02\xACW\x92a\x02 \x95\x92a\x02.a\x02J\x96\x93a\x01\xF2\x8C`\xC0\x98Qa\t\xFBV[\x81\x87\x01R[\x8AQ\x98\x89\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x8CV[\x88Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x95\x86\x94\x85\x93\x84\x930\x90\x85\x01a\nLV[\x03\x91Z\xFA\x92\x83\x15a\x02\xA7W\x92a\x02oW[Pa\x02k\x91\x92Q\x93\x84\x93\x84a\x04\x18V[\x03\x90\xF3[a\x02k\x92Pa\x02\x95\x90`\xC0=`\xC0\x11a\x02\xA0W[a\x02\x8D\x81\x83a\x04\x8CV[\x81\x01\x90a\n\x15V[PPPPP\x91a\x02[V[P=a\x02\x83V[a\t\x05V[\x87QcC#\xA5U`\xE0\x1B\x81R\x83\x90\xFD[\x90a\x02\xFF\x83\x92a\x01\xBC\x87\x96\x9E\x96a\x03\t\x96\x01Q\x94a\x02\xF7\x83a\x02\xF2a\x02\xECa\x02\xE4\x8A\x84a\n\xD0V[\x85Q\x90a\n\xA0V[\x98a\t\xE5V[a\n\xFCV[\x90Q\x90a\x0B\x1DV[\x85\x8A\x01RQa\n\x08V[\x88\x87\x01R\x88\x81Q\x10a\x02\xACW\x92a\x02 \x95\x92a\x02.a\x02J\x96\x93a\x030\x8C`\xC0\x98Qa\t\xFBV[\x86Ra\x01\xF7V[a\x03S\x91P=\x80\x8E\x83>a\x03K\x81\x83a\x04\x8CV[\x81\x01\x90a\t,V[\x8Ea\x01sV[\x91PPa\x03\x7F\x91\x92P``=``\x11a\x03\x88W[a\x03w\x81\x83a\x04\x8CV[\x81\x01\x90a\t\x11V[\x92\x91\x90\x8Ea\x013V[P=a\x03mV[a\x03\xB0\x91\x93P\x84=\x86\x11a\x03\xB7W[a\x03\xA8\x81\x83a\x04\x8CV[\x81\x01\x90a\x08\xF0V[\x91\x8Ba\0\xEEV[P=a\x03\x9EV[\x80\xFD[\x80\x15\x15\x03a\x03\xCBWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x03\xE3WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xD3V[\x90` \x91a\x04\x0C\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xD0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x046\x93\x92``\x92\x15\x15\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x03\xF3V[\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[a\x049V[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xCBWV[\x90` a\x046\x92\x81\x81R\x01\x90a\x03\xF3V[4a\x03\xCBW`\xA06`\x03\x19\x01\x12a\x03\xCBW`$5`\x045``6`C\x19\x01\x12a\x03\xCBW`@Qa\x04\xFF\x81a\x04OV[`D5\x90\x81\x81R` \x81\x01\x90`d5\x82Ra\x05-`\x845\x93a\x05 \x85a\x04\xAEV[`@\x83\x01\x94\x85R\x86a\n\xA0V[\x84\x01\x80\x85\x11a\x05\x81Wa\x02k\x95`@Q\x95` \x87\x01R`@\x86\x01R``\x85\x01RQ`\x80\x84\x01RQ`\xA0\x83\x01R`\x01\x80`\xA0\x1B\x03\x90Q\x16`\xC0\x82\x01R`\xC0\x81Ra\x05u\x81a\x04pV[`@Q\x91\x82\x91\x82a\x04\xBFV[a\t\xCFV[`@\x90a\x046\x93\x92\x15\x15\x81R\x81` \x82\x01R\x01\x90a\x03\xF3V[4a\x03\xCBW`\x806`\x03\x19\x01\x12a\x03\xCBW`\x04`$5\x815a\x05\xC0\x82a\x03\xC1V[`D5\x90`d5a\x05\xCFa\x08\xD1V[\x91a\x05\xD8a\x08\xD1V[\x92`\0\x95a\x05\xF2a\0\xBCa\0\xBC\x89T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x93`@\x96\x87Q\x90c+\xEE\x84\xF1`\xE2\x1B\x82R` \x94\x85\x83\x8D\x81\x8BZ\xFA\x80\x15a\x02\xA7W\x87``\x91\x8Ea\x06C\x96\x8F\x92a\x08BW[P\x8DQc3\x85N\xFD`\xE2\x1B\x81R\x90\x81\x01\x92\x83R\x95\x86\x92\x83\x91\x82\x91` \x01\x90V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xA7W\x8B\x88\x8Ea\x06\x93\x93\x8E\x97\x84\x91\x85\x91\x86\x91a\x08 W[P\x8C\x88\x01\x99\x88\x01R\x88R\x85R\x8DQ\x93\x84\x92\x83\x92c\xDC\x17\x83U`\xE0\x1B\x84R\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8CZ\xFA\x80\x15a\x02\xA7Wa\x06\xB8\x91\x8D\x91a\x08\x06W[P\x87\x80\x82Q\x83\x01\x01\x91\x01a\t\x95V[\x94\x15a\x07\xA5Wa\x07L\x98\x96\x94a\x06\xFCa\x06\xEDa\x02 \x9E\x96a\x070\x96a\x06\xE5`\xA0\x9D\x9B\x97a\x07\x02\x97Qa\n\x08V[\x8CRQa\n\x08V[\x80\x85\x8B\x01R\x89Q\x92Q\x90a\n\xA0V[\x90a\n\x08V[\x89\x87\x01R[\x88Q\x9A\x8B\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x86Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x95\x86\x94\x85\x93\x84\x930\x90\x85\x01a\nLV[\x03\x91Z\xFA\x92\x83\x15a\x02\xA7W\x92a\x07lW[Pa\x02k\x90Q\x92\x83\x92\x83a\x05\x86V[a\x02k\x91\x92Pa\x07\x93\x90`\xA0=`\xA0\x11a\x07\x9EW[a\x07\x8B\x81\x83a\x04\x8CV[\x81\x01\x90a\npV[PPPP\x91\x90a\x07]V[P=a\x07\x81V[\x81\x81Q\x10\x80\x15a\x07\xFCW[a\x07\xECWa\x07L\x98\x96\x94a\x06\xFCa\x06\xEDa\x02 \x9E\x96a\x070\x96a\x07\xDB`\xA0\x9D\x9B\x97a\x07\xE3\x97Qa\t\xFBV[\x8CRQa\t\xFBV[\x89\x87\x01Ra\x07\x07V[\x89QcC#\xA5U`\xE0\x1B\x81R\x8C\x90\xFD[P\x83\x83Q\x10a\x07\xB0V[a\x08\x1A\x91P=\x80\x8F\x83>a\x03K\x81\x83a\x04\x8CV[8a\x06\xA9V[\x91PPa\x08<\x91P``=``\x11a\x03\x88Wa\x03w\x81\x83a\x04\x8CV[8a\x06jV[a\x08Z\x91\x92P\x8A=\x8C\x11a\x03\xB7Wa\x03\xA8\x81\x83a\x04\x8CV[\x908a\x06#V[4a\x03\xCBW` 6`\x03\x19\x01\x12a\x03\xCBWa\x02k`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x08\x94\x81a\x04OV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x03\xF3V[4a\x03\xCBW`\x006`\x03\x19\x01\x12a\x03\xCBW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@Q\x90a\x08\xDE\x82a\x04OV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90\x81` \x91\x03\x12a\x03\xCBWQa\x046\x81a\x04\xAEV[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xCBW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[` \x81\x83\x03\x12a\x03\xCBW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x03\xCBW\x01\x90\x82`\x1F\x83\x01\x12\x15a\x03\xCBW\x81Q\x90\x81\x11a\x04kW`@Q\x92a\tw`\x1F\x83\x01`\x1F\x19\x16` \x01\x85a\x04\x8CV[\x81\x84R` \x82\x84\x01\x01\x11a\x03\xCBWa\x046\x91` \x80\x85\x01\x91\x01a\x03\xD0V[\x90\x81``\x91\x03\x12a\x03\xCBW`@\x80Q\x91a\t\xAE\x83a\x04OV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\t\xC7\x81a\x04\xAEV[`@\x82\x01R\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x05\x81WV[\x91\x90\x82\x03\x91\x82\x11a\x05\x81WV[\x91\x90\x82\x01\x80\x92\x11a\x05\x81WV[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xCBW\x81Qa\n,\x81a\x03\xC1V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x046\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x03\xF3V[\x90\x81`\xA0\x91\x03\x12a\x03\xCBW\x80Qa\n\x86\x81a\x03\xC1V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xCBW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xCBW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xCBWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xCBW\x04\x90V\xFE\xA2dipfsX\"\x12 \xEC\xB3\x93\x18,a\x81\x1A\x0B\\M?\xF5\xE8k\xE6u\xAB\xC7s\xCB\x15\x890\xF7Y/nKX\xB6\xCEdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUMSOLVER_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`@`\x80\x81R`\x04\x806\x10\x15a\0\x14W`\0\x80\xFD[`\0\x91\x825`\xE0\x1C\x92\x83c9(\xFF\x97\x14a\0oWPPP\x80c\x89\xEA\x85Y\x14a\0jW\x80c\x8A\x1A \xDE\x14a\0eW\x80c\xA4##\x87\x14a\0`Wc\xA8\xC6.v\x14a\0[W`\0\x80\xFD[a\x08\xA8V[a\x08aV[a\x05\x9FV[a\x04\xD0V[4a\x03\xBEW``6`\x03\x19\x01\x12a\x03\xBEW\x825`$5a\0\x8E\x81a\x03\xC1V[`D5\x91a\0\x9Aa\x08\xD1V[\x95a\0\xA3a\x08\xD1V[\x85T\x90\x94\x90a\0\xC8\x90a\0\xBC\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x87Qc+\xEE\x84\xF1`\xE2\x1B\x81R\x90\x94` \x92\x83\x83\x86\x81\x8AZ\xFA\x92\x83\x15a\x02\xA7W\x89\x93a\x03\x8FW[P``\x8AQ\x80\x94c3\x85N\xFD`\xE2\x1B\x82R\x81\x80a\x01\x12\x8B\x8B\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x02\xA7W\x8B\x93\x8A\x90\x8B\x90\x8C\x94a\x03YW[P\x85\x8D\x01\x93\x84R\x94\x86\x01\x94\x85R\x8CR\x8AQc\xDC\x17\x83U`\xE0\x1B\x81R\x86\x81\x01\x88\x81R\x8B\x90\x82\x90\x81\x90` \x01\x03\x81\x8CZ\xFA\x80\x15a\x02\xA7Wa\x01\x82\x91\x8C\x91a\x037W[P\x86\x80\x82Q\x83\x01\x01\x91\x01a\t\x95V[\x92\x15a\x02\xBCW\x90a\x01\xC3\x83\x92a\x01\xBC\x87a\x01\xCB\x96\x01Qa\x01\xB6a\x01\xB0a\x01\xA8\x83\x86a\n\xD0V[\x97Q\x85a\n\xFCV[\x91a\t\xE5V[\x90a\n\xFCV[\x9DQa\n\x08V[\x89RQa\n\x08V[\x88\x87\x01R\x88\x81Q\x10a\x02\xACW\x92a\x02 \x95\x92a\x02.a\x02J\x96\x93a\x01\xF2\x8C`\xC0\x98Qa\t\xFBV[\x81\x87\x01R[\x8AQ\x98\x89\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x03`\x1F\x19\x81\x01\x86R\x85a\x04\x8CV[\x88Qc\r\x17\xA7\xC7`\xE3\x1B\x81R\x95\x86\x94\x85\x93\x84\x930\x90\x85\x01a\nLV[\x03\x91Z\xFA\x92\x83\x15a\x02\xA7W\x92a\x02oW[Pa\x02k\x91\x92Q\x93\x84\x93\x84a\x04\x18V[\x03\x90\xF3[a\x02k\x92Pa\x02\x95\x90`\xC0=`\xC0\x11a\x02\xA0W[a\x02\x8D\x81\x83a\x04\x8CV[\x81\x01\x90a\n\x15V[PPPPP\x91a\x02[V[P=a\x02\x83V[a\t\x05V[\x87QcC#\xA5U`\xE0\x1B\x81R\x83\x90\xFD[\x90a\x02\xFF\x83\x92a\x01\xBC\x87\x96\x9E\x96a\x03\t\x96\x01Q\x94a\x02\xF7\x83a\x02\xF2a\x02\xECa\x02\xE4\x8A\x84a\n\xD0V[\x85Q\x90a\n\xA0V[\x98a\t\xE5V[a\n\xFCV[\x90Q\x90a\x0B\x1DV[\x85\x8A\x01RQa\n\x08V[\x88\x87\x01R\x88\x81Q\x10a\x02\xACW\x92a\x02 \x95\x92a\x02.a\x02J\x96\x93a\x030\x8C`\xC0\x98Qa\t\xFBV[\x86Ra\x01\xF7V[a\x03S\x91P=\x80\x8E\x83>a\x03K\x81\x83a\x04\x8CV[\x81\x01\x90a\t,V[\x8Ea\x01sV[\x91PPa\x03\x7F\x91\x92P``=``\x11a\x03\x88W[a\x03w\x81\x83a\x04\x8CV[\x81\x01\x90a\t\x11V[\x92\x91\x90\x8Ea\x013V[P=a\x03mV[a\x03\xB0\x91\x93P\x84=\x86\x11a\x03\xB7W[a\x03\xA8\x81\x83a\x04\x8CV[\x81\x01\x90a\x08\xF0V[\x91\x8Ba\0\xEEV[P=a\x03\x9EV[\x80\xFD[\x80\x15\x15\x03a\x03\xCBWV[`\0\x80\xFD[`\0[\x83\x81\x10a\x03\xE3WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x03\xD3V[\x90` \x91a\x04\x0C\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x03\xD0V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[a\x046\x93\x92``\x92\x15\x15\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x03\xF3V[\x90V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[a\x049V[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x04kW`@RV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x03\xCBWV[\x90` a\x046\x92\x81\x81R\x01\x90a\x03\xF3V[4a\x03\xCBW`\xA06`\x03\x19\x01\x12a\x03\xCBW`$5`\x045``6`C\x19\x01\x12a\x03\xCBW`@Qa\x04\xFF\x81a\x04OV[`D5\x90\x81\x81R` \x81\x01\x90`d5\x82Ra\x05-`\x845\x93a\x05 \x85a\x04\xAEV[`@\x83\x01\x94\x85R\x86a\n\xA0V[\x84\x01\x80\x85\x11a\x05\x81Wa\x02k\x95`@Q\x95` \x87\x01R`@\x86\x01R``\x85\x01RQ`\x80\x84\x01RQ`\xA0\x83\x01R`\x01\x80`\xA0\x1B\x03\x90Q\x16`\xC0\x82\x01R`\xC0\x81Ra\x05u\x81a\x04pV[`@Q\x91\x82\x91\x82a\x04\xBFV[a\t\xCFV[`@\x90a\x046\x93\x92\x15\x15\x81R\x81` \x82\x01R\x01\x90a\x03\xF3V[4a\x03\xCBW`\x806`\x03\x19\x01\x12a\x03\xCBW`\x04`$5\x815a\x05\xC0\x82a\x03\xC1V[`D5\x90`d5a\x05\xCFa\x08\xD1V[\x91a\x05\xD8a\x08\xD1V[\x92`\0\x95a\x05\xF2a\0\xBCa\0\xBC\x89T`\x01\x80`\xA0\x1B\x03\x16\x90V[\x93`@\x96\x87Q\x90c+\xEE\x84\xF1`\xE2\x1B\x82R` \x94\x85\x83\x8D\x81\x8BZ\xFA\x80\x15a\x02\xA7W\x87``\x91\x8Ea\x06C\x96\x8F\x92a\x08BW[P\x8DQc3\x85N\xFD`\xE2\x1B\x81R\x90\x81\x01\x92\x83R\x95\x86\x92\x83\x91\x82\x91` \x01\x90V[\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a\x02\xA7W\x8B\x88\x8Ea\x06\x93\x93\x8E\x97\x84\x91\x85\x91\x86\x91a\x08 W[P\x8C\x88\x01\x99\x88\x01R\x88R\x85R\x8DQ\x93\x84\x92\x83\x92c\xDC\x17\x83U`\xE0\x1B\x84R\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x81\x8CZ\xFA\x80\x15a\x02\xA7Wa\x06\xB8\x91\x8D\x91a\x08\x06W[P\x87\x80\x82Q\x83\x01\x01\x91\x01a\t\x95V[\x94\x15a\x07\xA5Wa\x07L\x98\x96\x94a\x06\xFCa\x06\xEDa\x02 \x9E\x96a\x070\x96a\x06\xE5`\xA0\x9D\x9B\x97a\x07\x02\x97Qa\n\x08V[\x8CRQa\n\x08V[\x80\x85\x8B\x01R\x89Q\x92Q\x90a\n\xA0V[\x90a\n\x08V[\x89\x87\x01R[\x88Q\x9A\x8B\x96\x87\x92\x83\x01\x91\x90\x91`@\x80``\x83\x01\x94\x80Q\x84R` \x81\x01Q` \x85\x01R\x01Q\x91\x01RV[\x86Qc\x8A\x04\xBD\xD5`\xE0\x1B\x81R\x95\x86\x94\x85\x93\x84\x930\x90\x85\x01a\nLV[\x03\x91Z\xFA\x92\x83\x15a\x02\xA7W\x92a\x07lW[Pa\x02k\x90Q\x92\x83\x92\x83a\x05\x86V[a\x02k\x91\x92Pa\x07\x93\x90`\xA0=`\xA0\x11a\x07\x9EW[a\x07\x8B\x81\x83a\x04\x8CV[\x81\x01\x90a\npV[PPPP\x91\x90a\x07]V[P=a\x07\x81V[\x81\x81Q\x10\x80\x15a\x07\xFCW[a\x07\xECWa\x07L\x98\x96\x94a\x06\xFCa\x06\xEDa\x02 \x9E\x96a\x070\x96a\x07\xDB`\xA0\x9D\x9B\x97a\x07\xE3\x97Qa\t\xFBV[\x8CRQa\t\xFBV[\x89\x87\x01Ra\x07\x07V[\x89QcC#\xA5U`\xE0\x1B\x81R\x8C\x90\xFD[P\x83\x83Q\x10a\x07\xB0V[a\x08\x1A\x91P=\x80\x8F\x83>a\x03K\x81\x83a\x04\x8CV[8a\x06\xA9V[\x91PPa\x08<\x91P``=``\x11a\x03\x88Wa\x03w\x81\x83a\x04\x8CV[8a\x06jV[a\x08Z\x91\x92P\x8A=\x8C\x11a\x03\xB7Wa\x03\xA8\x81\x83a\x04\x8CV[\x908a\x06#V[4a\x03\xCBW` 6`\x03\x19\x01\x12a\x03\xCBWa\x02k`@Q`\x02` \x82\x01R`\x045`@\x82\x01R`@\x81Ra\x08\x94\x81a\x04OV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x03\xF3V[4a\x03\xCBW`\x006`\x03\x19\x01\x12a\x03\xCBW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[`@Q\x90a\x08\xDE\x82a\x04OV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[\x90\x81` \x91\x03\x12a\x03\xCBWQa\x046\x81a\x04\xAEV[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x03\xCBW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[` \x81\x83\x03\x12a\x03\xCBW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x03\xCBW\x01\x90\x82`\x1F\x83\x01\x12\x15a\x03\xCBW\x81Q\x90\x81\x11a\x04kW`@Q\x92a\tw`\x1F\x83\x01`\x1F\x19\x16` \x01\x85a\x04\x8CV[\x81\x84R` \x82\x84\x01\x01\x11a\x03\xCBWa\x046\x91` \x80\x85\x01\x91\x01a\x03\xD0V[\x90\x81``\x91\x03\x12a\x03\xCBW`@\x80Q\x91a\t\xAE\x83a\x04OV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\t\xC7\x81a\x04\xAEV[`@\x82\x01R\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x03\x91\x82\x11a\x05\x81WV[\x91\x90\x82\x03\x91\x82\x11a\x05\x81WV[\x91\x90\x82\x01\x80\x92\x11a\x05\x81WV[\x91\x90\x82`\xC0\x91\x03\x12a\x03\xCBW\x81Qa\n,\x81a\x03\xC1V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[a\x046\x93\x92``\x92`\x01\x80`\xA0\x1B\x03\x16\x82R` \x82\x01R\x81`@\x82\x01R\x01\x90a\x03\xF3V[\x90\x81`\xA0\x91\x03\x12a\x03\xCBW\x80Qa\n\x86\x81a\x03\xC1V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x03\xCBW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xCBW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x03\xCBWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x03\xCBW\x04\x90V\xFE\xA2dipfsX\"\x12 \xEC\xB3\x93\x18,a\x81\x1A\x0B\\M?\xF5\xE8k\xE6u\xAB\xC7s\xCB\x15\x890\xF7Y/nKX\xB6\xCEdsolcC\0\x08\x16\x003";
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
        /// Calls the contract's `simulateAllocateOrDeallocate` (0x8a1a20de)
        /// function
        pub fn simulate_allocate_or_deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            is_allocate: bool,
            amount_x: ::ethers::core::types::U256,
            amount_y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, ::ethers::core::types::Bytes)>
        {
            self.0
                .method_hash(
                    [138, 26, 32, 222],
                    (pool_id, is_allocate, amount_x, amount_y),
                )
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
    /// Container type for all input parameters for the
    /// `simulateAllocateOrDeallocate` function with signature
    /// `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and
    /// selector `0x8a1a20de`
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
        name = "simulateAllocateOrDeallocate",
        abi = "simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)"
    )]
    pub struct SimulateAllocateOrDeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub is_allocate: bool,
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
        SimulateAllocateOrDeallocate(SimulateAllocateOrDeallocateCall),
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
                <SimulateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateAllocateOrDeallocate(decoded));
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
                Self::SimulateAllocateOrDeallocate(element) => {
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
                Self::SimulateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
    impl ::core::convert::From<SimulateAllocateOrDeallocateCall> for ConstantSumSolverCalls {
        fn from(value: SimulateAllocateOrDeallocateCall) -> Self {
            Self::SimulateAllocateOrDeallocate(value)
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
    /// Container type for all return fields from the
    /// `simulateAllocateOrDeallocate` function with signature
    /// `simulateAllocateOrDeallocate(uint256,bool,uint256,uint256)` and
    /// selector `0x8a1a20de`
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
    pub struct SimulateAllocateOrDeallocateReturn(pub bool, pub ::ethers::core::types::Bytes);
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

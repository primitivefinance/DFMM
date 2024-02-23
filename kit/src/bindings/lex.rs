pub use lex::*;
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
pub mod lex {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("arbiterTokenX_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("arbiterTokenY_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("price_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("admin"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admin"),
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
                (
                    ::std::borrow::ToOwned::to_owned("arbiterTokenX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("arbiterTokenX"),
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
                (
                    ::std::borrow::ToOwned::to_owned("arbiterTokenY"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("arbiterTokenY"),
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
                (
                    ::std::borrow::ToOwned::to_owned("price"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("price"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("setPrice"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPrice"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("PriceChange"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PriceChange"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Swap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountIn"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amountOut"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("to"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static LEX_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0\x9CW`\x1Fa\x05\x978\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xA1W\x80\x84\x92``\x94`@R\x839\x81\x01\x03\x12a\0\x9CWa\0G\x81a\0\xB7V[\x90`@a\0V` \x83\x01a\0\xB7V[\x91\x01Q\x91`\x01\x80`\xA0\x1B\x03\x19\x913\x83`\0T\x16\x17`\0U`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x83`\x01T\x16\x17`\x01U\x16\x90`\x02T\x16\x17`\x02U`\x03U`@Qa\x04\xCB\x90\x81a\0\xCC\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[Q\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\0\x9CWV\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c; IH\x14a\0zWP\x80c\x91\xB7\xF5\xED\x14a\0uW\x80c\xA05\xB1\xFE\x14a\0pW\x80c\xD0\x04\xF0\xF7\x14a\0kW\x80c\xD0\xC4r\xEC\x14a\0fWc\xF8Q\xA4@\x14a\0aW`\0\x80\xFD[a\x03\x8FV[a\x03fV[a\x01rV[a\x01TV[a\0\xA3V[4a\0\xA0W\x80`\x03\x196\x01\x12a\0\xA0W`\x01T`\x01`\x01`\xA0\x1B\x03\x16`\x80\x90\x81R` \x90\xF3[\x80\xFD[4a\x01OW` 6`\x03\x19\x01\x12a\x01OW`\0T`\x045\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\0W`@\x81\x7F\xFEk`l\xA0Gu\x92\xB5t\n\x0E\xB0\x0C\x8E\x91W\n]\x0E\xB76\xAB\xFA\x1Ac\t\xBD\x08\x1BJM\x92`\x03U\x81Q\x90\x81RB` \x82\x01R\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FOnly admin can call this functio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x80\xFD[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW` `\x03T`@Q\x90\x81R\xF3[4a\x01OW`@6`\x03\x19\x01\x12a\x01OW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x91\x82\x81\x03a\x01OW`\x01T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x82\x16\x85\x03a\x03\rWP`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x91a\x01\xCD`\x03T\x85a\x04tV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R` \x96\x87\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x02\xEBWa\x02\x16\x91`\0\x91a\x02\xF0W[Pa\x04\x14V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x86\x90\x84\x90`D\x90\x82\x90`\0\x90\x89\x16Z\xF1\x95\x86\x15a\x02\xEBW\x7F\xB3\x9C\x9B\xC4?\x81\x1E\x1A|\xE1Y\xC5\xF1GE\x8F\xDB\x80&k\xF2<\x172 \x131n'\xE0\x86\xD0\x96a\x02\xB9\x94a\x02\x81\x92`\0\x92a\x02\xBEW[PPa\x04\x14V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x94\x16` \x83\x01R\x92\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R3`\x80\x83\x01R\x81\x90`\xA0\x82\x01\x90V[\x03\x90\xA1\0[a\x02\xDD\x92P\x80=\x10a\x02\xE4W[a\x02\xD5\x81\x83a\x03\xB8V[\x81\x01\x90a\x03\xF0V[8\x80a\x02zV[P=a\x02\xCBV[a\x04\x08V[a\x03\x07\x91P\x88=\x8A\x11a\x02\xE4Wa\x02\xD5\x81\x83a\x03\xB8V[8a\x02\x10V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x85\x03a\x031W\x91a\x03,`\x03T\x85a\x04RV[a\x01\xCDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl$\xB7;0\xB64\xB2\x10:7\xB5\xB2\xB7`\x99\x1B`D\x82\x01R`d\x90\xFD[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xDAW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x01OWQ\x80\x15\x15\x81\x03a\x01OW\x90V[`@Q=`\0\x82>=\x90\xFD[\x15a\x04\x1BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01OW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01OWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V\xFE\xA2dipfsX\"\x12 \r+\x1D\xC6:\x96\x05\xCE\x9CnW\xEB\x92\x01\xD1\xAD\xD3\xA8\xC2fB\xE5\xAD>W \x12\xF6\xC2L\xBA\x08dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LEX_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81c; IH\x14a\0zWP\x80c\x91\xB7\xF5\xED\x14a\0uW\x80c\xA05\xB1\xFE\x14a\0pW\x80c\xD0\x04\xF0\xF7\x14a\0kW\x80c\xD0\xC4r\xEC\x14a\0fWc\xF8Q\xA4@\x14a\0aW`\0\x80\xFD[a\x03\x8FV[a\x03fV[a\x01rV[a\x01TV[a\0\xA3V[4a\0\xA0W\x80`\x03\x196\x01\x12a\0\xA0W`\x01T`\x01`\x01`\xA0\x1B\x03\x16`\x80\x90\x81R` \x90\xF3[\x80\xFD[4a\x01OW` 6`\x03\x19\x01\x12a\x01OW`\0T`\x045\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x01\0W`@\x81\x7F\xFEk`l\xA0Gu\x92\xB5t\n\x0E\xB0\x0C\x8E\x91W\n]\x0E\xB76\xAB\xFA\x1Ac\t\xBD\x08\x1BJM\x92`\x03U\x81Q\x90\x81RB` \x82\x01R\xA1\0[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`!`$\x82\x01R\x7FOnly admin can call this functio`D\x82\x01R`7`\xF9\x1B`d\x82\x01R`\x84\x90\xFD[`\0\x80\xFD[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW` `\x03T`@Q\x90\x81R\xF3[4a\x01OW`@6`\x03\x19\x01\x12a\x01OW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x81\x16\x91\x82\x81\x03a\x01OW`\x01T`$5\x92\x90`\x01`\x01`\xA0\x1B\x03\x16\x80\x82\x16\x85\x03a\x03\rWP`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x91a\x01\xCD`\x03T\x85a\x04tV[`@Qc#\xB8r\xDD`\xE0\x1B\x81R3`\x04\x82\x01R0`$\x82\x01R`D\x81\x01\x86\x90R` \x96\x87\x90\x82\x90`d\x90\x82\x90`\0\x90Z\xF1\x80\x15a\x02\xEBWa\x02\x16\x91`\0\x91a\x02\xF0W[Pa\x04\x14V[`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x92\x86\x90\x84\x90`D\x90\x82\x90`\0\x90\x89\x16Z\xF1\x95\x86\x15a\x02\xEBW\x7F\xB3\x9C\x9B\xC4?\x81\x1E\x1A|\xE1Y\xC5\xF1GE\x8F\xDB\x80&k\xF2<\x172 \x131n'\xE0\x86\xD0\x96a\x02\xB9\x94a\x02\x81\x92`\0\x92a\x02\xBEW[PPa\x04\x14V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x84\x16\x81R\x92\x90\x94\x16` \x83\x01R\x92\x81\x01\x93\x90\x93R``\x83\x01\x91\x90\x91R3`\x80\x83\x01R\x81\x90`\xA0\x82\x01\x90V[\x03\x90\xA1\0[a\x02\xDD\x92P\x80=\x10a\x02\xE4W[a\x02\xD5\x81\x83a\x03\xB8V[\x81\x01\x90a\x03\xF0V[8\x80a\x02zV[P=a\x02\xCBV[a\x04\x08V[a\x03\x07\x91P\x88=\x8A\x11a\x02\xE4Wa\x02\xD5\x81\x83a\x03\xB8V[8a\x02\x10V[`\x02T`\x01`\x01`\xA0\x1B\x03\x16\x85\x03a\x031W\x91a\x03,`\x03T\x85a\x04RV[a\x01\xCDV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl$\xB7;0\xB64\xB2\x10:7\xB5\xB2\xB7`\x99\x1B`D\x82\x01R`d\x90\xFD[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW`\x02T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01OW`\x006`\x03\x19\x01\x12a\x01OW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x03\xDAW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x01OWQ\x80\x15\x15\x81\x03a\x01OW\x90V[`@Q=`\0\x82>=\x90\xFD[\x15a\x04\x1BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x1C\x98[\x9C\xD9\x99\\\x88\x19\x98Z[\x19Y`\x8A\x1B`D\x82\x01R`d\x90\xFD[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01OW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01OWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V\xFE\xA2dipfsX\"\x12 \r+\x1D\xC6:\x96\x05\xCE\x9CnW\xEB\x92\x01\xD1\xAD\xD3\xA8\xC2fB\xE5\xAD>W \x12\xF6\xC2L\xBA\x08dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LEX_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Lex<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Lex<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Lex<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Lex<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Lex<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(Lex)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Lex<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LEX_ABI.clone(),
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
                LEX_ABI.clone(),
                LEX_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `admin` (0xf851a440) function
        pub fn admin(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbiterTokenX` (0x3b204948) function
        pub fn arbiter_token_x(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([59, 32, 73, 72], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `arbiterTokenY` (0xd0c472ec) function
        pub fn arbiter_token_y(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([208, 196, 114, 236], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `price` (0xa035b1fe) function
        pub fn price(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([160, 53, 177, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPrice` (0x91b7f5ed) function
        pub fn set_price(
            &self,
            price: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 183, 245, 237], price)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xd004f0f7) function
        pub fn swap(
            &self,
            token_in: ::ethers::core::types::Address,
            amount_in: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 4, 240, 247], (token_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `PriceChange` event
        pub fn price_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PriceChangeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LexEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Lex<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PriceChange", abi = "PriceChange(uint256,uint256)")]
    pub struct PriceChangeFilter {
        pub price: ::ethers::core::types::U256,
        pub timestamp: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Swap", abi = "Swap(address,address,uint256,uint256,address)")]
    pub struct SwapFilter {
        pub token_in: ::ethers::core::types::Address,
        pub token_out: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
        pub amount_out: ::ethers::core::types::U256,
        pub to: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
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
    pub enum LexEvents {
        PriceChangeFilter(PriceChangeFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for LexEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = PriceChangeFilter::decode_log(log) {
                return Ok(LexEvents::PriceChangeFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(LexEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LexEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::PriceChangeFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<PriceChangeFilter> for LexEvents {
        fn from(value: PriceChangeFilter) -> Self {
            Self::PriceChangeFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for LexEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `admin` function with signature `admin()` and selector `0xf851a440`
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
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    ///Container type for all input parameters for the `arbiterTokenX` function with signature `arbiterTokenX()` and selector `0x3b204948`
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
    #[ethcall(name = "arbiterTokenX", abi = "arbiterTokenX()")]
    pub struct ArbiterTokenXCall;
    ///Container type for all input parameters for the `arbiterTokenY` function with signature `arbiterTokenY()` and selector `0xd0c472ec`
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
    #[ethcall(name = "arbiterTokenY", abi = "arbiterTokenY()")]
    pub struct ArbiterTokenYCall;
    ///Container type for all input parameters for the `price` function with signature `price()` and selector `0xa035b1fe`
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
    #[ethcall(name = "price", abi = "price()")]
    pub struct PriceCall;
    ///Container type for all input parameters for the `setPrice` function with signature `setPrice(uint256)` and selector `0x91b7f5ed`
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
    #[ethcall(name = "setPrice", abi = "setPrice(uint256)")]
    pub struct SetPriceCall {
        pub price: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `swap` function with signature `swap(address,uint256)` and selector `0xd004f0f7`
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
    #[ethcall(name = "swap", abi = "swap(address,uint256)")]
    pub struct SwapCall {
        pub token_in: ::ethers::core::types::Address,
        pub amount_in: ::ethers::core::types::U256,
    }
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
    pub enum LexCalls {
        Admin(AdminCall),
        ArbiterTokenX(ArbiterTokenXCall),
        ArbiterTokenY(ArbiterTokenYCall),
        Price(PriceCall),
        SetPrice(SetPriceCall),
        Swap(SwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for LexCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admin(decoded));
            }
            if let Ok(decoded) = <ArbiterTokenXCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArbiterTokenX(decoded));
            }
            if let Ok(decoded) = <ArbiterTokenYCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ArbiterTokenY(decoded));
            }
            if let Ok(decoded) = <PriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Price(decoded));
            }
            if let Ok(decoded) = <SetPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPrice(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LexCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Admin(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ArbiterTokenX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ArbiterTokenY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Price(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LexCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Admin(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbiterTokenX(element) => ::core::fmt::Display::fmt(element, f),
                Self::ArbiterTokenY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Price(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AdminCall> for LexCalls {
        fn from(value: AdminCall) -> Self {
            Self::Admin(value)
        }
    }
    impl ::core::convert::From<ArbiterTokenXCall> for LexCalls {
        fn from(value: ArbiterTokenXCall) -> Self {
            Self::ArbiterTokenX(value)
        }
    }
    impl ::core::convert::From<ArbiterTokenYCall> for LexCalls {
        fn from(value: ArbiterTokenYCall) -> Self {
            Self::ArbiterTokenY(value)
        }
    }
    impl ::core::convert::From<PriceCall> for LexCalls {
        fn from(value: PriceCall) -> Self {
            Self::Price(value)
        }
    }
    impl ::core::convert::From<SetPriceCall> for LexCalls {
        fn from(value: SetPriceCall) -> Self {
            Self::SetPrice(value)
        }
    }
    impl ::core::convert::From<SwapCall> for LexCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    ///Container type for all return fields from the `admin` function with signature `admin()` and selector `0xf851a440`
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
    pub struct AdminReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `arbiterTokenX` function with signature `arbiterTokenX()` and selector `0x3b204948`
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
    pub struct ArbiterTokenXReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `arbiterTokenY` function with signature `arbiterTokenY()` and selector `0xd0c472ec`
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
    pub struct ArbiterTokenYReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `price` function with signature `price()` and selector `0xa035b1fe`
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
    pub struct PriceReturn(pub ::ethers::core::types::U256);
}

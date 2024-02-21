pub use constant_sum::*;
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
pub mod constant_sum {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "computeSwapConstant",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("dfmm"),
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
                    ::std::borrow::ToOwned::to_owned("getPoolParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPoolParams"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
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
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("internalParams"),
                            inputs: ::std::vec![
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("price"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("controller"),
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
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("name"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("update"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateAllocateOrDeallocate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateSwap"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("invariant"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityDelta"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRx"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("nextL"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotDFMM"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotDFMM"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static CONSTANTSUM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0tW`\x1Fa\r\xB08\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0yW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0tWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0tW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\r \x90\x81a\0\x90\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\x08}V[a\x08TV[a\x07\x05V[a\x06\xCBV[a\x05\xB4V[a\x03CV[a\x02\x8BV[a\x02?V[4a\x01;W`@6`\x03\x19\x01\x12a\x01;W`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01;W6`#\x83\x01\x12\x15a\x01;Wa\x017a\x01'a\x01\x02a\0\xF36`\x04\x87\x015`$\x88\x01a\x01\xB3V[` \x80\x82Q\x83\x01\x01\x91\x01a\x08\xB0V[\x90a\x01 a\x01\x11`\x045a\n\xEDV[` \x80\x82Q\x83\x01\x01\x91\x01a\x08\xCBV[Q\x92a\x0BaV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[a\x01>V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01pW`@Q\x91a\x01\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x91V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xFAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02+WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x02\nV[4a\x01\xFAW`\x006`\x03\x19\x01\x12a\x01\xFAWa\x017`@Qa\x02_\x81a\x01uV[`\x0B\x81RjConstantSum`\xA8\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFFV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\0R`\x01` R```@`\0 \x80T\x90`\x01\x81\x01T\x90`\x02`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xFAWV[\x90```\x03\x19\x83\x01\x12a\x01\xFAW`\x045a\x03\x01\x81a\x02\xD7V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xFAW\x80`#\x83\x01\x12\x15a\x01\xFAW\x81`\x04\x015\x93\x84\x11a\x01\xFAW`$\x84\x83\x01\x01\x11a\x01\xFAW`$\x01\x91\x90V[4a\x01\xFAWa\x03Q6a\x02\xE8V[\x92P\x90a\x03\xAB``a\x03ea\x01\x11\x84a\n\xEDV[`\0T\x90\x93\x90a\x03\x8B\x90a\x03\x7F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x80\x80\x95\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x92\x83\x15a\x05\xAFW`\0\x94`\0\x92`\0\x95a\x05rW[P\x90a\x03\xD2\x91\x81\x01\x90a\t\x11V[\x92\x91\x93\x90\x95`\0\x92\x80\x86\x11`\0\x14a\x04\xBFWPa\x045a\x04-a\x04e\x94a\x04\x1Ba\x03\xFFa\x04>\x95\x8Aa\tBV[a\x04\x10\x81a\x04\x0Ba\tTV[a\x0B\xB0V[` \x87\x01Q\x90a\x0C\xBEV[\x90a\x04(\x82a\x04\x0Ba\t\x86V[a\t\xB4V[\x96[\x85a\t\xC1V[\x95\x86\x12\x15a\t\xDAV[a\x04O\x85a\x04Ja\t\xF7V[a\x0B\xFAV[a\x04\\\x81Qa\x04\x0Ba\n0V[Q\x82\x86\x85a\x0BaV[\x93a\x04r\x85a\x04Ja\nSV[\x84`\x13\x19\x12\x92\x83a\x04\xB4W[a\x017\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x04~V[\x92PP\x81\x86\x11\x15a\x05\x14Wa\x04>a\x045a\x05\x0Ea\x04\xFAa\x04\xE3a\x04e\x96\x8Ba\tBV[a\x04\xEF\x81a\x04\x0Ba\tTV[` \x86\x01Q\x90a\x0C\xBEV[a\x05\x06\x81a\x04\x0Ba\t\x86V[\x84Q\x90a\x0C\x8EV[\x96a\x04/V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x94Pa\x05\x9D\x91\x95Pa\x03\xD2\x92P``=``\x11a\x05\xA8W[a\x05\x95\x81\x83a\x01\x91V[\x81\x01\x90a\x08\xB0V[\x91\x95\x91\x94\x90\x92a\x03\xC4V[P=a\x05\x8BV[a\t\x05V[4a\x01\xFAWa\x05\xC26a\x02\xE8V[`\0T\x91\x93P\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x06\xB9W\x82\x90a\x05\xE2a\n\x8CV[P\x81\x01\x03\x91`\xC0\x83\x12a\x01\xFAW\x805\x91` \x82\x015\x91```@\x82\x015\x95`_\x19\x01\x12a\x01\xFAWa\x06w\x91`\x01a\x06l`@Q\x93a\x06\x1F\x85a\x01TV[``\x81\x015\x80\x86R`\xA0` \x87\x01\x92`\x80\x81\x015\x84R\x015a\x06@\x81a\x02\xD7V[`@\x87\x01Ra\x06Y\x85`\0R`\x01` R`@`\0 \x90V[UQ\x92`\0R`\x01` R`@`\0 \x90V[\x01UQ\x84\x83\x85a\x0BaV[\x92\x83`\x13\x19\x12\x91\x82a\x06\xAEW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x06\x84V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xFAWa\x06\xFCa\x01\x11a\x06\xF2a\x06wa\x06\xE56a\x02\xE8V[\x90\x80\x92\x95\x93P\x01\x90a\t\x11V[\x95\x91\x94\x90\x93a\n\xEDV[Q\x84\x83\x85a\x0BaV[4a\x01\xFAWa\x07\x136a\x02\xE8V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\x06\xB9Wa\x07Ta\x03\x7F`\x02a\x07F\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\x08BWa\x07g\x83\x82\x01\x82a\n\xB5V[a\x07p\x81a\n\xCDV[`\x02\x81\x03a\x07\xA5WPa\x07\x8Ca\x07\x91\x91a\x07\xA2\x93\x946\x91a\x01\xB3V[a\x0CQV[\x91`\0R`\x01` R`@`\0 \x90V[U\0[a\x07\xAE\x81a\n\xCDV[`\x01\x81\x03a\x07\xE1WPa\x07\xCCa\x07\x8Ca\x07\xDD\x92`\x01\x94\x956\x91a\x01\xB3V[\x92`\0R`\x01` R`@`\0 \x90V[\x01U\0[\x80a\x07\xED`\x03\x92a\n\xCDV[\x03a\x080Wa\x08\x0Ea\x07\xCCa\x08\t`\x02\x93a\x08.\x966\x91a\x01\xB3V[a\x0C)V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xFAW`\x006`\x03\x19\x01\x12a\x01\xFAW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAWa\x017a\x08\x9C`\x045a\n\xEDV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFFV[\x90\x81``\x91\x03\x12a\x01\xFAW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81``\x91\x03\x12a\x01\xFAW`@\x80Q\x91a\x08\xE4\x83a\x01TV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\x08\xFD\x81a\x02\xD7V[`@\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x01\xFAW\x805\x91`@` \x83\x015\x92\x015\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\tOWV[a\t,V[`@Q\x90a\ta\x82a\x01uV[`\x16\x82Ru\x03\x0Bk{\xABs\xA2Kq\x03Kq\x03\xB3\x0BcK#\x0B\xA3)\xD1`U\x1B` \x83\x01RV[`@Q\x90a\t\x93\x82a\x01uV[`\x12\x82Rq\x033++\x99\x03Kq\x03\xB3\x0BcK#\x0B\xA3)\xD1`u\x1B` \x83\x01RV[\x91\x90\x82\x01\x80\x92\x11a\tOWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\tOWV[\x15a\t\xE1WV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`@Q\x90a\n\x04\x82a\x01uV[`\x1C\x82R\x7FliquidityDelta in validate: \0\0\0\0` \x83\x01RV[`@Q\x90a\n=\x82a\x01uV[`\x07\x82Rf\x03\x83\x93K\x1B)\xD1`\xCD\x1B` \x83\x01RV[`@Q\x90a\n`\x82a\x01uV[`\x17\x82R\x7Finvariant in validate: \0\0\0\0\0\0\0\0\0` \x83\x01RV[`@Q\x90a\n\x99\x82a\x01TV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`\x04\x11\x15a\x01\xFAWV[\x90\x81` \x91\x03\x12a\x01\xFAW5a\n\xCA\x81a\n\xABV[\x90V[`\x04\x11\x15a\n\xD7WV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\n\xF5a\n\x8CV[\x90\x80`\0R`\x01` R`@\x90\x81`\0 T\x83R`\0R`\x01` R`\x01\x81`\0 \x01T\x91` \x81\x01\x92\x83R\x81Q\x92\x81Q` \x85\x01RQ\x82\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16``\x83\x01R``\x82R`\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01pWR\x90V[\x82\x93a\x0Bsa\x0B\x7F\x94a\x0By\x93a\x0C\x8EV[\x94a\x0C\xBEV[\x90a\x0C\x8EV[\x90`\0\x82\x82\x01\x92\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\tOWg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\tOW\x90V[a\x0B\xF3a\x0B\xDF\x91a\x0B\xF8\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x01\xFFV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x91V[a\x0CsV[V[a\x0B\xF3a\x0B\xDF\x91a\x0B\xF8\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x01\xFFV[`@\x81\x80Q\x81\x01\x03\x12a\x01\xFAW\x80a\x0CF` `@\x93\x01Qa\n\xABV[\x01Qa\x03\x7F\x81a\x02\xD7V[`@\x81\x80Q\x81\x01\x03\x12a\x01\xFAW\x80a\x0Cn` `@\x93\x01Qa\n\xABV[\x01Q\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xFAW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xFAW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 yI\x90u\xBD\xD3\x95\x83\xB9\xF3\tC\0N$\x99\xFD\t\x04\xE4`\x91\xBB\xBAu\x05\x12\xBF\x05P@fdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static CONSTANTSUM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\x08}V[a\x08TV[a\x07\x05V[a\x06\xCBV[a\x05\xB4V[a\x03CV[a\x02\x8BV[a\x02?V[4a\x01;W`@6`\x03\x19\x01\x12a\x01;W`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01;W6`#\x83\x01\x12\x15a\x01;Wa\x017a\x01'a\x01\x02a\0\xF36`\x04\x87\x015`$\x88\x01a\x01\xB3V[` \x80\x82Q\x83\x01\x01\x91\x01a\x08\xB0V[\x90a\x01 a\x01\x11`\x045a\n\xEDV[` \x80\x82Q\x83\x01\x01\x91\x01a\x08\xCBV[Q\x92a\x0BaV[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[\x03\x90\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[``\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[a\x01>V[`@\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01pW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01pW`@Q\x91a\x01\xDD`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01\x91V[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xFAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02+WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x02\nV[4a\x01\xFAW`\x006`\x03\x19\x01\x12a\x01\xFAWa\x017`@Qa\x02_\x81a\x01uV[`\x0B\x81RjConstantSum`\xA8\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFFV[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAW`\x045`\0R`\x01` R```@`\0 \x80T\x90`\x01\x81\x01T\x90`\x02`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90`@Q\x92\x83R` \x83\x01R`@\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xFAWV[\x90```\x03\x19\x83\x01\x12a\x01\xFAW`\x045a\x03\x01\x81a\x02\xD7V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xFAW\x80`#\x83\x01\x12\x15a\x01\xFAW\x81`\x04\x015\x93\x84\x11a\x01\xFAW`$\x84\x83\x01\x01\x11a\x01\xFAW`$\x01\x91\x90V[4a\x01\xFAWa\x03Q6a\x02\xE8V[\x92P\x90a\x03\xAB``a\x03ea\x01\x11\x84a\n\xEDV[`\0T\x90\x93\x90a\x03\x8B\x90a\x03\x7F\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x80\x80\x95\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x92\x83\x15a\x05\xAFW`\0\x94`\0\x92`\0\x95a\x05rW[P\x90a\x03\xD2\x91\x81\x01\x90a\t\x11V[\x92\x91\x93\x90\x95`\0\x92\x80\x86\x11`\0\x14a\x04\xBFWPa\x045a\x04-a\x04e\x94a\x04\x1Ba\x03\xFFa\x04>\x95\x8Aa\tBV[a\x04\x10\x81a\x04\x0Ba\tTV[a\x0B\xB0V[` \x87\x01Q\x90a\x0C\xBEV[\x90a\x04(\x82a\x04\x0Ba\t\x86V[a\t\xB4V[\x96[\x85a\t\xC1V[\x95\x86\x12\x15a\t\xDAV[a\x04O\x85a\x04Ja\t\xF7V[a\x0B\xFAV[a\x04\\\x81Qa\x04\x0Ba\n0V[Q\x82\x86\x85a\x0BaV[\x93a\x04r\x85a\x04Ja\nSV[\x84`\x13\x19\x12\x92\x83a\x04\xB4W[a\x017\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x04~V[\x92PP\x81\x86\x11\x15a\x05\x14Wa\x04>a\x045a\x05\x0Ea\x04\xFAa\x04\xE3a\x04e\x96\x8Ba\tBV[a\x04\xEF\x81a\x04\x0Ba\tTV[` \x86\x01Q\x90a\x0C\xBEV[a\x05\x06\x81a\x04\x0Ba\t\x86V[\x84Q\x90a\x0C\x8EV[\x96a\x04/V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x94Pa\x05\x9D\x91\x95Pa\x03\xD2\x92P``=``\x11a\x05\xA8W[a\x05\x95\x81\x83a\x01\x91V[\x81\x01\x90a\x08\xB0V[\x91\x95\x91\x94\x90\x92a\x03\xC4V[P=a\x05\x8BV[a\t\x05V[4a\x01\xFAWa\x05\xC26a\x02\xE8V[`\0T\x91\x93P\x90`\x01`\x01`\xA0\x1B\x03\x163\x03a\x06\xB9W\x82\x90a\x05\xE2a\n\x8CV[P\x81\x01\x03\x91`\xC0\x83\x12a\x01\xFAW\x805\x91` \x82\x015\x91```@\x82\x015\x95`_\x19\x01\x12a\x01\xFAWa\x06w\x91`\x01a\x06l`@Q\x93a\x06\x1F\x85a\x01TV[``\x81\x015\x80\x86R`\xA0` \x87\x01\x92`\x80\x81\x015\x84R\x015a\x06@\x81a\x02\xD7V[`@\x87\x01Ra\x06Y\x85`\0R`\x01` R`@`\0 \x90V[UQ\x92`\0R`\x01` R`@`\0 \x90V[\x01UQ\x84\x83\x85a\x0BaV[\x92\x83`\x13\x19\x12\x91\x82a\x06\xAEW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x06\x84V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xFAWa\x06\xFCa\x01\x11a\x06\xF2a\x06wa\x06\xE56a\x02\xE8V[\x90\x80\x92\x95\x93P\x01\x90a\t\x11V[\x95\x91\x94\x90\x93a\n\xEDV[Q\x84\x83\x85a\x0BaV[4a\x01\xFAWa\x07\x136a\x02\xE8V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\x06\xB9Wa\x07Ta\x03\x7F`\x02a\x07F\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\x08BWa\x07g\x83\x82\x01\x82a\n\xB5V[a\x07p\x81a\n\xCDV[`\x02\x81\x03a\x07\xA5WPa\x07\x8Ca\x07\x91\x91a\x07\xA2\x93\x946\x91a\x01\xB3V[a\x0CQV[\x91`\0R`\x01` R`@`\0 \x90V[U\0[a\x07\xAE\x81a\n\xCDV[`\x01\x81\x03a\x07\xE1WPa\x07\xCCa\x07\x8Ca\x07\xDD\x92`\x01\x94\x956\x91a\x01\xB3V[\x92`\0R`\x01` R`@`\0 \x90V[\x01U\0[\x80a\x07\xED`\x03\x92a\n\xCDV[\x03a\x080Wa\x08\x0Ea\x07\xCCa\x08\t`\x02\x93a\x08.\x966\x91a\x01\xB3V[a\x0C)V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[\0[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xFAW`\x006`\x03\x19\x01\x12a\x01\xFAW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xFAW` 6`\x03\x19\x01\x12a\x01\xFAWa\x017a\x08\x9C`\x045a\n\xEDV[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFFV[\x90\x81``\x91\x03\x12a\x01\xFAW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81``\x91\x03\x12a\x01\xFAW`@\x80Q\x91a\x08\xE4\x83a\x01TV[\x80Q\x83R` \x81\x01Q` \x84\x01R\x01Qa\x08\xFD\x81a\x02\xD7V[`@\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[\x90\x81``\x91\x03\x12a\x01\xFAW\x805\x91`@` \x83\x015\x92\x015\x90V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\tOWV[a\t,V[`@Q\x90a\ta\x82a\x01uV[`\x16\x82Ru\x03\x0Bk{\xABs\xA2Kq\x03Kq\x03\xB3\x0BcK#\x0B\xA3)\xD1`U\x1B` \x83\x01RV[`@Q\x90a\t\x93\x82a\x01uV[`\x12\x82Rq\x033++\x99\x03Kq\x03\xB3\x0BcK#\x0B\xA3)\xD1`u\x1B` \x83\x01RV[\x91\x90\x82\x01\x80\x92\x11a\tOWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\tOWV[\x15a\t\xE1WV[cNH{q`\xE0\x1B`\0R`\x01`\x04R`$`\0\xFD[`@Q\x90a\n\x04\x82a\x01uV[`\x1C\x82R\x7FliquidityDelta in validate: \0\0\0\0` \x83\x01RV[`@Q\x90a\n=\x82a\x01uV[`\x07\x82Rf\x03\x83\x93K\x1B)\xD1`\xCD\x1B` \x83\x01RV[`@Q\x90a\n`\x82a\x01uV[`\x17\x82R\x7Finvariant in validate: \0\0\0\0\0\0\0\0\0` \x83\x01RV[`@Q\x90a\n\x99\x82a\x01TV[`\0`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`\x04\x11\x15a\x01\xFAWV[\x90\x81` \x91\x03\x12a\x01\xFAW5a\n\xCA\x81a\n\xABV[\x90V[`\x04\x11\x15a\n\xD7WV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[a\n\xF5a\n\x8CV[\x90\x80`\0R`\x01` R`@\x90\x81`\0 T\x83R`\0R`\x01` R`\x01\x81`\0 \x01T\x91` \x81\x01\x92\x83R\x81Q\x92\x81Q` \x85\x01RQ\x82\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16``\x83\x01R``\x82R`\x80\x82\x01\x90\x82\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01pWR\x90V[\x82\x93a\x0Bsa\x0B\x7F\x94a\x0By\x93a\x0C\x8EV[\x94a\x0C\xBEV[\x90a\x0C\x8EV[\x90`\0\x82\x82\x01\x92\x83\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\tOWg\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\tOW\x90V[a\x0B\xF3a\x0B\xDF\x91a\x0B\xF8\x93`@Q\x93\x84\x92c-\x83\x9C\xB3`\xE2\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x01\xFFV[\x90`D\x83\x01R\x03`\x1F\x19\x81\x01\x83R\x82a\x01\x91V[a\x0CsV[V[a\x0B\xF3a\x0B\xDF\x91a\x0B\xF8\x93`@Q\x93\x84\x92c\x1ES\x13G`\xE1\x1B` \x85\x01R`@`$\x85\x01R`d\x84\x01\x90a\x01\xFFV[`@\x81\x80Q\x81\x01\x03\x12a\x01\xFAW\x80a\x0CF` `@\x93\x01Qa\n\xABV[\x01Qa\x03\x7F\x81a\x02\xD7V[`@\x81\x80Q\x81\x01\x03\x12a\x01\xFAW\x80a\x0Cn` `@\x93\x01Qa\n\xABV[\x01Q\x90V[`\0\x80\x91` \x81Q\x91\x01jconsole.logZ\xFAPV[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xFAW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xFAW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 yI\x90u\xBD\xD3\x95\x83\xB9\xF3\tC\0N$\x99\xFD\t\x04\xE4`\x91\xBB\xBAu\x05\x12\xBF\x05P@fdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static CONSTANTSUM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ConstantSum<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConstantSum<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConstantSum<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConstantSum<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConstantSum<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ConstantSum))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConstantSum<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONSTANTSUM_ABI.clone(),
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
                CONSTANTSUM_ABI.clone(),
                CONSTANTSUM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `computeSwapConstant` (0x002e524b) function
        pub fn compute_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([0, 46, 82, 75], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `dfmm` (0xafba13c4) function
        pub fn dfmm(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `init` (0x73cb2d03) function
        pub fn init(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([115, 203, 45, 3], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0xacad2989) function
        pub fn update(
            &self,
            sender: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([172, 173, 41, 137], (sender, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateAllocateOrDeallocate` (0x8a04bdd5) function
        pub fn validate_allocate_or_deallocate(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([138, 4, 189, 213], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSwap` (0x68bd3e38) function
        pub fn validate_swap(
            &self,
            p0: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                ::ethers::core::types::I256,
                ::ethers::core::types::I256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([104, 189, 62, 56], (p0, pool_id, data))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConstantSum<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidSender` with signature `InvalidSender()` and selector `0xddb5de5e`
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
        Hash
    )]
    #[etherror(name = "InvalidSender", abi = "InvalidSender()")]
    pub struct InvalidSender;
    ///Custom Error type `InvalidUpdateCode` with signature `InvalidUpdateCode()` and selector `0x235d2b3d`
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
        Hash
    )]
    #[etherror(name = "InvalidUpdateCode", abi = "InvalidUpdateCode()")]
    pub struct InvalidUpdateCode;
    ///Custom Error type `NotDFMM` with signature `NotDFMM()` and selector `0x6853cba7`
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
        Hash
    )]
    #[etherror(name = "NotDFMM", abi = "NotDFMM()")]
    pub struct NotDFMM;
    ///Container type for all of the contract's custom errors
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
    pub enum ConstantSumErrors {
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantSumErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ConstantSumErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateCode as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ConstantSumErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for ConstantSumErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for ConstantSumErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for ConstantSumErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
        }
    }
    ///Container type for all input parameters for the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    #[ethcall(name = "computeSwapConstant", abi = "computeSwapConstant(uint256,bytes)")]
    pub struct ComputeSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    #[ethcall(name = "dfmm", abi = "dfmm()")]
    pub struct DfmmCall;
    ///Container type for all input parameters for the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    #[ethcall(name = "getPoolParams", abi = "getPoolParams(uint256)")]
    pub struct GetPoolParamsCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `init` function with signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    #[ethcall(name = "init", abi = "init(address,uint256,bytes)")]
    pub struct InitCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    #[ethcall(name = "internalParams", abi = "internalParams(uint256)")]
    pub struct InternalParamsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `update` function with signature `update(address,uint256,bytes)` and selector `0xacad2989`
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
    #[ethcall(name = "update", abi = "update(address,uint256,bytes)")]
    pub struct UpdateCall {
        pub sender: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(address,uint256,bytes)` and selector `0x8a04bdd5`
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
    #[ethcall(
        name = "validateAllocateOrDeallocate",
        abi = "validateAllocateOrDeallocate(address,uint256,bytes)"
    )]
    pub struct ValidateAllocateOrDeallocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateSwap` function with signature `validateSwap(address,uint256,bytes)` and selector `0x68bd3e38`
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
    #[ethcall(name = "validateSwap", abi = "validateSwap(address,uint256,bytes)")]
    pub struct ValidateSwapCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum ConstantSumCalls {
        ComputeSwapConstant(ComputeSwapConstantCall),
        Dfmm(DfmmCall),
        GetPoolParams(GetPoolParamsCall),
        Init(InitCall),
        InternalParams(InternalParamsCall),
        Name(NameCall),
        Update(UpdateCall),
        ValidateAllocateOrDeallocate(ValidateAllocateOrDeallocateCall),
        ValidateSwap(ValidateSwapCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConstantSumCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InternalParams(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <ValidateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConstantSumCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConstantSumCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Dfmm(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPoolParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::InternalParams(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateSwap(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ComputeSwapConstantCall> for ConstantSumCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for ConstantSumCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for ConstantSumCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for ConstantSumCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for ConstantSumCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for ConstantSumCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for ConstantSumCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for ConstantSumCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for ConstantSumCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    ///Container type for all return fields from the `computeSwapConstant` function with signature `computeSwapConstant(uint256,bytes)` and selector `0x002e524b`
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
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `dfmm` function with signature `dfmm()` and selector `0xafba13c4`
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
    pub struct DfmmReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getPoolParams` function with signature `getPoolParams(uint256)` and selector `0xdc178355`
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
    pub struct GetPoolParamsReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `init` function with signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    pub struct InitReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `internalParams` function with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    pub struct InternalParamsReturn {
        pub price: ::ethers::core::types::U256,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    ///Container type for all return fields from the `validateAllocateOrDeallocate` function with signature `validateAllocateOrDeallocate(address,uint256,bytes)` and selector `0x8a04bdd5`
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
    pub struct ValidateAllocateOrDeallocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateSwap` function with signature `validateSwap(address,uint256,bytes)` and selector `0x68bd3e38`
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
    pub struct ValidateSwapReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub next_rx: ::ethers::core::types::U256,
        pub next_ry: ::ethers::core::types::U256,
        pub next_l: ::ethers::core::types::U256,
    }
}

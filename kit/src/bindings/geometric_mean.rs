pub use geometric_mean::*;
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
pub mod geometric_mean {
    pub use super::super::shared_types::*;
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
                                    name: ::std::borrow::ToOwned::to_owned("wX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
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
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidWeightX"),
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
    pub static GEOMETRICMEAN_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA04a\0\x7FW`\x1Fa\x12\xD58\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\x84W\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0\x7FWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0\x7FW`\x80R`@Qa\x12:\x90\x81a\0\x9B\x829`\x80Q\x81\x81\x81a\x03\xC1\x01R\x81\x81a\x06\x1C\x01R\x81\x81a\x07\xD0\x01Ra\t.\x01R\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\t]V[a\t\x18V[a\x07\xB7V[a\x07}V[a\x06\0V[a\x03;V[a\x02\xB8V[a\x02!V[4a\x01\x1BW`@6`\x03\x19\x01\x12a\x01\x1BW`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1BW` a\x01\x13a\0\xF0a\0\xE26`\x04\x87\x01a\x01\xC3V[\x83\x80\x82Q\x83\x01\x01\x91\x01a\t\x90V[\x90a\x01\ra\0\xFF`\x045a\n\x89V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\t\xABV[\x92a\x0BrV[`@Q\x90\x81R\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[a\x01\x1EV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01PW`@Q\x91a\x01\xA1`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01UV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xBEW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xBEW\x81` a\x01\xDE\x935\x91\x01a\x01wV[\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02\rWPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x01\xECV[4a\x01\xBEW`\x006`\x03\x19\x01\x12a\x01\xBEW`@Q`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01PWa\x02\x82\x91`@R`\r\x81Rl#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xE1V[\x03\x90\xF3[\x90`@Qa\x02\x93\x81a\x014V[```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x01\xBEW` 6`\x03\x19\x01\x12a\x01\xBEW`\x045`\0R`\0` R`\xC0`@`\0 a\x02\xE4\x81a\x02\x86V[\x90`\x04\x81\x01T\x90`\x05`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90```@Q\x93\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xBEWV[4a\x01\xBEW``6`\x03\x19\x01\x12a\x01\xBEWa\x03W`\x045a\x03*V[`$5`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xBEWa\x03z\x906\x90`\x04\x01a\x01\xC3V[\x90a\x03\x84\x81a\n\x89V[\x90a\x03\x9A\x82Q\x92` \x80\x80\x95\x83\x01\x01\x91\x01a\t\xABV[`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x90\x92\x90``\x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x94\x85\x15a\x05\xA0W`\0\x90\x81\x92\x82\x97a\x05gW[P\x80\x84\x80a\x04\t\x93Q\x83\x01\x01\x91\x01a\t\x90V[\x94\x91\x95\x90\x97\x87\x87\x85\x81\x11`\0\x14a\x04\xCEW\x93a\x04^\x86\x94a\x04X\x86a\x04Sa\x04\x80\x9B\x97a\x04Na\x04k\x98`@a\x04Ea\x04w\x9Fa\x04q\x9Fa\n\x12V[\x91\x01Q\x90a\x0FRV[a\x0FRV[a\x0F~V[Pa\n\x89V[\x80Q\x81\x01\x82\x01\x91\x01a\t\xABV[\x91a\x0B\xC3V[\x83a\n1V[\x93\x82\x86\x85a\x0BrV[\x93\x84`\x13\x19\x12\x92\x83a\x04\xC3W[a\x02\x82\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x04\x8DV[PP\x91\x92\x90\x93\x80\x89\x11`\0\x14a\x05\tWa\x04ka\x04w\x94a\x04^a\x04\x80\x97a\x04X\x85a\x04S\x8F\x99\x8Fa\x04N\x90`@a\x04E\x86a\x04q\x9Fa\n\x12V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x96Pa\x04\t\x92Pa\x05\x91\x91P``=``\x11a\x05\x99W[a\x05\x89\x81\x83a\x01UV[\x81\x01\x90a\t\x90V[\x96\x90\x92a\x03\xF6V[P=a\x05\x7FV[a\t\xF0V[\x90```\x03\x19\x83\x01\x12a\x01\xBEW`\x045a\x05\xBE\x81a\x03*V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xBEW\x80`#\x83\x01\x12\x15a\x01\xBEW\x81`\x04\x015\x93\x84\x11a\x01\xBEW`$\x84\x83\x01\x01\x11a\x01\xBEW`$\x01\x91\x90V[4a\x01\xBEWa\x06\x0E6a\x05\xA5V[\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x163\x03a\x07kW\x81`\xC0\x91\x81\x01\x03\x12a\x01\xBEW\x805\x91` \x82\x015\x91`@\x81\x015\x94``\x82\x015\x90`\xA0\x83\x015\x92a\x06s\x84a\x03*V[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x10\x15a\x07YWa\x07\x17\x94a\x07\x0F\x94`\x80a\x06\xFB\x93a\x07\0\x96a\x06\xA9\x87`\0R`\0` R`@`\0 \x90V[U\x015`\x04a\x06\xC2\x86`\0R`\0` R`@`\0 \x90V[\x01U\x16`\x05a\x06\xDB\x84`\0R`\0` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\n\x89V[` \x80\x82Q\x83\x01\x01\x91\x01a\t\xABV[\x84\x83\x85a\x0BrV[\x92\x83`\x13\x19\x12\x91\x82a\x07NW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x07$V[`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x90\xFD[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xBEW``a\x07\x8D6a\x05\xA5V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\xBEW\x805\x90a\x07\x17a\x07\x0Fa\x07\0`@` \x85\x015\x94\x015\x95a\n\x89V[4a\x01\xBEWa\x07\xC56a\x05\xA5V[\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x163\x03a\x07kWa\x08-a\x08!`\x05a\x08\x13\x87`\0R`\0` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\t\x06Wa\x08@\x83\x82\x01\x82a\nTV[a\x08I\x81a\niV[`\x01\x81\x03a\x08\x82WPa\x08la\x08ga\x08}\x92`\x04\x94\x956\x91a\x01wV[a\x0C\xB4V[\x92`\0R`\0` R`@`\0 \x90V[\x01U[\0[a\x08\x8B\x81a\niV[`\x02\x81\x03a\x08\xC7WP\x90a\x08\xAFa\x08\xAAa\x08\xC2\x93a\x08\x80\x956\x91a\x01wV[a\x0C\nV[\x92\x90\x91`\0R`\0` R`@`\0 \x90V[a\x0C2V[\x80a\x08\xD3`\x03\x92a\niV[\x03a\x08\xF4Wa\x06\xDBa\x08la\x08\xEF`\x05\x93a\x08\x80\x966\x91a\x01wV[a\x0B\xE2V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xBEW`\x006`\x03\x19\x01\x12a\x01\xBEW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xBEW` 6`\x03\x19\x01\x12a\x01\xBEWa\x02\x82a\t|`\x045a\n\x89V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xE1V[\x90\x81``\x91\x03\x12a\x01\xBEW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\x80\x91\x03\x12a\x01\xBEW```@Q\x91a\t\xC5\x83a\x014V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x01Qa\t\xE8\x81a\x03*V[``\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\n\x1FWV[a\t\xFCV[\x91\x90\x82\x01\x80\x92\x11a\n\x1FWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\x1FWV[`\x04\x11\x15a\x01\xBEWV[\x90\x81` \x91\x03\x12a\x01\xBEW5a\x01\xDE\x81a\nJV[`\x04\x11\x15a\nsWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x80Qa\n\x96\x81a\x014V[`\0\x91\x82\x82R` \x82\x01\x93\x83\x85R\x81\x83\x01\x84\x81R``\x84\x01\x90\x85\x82R\x82\x86R\x85` Ra\n\xCCa\n\xC7\x85\x88 a\x02\x86V[a\x0C\xE9V[\x80\x86Rg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\n\x1FW\x84a\x01\xDE\x97a\x0B)\x95a\x0B\x1C\x94`\x05\x94a\x0Bd\x9CR\x81\x83R\x82` R`\x04\x84\x84 \x01T\x90R\x81R\x80` R \x01`\x01\x80`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Q\x92\x83\x91` \x83\x01\x91\x90\x91```\x80\x82\x01\x93\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x01UV[\x92` a\x0B\x9B\x84a\x0B\x95a\x0B\x8Da\x0B\xA4\x96\x97a\x0B\xAA\x99a\x0F\xAEV[\x85Q\x90a\r\x92V[\x95a\x0F\xAEV[\x91\x01Q\x90a\r\x92V[\x90a\x0FRV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\n\x1FW\x90V[a\x01\xDE\x92\x91` a\x0B\xD9a\x0B\xA4\x93\x85Q\x90a\r\x92V[\x93\x01Q\x90a\r\x92V[`@\x81\x80Q\x81\x01\x03\x12a\x01\xBEW\x80a\x0B\xFF` `@\x93\x01Qa\nJV[\x01Qa\x08!\x81a\x03*V[``\x81\x80Q\x81\x01\x03\x12a\x01\xBEWa\x0C$` \x82\x01Qa\nJV[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0C\xA2Wa\x0CHa\n\xC7\x84a\x02\x86V[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\n\x1FWa\x0Cf\x91a\n1V[B\x83\x14a\x0C\x8CW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\n\x1FW`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\xBEW\x80a\x0C\xD1` `@\x93\x01Qa\nJV[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\n\x1FWV[``\x81\x01Q\x90` \x81\x01Q\x80\x83\x14a\rhW\x80B\x11`\0\x14a\r`W\x91[\x82\x03\x91\x82\x11a\n\x1FW`@\x81\x01\x90\x81Q`\0\x81\x13`\0\x14a\r:WPa\x01\xDE\x92a\r4\x91Q\x92Q\x90a\x0C\xD6V[\x90a\n$V[\x90Q\x91P`\x01`\xFF\x1B\x81\x14a\n\x1FWa\x01\xDE\x92a\rZ\x91`\0\x03\x90a\x0C\xD6V[\x90a\n\x12V[PB\x91a\r\x07V[P\x90PQ\x90V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\n\x1FW\x81\x84\x05\x14\x90\x15\x17\x15a\n\x1FWV[a\x0F?a\x01\xDE\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x0FM\x93a\r\xC8`\0\x82\x13a\x0F\xD0V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\r\xE4\x82a\x11\x92V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\roV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x10\x08V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xBEW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xBEW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xBEW\x04\x90V[\x15a\x0F\xD7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x11\x8CWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x11XWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\x11\x9D\x81\x15\x15a\x0F\xD0V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V\xFE\xA2dipfsX\"\x12 f.\x92R$\xF9\x06\x10\x13m\xEC\x12(\x1ECc\xEA\xD2y\x15I\x04\x1E\x06\xD9G\x0F\x9C\x99\xB5;\x82dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static GEOMETRICMEAN_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\t]V[a\t\x18V[a\x07\xB7V[a\x07}V[a\x06\0V[a\x03;V[a\x02\xB8V[a\x02!V[4a\x01\x1BW`@6`\x03\x19\x01\x12a\x01\x1BW`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1BW` a\x01\x13a\0\xF0a\0\xE26`\x04\x87\x01a\x01\xC3V[\x83\x80\x82Q\x83\x01\x01\x91\x01a\t\x90V[\x90a\x01\ra\0\xFF`\x045a\n\x89V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\t\xABV[\x92a\x0BrV[`@Q\x90\x81R\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\x80\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[a\x01\x1EV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01PW`@Q\x91a\x01\xA1`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01UV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xBEW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xBEW\x81` a\x01\xDE\x935\x91\x01a\x01wV[\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02\rWPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x01\xECV[4a\x01\xBEW`\x006`\x03\x19\x01\x12a\x01\xBEW`@Q`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01PWa\x02\x82\x91`@R`\r\x81Rl#\xB2\xB7\xB6\xB2\xBA94\xB1\xA6\xB2\xB0\xB7`\x99\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xE1V[\x03\x90\xF3[\x90`@Qa\x02\x93\x81a\x014V[```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x01\xBEW` 6`\x03\x19\x01\x12a\x01\xBEW`\x045`\0R`\0` R`\xC0`@`\0 a\x02\xE4\x81a\x02\x86V[\x90`\x04\x81\x01T\x90`\x05`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x90```@Q\x93\x80Q\x85R` \x81\x01Q` \x86\x01R`@\x81\x01Q`@\x86\x01R\x01Q``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xBEWV[4a\x01\xBEW``6`\x03\x19\x01\x12a\x01\xBEWa\x03W`\x045a\x03*V[`$5`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x01\xBEWa\x03z\x906\x90`\x04\x01a\x01\xC3V[\x90a\x03\x84\x81a\n\x89V[\x90a\x03\x9A\x82Q\x92` \x80\x80\x95\x83\x01\x01\x91\x01a\t\xABV[`@Qc3\x85N\xFD`\xE2\x1B\x81R`\x04\x81\x01\x83\x90R\x90\x92\x90``\x81`$\x81`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16Z\xFA\x94\x85\x15a\x05\xA0W`\0\x90\x81\x92\x82\x97a\x05gW[P\x80\x84\x80a\x04\t\x93Q\x83\x01\x01\x91\x01a\t\x90V[\x94\x91\x95\x90\x97\x87\x87\x85\x81\x11`\0\x14a\x04\xCEW\x93a\x04^\x86\x94a\x04X\x86a\x04Sa\x04\x80\x9B\x97a\x04Na\x04k\x98`@a\x04Ea\x04w\x9Fa\x04q\x9Fa\n\x12V[\x91\x01Q\x90a\x0FRV[a\x0FRV[a\x0F~V[Pa\n\x89V[\x80Q\x81\x01\x82\x01\x91\x01a\t\xABV[\x91a\x0B\xC3V[\x83a\n1V[\x93\x82\x86\x85a\x0BrV[\x93\x84`\x13\x19\x12\x92\x83a\x04\xC3W[a\x02\x82\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x04\x8DV[PP\x91\x92\x90\x93\x80\x89\x11`\0\x14a\x05\tWa\x04ka\x04w\x94a\x04^a\x04\x80\x97a\x04X\x85a\x04S\x8F\x99\x8Fa\x04N\x90`@a\x04E\x86a\x04q\x9Fa\n\x12V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x96Pa\x04\t\x92Pa\x05\x91\x91P``=``\x11a\x05\x99W[a\x05\x89\x81\x83a\x01UV[\x81\x01\x90a\t\x90V[\x96\x90\x92a\x03\xF6V[P=a\x05\x7FV[a\t\xF0V[\x90```\x03\x19\x83\x01\x12a\x01\xBEW`\x045a\x05\xBE\x81a\x03*V[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xBEW\x80`#\x83\x01\x12\x15a\x01\xBEW\x81`\x04\x015\x93\x84\x11a\x01\xBEW`$\x84\x83\x01\x01\x11a\x01\xBEW`$\x01\x91\x90V[4a\x01\xBEWa\x06\x0E6a\x05\xA5V[\x91\x92P`\x01`\x01`\xA0\x1B\x03\x91\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x163\x03a\x07kW\x81`\xC0\x91\x81\x01\x03\x12a\x01\xBEW\x805\x91` \x82\x015\x91`@\x81\x015\x94``\x82\x015\x90`\xA0\x83\x015\x92a\x06s\x84a\x03*V[g\r\xE0\xB6\xB3\xA7d\0\0\x83\x10\x15a\x07YWa\x07\x17\x94a\x07\x0F\x94`\x80a\x06\xFB\x93a\x07\0\x96a\x06\xA9\x87`\0R`\0` R`@`\0 \x90V[U\x015`\x04a\x06\xC2\x86`\0R`\0` R`@`\0 \x90V[\x01U\x16`\x05a\x06\xDB\x84`\0R`\0` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\n\x89V[` \x80\x82Q\x83\x01\x01\x91\x01a\t\xABV[\x84\x83\x85a\x0BrV[\x92\x83`\x13\x19\x12\x91\x82a\x07NW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x07$V[`@Qc\xE8\xA3\x8Aa`\xE0\x1B\x81R`\x04\x90\xFD[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xBEW``a\x07\x8D6a\x05\xA5V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\xBEW\x805\x90a\x07\x17a\x07\x0Fa\x07\0`@` \x85\x015\x94\x015\x95a\n\x89V[4a\x01\xBEWa\x07\xC56a\x05\xA5V[\x92`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x163\x03a\x07kWa\x08-a\x08!`\x05a\x08\x13\x87`\0R`\0` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\t\x06Wa\x08@\x83\x82\x01\x82a\nTV[a\x08I\x81a\niV[`\x01\x81\x03a\x08\x82WPa\x08la\x08ga\x08}\x92`\x04\x94\x956\x91a\x01wV[a\x0C\xB4V[\x92`\0R`\0` R`@`\0 \x90V[\x01U[\0[a\x08\x8B\x81a\niV[`\x02\x81\x03a\x08\xC7WP\x90a\x08\xAFa\x08\xAAa\x08\xC2\x93a\x08\x80\x956\x91a\x01wV[a\x0C\nV[\x92\x90\x91`\0R`\0` R`@`\0 \x90V[a\x0C2V[\x80a\x08\xD3`\x03\x92a\niV[\x03a\x08\xF4Wa\x06\xDBa\x08la\x08\xEF`\x05\x93a\x08\x80\x966\x91a\x01wV[a\x0B\xE2V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xBEW`\x006`\x03\x19\x01\x12a\x01\xBEW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x01\xBEW` 6`\x03\x19\x01\x12a\x01\xBEWa\x02\x82a\t|`\x045a\n\x89V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xE1V[\x90\x81``\x91\x03\x12a\x01\xBEW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\x80\x91\x03\x12a\x01\xBEW```@Q\x91a\t\xC5\x83a\x014V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x01Qa\t\xE8\x81a\x03*V[``\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\n\x1FWV[a\t\xFCV[\x91\x90\x82\x01\x80\x92\x11a\n\x1FWV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\x1FWV[`\x04\x11\x15a\x01\xBEWV[\x90\x81` \x91\x03\x12a\x01\xBEW5a\x01\xDE\x81a\nJV[`\x04\x11\x15a\nsWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@\x80Qa\n\x96\x81a\x014V[`\0\x91\x82\x82R` \x82\x01\x93\x83\x85R\x81\x83\x01\x84\x81R``\x84\x01\x90\x85\x82R\x82\x86R\x85` Ra\n\xCCa\n\xC7\x85\x88 a\x02\x86V[a\x0C\xE9V[\x80\x86Rg\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x03\x90\x81\x11a\n\x1FW\x84a\x01\xDE\x97a\x0B)\x95a\x0B\x1C\x94`\x05\x94a\x0Bd\x9CR\x81\x83R\x82` R`\x04\x84\x84 \x01T\x90R\x81R\x80` R \x01`\x01\x80`\xA0\x1B\x03\x90T\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x90RV[Q\x92\x83\x91` \x83\x01\x91\x90\x91```\x80\x82\x01\x93\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R\x81`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16\x91\x01RV[\x03`\x1F\x19\x81\x01\x83R\x82a\x01UV[\x92` a\x0B\x9B\x84a\x0B\x95a\x0B\x8Da\x0B\xA4\x96\x97a\x0B\xAA\x99a\x0F\xAEV[\x85Q\x90a\r\x92V[\x95a\x0F\xAEV[\x91\x01Q\x90a\r\x92V[\x90a\x0FRV[g\r\xE0\xB6\xB3\xA7c\xFF\xFF\x19\x81\x01\x90\x81\x13`\x01\x16a\n\x1FW\x90V[a\x01\xDE\x92\x91` a\x0B\xD9a\x0B\xA4\x93\x85Q\x90a\r\x92V[\x93\x01Q\x90a\r\x92V[`@\x81\x80Q\x81\x01\x03\x12a\x01\xBEW\x80a\x0B\xFF` `@\x93\x01Qa\nJV[\x01Qa\x08!\x81a\x03*V[``\x81\x80Q\x81\x01\x03\x12a\x01\xBEWa\x0C$` \x82\x01Qa\nJV[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0C\xA2Wa\x0CHa\n\xC7\x84a\x02\x86V[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\n\x1FWa\x0Cf\x91a\n1V[B\x83\x14a\x0C\x8CW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\n\x1FW`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\xBEW\x80a\x0C\xD1` `@\x93\x01Qa\nJV[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\n\x1FWV[``\x81\x01Q\x90` \x81\x01Q\x80\x83\x14a\rhW\x80B\x11`\0\x14a\r`W\x91[\x82\x03\x91\x82\x11a\n\x1FW`@\x81\x01\x90\x81Q`\0\x81\x13`\0\x14a\r:WPa\x01\xDE\x92a\r4\x91Q\x92Q\x90a\x0C\xD6V[\x90a\n$V[\x90Q\x91P`\x01`\xFF\x1B\x81\x14a\n\x1FWa\x01\xDE\x92a\rZ\x91`\0\x03\x90a\x0C\xD6V[\x90a\n\x12V[PB\x91a\r\x07V[P\x90PQ\x90V[\x81\x81\x02\x92\x91`\0\x82\x12`\x01`\xFF\x1B\x82\x14\x16a\n\x1FW\x81\x84\x05\x14\x90\x15\x17\x15a\n\x1FWV[a\x0F?a\x01\xDE\x92}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84a\x0FM\x93a\r\xC8`\0\x82\x13a\x0F\xD0V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\r\xE4\x82a\x11\x92V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1Da\roV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x10\x08V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xBEW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xBEW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xBEW\x04\x90V[\x15a\x0F\xD7WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x11\x8CWh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x11XWe\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[P`\0\x90V[a\x11\x9D\x81\x15\x15a\x0F\xD0V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V\xFE\xA2dipfsX\"\x12 f.\x92R$\xF9\x06\x10\x13m\xEC\x12(\x1ECc\xEA\xD2y\x15I\x04\x1E\x06\xD9G\x0F\x9C\x99\xB5;\x82dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static GEOMETRICMEAN_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GeometricMean<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GeometricMean<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GeometricMean<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GeometricMean<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GeometricMean<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GeometricMean))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GeometricMean<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GEOMETRICMEAN_ABI.clone(),
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
                GEOMETRICMEAN_ABI.clone(),
                GEOMETRICMEAN_BYTECODE.clone().into(),
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
            (DynamicParam, ::ethers::core::types::U256, ::ethers::core::types::Address),
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
    for GeometricMean<M> {
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
    ///Custom Error type `InvalidUpdateEnd` with signature `InvalidUpdateEnd()` and selector `0xcde205da`
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
    #[etherror(name = "InvalidUpdateEnd", abi = "InvalidUpdateEnd()")]
    pub struct InvalidUpdateEnd;
    ///Custom Error type `InvalidWeightX` with signature `InvalidWeightX()` and selector `0xe8a38a61`
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
    #[etherror(name = "InvalidWeightX", abi = "InvalidWeightX()")]
    pub struct InvalidWeightX;
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
    pub enum GeometricMeanErrors {
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        InvalidWeightX(InvalidWeightX),
        NotDFMM(NotDFMM),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for GeometricMeanErrors {
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
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) = <InvalidWeightX as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidWeightX(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotDFMM(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GeometricMeanErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidSender(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateCode(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidUpdateEnd(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWeightX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for GeometricMeanErrors {
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
                    == <InvalidUpdateEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidWeightX as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for GeometricMeanErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeightX(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for GeometricMeanErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for GeometricMeanErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for GeometricMeanErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for GeometricMeanErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<InvalidWeightX> for GeometricMeanErrors {
        fn from(value: InvalidWeightX) -> Self {
            Self::InvalidWeightX(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for GeometricMeanErrors {
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
    pub enum GeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiDecode for GeometricMeanCalls {
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
    impl ::ethers::core::abi::AbiEncode for GeometricMeanCalls {
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
    impl ::core::fmt::Display for GeometricMeanCalls {
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
    impl ::core::convert::From<ComputeSwapConstantCall> for GeometricMeanCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for GeometricMeanCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for GeometricMeanCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for GeometricMeanCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for GeometricMeanCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for GeometricMeanCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for GeometricMeanCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for GeometricMeanCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for GeometricMeanCalls {
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
        pub w_x: DynamicParam,
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

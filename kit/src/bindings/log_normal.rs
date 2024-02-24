pub use log_normal::*;
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
pub mod log_normal {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("dfmm_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("computeSwapConstant"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("computeSwapConstant",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    ::std::borrow::ToOwned::to_owned("dfmm"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("dfmm"),
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("internalParams"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("internalParams"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("sigma"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("tau"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("strike"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("struct DynamicParam"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("swapFee"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("name"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("name"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::String,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("string"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("validateAllocateOrDeallocate",),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                    ::std::borrow::ToOwned::to_owned("validateSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextRy"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextL"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Infinity"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Infinity"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSender"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSender"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateCode"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidUpdateEnd"),
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
                    ::std::borrow::ToOwned::to_owned("NotDFMM"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotDFMM"),
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
    pub static LOGNORMAL_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x804a\0tW`\x1Fa\x1B\xC48\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0yW\x80\x84\x92` \x94`@R\x839\x81\x01\x03\x12a\0tWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x90\x81\x90\x03a\0tW`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90U`@Qa\x1B4\x90\x81a\0\x90\x829\xF3[`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD\xFE`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\t\xE4V[a\t\xBBV[a\x08\x14V[a\x07\xDAV[a\x06tV[a\x03\xDCV[a\x02\xE1V[a\x02=V[4a\x01\x1BW`@6`\x03\x19\x01\x12a\x01\x1BW`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1BW` a\x01\x13a\0\xF0a\0\xE26`\x04\x87\x01a\x01\xDFV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\n\x17V[\x90a\x01\ra\0\xFF`\x045a\x0B\x92V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\n2V[\x92a\r\x8DV[`@Q\x90\x81R\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[a\x01\x1EV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01PW`@Q\x91a\x01\xBD`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01qV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xDAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDAW\x81` a\x01\xFA\x935\x91\x01a\x01\x93V[\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02)WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x02\x08V[4a\x01\xDAW`\x006`\x03\x19\x01\x12a\x01\xDAW`@Q`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01PWa\x02\x9A\x91`@R`\t\x81Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[\x03\x90\xF3[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@R```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x01\xDAW` 6`\x03\x19\x01\x12a\x01\xDAW`\x045`\0R`\x01` Ra\x01\xC0`@`\0 a\x03\x0E\x81a\x02\x9EV[\x90a\x03\x1B`\x04\x82\x01a\x02\x9EV[\x90a\x03\xBDa\x03+`\x08\x83\x01a\x02\x9EV[a\x03\x93`\x0C\x84\x01T\x93`\r`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x94a\x03m`@Q\x80\x98``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x80Q`\x80\x88\x01R` \x81\x01Q`\xA0\x88\x01R`@\x81\x01Q`\xC0\x88\x01R``\x01Q`\xE0\x87\x01RV[\x80Qa\x01\0\x86\x01R` \x81\x01Qa\x01 \x86\x01R`@\x81\x01Qa\x01@\x86\x01R``\x01Qa\x01`\x85\x01RV[a\x01\x80\x83\x01Ra\x01\xA0\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xDAWV[4a\x01\xDAW``\x80`\x03\x196\x01\x12a\x01\xDAWa\x03\xF9`\x045a\x03\xCBV[`$5`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xDAWa\x04 a\x04\x82\x936\x90`\x04\x01a\x01\xDFV[\x81a\x04<a\x04-\x85a\x0B\x92V[` \x80\x82Q\x83\x01\x01\x91\x01a\n2V[`\0T\x90\x94\x90a\x04b\x90a\x04V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x80\x80\x98\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x80\x15a\x06\x14W`\0\x91\x82\x95\x83\x92a\x05\xD9W[P\x80` \x80a\x04\xAC\x93Q\x83\x01\x01\x91\x01a\n\x17V[\x93\x91\x94\x90\x96\x82\x86\x11`\0\x14a\x05LWP\x91a\x04\xED\x82a\x04\xE8\x83a\x04\xE3a\x04\xFE\x97a\x04\xD9a\x04\xF5\x98\x8Ca\n\xA3V[\x90\x8C\x01Q\x90a\x12\xAFV[a\x12\xAFV[a\x12\xDBV[P[\x83a\n\xFEV[\x93\x82\x86\x85a\r\x8DV[\x93\x84`\x13\x19\x12\x92\x83a\x05AW[a\x02\x9A\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x05\x0BV[\x90\x92\x90\x91P\x81\x87\x11\x15a\x05{Wa\x05u\x82a\x04\xE8\x83a\x04\xE3a\x04\xFE\x97a\x04\xD9a\x04\xF5\x98\x8Ea\n\xA3V[Pa\x04\xEFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x95Pa\x04\xAC\x92Pa\x06\x01\x91P\x83=\x85\x11a\x06\rW[a\x05\xF9\x81\x83a\x01qV[\x81\x01\x90a\n\x17V[\x95\x91\x92\x90\x95\x91\x90a\x04\x98V[P=a\x05\xEFV[a\n\x81V[\x90```\x03\x19\x83\x01\x12a\x01\xDAW`\x045a\x062\x81a\x03\xCBV[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xDAW\x80`#\x83\x01\x12\x15a\x01\xDAW\x81`\x04\x015\x93\x84\x11a\x01\xDAW`$\x84\x83\x01\x01\x11a\x01\xDAW`$\x01\x91\x90V[4a\x01\xDAWa\x06\x826a\x06\x19V[`\0T\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x07\xC8Wa\x07\x86a\x07~a\x04-a\x07y\x95a\x06\xC0`\x80\x95a\x07C\x97a\x06\xB7a\x0BgV[P\x81\x01\x90a\x0E\x8CV[` \x81\x9B\x94\x9A\x93\x99\x92\x9B\x01Qa\x06\xE0\x86`\0R`\x01` R`@`\0 \x90V[U`@\x81\x01Q`\x04a\x06\xFC\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x80Q`\x08a\x07\x16\x87`\0R`\x01` R`@`\0 \x90V[\x01U``\x81\x01Q`\x0Ca\x073\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\ra\x07Y\x84`\0R`\x01` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0B\x92V[\x84\x83\x85a\r\x8DV[\x92\x83`\x13\x19\x12\x91\x82a\x07\xBDW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x07\x93V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xDAW``a\x07\xEA6a\x06\x19V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\xDAW\x805\x90a\x07\x86a\x07~a\x04-`@` \x85\x015\x94\x015\x95a\x0B\x92V[4a\x01\xDAWa\x08\"6a\x06\x19V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\x07\xC8Wa\x08ca\x04V`\ra\x08U\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\t\xA9Wa\x08v\x83\x82\x01\x82a\x0B2V[a\x08\x7F\x81a\x0BGV[`\x01\x81\x03a\x08\xB8WPa\x08\xA2a\x08\x9Da\x08\xB3\x92`\x0C\x94\x956\x91a\x01\x93V[a\x0F\xC3V[\x92`\0R`\x01` R`@`\0 \x90V[\x01U[\0[a\x08\xC1\x81a\x0BGV[`\x03\x81\x03a\x08\xFDWP\x90a\x08\xE5a\x08\xE0a\x08\xF8\x93a\x08\xB6\x956\x91a\x01\x93V[a\x0F\x19V[\x92\x90\x91`\0R`\x01` R`@`\0 \x90V[a\x0FAV[a\t\x06\x81a\x0BGV[`\x04\x81\x03a\t@WP\x90`\x04a\t'a\x08\xE0a\t:\x94a\x08\xB6\x966\x91a\x01\x93V[\x93\x90\x92`\0R`\x01` R`@`\0 \x90V[\x01a\x0FAV[a\tI\x81a\x0BGV[`\x02\x81\x03a\tjWP\x90`\x08a\t'a\x08\xE0a\t:\x94a\x08\xB6\x966\x91a\x01\x93V[\x80a\tv`\x05\x92a\x0BGV[\x03a\t\x97Wa\x07Ya\x08\xA2a\t\x92`\r\x93a\x08\xB6\x966\x91a\x01\x93V[a\x0E\xF1V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xDAW`\x006`\x03\x19\x01\x12a\x01\xDAW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xDAW` 6`\x03\x19\x01\x12a\x01\xDAWa\x02\x9Aa\n\x03`\x045a\x0B\x92V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[\x90\x81``\x91\x03\x12a\x01\xDAW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\xA0\x91\x03\x12a\x01\xDAW`\x80`@Q\x91a\nL\x83a\x014V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Qa\ny\x81a\x03\xCBV[`\x80\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\n\xB0WV[a\n\x8DV[\x91\x90\x82\x01\x80\x92\x11a\n\xB0WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\n\xB0WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\n\xB0WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xB0WV[`\x01`\xFF\x1B\x81\x14a\n\xB0W`\0\x03\x90V[`\x06\x11\x15a\x01\xDAWV[\x90\x81` \x91\x03\x12a\x01\xDAW5a\x01\xFA\x81a\x0B(V[`\x06\x11\x15a\x0BQWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@Q\x90a\x0Bt\x82a\x014V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[a\x0B\x9Aa\x0BgV[\x81`\0R`\x01` Ra\x0B\xB8a\x0B\xB3`@`\0 a\x02\x9EV[a\x0F\xF8V[\x91` \x82\x01\x92\x83R\x80`\0R`\x01` Ra\x0B\xDCa\x0B\xB3`\x08`@`\0 \x01a\x02\x9EV[\x82R`\x0Ca\x0C\x1Da\x0C\x05a\x0B\xB3`\x04a\x0B\xFF\x86`\0R`\x01` R`@`\0 \x90V[\x01a\x02\x9EV[\x92`@\x85\x01\x93\x84R`\0R`\x01` R`@`\0 \x90V[\x01T\x90``\x83\x01\x91\x82R`@Q\x93\x83Q` \x86\x01RQ`@\x85\x01RQ``\x84\x01RQ`\x80\x83\x01R`\x80`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xA0\x82\x01R`\xA0\x81Ra\x01\xFA\x81a\x01UV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xB0WV[\x90\x92\x82\x82\x10\x15a\x0EGWa\x01\xFA\x93a\x0E\n\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\r\xB5\x83\x83a\x13\x0BV[\x10a\x0E4WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\r\xDDa\r\xD7\x83\x85a\x13-V[\x85a\x13\x0BV[\x10a\x0E\x0FWP`\x01`\x01`\xFF\x1B\x03\x92a\x0E\x04\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x12\x86V[\x92a\rqV[a\rqV[a\x0E\x04\x92a\x0E#a\x0E)\x92a\x0E.\x94a\x13-V[\x90a\x13\x0BV[a\x10\xB3V[\x91a\r\xF4V[a\x0EA\x91a\x0E)\x91a\x13\x0BV[\x94a\r\xC7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80\x91\x03a\x01\0\x81\x12a\x01\xDAW\x815\x92` \x83\x015\x92`\xA0`@\x82\x015\x93`_\x19\x01\x12a\x01\xDAW`\xE0`@Q\x91a\x0E\xC1\x83a\x014V[``\x81\x015\x83R`\x80\x81\x015` \x84\x01R`\xA0\x81\x015`@\x84\x01R`\xC0\x81\x015``\x84\x01R\x015a\ny\x81a\x03\xCBV[`@\x81\x80Q\x81\x01\x03\x12a\x01\xDAW\x80a\x0F\x0E` `@\x93\x01Qa\x0B(V[\x01Qa\x04V\x81a\x03\xCBV[``\x81\x80Q\x81\x01\x03\x12a\x01\xDAWa\x0F3` \x82\x01Qa\x0B(V[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0F\xB1Wa\x0FWa\x0B\xB3\x84a\x02\x9EV[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\n\xB0Wa\x0Fu\x91a\n\xFEV[B\x83\x14a\x0F\x9BW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\n\xB0W`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\xDAW\x80a\x0F\xE0` `@\x93\x01Qa\x0B(V[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\n\xB0WV[``\x81\x01Q` \x82\x01Q\x80\x82\x14a\x10sW\x80B\x11`\0\x14a\x10kW\x90[\x81\x03\x90\x81\x11a\n\xB0W`@\x82\x01\x90\x81Q`\0\x81\x13`\0\x14a\x10HWPa\x10B\x90a\x01\xFA\x93Q\x92Q\x90a\x0F\xE5V[\x90a\n\xB5V[\x92a\x10_\x92Pa\x10Y\x90Q\x93a\x0B\x17V[\x90a\x0F\xE5V[\x81\x03\x90\x81\x11a\n\xB0W\x90V[PB\x90a\x10\x15V[PPQ\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\n\xB0WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\n\xB0W`\0\x19\x83\x05\x03a\n\xB0WV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x12\x80Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x12*W\x81\x15a\x12KW`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\n\xB0W`\0\x83\x12\x80\x15a\x12oW[a\x12]W\x82\x15a\x12*Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x12KW\x82\x12\x91\x82\x15a\x12<W\x92[a\x11$\x84a\x18\xDBV[\x80\x15a\x12*Wa\x11\x96a\x11Ua\x11Pa\x11Ka\x11Fa\x11\x9B\x95\x99\x97\x96\x99a\x14oV[a\x19\x9CV[a\x13yV[a\x10yV[a\x11\x91a\x11ia\x11d\x83a\x19\x06V[a\x0CcV[a\x11\x8Ba\x11\x86a\x11\x80a\x11{\x86a\x191V[a\x0C{V[\x85a\x1A\x13V[a\x0C\x93V[\x90a\x19zV[a\n\xFEV[a\x19\xC4V[\x93`\0\x92[\x81\x84\x10a\x11\xD2WPPPPa\x01\xFA\x91a\x11\xBF\x91`\0\x14a\x11\xC4Wa\x18\xB4V[a\x0B\x17V[a\x11\xCD\x90a\x0B\x17V[a\x18\xB4V[\x90\x91a\x12 \x86a\x12\x1Aa\x11\xEA\x85a\x11\x91\x86\x99\x9Ba\x16\x10V[a\x11\x8Ba\x12\na\x12\x05a\x12\0a\x11\xBF\x87\x80a\x1A\x13V[a\x170V[a\x19\xECV[a\x12\x14\x83\x86a\x1A\x13V[\x90a\n\xFEV[\x90a\rqV[\x95\x01\x92\x91\x90a\x11\xA0V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x12E\x90a\n\xC2V[\x92a\x11\x1BV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x10\xF7V[P`\0\x90V[\x90a\x12\x90\x90a\x13yV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\n\xB0Wa\x01\xFA\x91a\x13-V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xDAW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xDAW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x14 W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x14\x13W[e\x01\0\0\0\0\0\x81\x10\x15a\x14\x06W[c\x01\0\0\0\x81\x10\x15a\x13\xF9W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x13\xBDV[` \x1C\x91`\x10\x1B\x91a\x13\xB0V[`@\x1C\x91` \x1B\x91a\x13\xA1V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x13\x89V[\x15a\x14>WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x14\x9B`\0\x82\x13a\x147V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x14\xB7\x82a\x1A_V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[\x80\x15a\x17#WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x12\x80WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x17\x16W`\0a\x17\x06a\x16E\x83a\x1A\xD1V[a\x16\xCEa\x12\0a\x16_a\x16Za\x11\x86\x85a\x13NV[a\x19[V[\x92a\x0E\na\x17\x01a\x16\xFCa\x16\xF5a\x16\xEFa\x16\xEAa\x16\xE4a\x16\xDFa\x16\xD9a\x16\xD4\x8Da\x16\xCEa\x16\xC9a\x16\xC3a\x16\xBEa\x11\x80a\x16\xB9a\x16\xB3a\x16\xAEa\x16\xA8a\x16\xA3\x8Aa\x1A4V[a\x0C\xABV[\x89a\x1A\x13V[a\x0C\xC5V[\x87a\x1A\x13V[a\x0C\xDDV[a\x0C\xF7V[\x83a\x1A\x13V[a\r\x0FV[\x90a\x1A\x13V[a\r)V[\x8Ca\x1A\x13V[a\rAV[\x8Aa\x1A\x13V[a\rYV[\x88a\x1A\x13V[\x93\x80a\x1A\x13V[a\x10\x92V[a\n\xE5V[\x91\x12\x15a\x01\xFAWa\x01\xFA\x90a\n\xC2V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x12\x80Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x18\x80We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xDAWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xDAW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1Aj\x81\x15\x15a\x147V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\xFF\x1B\x81\x14a\x1A\xECW`\0\x81\x12\x15a\x01\xFAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xF2\x0B\x1C\x81\xB7F\x92\xEE\xD9}\xD37= 7\x1C\0\xEE\x13\xD0c\x1F\xC3\xCF)?1\xDB\xC8\xB1)\xBEdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static LOGNORMAL_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0\x12W`\0\x80\xFD[`\0\x805`\xE0\x1C\x90\x81b.RK\x14a\0\xA9WP\x80c\x06\xFD\xDE\x03\x14a\0\xA4W\x80c\x1E\xDBq\xE5\x14a\0\x9FW\x80ch\xBD>8\x14a\0\x9AW\x80cs\xCB-\x03\x14a\0\x95W\x80c\x8A\x04\xBD\xD5\x14a\0\x90W\x80c\xAC\xAD)\x89\x14a\0\x8BW\x80c\xAF\xBA\x13\xC4\x14a\0\x86Wc\xDC\x17\x83U\x14a\0\x81W`\0\x80\xFD[a\t\xE4V[a\t\xBBV[a\x08\x14V[a\x07\xDAV[a\x06tV[a\x03\xDCV[a\x02\xE1V[a\x02=V[4a\x01\x1BW`@6`\x03\x19\x01\x12a\x01\x1BW`$5\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01\x1BW` a\x01\x13a\0\xF0a\0\xE26`\x04\x87\x01a\x01\xDFV[\x83\x80\x82Q\x83\x01\x01\x91\x01a\n\x17V[\x90a\x01\ra\0\xFF`\x045a\x0B\x92V[\x86\x80\x82Q\x83\x01\x01\x91\x01a\n2V[\x92a\r\x8DV[`@Q\x90\x81R\xF3[\x80\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\xA0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[a\x01\x1EV[`\xC0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@RV[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x01PW`@Q\x91a\x01\xBD`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x01qV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x01\xDAW\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[`\0\x80\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x01\xDAW\x81` a\x01\xFA\x935\x91\x01a\x01\x93V[\x90V[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x02)WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x02\x08V[4a\x01\xDAW`\x006`\x03\x19\x01\x12a\x01\xDAW`@Q`@\x81\x01\x90\x80\x82\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11\x17a\x01PWa\x02\x9A\x91`@R`\t\x81Rh\x13\x1B\xD9\xD3\x9B\xDC\x9BX[`\xBA\x1B` \x82\x01R`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[\x03\x90\xF3[\x90`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01PW`@R```\x03\x82\x94\x80T\x84R`\x01\x81\x01T` \x85\x01R`\x02\x81\x01T`@\x85\x01R\x01T\x91\x01RV[4a\x01\xDAW` 6`\x03\x19\x01\x12a\x01\xDAW`\x045`\0R`\x01` Ra\x01\xC0`@`\0 a\x03\x0E\x81a\x02\x9EV[\x90a\x03\x1B`\x04\x82\x01a\x02\x9EV[\x90a\x03\xBDa\x03+`\x08\x83\x01a\x02\x9EV[a\x03\x93`\x0C\x84\x01T\x93`\r`\x01\x80`\xA0\x1B\x03\x91\x01T\x16\x94a\x03m`@Q\x80\x98``\x80\x91\x80Q\x84R` \x81\x01Q` \x85\x01R`@\x81\x01Q`@\x85\x01R\x01Q\x91\x01RV[\x80Q`\x80\x88\x01R` \x81\x01Q`\xA0\x88\x01R`@\x81\x01Q`\xC0\x88\x01R``\x01Q`\xE0\x87\x01RV[\x80Qa\x01\0\x86\x01R` \x81\x01Qa\x01 \x86\x01R`@\x81\x01Qa\x01@\x86\x01R``\x01Qa\x01`\x85\x01RV[a\x01\x80\x83\x01Ra\x01\xA0\x82\x01R\xF3[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x01\xDAWV[4a\x01\xDAW``\x80`\x03\x196\x01\x12a\x01\xDAWa\x03\xF9`\x045a\x03\xCBV[`$5`D5\x91g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x11a\x01\xDAWa\x04 a\x04\x82\x936\x90`\x04\x01a\x01\xDFV[\x81a\x04<a\x04-\x85a\x0B\x92V[` \x80\x82Q\x83\x01\x01\x91\x01a\n2V[`\0T\x90\x94\x90a\x04b\x90a\x04V\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q\x80\x80\x98\x81\x94c3\x85N\xFD`\xE2\x1B\x83R`\x04\x83\x01\x91\x90` \x83\x01\x92RV[\x03\x91Z\xFA\x80\x15a\x06\x14W`\0\x91\x82\x95\x83\x92a\x05\xD9W[P\x80` \x80a\x04\xAC\x93Q\x83\x01\x01\x91\x01a\n\x17V[\x93\x91\x94\x90\x96\x82\x86\x11`\0\x14a\x05LWP\x91a\x04\xED\x82a\x04\xE8\x83a\x04\xE3a\x04\xFE\x97a\x04\xD9a\x04\xF5\x98\x8Ca\n\xA3V[\x90\x8C\x01Q\x90a\x12\xAFV[a\x12\xAFV[a\x12\xDBV[P[\x83a\n\xFEV[\x93\x82\x86\x85a\r\x8DV[\x93\x84`\x13\x19\x12\x92\x83a\x05AW[a\x02\x9A\x93\x94`@Q\x96\x87\x96\x87\x92`\xA0\x94\x91\x97\x96\x95\x92`\xC0\x85\x01\x98\x15\x15\x85R` \x85\x01R`@\x84\x01R``\x83\x01R`\x80\x82\x01R\x01RV[`\x14\x86\x12\x93Pa\x05\x0BV[\x90\x92\x90\x91P\x81\x87\x11\x15a\x05{Wa\x05u\x82a\x04\xE8\x83a\x04\xE3a\x04\xFE\x97a\x04\xD9a\x04\xF5\x98\x8Ea\n\xA3V[Pa\x04\xEFV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`0`$\x82\x01R\x7Finvalid swap: inputs x and y hav`D\x82\x01Roe the same sign!`\x80\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x95Pa\x04\xAC\x92Pa\x06\x01\x91P\x83=\x85\x11a\x06\rW[a\x05\xF9\x81\x83a\x01qV[\x81\x01\x90a\n\x17V[\x95\x91\x92\x90\x95\x91\x90a\x04\x98V[P=a\x05\xEFV[a\n\x81V[\x90```\x03\x19\x83\x01\x12a\x01\xDAW`\x045a\x062\x81a\x03\xCBV[\x91`$5\x91`D5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x01\xDAW\x80`#\x83\x01\x12\x15a\x01\xDAW\x81`\x04\x015\x93\x84\x11a\x01\xDAW`$\x84\x83\x01\x01\x11a\x01\xDAW`$\x01\x91\x90V[4a\x01\xDAWa\x06\x826a\x06\x19V[`\0T\x92\x93P\x91`\x01`\x01`\xA0\x1B\x03\x163\x03a\x07\xC8Wa\x07\x86a\x07~a\x04-a\x07y\x95a\x06\xC0`\x80\x95a\x07C\x97a\x06\xB7a\x0BgV[P\x81\x01\x90a\x0E\x8CV[` \x81\x9B\x94\x9A\x93\x99\x92\x9B\x01Qa\x06\xE0\x86`\0R`\x01` R`@`\0 \x90V[U`@\x81\x01Q`\x04a\x06\xFC\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x80Q`\x08a\x07\x16\x87`\0R`\x01` R`@`\0 \x90V[\x01U``\x81\x01Q`\x0Ca\x073\x87`\0R`\x01` R`@`\0 \x90V[\x01U\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\ra\x07Y\x84`\0R`\x01` R`@`\0 \x90V[\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90UV[a\x0B\x92V[\x84\x83\x85a\r\x8DV[\x92\x83`\x13\x19\x12\x91\x82a\x07\xBDW[`@\x80Q\x93\x15\x15\x84R` \x84\x01\x95\x90\x95R\x93\x82\x01\x92\x90\x92R``\x81\x01\x92\x90\x92R`\x80\x82\x01R`\xA0\x90\xF3[`\x14\x85\x12\x92Pa\x07\x93V[`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x90\xFD[4a\x01\xDAW``a\x07\xEA6a\x06\x19V[\x81\x80\x94P\x94\x92\x94\x01\x03\x12a\x01\xDAW\x805\x90a\x07\x86a\x07~a\x04-`@` \x85\x015\x94\x015\x95a\x0B\x92V[4a\x01\xDAWa\x08\"6a\x06\x19V[`\0T\x90\x93\x90`\x01`\x01`\xA0\x1B\x03\x90\x81\x163\x03a\x07\xC8Wa\x08ca\x04V`\ra\x08U\x87`\0R`\x01` R`@`\0 \x90V[\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x91\x16\x03a\t\xA9Wa\x08v\x83\x82\x01\x82a\x0B2V[a\x08\x7F\x81a\x0BGV[`\x01\x81\x03a\x08\xB8WPa\x08\xA2a\x08\x9Da\x08\xB3\x92`\x0C\x94\x956\x91a\x01\x93V[a\x0F\xC3V[\x92`\0R`\x01` R`@`\0 \x90V[\x01U[\0[a\x08\xC1\x81a\x0BGV[`\x03\x81\x03a\x08\xFDWP\x90a\x08\xE5a\x08\xE0a\x08\xF8\x93a\x08\xB6\x956\x91a\x01\x93V[a\x0F\x19V[\x92\x90\x91`\0R`\x01` R`@`\0 \x90V[a\x0FAV[a\t\x06\x81a\x0BGV[`\x04\x81\x03a\t@WP\x90`\x04a\t'a\x08\xE0a\t:\x94a\x08\xB6\x966\x91a\x01\x93V[\x93\x90\x92`\0R`\x01` R`@`\0 \x90V[\x01a\x0FAV[a\tI\x81a\x0BGV[`\x02\x81\x03a\tjWP\x90`\x08a\t'a\x08\xE0a\t:\x94a\x08\xB6\x966\x91a\x01\x93V[\x80a\tv`\x05\x92a\x0BGV[\x03a\t\x97Wa\x07Ya\x08\xA2a\t\x92`\r\x93a\x08\xB6\x966\x91a\x01\x93V[a\x0E\xF1V[`@Qc#]+=`\xE0\x1B\x81R`\x04\x90\xFD[`@Qcn\xDA\xEF/`\xE1\x1B\x81R`\x04\x90\xFD[4a\x01\xDAW`\x006`\x03\x19\x01\x12a\x01\xDAW`\0T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[4a\x01\xDAW` 6`\x03\x19\x01\x12a\x01\xDAWa\x02\x9Aa\n\x03`\x045a\x0B\x92V[`@Q\x91\x82\x91` \x83R` \x83\x01\x90a\x01\xFDV[\x90\x81``\x91\x03\x12a\x01\xDAW\x80Q\x91`@` \x83\x01Q\x92\x01Q\x90V[\x90\x81`\xA0\x91\x03\x12a\x01\xDAW`\x80`@Q\x91a\nL\x83a\x014V[\x80Q\x83R` \x81\x01Q` \x84\x01R`@\x81\x01Q`@\x84\x01R``\x81\x01Q``\x84\x01R\x01Qa\ny\x81a\x03\xCBV[`\x80\x82\x01R\x90V[`@Q=`\0\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\n\xB0WV[a\n\x8DV[\x91\x90\x82\x01\x80\x92\x11a\n\xB0WV[\x90g\x1B\xC1mgN\xC8\0\0`\0\x83\x82\x03\x93\x12\x81\x84\x12\x81\x16\x91\x84\x13\x90\x15\x16\x17a\n\xB0WV[\x90g\x11\x90\0\xAB\x10\x0F\xFB\xFF\x19\x82\x01\x91\x82\x13`\x01\x16a\n\xB0WV[\x81\x81\x03\x92\x91`\0\x13\x80\x15\x82\x85\x13\x16\x91\x84\x12\x16\x17a\n\xB0WV[`\x01`\xFF\x1B\x81\x14a\n\xB0W`\0\x03\x90V[`\x06\x11\x15a\x01\xDAWV[\x90\x81` \x91\x03\x12a\x01\xDAW5a\x01\xFA\x81a\x0B(V[`\x06\x11\x15a\x0BQWV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`@Q\x90a\x0Bt\x82a\x014V[`\0`\x80\x83\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x01RV[a\x0B\x9Aa\x0BgV[\x81`\0R`\x01` Ra\x0B\xB8a\x0B\xB3`@`\0 a\x02\x9EV[a\x0F\xF8V[\x91` \x82\x01\x92\x83R\x80`\0R`\x01` Ra\x0B\xDCa\x0B\xB3`\x08`@`\0 \x01a\x02\x9EV[\x82R`\x0Ca\x0C\x1Da\x0C\x05a\x0B\xB3`\x04a\x0B\xFF\x86`\0R`\x01` R`@`\0 \x90V[\x01a\x02\x9EV[\x92`@\x85\x01\x93\x84R`\0R`\x01` R`@`\0 \x90V[\x01T\x90``\x83\x01\x91\x82R`@Q\x93\x83Q` \x86\x01RQ`@\x85\x01RQ``\x84\x01RQ`\x80\x83\x01R`\x80`\x01\x80`\xA0\x1B\x03\x91\x01Q\x16`\xA0\x82\x01R`\xA0\x81Ra\x01\xFA\x81a\x01UV[\x90\x81g \x05\xFEO&\x8E\xA0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xC5R\x7Fd, \0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xE0\xB6\xB3\xA7d\0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x14\xA8EL\x19\xE1\xAC\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x0F\xC1\x0E\x01W\x82w\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x03\xDE\xBD\x08;\x8C|\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x02\x95\xD4\0\xEA2W\xFF\x19\x01\x91\x82\x12\x15`\x01\x16a\n\xB0WV[\x90\x81g\x01W\xD8\xB2\xEC\xC7\x08\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\x051\n\xA7\xD5!0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x90\x81g\r\xE0\xCC=\x15a\0\0\x01\x91\x82\x12`\x01\x16a\n\xB0WV[\x91\x90\x91`\0\x83\x82\x01\x93\x84\x12\x91\x12\x90\x80\x15\x82\x16\x91\x15\x16\x17a\n\xB0WV[\x90\x92\x82\x82\x10\x15a\x0EGWa\x01\xFA\x93a\x0E\n\x92\x84g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82a\r\xB5\x83\x83a\x13\x0BV[\x10a\x0E4WP`\x01`\x01`\xFF\x1B\x03\x95\x90P[\x83Q\x91a\r\xDDa\r\xD7\x83\x85a\x13-V[\x85a\x13\x0BV[\x10a\x0E\x0FWP`\x01`\x01`\xFF\x1B\x03\x92a\x0E\x04\x92P\x90P[`@` \x82\x01Q\x91\x01Q\x90a\x12\x86V[\x92a\rqV[a\rqV[a\x0E\x04\x92a\x0E#a\x0E)\x92a\x0E.\x94a\x13-V[\x90a\x13\x0BV[a\x10\xB3V[\x91a\r\xF4V[a\x0EA\x91a\x0E)\x91a\x13\x0BV[\x94a\r\xC7V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FtradingFunction: invalid x\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x80\x91\x03a\x01\0\x81\x12a\x01\xDAW\x815\x92` \x83\x015\x92`\xA0`@\x82\x015\x93`_\x19\x01\x12a\x01\xDAW`\xE0`@Q\x91a\x0E\xC1\x83a\x014V[``\x81\x015\x83R`\x80\x81\x015` \x84\x01R`\xA0\x81\x015`@\x84\x01R`\xC0\x81\x015``\x84\x01R\x015a\ny\x81a\x03\xCBV[`@\x81\x80Q\x81\x01\x03\x12a\x01\xDAW\x80a\x0F\x0E` `@\x93\x01Qa\x0B(V[\x01Qa\x04V\x81a\x03\xCBV[``\x81\x80Q\x81\x01\x03\x12a\x01\xDAWa\x0F3` \x82\x01Qa\x0B(V[```@\x82\x01Q\x91\x01Q\x90\x91V[\x91\x90B\x82\x11\x15a\x0F\xB1Wa\x0FWa\x0B\xB3\x84a\x02\x9EV[\x90\x81\x84UB`\x03\x85\x01UB\x83\x03\x91\x83\x83\x11a\n\xB0Wa\x0Fu\x91a\n\xFEV[B\x83\x14a\x0F\x9BW`\x01`\xFF\x1B\x81\x14`\0\x19\x83\x14\x16a\n\xB0W`\x02\x92`\x01\x85\x01U\x05\x91\x01UV[cNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`@Qcf\xF1\x02\xED`\xE1\x1B\x81R`\x04\x90\xFD[`@\x81\x80Q\x81\x01\x03\x12a\x01\xDAW\x80a\x0F\xE0` `@\x93\x01Qa\x0B(V[\x01Q\x90V[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\n\xB0WV[``\x81\x01Q` \x82\x01Q\x80\x82\x14a\x10sW\x80B\x11`\0\x14a\x10kW\x90[\x81\x03\x90\x81\x11a\n\xB0W`@\x82\x01\x90\x81Q`\0\x81\x13`\0\x14a\x10HWPa\x10B\x90a\x01\xFA\x93Q\x92Q\x90a\x0F\xE5V[\x90a\n\xB5V[\x92a\x10_\x92Pa\x10Y\x90Q\x93a\x0B\x17V[\x90a\x0F\xE5V[\x81\x03\x90\x81\x11a\n\xB0W\x90V[PB\x90a\x10\x15V[PPQ\x90V[\x90c;\x9A\xCA\0\x91\x82\x81\x02\x92\x81\x84\x05\x14\x90\x15\x17\x15a\n\xB0WV[`\0\x81\x90\x03\x91\x90`\x01`\xFF\x1B\x81\x14`\x01\x16a\n\xB0W`\0\x19\x83\x05\x03a\n\xB0WV[g\x06\xF0[Y\xD3\xB2\0\0\x81\x14a\x12\x80Wg\r\xE0\xB6\xB3\xA7d\0\0\x80\x82\x12\x15a\x12*W\x81\x15a\x12KW`\x01\x82`\x01\x1B\x91`\x02\x93\x83\x05`\x02\x03a\n\xB0W`\0\x83\x12\x80\x15a\x12oW[a\x12]W\x82\x15a\x12*Wg\x1B\xC1mgN\xC8\0\0\x83\x14a\x12KW\x82\x12\x91\x82\x15a\x12<W\x92[a\x11$\x84a\x18\xDBV[\x80\x15a\x12*Wa\x11\x96a\x11Ua\x11Pa\x11Ka\x11Fa\x11\x9B\x95\x99\x97\x96\x99a\x14oV[a\x19\x9CV[a\x13yV[a\x10yV[a\x11\x91a\x11ia\x11d\x83a\x19\x06V[a\x0CcV[a\x11\x8Ba\x11\x86a\x11\x80a\x11{\x86a\x191V[a\x0C{V[\x85a\x1A\x13V[a\x0C\x93V[\x90a\x19zV[a\n\xFEV[a\x19\xC4V[\x93`\0\x92[\x81\x84\x10a\x11\xD2WPPPPa\x01\xFA\x91a\x11\xBF\x91`\0\x14a\x11\xC4Wa\x18\xB4V[a\x0B\x17V[a\x11\xCD\x90a\x0B\x17V[a\x18\xB4V[\x90\x91a\x12 \x86a\x12\x1Aa\x11\xEA\x85a\x11\x91\x86\x99\x9Ba\x16\x10V[a\x11\x8Ba\x12\na\x12\x05a\x12\0a\x11\xBF\x87\x80a\x1A\x13V[a\x170V[a\x19\xECV[a\x12\x14\x83\x86a\x1A\x13V[\x90a\n\xFEV[\x90a\rqV[\x95\x01\x92\x91\x90a\x11\xA0V[`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x90\xFD[a\x12E\x90a\n\xC2V[\x92a\x11\x1BV[`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x90\xFD[`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x90\xFD[Pg\x1B\xC1mgN\xC8\0\0\x83\x13a\x10\xF7V[P`\0\x90V[\x90a\x12\x90\x90a\x13yV[c;\x9A\xCA\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x15a\n\xB0Wa\x01\xFA\x91a\x13-V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAW`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x01\xDAW`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xDAW\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\x1B\xC1mgN\xC8\0\0\x90\x04\x90V[`\xB5\x81`\x01`\x88\x1B\x81\x10\x15a\x14 W[\x80i\x01\0\0\0\0\0\0\0\0\0b\x01\0\0\x92\x10\x15a\x14\x13W[e\x01\0\0\0\0\0\x81\x10\x15a\x14\x06W[c\x01\0\0\0\x81\x10\x15a\x13\xF9W[\x01\x02`\x12\x1C`\x01\x90\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x80\x80\x92\x04\x10\x90\x03\x90V[`\x10\x1C\x91`\x08\x1B\x91a\x13\xBDV[` \x1C\x91`\x10\x1B\x91a\x13\xB0V[`@\x1C\x91` \x1B\x91a\x13\xA1V[Ph\xB5\0\0\0\0\0\0\0\0\x90P`\x80\x82\x90\x1Ca\x13\x89V[\x15a\x14>WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x90\xFD[}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x90a\x14\x9B`\0\x82\x13a\x147V[q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06a\x14\xB7\x82a\x1A_V[``\x92\x83\x82`\x9F\x03\x01\x1B`\x9F\x1C\x90`_\x19\x01}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x02\x92l\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x82m\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x81m\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x81m\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x81m\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x81m\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x81lFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x02\x8C\x1D\x01\x02\x8A\x1D\x01\x02\x88\x1D\x01\x02\x86\x1D\x01\x02\x84\x1D\x01\x02\x82\x1D\x01\x91x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x91l\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x90\x82m\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x81l\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x81m\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x81m\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x81l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x81\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x02\x83\x1D\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x01\x01`\xAE\x1D\x90V[\x80\x15a\x17#WgV\x98\xEE\xF0fp\0\0\x81\x12\x15a\x12\x80WgV\x98\xEE\xF0fo\xFF\xFF\x19\x81\x13\x15a\x17\x16W`\0a\x17\x06a\x16E\x83a\x1A\xD1V[a\x16\xCEa\x12\0a\x16_a\x16Za\x11\x86\x85a\x13NV[a\x19[V[\x92a\x0E\na\x17\x01a\x16\xFCa\x16\xF5a\x16\xEFa\x16\xEAa\x16\xE4a\x16\xDFa\x16\xD9a\x16\xD4\x8Da\x16\xCEa\x16\xC9a\x16\xC3a\x16\xBEa\x11\x80a\x16\xB9a\x16\xB3a\x16\xAEa\x16\xA8a\x16\xA3\x8Aa\x1A4V[a\x0C\xABV[\x89a\x1A\x13V[a\x0C\xC5V[\x87a\x1A\x13V[a\x0C\xDDV[a\x0C\xF7V[\x83a\x1A\x13V[a\r\x0FV[\x90a\x1A\x13V[a\r)V[\x8Ca\x1A\x13V[a\rAV[\x8Aa\x1A\x13V[a\rYV[\x88a\x1A\x13V[\x93\x80a\x1A\x13V[a\x10\x92V[a\n\xE5V[\x91\x12\x15a\x01\xFAWa\x01\xFA\x90a\n\xC2V[Pg\x1B\xC1mgN\xC8\0\0\x90V[Pg\r\xE0\xB6\xB3\xA7d\0\0\x90V[h\x02H\xCE6\xA7\x0C\xB2k>\x19\x81\x13\x15a\x12\x80Wh\x07U\xBFy\x8BJ\x1B\xF1\xE5\x81\x12\x15a\x18\x80We\x03x-\xAC\xE9\xD9\x90`N\x1B\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x91``\x90`\x01`_\x1B\x84\x82\x84\x1B\x05\x01\x82\x1D\x93\x84\x02\x90\x03\x80l\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x81\x01\x02\x82\x1D\x90n\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dPn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x82n\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x81m\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x81m\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x81m\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x81l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x02\x8D\x1D\x01\x02\x8B\x1D\x01\x02\x89\x1D\x01\x02\x87\x1D\x01\x02\x85\x1D\x01\x93m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x93m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEA\x81\x01\x90\x84m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x91\x01\x01\x02\x90\x1D\x01\x02\x01\x05\x02\x90`\xC3\x03\x1C\x90V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x90\xFD[g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\x1B\xC1mgN\xC8\0\0\x90\x05\x90V[g\x03\xC1f\\z\xAB \0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[f\x9F2u$b\xA0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[`\x01\x81\x15\x15\x16\x15a\x01\xDAWn\xC0\x97\xCE{\xC9\x07\x15\xB3K\x9F\x10\0\0\0\0\x05\x90V[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x01\xDAW\x05\x90V[g\x1B\xC1mgN\xC7\xFF\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\t\xD0(\xCCo _\xFF\x19\x81\x81\x02\x91`\x01\x91\x83\x05\x14\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x90\x80\x82\x02\x91\x82\x05\x14`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[g\x02_\x0F\xE1\x05\xA3\x14\0\x90\x81\x81\x02\x91\x81\x83\x05\x14\x90\x15\x17`\x01\x16\x15a\x01\xDAWg\r\xE0\xB6\xB3\xA7d\0\0\x90\x05\x90V[a\x1Aj\x81\x15\x15a\x147V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x07\x1B\x81\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x81\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x81\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x81\x81\x1C`\xFF\x10`\x03\x1B\x17\x81\x81\x1C`\x0F\x10`\x02\x1B\x17\x81\x81\x1C`\x03\x10`\x01\x1B\x17\x90\x81\x1C`\x01\x10\x17\x90V[`\x01`\xFF\x1B\x81\x14a\x1A\xECW`\0\x81\x12\x15a\x01\xFAW\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD\xFE\xA2dipfsX\"\x12 \xF2\x0B\x1C\x81\xB7F\x92\xEE\xD9}\xD37= 7\x1C\0\xEE\x13\xD0c\x1F\xC3\xCF)?1\xDB\xC8\xB1)\xBEdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static LOGNORMAL_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct LogNormal<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LogNormal<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LogNormal<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LogNormal<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LogNormal<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(LogNormal))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LogNormal<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                LOGNORMAL_ABI.clone(),
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
                LOGNORMAL_ABI.clone(),
                LOGNORMAL_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `computeSwapConstant` (0x002e524b) function
        pub fn compute_swap_constant(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([0, 46, 82, 75], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `dfmm` (0xafba13c4) function
        pub fn dfmm(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([175, 186, 19, 196], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPoolParams` (0xdc178355) function
        pub fn get_pool_params(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
            self.0
                .method_hash([220, 23, 131, 85], pool_id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `init` (0x73cb2d03) function
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
        /// Calls the contract's `internalParams` (0x1edb71e5) function
        pub fn internal_params(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                DynamicParam,
                DynamicParam,
                DynamicParam,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([30, 219, 113, 229], p0)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `update` (0xacad2989) function
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
        /// Calls the contract's `validateAllocateOrDeallocate` (0x8a04bdd5)
        /// function
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
        /// Calls the contract's `validateSwap` (0x68bd3e38) function
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
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for LogNormal<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
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
    /// Custom Error type `InvalidSender` with signature `InvalidSender()` and
    /// selector `0xddb5de5e`
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
    #[etherror(name = "InvalidSender", abi = "InvalidSender()")]
    pub struct InvalidSender;
    /// Custom Error type `InvalidUpdateCode` with signature
    /// `InvalidUpdateCode()` and selector `0x235d2b3d`
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
    #[etherror(name = "InvalidUpdateCode", abi = "InvalidUpdateCode()")]
    pub struct InvalidUpdateCode;
    /// Custom Error type `InvalidUpdateEnd` with signature `InvalidUpdateEnd()`
    /// and selector `0xcde205da`
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
    #[etherror(name = "InvalidUpdateEnd", abi = "InvalidUpdateEnd()")]
    pub struct InvalidUpdateEnd;
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
    /// Custom Error type `NotDFMM` with signature `NotDFMM()` and selector
    /// `0x6853cba7`
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
    #[etherror(name = "NotDFMM", abi = "NotDFMM()")]
    pub struct NotDFMM;
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
    pub enum LogNormalErrors {
        Infinity(Infinity),
        InvalidSender(InvalidSender),
        InvalidUpdateCode(InvalidUpdateCode),
        InvalidUpdateEnd(InvalidUpdateEnd),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        NotDFMM(NotDFMM),
        OutOfBounds(OutOfBounds),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LogNormalErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) = <InvalidSender as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSender(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateCode as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateCode(decoded));
            }
            if let Ok(decoded) = <InvalidUpdateEnd as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidUpdateEnd(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <NegativeInfinity as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NegativeInfinity(decoded));
            }
            if let Ok(decoded) = <NotDFMM as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotDFMM(decoded));
            }
            if let Ok(decoded) = <OutOfBounds as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OutOfBounds(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSender(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateCode(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidUpdateEnd(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotDFMM(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LogNormalErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidSender as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidUpdateCode as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidUpdateEnd as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <NotDFMM as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LogNormalErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSender(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidUpdateEnd(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotDFMM(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LogNormalErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<Infinity> for LogNormalErrors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<InvalidSender> for LogNormalErrors {
        fn from(value: InvalidSender) -> Self {
            Self::InvalidSender(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateCode> for LogNormalErrors {
        fn from(value: InvalidUpdateCode) -> Self {
            Self::InvalidUpdateCode(value)
        }
    }
    impl ::core::convert::From<InvalidUpdateEnd> for LogNormalErrors {
        fn from(value: InvalidUpdateEnd) -> Self {
            Self::InvalidUpdateEnd(value)
        }
    }
    impl ::core::convert::From<Min> for LogNormalErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for LogNormalErrors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<NotDFMM> for LogNormalErrors {
        fn from(value: NotDFMM) -> Self {
            Self::NotDFMM(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for LogNormalErrors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    /// Container type for all input parameters for the `computeSwapConstant`
    /// function with signature `computeSwapConstant(uint256,bytes)` and
    /// selector `0x002e524b`
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
        name = "computeSwapConstant",
        abi = "computeSwapConstant(uint256,bytes)"
    )]
    pub struct ComputeSwapConstantCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `dfmm` function with
    /// signature `dfmm()` and selector `0xafba13c4`
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
    #[ethcall(name = "dfmm", abi = "dfmm()")]
    pub struct DfmmCall;
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
    /// Container type for all input parameters for the `init` function with
    /// signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    #[ethcall(name = "init", abi = "init(address,uint256,bytes)")]
    pub struct InitCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `internalParams`
    /// function with signature `internalParams(uint256)` and selector
    /// `0x1edb71e5`
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
    #[ethcall(name = "internalParams", abi = "internalParams(uint256)")]
    pub struct InternalParamsCall(pub ::ethers::core::types::U256);
    /// Container type for all input parameters for the `name` function with
    /// signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    /// Container type for all input parameters for the `update` function with
    /// signature `update(address,uint256,bytes)` and selector `0xacad2989`
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
    #[ethcall(name = "update", abi = "update(address,uint256,bytes)")]
    pub struct UpdateCall {
        pub sender: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the
    /// `validateAllocateOrDeallocate` function with signature
    /// `validateAllocateOrDeallocate(address,uint256,bytes)` and selector
    /// `0x8a04bdd5`
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
        name = "validateAllocateOrDeallocate",
        abi = "validateAllocateOrDeallocate(address,uint256,bytes)"
    )]
    pub struct ValidateAllocateOrDeallocateCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `validateSwap` function
    /// with signature `validateSwap(address,uint256,bytes)` and selector
    /// `0x68bd3e38`
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
    #[ethcall(name = "validateSwap", abi = "validateSwap(address,uint256,bytes)")]
    pub struct ValidateSwapCall {
        pub p0: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
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
    pub enum LogNormalCalls {
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
    impl ::ethers::core::abi::AbiDecode for LogNormalCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <ComputeSwapConstantCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ComputeSwapConstant(decoded));
            }
            if let Ok(decoded) = <DfmmCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Dfmm(decoded));
            }
            if let Ok(decoded) = <GetPoolParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetPoolParams(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) =
                <InternalParamsCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InternalParams(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) =
                <ValidateAllocateOrDeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateAllocateOrDeallocate(decoded));
            }
            if let Ok(decoded) = <ValidateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ValidateSwap(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LogNormalCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::ComputeSwapConstant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Dfmm(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPoolParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InternalParams(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateAllocateOrDeallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for LogNormalCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ComputeSwapConstant(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<ComputeSwapConstantCall> for LogNormalCalls {
        fn from(value: ComputeSwapConstantCall) -> Self {
            Self::ComputeSwapConstant(value)
        }
    }
    impl ::core::convert::From<DfmmCall> for LogNormalCalls {
        fn from(value: DfmmCall) -> Self {
            Self::Dfmm(value)
        }
    }
    impl ::core::convert::From<GetPoolParamsCall> for LogNormalCalls {
        fn from(value: GetPoolParamsCall) -> Self {
            Self::GetPoolParams(value)
        }
    }
    impl ::core::convert::From<InitCall> for LogNormalCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<InternalParamsCall> for LogNormalCalls {
        fn from(value: InternalParamsCall) -> Self {
            Self::InternalParams(value)
        }
    }
    impl ::core::convert::From<NameCall> for LogNormalCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for LogNormalCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<ValidateAllocateOrDeallocateCall> for LogNormalCalls {
        fn from(value: ValidateAllocateOrDeallocateCall) -> Self {
            Self::ValidateAllocateOrDeallocate(value)
        }
    }
    impl ::core::convert::From<ValidateSwapCall> for LogNormalCalls {
        fn from(value: ValidateSwapCall) -> Self {
            Self::ValidateSwap(value)
        }
    }
    /// Container type for all return fields from the `computeSwapConstant`
    /// function with signature `computeSwapConstant(uint256,bytes)` and
    /// selector `0x002e524b`
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
    pub struct ComputeSwapConstantReturn(pub ::ethers::core::types::I256);
    /// Container type for all return fields from the `dfmm` function with
    /// signature `dfmm()` and selector `0xafba13c4`
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
    pub struct DfmmReturn(pub ::ethers::core::types::Address);
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
    pub struct GetPoolParamsReturn(pub ::ethers::core::types::Bytes);
    /// Container type for all return fields from the `init` function with
    /// signature `init(address,uint256,bytes)` and selector `0x73cb2d03`
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
    pub struct InitReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `internalParams` function
    /// with signature `internalParams(uint256)` and selector `0x1edb71e5`
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
    pub struct InternalParamsReturn {
        pub sigma: DynamicParam,
        pub tau: DynamicParam,
        pub strike: DynamicParam,
        pub swap_fee: ::ethers::core::types::U256,
        pub controller: ::ethers::core::types::Address,
    }
    /// Container type for all return fields from the `name` function with
    /// signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn(pub ::std::string::String);
    /// Container type for all return fields from the
    /// `validateAllocateOrDeallocate` function with signature
    /// `validateAllocateOrDeallocate(address,uint256,bytes)` and selector
    /// `0x8a04bdd5`
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
    pub struct ValidateAllocateOrDeallocateReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `validateSwap` function
    /// with signature `validateSwap(address,uint256,bytes)` and selector
    /// `0x68bd3e38`
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
    pub struct ValidateSwapReturn {
        pub valid: bool,
        pub invariant: ::ethers::core::types::I256,
        pub liquidity_delta: ::ethers::core::types::I256,
        pub next_rx: ::ethers::core::types::U256,
        pub next_ry: ::ethers::core::types::U256,
        pub next_l: ::ethers::core::types::U256,
    }
}

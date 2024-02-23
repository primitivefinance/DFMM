pub use dfmm::*;
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
pub mod dfmm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("weth_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allocate"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deallocate"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getPool"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getReservesAndLiquidity"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "getReservesAndLiquidity",
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
                            ],
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct IDFMM.InitParams"),
                                    ),
                                },
                            ],
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityOf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("liquidityOf"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
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
                            ],
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
                    ::std::borrow::ToOwned::to_owned("lpTokenImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lpTokenImplementation",
                            ),
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
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("nonce"),
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
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pools"),
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
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("liquidityToken"),
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
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("swap"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("weth"),
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
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Allocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("deltaL"),
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
                    ::std::borrow::ToOwned::to_owned("Init"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Init"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("strategy"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("lpToken"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
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
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("poolId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("isSwapXForY"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("inputAmount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("outputAmount"),
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
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ERC1167FailedCreateClone",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Invalid"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("negative"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "swapConstantGrowth",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapInputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapInputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapOutputTransfer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSwapOutputTransfer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Locked"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Min"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("Min"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC04b\0\x01mW`\x1Fb\0.i8\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x92`\x01`\x01`@\x1B\x03\x92\x90\x91\x83\x85\x11\x83\x86\x10\x17b\0\x01WW\x81` \x92\x84\x92`@\x97\x88R\x839\x81\x01\x03\x12b\0\x01mWQ`\x01`\x01`\xA0\x1B\x03\x91\x90\x82\x81\x16\x81\x03b\0\x01mW`\x01\x80U`\xA0R\x82Q\x91a\x0F\r\x92\x83\x81\x01\x93\x81\x85\x10\x84\x86\x11\x17b\0\x01WWb\0\x1F\\\x829\x80`\0\x94\x03\x90\x84\xF0\x80\x15b\0\x01MW\x16\x80`\x80R\x80;\x15b\0\x01IW\x90\x82\x80\x92`\x84\x86Q\x80\x96\x81\x93c&lE\xBB`\xE1\x1B\x83R\x89`\x04\x84\x01R\x81`D\x84\x01R```$\x84\x01R\x81`d\x84\x01RZ\xF1\x80\x15b\0\x01?Wb\0\x01\x18W[\x83Qa\x1D\xE9\x90\x81b\0\x01s\x829`\x80Q\x81\x81\x81a\x03|\x01Ra\x0C\x81\x01R`\xA0Q\x81\x81\x81a\t\xC5\x01R\x81\x81a\x11\xD9\x01R\x81\x81a\x14\xB5\x01Ra\x15\xE2\x01R\xF3[\x82\x11b\0\x01+WP\x81R8\x80\x80b\0\0\xDBV[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[\x84Q=\x84\x82>=\x90\xFD[\x82\x80\xFD[\x84Q=\x85\x82>=\x90\xFD[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD\xFE`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a\x11\xD7V[\0[`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xE3W\x80c\x06\x8B\xCD\x8D\x14a\0\xDEW\x80c\x14U\xF1\xFC\x14a\0\xD9W\x80c.\xC3\x81\x88\x14a\0\xD4W\x80c;\xE6\xA3A\x14a\0\xCFW\x80c?\xC8\xCE\xF3\x14a\0\xCAW\x80c\x9D\x94/\x9A\x14a\0\xC5W\x80c\xACJ\xFA8\x14a\0\xC0W\x80c\xAF\xFE\xD0\xE0\x14a\0\xBBW\x80c\xB4b\xCD%\x14a\0\xB6W\x80c\xBD\x06%\xAB\x14a\0\xB1Wc\xCE\x15;\xF4\x03a\0\x0EWa\r\xF4V[a\x0C\xB0V[a\x0CkV[a\x0CMV[a\x0B\xC0V[a\t\xF4V[a\t\xAFV[a\x08tV[a\x06gV[a\x02\x93V[a\x01\xE4V[a\x01:V[`@`\x03\x19\x82\x01\x12a\x015W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x015W\x80`#\x83\x01\x12\x15a\x015W\x81`\x04\x015\x93\x84\x11a\x015W`$\x84\x83\x01\x01\x11a\x015W`$\x01\x91\x90V[`\0\x80\xFD[4a\x015Wa\x01H6a\0\xE8V[\x91\x90`\x01T\x92`\x02`\0\x94\x14a\x01\xD2W\x83\x91`\x02`\x01Ua\x01h\x84a\x0BrV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xCEW\x83a\x01\x9E\x95`@Q\x96\x87\x95\x86\x94\x85\x93c\xAC\xAD)\x89`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xBFV[\x03\x92Z\xF1\x80\x15a\x01\xC9Wa\x01\xBAW[Pa\x01\xB7`\x01\x80UV[\x80\xF3[a\x01\xC3\x90a\x0EhV[8a\x01\xADV[a\x0E\xFAV[\x83\x80\xFD[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[4a\x015W` 6`\x03\x19\x01\x12a\x015W`@Qa\x02\x01\x81a\x0E\x81V[`\xC0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\xE0a\x02?a\x029`\x045a\x0BrV[Pa\x0F\x15V[`@Q\x90`\xC0`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x84R\x82` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q`\xA0\x85\x01R\x01Q\x16`\xC0\x82\x01R\xF3[`\x03\x19` 6\x82\x01\x12a\x015W`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x015W`\x80\x81\x83\x01\x93\x826\x03\x01\x12a\x015W`\x02`\x01T\x14a\x06WW`\x02`\x01U`$\x81\x01\x90a\x02\xDF\x82a\x0F\xA6V[`D\x82\x01a\x02\xFBa\x02\xEF\x82a\x0F\xA6V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x14a\x06FWa\x03\x1Aa\x02\xEFa\x02\xEF\x88a\x0F\xA6V[\x91`\0\x96`\xA0a\x03/`d\x8AT\x97\x01\x83a\x0F\xB3V[\x95`@\x97\x8B\x8Ba\x03T\x8BQ\x9A\x8B\x96\x87\x95\x86\x94cs\xCB-\x03`\xE0\x1B\x86R3\x90\x86\x01a\x0E\xBFV[\x03\x92Z\xF1\x91\x82\x15a\x01\xC9W\x88\x97\x89\x8A\x99\x8B\x97\x8C\x96a\x06\x04W[P\x15a\x05\xCEWPa\x03\xA0a\x02\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12HV[\x91a\x03\xC5a\x03\xAD\x85a\x0F\xA6V[a\x03\xB6\x8Ba\x0F\xA6V[a\x03\xBF\x89a\x0F\xA6V[\x91a\x13\xB1V[\x92\x16\x91\x8A\x83;\x15a\x05\xCBW\x88Qc&lE\xBB`\xE1\x1B\x81R\x91\x82\x90\x81\x90a\x03\xEE\x90\x80\x87\x84\x01a\x10iV[\x03\x81\x83\x87Z\xF1\x80\x15a\x01\xC9Wa\x05\xB8W[Pa\x04\t\x84a\x10\xA4V[\x82;\x15a\x05\xA1W\x87Qc@\xC1\x0F\x19`\xE0\x1B\x80\x82R3\x84\x83\x01\x90\x81R` \x81\x01\x93\x90\x93R\x91\x8C\x90\x82\x90\x81\x90`@\x01\x03\x81\x83\x88Z\xF1\x80\x15a\x01\xC9Wa\x05\xA5W[P\x82;\x15a\x05\xA1W\x99\x80\x91a\x04v\x99\x9A\x9B\x89Q\x9A\x8B\x92\x83\x92\x83R\x82\x01\x90a\x03\xE8` `@\x84\x01\x93`\0\x81R\x01RV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x01\xC9Wa\x05Da\x05Y\x96a\x05I\x8Da\x05Da\x05>\x8F\x97a\x05\x84\x9F\x8F\x99\x8F\x99a\x05N\x9Ba\x04\xC6a\x05T\x9Fa\x05\x0F\x93a\x058\x96a\x04\xC0\x92a\x05\x88W[Pa\x0F\xA6V[\x93a\x0F\xA6V[\x90a\x04\xFEa\x04\xD3\x8Ca\x0F\xA6V[\x92a\x04\xEEa\x04\xDFa\x0F\x06V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x01RV[``\x81\x01\x86\x90R`\x80\x81\x01\x8A\x90R`\xA0\x81\x01\x8E\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\xC0\x82\x01Ra\x10\xD5V[Ta\x10\xB9V[\x9Ea\x0F\xA6V[a\x14\xABV[a\x0F\xA6V[\x87a\x15UV[a\x10\xA4V[\x91a\x05c`\x01\x80UV[Q\x94\x85\x94\x85\x90\x94\x93\x92``\x92`\x80\x83\x01\x96\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x90\xF3[\x80a\x05\x95a\x05\x9B\x92a\x0EhV[\x80a\t\xA4V[8a\x04\xBAV[\x8A\x80\xFD[\x80a\x05\x95a\x05\xB2\x92a\x0EhV[8a\x04GV[\x80a\x05\x95a\x05\xC5\x92a\x0EhV[8a\x03\xFFV[\x80\xFD[a\x06\0\x88\x8C\x93a\x05\xDD\x84a\x12\x1BV[\x91Qcw`m)`\xE1\x1B\x81R\x94\x90\x93\x12\x92\x84\x01\x92\x83R` \x83\x01R\x82\x91`@\x01\x90V[\x03\x90\xFD[\x93\x9APPP\x92Pa\x06.\x91\x94P`\xA0=`\xA0\x11a\x06?W[a\x06&\x81\x83a\x0E\x9DV[\x81\x01\x90a\x0F\xF3V[\x91\x99\x90\x96\x91\x94\x91\x93\x90\x92\x908a\x03mV[P=a\x06\x1CV[`@Qc3\x91\n\xEF`\xE1\x1B\x81R\x85\x90\xFD[P`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R\xFD[a\x06p6a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\x06\xC7\x92`\xA0\x91`\x02`\x01Ua\x06\xA8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90`@Q\x80\x96\x81\x94\x82\x93c\x8A\x04\xBD\xD5`\xE0\x1B\x84R\x883`\x04\x86\x01a\x0E\xBFV[\x03\x91Z\xFA\x90\x81\x15a\x01\xC9W`\0\x90\x81\x82\x80\x95\x81\x95a\x086W[P\x15a\x08\0WP\x90\x81a\x07\x03`\x03a\x06\xFAa\x05\x84\x95a\x0BrV[P\x01T\x83a\x10\xC8V[\x93a\x07\x1B`\x04a\x07\x12\x84a\x0BrV[P\x01T\x87a\x10\xC8V[\x95a\x07*`\x05a\x06\xFA\x85a\x0BrV[\x93a\x075\x85\x85a\x1B\x90V[`\x03a\x07@\x85a\x0BrV[P\x01U`\x04a\x07N\x84a\x0BrV[P\x01U`\x05a\x07\\\x83a\x0BrV[P\x01Ua\x07\x98\x85`\x01a\x07\x84\x87a\x07r\x86a\x0BrV[P\x83\x80`\xA0\x1B\x03\x93\x84\x91\x01T\x16a\x14\xABV[`\x02a\x07\x8F\x85a\x0BrV[P\x01T\x16a\x14\xABV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2`\x01\x80U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x06\0a\x08\x0F`\0\x93a\x12\x1BV[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93PPP\x92Pa\x08U\x91P`\xA0=`\xA0\x11a\x06?Wa\x06&\x81\x83a\x0E\x9DV[\x94\x91\x90\x92\x90\x92\x94\x938a\x06\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x015WV[4a\x015W`@6`\x03\x19\x01\x12a\x015W`\x045a\x08\x91\x81a\x08cV[`$5\x90a\x08\xB8a\x02\xEFa\x02\xEF`\x06a\x08\xA9\x86a\x0BrV[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R` \x80\x83`$\x81\x85Z\xFA\x91\x82\x15a\x01\xC9W`\x04\x93`\0\x93a\t\x83W[P\x81\x90`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x01\xC9Wa\x05\x84\x94a\t=\x94a\t7\x93`\0\x93a\tMW[PPa\t/`\x05\x91a\x0BrV[P\x01Ta\x1D\x14V[\x90a\x1D6V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x05\x92\x93Pa\t/\x91\x81a\tu\x92\x90=\x10a\t|W[a\tm\x81\x83a\x0E\x9DV[\x81\x01\x90a\x11\x93V[\x92\x91a\t\"V[P=a\tcV[\x82\x91\x93Pa\t\x9D\x90\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x90a\x08\xF2V[`\0\x91\x03\x12a\x015WV[4a\x015W`\x006`\x03\x19\x01\x12a\x015W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x015Wa\n\x026a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\n,\x92`\xA0\x91`\x02`\x01Ua\x06\xA8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[\x03\x91Z\xFA\x90\x81\x15a\x01\xC9W`\0\x90\x81\x82\x80\x95\x81\x95a\x0BEW[P\x15a\x08\0WP\x90\x81a\nh\x82`\x03a\n`a\x05\x84\x96a\x0BrV[P\x01Ta\x10\xC8V[\x93a\nx\x86`\x04a\n`\x85a\x0BrV[\x95a\n\x88\x82`\x05a\n`\x86a\x0BrV[\x93a\n\x93\x85\x85a\x1C\\V[`\x03a\n\x9E\x85a\x0BrV[P\x01U`\x04a\n\xAC\x84a\x0BrV[P\x01U`\x05a\n\xBA\x83a\x0BrV[P\x01Ua\n\xFD\x85a\n\xCA\x83a\x0BrV[P`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x90a\n\xE7\x90\x88\x903\x90\x84\x16a\x15\xD8V[a\n\xF0\x84a\x0BrV[P`\x02\x01T3\x91\x16a\x15\xD8V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x07\xDCV[\x93PPP\x92Pa\x0Bd\x91P`\xA0=`\xA0\x11a\x06?Wa\x06&\x81\x83a\x0E\x9DV[\x94\x91\x90\x92\x90\x92\x94\x938a\nEV[\x90`\0\x91\x82T\x81\x10\x15a\x0B\xACW`\x07\x90\x83\x80R\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91\x90V[cNH{q`\xE0\x1B\x83R`2`\x04R`$\x83\xFD[4a\x015W` 6`\x03\x19\x01\x12a\x015W`\x045`\0T\x81\x10\x15a\x015Wa\x0B\xE7\x90a\x0BrV[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R``\x84\x01\x91\x90\x91R`\x80\x83\x01R`\xA0\x82\x01\x93\x90\x93R\x91\x16`\xC0\x82\x01R`\xE0\x90\xF3[4a\x015W`\x006`\x03\x19\x01\x12a\x015W` `\0T`@Q\x90\x81R\xF3[4a\x015W`\x006`\x03\x19\x01\x12a\x015W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x015Wa\x0C\xBE6a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\r\x07\x92`\xC0\x91`\x02`\x01Ua\x0C\xE8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[\x90`@Q\x80\x96\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R\x883`\x04\x86\x01a\x0E\xBFV[\x03\x91Z\xFA\x91\x82\x15a\x01\xC9W`\0\x80\x93\x81\x80\x93\x81\x92a\r\xB7W[P\x15a\r\xA8W\x83\x94P`\x05a\r7a\r@\x95a\x0BrV[P\x01U\x83a\x16\xFEV[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\r\x8B\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\r\x97`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x06\0a\x08\x0F\x82a\x12\x1BV[\x93PPPPa\r\xDF\x91\x92P`\xC0=`\xC0\x11a\r\xEDW[a\r\xD7\x81\x83a\x0E\x9DV[\x81\x01\x90a\x11\xA2V[\x93\x95\x94\x90\x93\x91\x92P8a\r V[P=a\r\xCDV[4a\x015W` 6`\x03\x19\x01\x12a\x015W`\x045`\x03a\x0E\x13\x82a\x0BrV[P\x01Ta\x05\x84`\x05a\x0E2`\x04a\x0E)\x86a\x0BrV[P\x01T\x94a\x0BrV[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E|W`@RV[a\x0ERV[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@RV[\x92\x84\x92`\x80\x95\x92`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R```@\x85\x01R\x81``\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`@Q\x90a\x0F\x13\x82a\x0E\x81V[V[\x90a\x0F\x13`@Qa\x0F%\x81a\x0E\x81V[`\xC0a\x0F\x98`\x06\x83\x96`\x01\x80`\xA0\x1B\x03\x80\x82T\x16\x86R`\x01\x82\x01T\x16` \x86\x01Ra\x0Fla\x0F\\`\x02\x83\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`@\x87\x01RV[`\x03\x81\x01T``\x86\x01R`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[5a\x0F\xB0\x81a\x08cV[\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x015W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x015W` \x01\x91\x816\x03\x83\x13a\x015WV[Q\x90\x81\x15\x15\x82\x03a\x015WV[\x90\x81`\xA0\x91\x03\x12a\x015Wa\x10\x07\x81a\x0F\xE6V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[`\0[\x83\x81\x10a\x104WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x10$V[\x90` \x91a\x10]\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x10!V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x91a\x10\x80a\x0F\xB0\x93`@\x84R`@\x84\x01\x90a\x10DV[\x91` \x81\x84\x03\x91\x01Ra\x10DV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x10\xB4WV[a\x10\x8EV[`\0\x19\x81\x01\x91\x90\x82\x11a\x10\xB4WV[\x91\x90\x82\x03\x91\x82\x11a\x10\xB4WV[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0E|W\x80`\x01a\x10\xF9\x92\x01`\0Ua\x0BrV[a\x11}W\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x90\x93\x01Q`\x06\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x015WQ\x90V[\x91\x90\x82`\xC0\x91\x03\x12a\x015Wa\x11\xB7\x82a\x0F\xE6V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x12\tWV[`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x126W`\0\x81\x12\x15a\x0F\xB0W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x12\x9EWV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD[` \x81\x83\x03\x12a\x015W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x015W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x015W\x81Q\x90\x81\x11a\x0E|W`@Q\x92a\x12\xFB`\x1F\x83\x01`\x1F\x19\x16` \x01\x85a\x0E\x9DV[\x81\x84R` \x82\x84\x01\x01\x11a\x015Wa\x0F\xB0\x91` \x80\x85\x01\x91\x01a\x10!V[a\x0F\x13\x92\x94\x93`(\x92`@Q\x96\x87\x93dDFMM-`\xD8\x1B` \x86\x01Ra\x13J\x81Q\x80\x92` `%\x89\x01\x91\x01a\x10!V[\x84\x01\x91`-`\xF8\x1B\x92\x83`%\x82\x01Ra\x13m\x82Q\x80\x93` `&\x85\x01\x91\x01a\x10!V[\x01\x82`&\x82\x01Ra\x13\x88\x82Q\x80\x93` `'\x85\x01\x91\x01a\x10!V[\x01\x90`'\x82\x01Ra\x13\xA2\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x10!V[\x01\x03`\x08\x81\x01\x85R\x01\x83a\x0E\x9DV[`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x92`\0\x92\x90\x91\x90\x83\x90\x85\x90`\x04\x90\x82\x90\x89\x16Z\xFA\x93\x84\x15a\x01\xC9W\x83\x94a\x14\x8FW[P`@Q\x90\x83\x82`\x04\x81\x89c\x95\xD8\x9BA`\xE0\x1B\x97\x88\x83R\x16Z\xFA\x92\x83\x15a\x01\xC9W\x84\x92\x83\x94a\x14pW[P`\x04\x90`@Q\x97\x88\x93\x84\x92\x83R\x16Z\xFA\x91\x82\x15a\x01\xC9Wa\x0F\xB0\x94\x81\x93a\x14HW[Pa\x14B\x90Ta\x19\x98V[\x92a\x13\x19V[a\x14B\x91\x93Pa\x14i\x90=\x80\x86\x83>a\x14a\x81\x83a\x0E\x9DV[\x81\x01\x90a\x12\xB0V[\x92\x90a\x147V[`\x04\x91\x94Pa\x14\x88\x90=\x80\x86\x83>a\x14a\x81\x83a\x0E\x9DV[\x93\x90a\x14\x14V[a\x14\xA4\x91\x94P=\x80\x85\x83>a\x14a\x81\x83a\x0E\x9DV[\x928a\x13\xEAV[G\x82\x11a\x15,WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x015W`\0\x90`\x04`@Q\x80\x94\x81\x93c\r\x0E0\xDB`\xE4\x1B\x83RZ\xF1\x80\x15a\x01\xC9Wa\x15\x19W[PGa\x15\x0FWV[a\x0F\x13G3a\x1BEV[\x80a\x05\x95a\x15&\x92a\x0EhV[8a\x15\x07V[a\x15Ba\x0F\x13\x92a\x15<\x83a\x1A1V[\x90a\x1DWV[\x900\x903\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1A\xC4V[\x90a\x15ba\x029\x83a\x0BrV[`\x01\x80`\xA0\x1B\x03\x91\x82\x82Q\x16\x91\x83` \x82\x01Q\x16\x93\x80`@\x83\x01Q\x16\x95``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q\x94`@Q\x96\x87R\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E`\xC03\x92\xA4V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x03a\x16fWPP\x82;\x15a\x015W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x92`\0\x90\x84\x90`$\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x01\xC9Wa\x0F\x13\x93a\x16SW[Pa\x1BEV[\x80a\x05\x95a\x16`\x92a\x0EhV[8a\x16MV[` \x92\x94P\x92a\x16ya\x16\x7F\x92\x94a\x1A1V[\x90a\x1D\x14V[`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x16\xBDWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a\x10\xB4WV[\x92\x91\x90`\x03a\x17\x0C\x85a\x0BrV[P\x01T\x92a\x17\x19\x85a\x0BrV[P`\x04\x90\x81\x01T\x93\x85\x84\x11\x91\x90\x82\x15a\x19XW\x85\x81\x10\x15a\x19HW\x81a\x17~a\x17F`\x01a\x08\xA9\x8Ca\x0BrV[\x99a\x17i\x84a\x17ca\x17\\`\x02a\x08\xA9\x86a\x0BrV[\x9C\x8Ba\x10\xC8V[\x9Aa\x10\xC8V[\x97[`\x03a\x17v\x83a\x0BrV[P\x01Ua\x0BrV[P\x01U`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x84\x83\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91` \x91\x90\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x01\xC9W`\0\x95a\x19)W[P\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x91\x8D\x16\x95\x90\x94\x90\x91\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x01\xC9W\x8D\x8F\x8E\x90\x8E\x93`\0\x99a\x18\xFCW[Pa\x18\x12\x93\x92\x91a\x18\x0B\x91a\x14\xABV[3\x90a\x15\xD8V[\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x01\xC9W`\0\x94a\x18\xDBW[P\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x01\xC9W\x8B\x92`\0\x96a\x18\xB4W[PP\x90a\x18{\x91a\x16\xF1V[\x11a\x18\xA5W\x86a\x18\x8A\x91a\x10\xC8V[\x11a\x18\x98WPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x18{\x93\x92\x96P\x90\x81a\x18\xD2\x92\x90=\x10a\t|Wa\tm\x81\x83a\x0E\x9DV[\x94\x90\x918a\x18oV[\x83\x91\x94Pa\x18\xF5\x90\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x93\x90a\x18?V[a\x18\x0B\x91\x99P\x91a\x19\x1Ea\x18\x12\x95\x94\x93\x89=\x8B\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x99\x91P\x91\x92\x93a\x17\xFBV[a\x19A\x91\x95P\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x938a\x17\xC3V[P`@Qc\x11\x15vg`\xE0\x1B\x81R\xFD[\x86\x85\x97\x96\x97\x10\x15a\x19HW\x81a\x17~a\x19u`\x02a\x08\xA9\x8Ca\x0BrV[\x99a\x19\x92\x88a\x17ca\x19\x8B`\x01a\x08\xA9\x86a\x0BrV[\x9C\x87a\x10\xC8V[\x97a\x17kV[\x90\x81\x15a\x19\xDBW`N\x91`@Q\x90\x83\x82R\x80`\x80\x83\x01`@R[a\x19\xC1WP\x82\x01\x91`N\x03\x82RV[\x92`\n\x90\x81\x85\x06`0\x01\x81\x84\x01R`\0\x19\x01\x93\x04\x80a\x19\xB2V[\x90P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`M\x81\x11a\x10\xB4W`\n\n\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x80\x83\x02\x92\x83\x04\x03a\x10\xB4WV[`@Qc1<\xE5g`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xC9W`\0\x91a\x1A\x83W[P`\xFF\x16`\x12\x03`\x12\x81\x11a\x10\xB4Wa\x1A~a\x0F\xB0\x91a\x1A\x0BV[a\x1A\x19V[` \x81=` \x11a\x1A\xBCW[\x81a\x1A\x9C` \x93\x83a\x0E\x9DV[\x81\x01\x03\x12a\x1A\xB8WQ\x90`\xFF\x82\x16\x82\x03a\x05\xCBWP`\xFFa\x1AcV[P\x80\xFD[=\x91Pa\x1A\x8FV[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1B\x0CWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\0\x80\x80\x93\x81\x93Z\xF1\x15a\x1BUWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[a\x1B\xA4a\x02\xEFa\x02\xEF`\x06a\x08\xA9\x85a\x0BrV[\x91`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x01\xC9Wa\x1B\xE6\x93a\t7\x92`\0\x92a\x1C4W[Pa\x1B\xDD`\x05\x91a\x0BrV[P\x01T\x90a\x1D\x14V[\x90\x80;\x15a\x015W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x01\xC9Wa\x1C'WPV[\x80a\x05\x95a\x0F\x13\x92a\x0EhV[`\x05\x91\x92Pa\x1CTa\x1B\xDD\x91` =` \x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x91Pa\x1B\xD1V[a\x1Cpa\x02\xEFa\x02\xEF`\x06a\x08\xA9\x85a\x0BrV[\x91`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x01\xC9Wa\x1C\xB8\x93a\x1C\xB2\x92`\0\x92a\x1C\xECW[Pa\x1C\xA9`\x05\x91a\x0BrV[P\x01T\x90a\x1DWV[\x90a\x1D\x87V[\x90\x80;\x15a\x015W`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a\x1C\x16V[`\x05\x91\x92Pa\x1D\x0Ca\x1C\xA9\x91` =` \x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x91Pa\x1C\x9DV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x015W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x015Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x015W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x015W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 \xA0(RF\xCC\xA8i\xAE\x878\x81\xF8\x0FM\x90E\xEC\xD4L\x1C*!\xC5\x12\xE1\xA0\xAFq|\x94\x91\xFAdsolcC\0\x08\x16\x003`\x80\x80`@R4a\0\x16Wa\x0E\xF1\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x806\x10\x15a\0\x15W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x90\x81c\x06\xFD\xDE\x03\x14a\x0B5WP\x80c\t^\xA7\xB3\x14a\n\xC7W\x80c\x15\x8E\xF9>\x14a\n\xA0W\x80c\x18\x16\r\xDD\x14a\n\x81W\x80c#\xB8r\xDD\x14a\t\xC0W\x80c1<\xE5g\x14a\t\xA4W\x80c6D\xE5\x15\x14a\t\x80W\x80c@\xC1\x0F\x19\x14a\x08\xF9W\x80cL\xD8\x8Bv\x14a\x05\xDEW\x80cp\xA0\x821\x14a\x05\xA6W\x80c~\xCE\xBE\0\x14a\x05nW\x80c\x95\xD8\x9BA\x14a\x04\x88W\x80c\x9D\xC2\x9F\xAC\x14a\x04\x08W\x80c\xA9\x05\x9C\xBB\x14a\x03\x96W\x80c\xAF\xBA\x13\xC4\x14a\x03mW\x80c\xD5\x05\xAC\xCF\x14a\x01)Wc\xDDb\xED>\x14a\0\xDEW`\0\x80\xFD[4a\x01%W\x81`\x03\x196\x01\x12a\x01%W` \x92\x82\x91a\0\xFBa\x0C\x81V[a\x01\x03a\x0C\x9CV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x84R\x91\x86R\x83\x83 \x91\x16\x82R\x84R T\x90Q\x90\x81R\xF3[\x82\x80\xFD[P\x91\x904a\x03iW`\xE06`\x03\x19\x01\x12a\x03iWa\x01Ea\x0C\x81V[\x90a\x01Na\x0C\x9CV[\x91`D5`d5\x92`\x845\x92`\xFF\x84\x16\x80\x94\x03a\x03eWB\x85\x10a\x03\"Wa\x01ta\r,V[\x95`\x01\x80`\xA0\x1B\x03\x80\x92\x16\x95\x86\x89R` \x95`\x07\x87R\x84\x8A \x98\x89T\x99`\x01\x8B\x01\x90U\x85Q\x92\x85\x89\x85\x01\x95\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x87R\x8B\x89\x87\x01R\x16\x9A\x8B``\x86\x01R\x88`\x80\x86\x01R`\xA0\x85\x01R`\xC0\x84\x01R`\xC0\x83R`\xE0\x83\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x94\x84\x82\x10\x86\x83\x11\x17a\x03\x0EW\x81\x88R\x84Q\x90 a\x01\0\x85\x01\x92a\x19\x01`\xF0\x1B\x84Ra\x01\x02\x86\x01Ra\x01\"\x85\x01R`B\x81Ra\x01`\x84\x01\x94\x81\x86\x10\x90\x86\x11\x17a\x02\xFBW\x84\x87RQ\x90 \x83Ra\x01\x80\x82\x01R`\xA45a\x01\xA0\x82\x01R`\xC45a\x01\xC0\x90\x91\x01R\x87\x80R\x84\x90\x88\x90`\x80\x90`\x01Z\xFA\x15a\x02\xF1W\x86Q\x16\x96\x87\x15\x15\x80a\x02\xE8W[\x15a\x02\xB6W\x86\x97\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x95\x96\x97R\x83R\x80\x87 \x86\x88R\x83R\x81\x81\x88 UQ\x90\x81R\xA3\x80\xF3[\x83`d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R\xFD[P\x84\x88\x14a\x02sV[\x81Q=\x88\x82>=\x90\xFD[cNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[PcNH{q`\xE0\x1B\x8CR`A\x8DR`$\x8C\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R` \x81\x8A\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[\x86\x80\xFD[P\x80\xFD[PP4a\x03iW\x81`\x03\x196\x01\x12a\x03iW`\x08T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x90\xF3[PP4a\x03iW\x80`\x03\x196\x01\x12a\x03iW` \x91a\x03\xB3a\x0C\x81V[\x82`$5\x913\x84R`\x03\x86R\x81\x84 a\x03\xCD\x84\x82Ta\r\tV[\x90U`\x01`\x01`\xA0\x1B\x03\x16\x80\x84R`\x03\x86R\x92 \x80T\x82\x01\x90U\x82Q\x90\x81R3\x90`\0\x80Q` a\x0E\x9C\x839\x81Q\x91R\x90\x85\x90\xA3Q`\x01\x81R\xF3[P4a\x01%W\x81`\x03\x196\x01\x12a\x01%Wa\x04!a\x0C\x81V[`\x08T`$5\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x163\x03a\x04zWP\x84\x93\x92`\0\x80Q` a\x0E\x9C\x839\x81Q\x91R\x92` \x92\x16\x93\x84\x86R`\x03\x83R\x80\x86 a\x04h\x83\x82Ta\r\tV[\x90U\x81`\x02T\x03`\x02UQ\x90\x81R\xA3\x80\xF3[\x84QchS\xCB\xA7`\xE0\x1B\x81R\xFD[\x82\x844a\x05kW\x80`\x03\x196\x01\x12a\x05kW\x81Q\x90\x80`\x01\x80T\x90a\x04\xAC\x82a\x0B\xC6V[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x05>WP`\x01\x14a\x04\xE6W[a\x04\xE2\x86\x88a\x04\xD8\x82\x89\x03\x83a\x0C\0V[Q\x91\x82\x91\x82a\x0C8V[\x03\x90\xF3[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x05+WPPPP\x81\x01` \x01a\x04\xD8\x82a\x04\xE2\x86a\x04\xC7V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x05\x0EV[\x90Pa\x04\xE2\x97\x95P\x86\x93P` \x92Pa\x04\xD8\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x94\x86a\x04\xC7V[\x80\xFD[PP4a\x03iW` 6`\x03\x19\x01\x12a\x03iW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\x96a\x0C\x81V[\x16\x81R`\x07\x84R T\x90Q\x90\x81R\xF3[PP4a\x03iW` 6`\x03\x19\x01\x12a\x03iW` \x91\x81\x90`\x01`\x01`\xA0\x1B\x03a\x05\xCEa\x0C\x81V[\x16\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[P\x82\x904a\x03iW\x82`\x03\x196\x01\x12a\x03iWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x08\xF5Wa\x06\x10\x906\x90\x84\x01a\x0C\xB2V[\x91`$5\x82\x81\x11a\x08\xF1Wa\x06(\x906\x90\x83\x01a\x0C\xB2V[\x94`\x08T\x90`\xFF\x82`\xA0\x1C\x16a\x08\xE3WP`\x01`\x01`\xA0\x1B\x03\x19\x163\x17`\x08U\x82Q\x82\x81\x11a\x08\xD0W\x80a\x06\\\x86Ta\x0B\xC6V[\x94`\x1F\x95\x86\x81\x11a\x08wW[P` \x90\x86\x83\x11`\x01\x14a\x08\x08W\x87\x92a\x07\xFDW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x07\xEAWP`\x01\x91a\x06\xA9\x83Ta\x0B\xC6V[\x81\x81\x11a\x07\x88W[P` \x90\x82\x11`\x01\x14a\x07\rW\x83\x94\x82\x93\x94\x92a\x07\x02W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x90U[F`\x05Ua\x06\xE9a\rFV[`\x06U`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90U\x80\xF3[\x01Q\x90P\x84\x80a\x06\xC9V[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x07rWP\x95\x83\x85\x96\x97\x10a\x07YW[PPP\x81\x1B\x01\x90Ua\x06\xDDV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x07LV[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x079V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x07\xE1W[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x07\xD6WPPa\x06\xB1V[\x86\x81U\x01\x84\x90a\x07\xC8V[\x92P\x81\x92a\x07\xBFV[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x06}V[\x87\x80R`\0\x80Q` a\x0E|\x839\x81Q\x91R\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x08_WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x08FW[PPP\x81\x1B\x01\x84Ua\x06\x92V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x089V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x08#V[\x90\x91P\x86\x80R`\0\x80Q` a\x0E|\x839\x81Q\x91R\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x08\xC7W[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x08\xB9WPa\x06hV[\x88\x81U\x84\x93P`\x01\x01a\x08\xACV[\x92P\x81\x92a\x08\x9FV[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[Qb\xDC\x14\x9F`\xE4\x1B\x81R\x90P\xFD[\x84\x80\xFD[\x83\x80\xFD[P4a\x01%W\x81`\x03\x196\x01\x12a\x01%Wa\t\x12a\x0C\x81V[`\x08T`$5\x92`\x01`\x01`\xA0\x1B\x03\x92\x91\x83\x163\x03a\x04zW`\x02T\x90\x84\x82\x01\x80\x92\x11a\tmWP\x92`\0\x80Q` a\x0E\x9C\x839\x81Q\x91R\x92` \x92\x87\x95`\x02U\x16\x94\x85\x85R`\x03\x83R\x80\x85 \x82\x81T\x01\x90UQ\x90\x81R\xA3\x80\xF3[cNH{q`\xE0\x1B\x87R`\x11\x90R`$\x86\xFD[PP4a\x03iW\x81`\x03\x196\x01\x12a\x03iW` \x90a\t\x9Da\r,V[\x90Q\x90\x81R\xF3[PP4a\x03iW\x81`\x03\x196\x01\x12a\x03iW` \x90Q`\x12\x81R\xF3[P\x914a\x05kW``6`\x03\x19\x01\x12a\x05kWa\t\xDBa\x0C\x81V[`\0\x80Q` a\x0E\x9C\x839\x81Q\x91Ra\t\xF2a\x0C\x9CV[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x80\x85R` \x87\x81R\x86\x86 3\x87R\x81R\x86\x86 T\x90\x97\x91\x94\x88\x93`D5\x93\x89\x93\x85`\x01\x82\x01a\n^W[PPP\x86\x88R`\x03\x85R\x82\x88 a\n?\x85\x82Ta\r\tV[\x90U\x16\x95\x86\x81R`\x03\x84R \x81\x81T\x01\x90U\x85Q\x90\x81R\xA3Q`\x01\x81R\xF3[a\ng\x91a\r\tV[\x90\x88\x8AR\x86R\x83\x89 3\x8AR\x86R\x83\x89 U8\x80\x85a\n'V[PP4a\x03iW\x81`\x03\x196\x01\x12a\x03iW` \x90`\x02T\x90Q\x90\x81R\xF3[PP4a\x03iW\x81`\x03\x196\x01\x12a\x03iW` \x90`\xFF`\x08T`\xA0\x1C\x16\x90Q\x90\x15\x15\x81R\xF3[P4a\x01%W\x81`\x03\x196\x01\x12a\x01%W` \x92a\n\xE3a\x0C\x81V[\x91\x83`$5\x92\x83\x923\x82R\x87R\x81\x81 \x94`\x01\x80`\xA0\x1B\x03\x16\x94\x85\x82R\x87R U\x82Q\x90\x81R\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x843\x92\xA3Q`\x01\x81R\xF3[\x83\x90\x854a\x05kW\x80`\x03\x196\x01\x12a\x05kW\x80T\x81a\x0BT\x82a\x0B\xC6V[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x05>WP`\x01\x14a\x0B\x81Wa\x04\xE2\x86\x88a\x04\xD8\x82\x89\x03\x83a\x0C\0V[\x80\x80\x95PR`\0\x80Q` a\x0E|\x839\x81Q\x91R[\x83\x85\x10a\x0B\xB3WPPPP\x81\x01` \x01a\x04\xD8\x82a\x04\xE2\x86a\x04\xC7V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x0B\x96V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B\xF6W[` \x83\x10\x14a\x0B\xE0WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0B\xD5V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\"W`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\x0CmWPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\x0CKV[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0C\x97WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0C\x97WV[\x81`\x1F\x82\x01\x12\x15a\x0C\x97W\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0C\"W`@Q\x92a\x0C\xE7`\x1F\x84\x01`\x1F\x19\x16` \x01\x85a\x0C\0V[\x82\x84R` \x83\x83\x01\x01\x11a\x0C\x97W\x81`\0\x92` \x80\x93\x01\x83\x86\x017\x83\x01\x01R\x90V[\x91\x90\x82\x03\x91\x82\x11a\r\x16WV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\x05TF\x03a\r;W`\x06T\x90V[a\rCa\rFV[\x90V[`@Q`\0\x90`\0T\x90a\rY\x82a\x0B\xC6V[\x80\x82R\x81` \x94\x85\x82\x01\x94`\x01\x90\x87`\x01\x82\x16\x91\x82`\0\x14a\x0E]WPP`\x01\x14a\x0E\x15W[Pa\r\x8C\x92P\x03\x82a\x0C\0V[Q\x90 \x90`@Q\x90\x81\x01\x91\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x83R`@\x82\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xA0\x81R`\xC0\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0C\"W`@RQ\x90 \x90V[`\0\x80\x80R\x87\x92P\x90`\0\x80Q` a\x0E|\x839\x81Q\x91R[\x85\x83\x10a\x0EEWPPa\r\x8C\x93P\x82\x01\x018a\r\x7FV[\x80T\x83\x88\x01\x85\x01R\x86\x94P\x88\x93\x90\x92\x01\x91\x81\x01a\x0E.V[`\xFF\x19\x16\x88Ra\r\x8C\x95\x15\x15`\x05\x1B\x85\x01\x01\x92P8\x91Pa\r\x7F\x90PV\xFE)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xCB\x98\xD0\xE2\xA9#\xF0b\x08\xA7\xD9SR\xC1\xE2\x93\xC1\xAB\x04\xB4\xC6*\t\x81\xE8HJG&\xB1,\x0CdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a\x11\xD7V[\0[`\x005`\xE0\x1C\x80c\x02\x16\xB88\x14a\0\xE3W\x80c\x06\x8B\xCD\x8D\x14a\0\xDEW\x80c\x14U\xF1\xFC\x14a\0\xD9W\x80c.\xC3\x81\x88\x14a\0\xD4W\x80c;\xE6\xA3A\x14a\0\xCFW\x80c?\xC8\xCE\xF3\x14a\0\xCAW\x80c\x9D\x94/\x9A\x14a\0\xC5W\x80c\xACJ\xFA8\x14a\0\xC0W\x80c\xAF\xFE\xD0\xE0\x14a\0\xBBW\x80c\xB4b\xCD%\x14a\0\xB6W\x80c\xBD\x06%\xAB\x14a\0\xB1Wc\xCE\x15;\xF4\x03a\0\x0EWa\r\xF4V[a\x0C\xB0V[a\x0CkV[a\x0CMV[a\x0B\xC0V[a\t\xF4V[a\t\xAFV[a\x08tV[a\x06gV[a\x02\x93V[a\x01\xE4V[a\x01:V[`@`\x03\x19\x82\x01\x12a\x015W`\x045\x91`$5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x82\x11a\x015W\x80`#\x83\x01\x12\x15a\x015W\x81`\x04\x015\x93\x84\x11a\x015W`$\x84\x83\x01\x01\x11a\x015W`$\x01\x91\x90V[`\0\x80\xFD[4a\x015Wa\x01H6a\0\xE8V[\x91\x90`\x01T\x92`\x02`\0\x94\x14a\x01\xD2W\x83\x91`\x02`\x01Ua\x01h\x84a\x0BrV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90\x81;\x15a\x01\xCEW\x83a\x01\x9E\x95`@Q\x96\x87\x95\x86\x94\x85\x93c\xAC\xAD)\x89`\xE0\x1B\x85R3`\x04\x86\x01a\x0E\xBFV[\x03\x92Z\xF1\x80\x15a\x01\xC9Wa\x01\xBAW[Pa\x01\xB7`\x01\x80UV[\x80\xF3[a\x01\xC3\x90a\x0EhV[8a\x01\xADV[a\x0E\xFAV[\x83\x80\xFD[`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x90\xFD[4a\x015W` 6`\x03\x19\x01\x12a\x015W`@Qa\x02\x01\x81a\x0E\x81V[`\xC0`\0\x91\x82\x81R\x82` \x82\x01R\x82`@\x82\x01R\x82``\x82\x01R\x82`\x80\x82\x01R\x82`\xA0\x82\x01R\x01R`\xE0a\x02?a\x029`\x045a\x0BrV[Pa\x0F\x15V[`@Q\x90`\xC0`\x01\x80`\xA0\x1B\x03\x91\x82\x81Q\x16\x84R\x82` \x82\x01Q\x16` \x85\x01R\x82`@\x82\x01Q\x16`@\x85\x01R``\x81\x01Q``\x85\x01R`\x80\x81\x01Q`\x80\x85\x01R`\xA0\x81\x01Q`\xA0\x85\x01R\x01Q\x16`\xC0\x82\x01R\xF3[`\x03\x19` 6\x82\x01\x12a\x015W`\x04\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x015W`\x80\x81\x83\x01\x93\x826\x03\x01\x12a\x015W`\x02`\x01T\x14a\x06WW`\x02`\x01U`$\x81\x01\x90a\x02\xDF\x82a\x0F\xA6V[`D\x82\x01a\x02\xFBa\x02\xEF\x82a\x0F\xA6V[`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x14a\x06FWa\x03\x1Aa\x02\xEFa\x02\xEF\x88a\x0F\xA6V[\x91`\0\x96`\xA0a\x03/`d\x8AT\x97\x01\x83a\x0F\xB3V[\x95`@\x97\x8B\x8Ba\x03T\x8BQ\x9A\x8B\x96\x87\x95\x86\x94cs\xCB-\x03`\xE0\x1B\x86R3\x90\x86\x01a\x0E\xBFV[\x03\x92Z\xF1\x91\x82\x15a\x01\xC9W\x88\x97\x89\x8A\x99\x8B\x97\x8C\x96a\x06\x04W[P\x15a\x05\xCEWPa\x03\xA0a\x02\xEF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x12HV[\x91a\x03\xC5a\x03\xAD\x85a\x0F\xA6V[a\x03\xB6\x8Ba\x0F\xA6V[a\x03\xBF\x89a\x0F\xA6V[\x91a\x13\xB1V[\x92\x16\x91\x8A\x83;\x15a\x05\xCBW\x88Qc&lE\xBB`\xE1\x1B\x81R\x91\x82\x90\x81\x90a\x03\xEE\x90\x80\x87\x84\x01a\x10iV[\x03\x81\x83\x87Z\xF1\x80\x15a\x01\xC9Wa\x05\xB8W[Pa\x04\t\x84a\x10\xA4V[\x82;\x15a\x05\xA1W\x87Qc@\xC1\x0F\x19`\xE0\x1B\x80\x82R3\x84\x83\x01\x90\x81R` \x81\x01\x93\x90\x93R\x91\x8C\x90\x82\x90\x81\x90`@\x01\x03\x81\x83\x88Z\xF1\x80\x15a\x01\xC9Wa\x05\xA5W[P\x82;\x15a\x05\xA1W\x99\x80\x91a\x04v\x99\x9A\x9B\x89Q\x9A\x8B\x92\x83\x92\x83R\x82\x01\x90a\x03\xE8` `@\x84\x01\x93`\0\x81R\x01RV[\x03\x81\x83\x86Z\xF1\x92\x83\x15a\x01\xC9Wa\x05Da\x05Y\x96a\x05I\x8Da\x05Da\x05>\x8F\x97a\x05\x84\x9F\x8F\x99\x8F\x99a\x05N\x9Ba\x04\xC6a\x05T\x9Fa\x05\x0F\x93a\x058\x96a\x04\xC0\x92a\x05\x88W[Pa\x0F\xA6V[\x93a\x0F\xA6V[\x90a\x04\xFEa\x04\xD3\x8Ca\x0F\xA6V[\x92a\x04\xEEa\x04\xDFa\x0F\x06V[`\x01`\x01`\xA0\x1B\x03\x90\x97\x16\x87RV[`\x01`\x01`\xA0\x1B\x03\x16` \x86\x01RV[`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90\x83\x01RV[``\x81\x01\x86\x90R`\x80\x81\x01\x8A\x90R`\xA0\x81\x01\x8E\x90R`\x01`\x01`\xA0\x1B\x03\x8C\x16`\xC0\x82\x01Ra\x10\xD5V[Ta\x10\xB9V[\x9Ea\x0F\xA6V[a\x14\xABV[a\x0F\xA6V[\x87a\x15UV[a\x10\xA4V[\x91a\x05c`\x01\x80UV[Q\x94\x85\x94\x85\x90\x94\x93\x92``\x92`\x80\x83\x01\x96\x83R` \x83\x01R`@\x82\x01R\x01RV[\x03\x90\xF3[\x80a\x05\x95a\x05\x9B\x92a\x0EhV[\x80a\t\xA4V[8a\x04\xBAV[\x8A\x80\xFD[\x80a\x05\x95a\x05\xB2\x92a\x0EhV[8a\x04GV[\x80a\x05\x95a\x05\xC5\x92a\x0EhV[8a\x03\xFFV[\x80\xFD[a\x06\0\x88\x8C\x93a\x05\xDD\x84a\x12\x1BV[\x91Qcw`m)`\xE1\x1B\x81R\x94\x90\x93\x12\x92\x84\x01\x92\x83R` \x83\x01R\x82\x91`@\x01\x90V[\x03\x90\xFD[\x93\x9APPP\x92Pa\x06.\x91\x94P`\xA0=`\xA0\x11a\x06?W[a\x06&\x81\x83a\x0E\x9DV[\x81\x01\x90a\x0F\xF3V[\x91\x99\x90\x96\x91\x94\x91\x93\x90\x92\x908a\x03mV[P=a\x06\x1CV[`@Qc3\x91\n\xEF`\xE1\x1B\x81R\x85\x90\xFD[P`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R\xFD[a\x06p6a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\x06\xC7\x92`\xA0\x91`\x02`\x01Ua\x06\xA8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[PT`\x01`\x01`\xA0\x1B\x03\x16\x90V[\x90`@Q\x80\x96\x81\x94\x82\x93c\x8A\x04\xBD\xD5`\xE0\x1B\x84R\x883`\x04\x86\x01a\x0E\xBFV[\x03\x91Z\xFA\x90\x81\x15a\x01\xC9W`\0\x90\x81\x82\x80\x95\x81\x95a\x086W[P\x15a\x08\0WP\x90\x81a\x07\x03`\x03a\x06\xFAa\x05\x84\x95a\x0BrV[P\x01T\x83a\x10\xC8V[\x93a\x07\x1B`\x04a\x07\x12\x84a\x0BrV[P\x01T\x87a\x10\xC8V[\x95a\x07*`\x05a\x06\xFA\x85a\x0BrV[\x93a\x075\x85\x85a\x1B\x90V[`\x03a\x07@\x85a\x0BrV[P\x01U`\x04a\x07N\x84a\x0BrV[P\x01U`\x05a\x07\\\x83a\x0BrV[P\x01Ua\x07\x98\x85`\x01a\x07\x84\x87a\x07r\x86a\x0BrV[P\x83\x80`\xA0\x1B\x03\x93\x84\x91\x01T\x16a\x14\xABV[`\x02a\x07\x8F\x85a\x0BrV[P\x01T\x16a\x14\xABV[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90\x80`\x80\x81\x01[\x03\x90\xA2`\x01\x80U`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[\x80a\x06\0a\x08\x0F`\0\x93a\x12\x1BV[`@Qcw`m)`\xE1\x1B\x81R\x93\x90\x92\x12`\x04\x84\x01R`$\x83\x01\x91\x90\x91R\x81\x90`D\x82\x01\x90V[\x93PPP\x92Pa\x08U\x91P`\xA0=`\xA0\x11a\x06?Wa\x06&\x81\x83a\x0E\x9DV[\x94\x91\x90\x92\x90\x92\x94\x938a\x06\xE0V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x03a\x015WV[4a\x015W`@6`\x03\x19\x01\x12a\x015W`\x045a\x08\x91\x81a\x08cV[`$5\x90a\x08\xB8a\x02\xEFa\x02\xEF`\x06a\x08\xA9\x86a\x0BrV[P\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16`\x04\x83\x01R` \x80\x83`$\x81\x85Z\xFA\x91\x82\x15a\x01\xC9W`\x04\x93`\0\x93a\t\x83W[P\x81\x90`@Q\x94\x85\x80\x92c\x18\x16\r\xDD`\xE0\x1B\x82RZ\xFA\x80\x15a\x01\xC9Wa\x05\x84\x94a\t=\x94a\t7\x93`\0\x93a\tMW[PPa\t/`\x05\x91a\x0BrV[P\x01Ta\x1D\x14V[\x90a\x1D6V[`@Q\x90\x81R\x90\x81\x90` \x82\x01\x90V[`\x05\x92\x93Pa\t/\x91\x81a\tu\x92\x90=\x10a\t|W[a\tm\x81\x83a\x0E\x9DV[\x81\x01\x90a\x11\x93V[\x92\x91a\t\"V[P=a\tcV[\x82\x91\x93Pa\t\x9D\x90\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x90a\x08\xF2V[`\0\x91\x03\x12a\x015WV[4a\x015W`\x006`\x03\x19\x01\x12a\x015W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x015Wa\n\x026a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\n,\x92`\xA0\x91`\x02`\x01Ua\x06\xA8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[\x03\x91Z\xFA\x90\x81\x15a\x01\xC9W`\0\x90\x81\x82\x80\x95\x81\x95a\x0BEW[P\x15a\x08\0WP\x90\x81a\nh\x82`\x03a\n`a\x05\x84\x96a\x0BrV[P\x01Ta\x10\xC8V[\x93a\nx\x86`\x04a\n`\x85a\x0BrV[\x95a\n\x88\x82`\x05a\n`\x86a\x0BrV[\x93a\n\x93\x85\x85a\x1C\\V[`\x03a\n\x9E\x85a\x0BrV[P\x01U`\x04a\n\xAC\x84a\x0BrV[P\x01U`\x05a\n\xBA\x83a\x0BrV[P\x01Ua\n\xFD\x85a\n\xCA\x83a\x0BrV[P`\x01\x01T`\x01`\x01`\xA0\x1B\x03\x90a\n\xE7\x90\x88\x903\x90\x84\x16a\x15\xD8V[a\n\xF0\x84a\x0BrV[P`\x02\x01T3\x91\x16a\x15\xD8V[`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x81\x01\x85\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90\x80`\x80\x81\x01a\x07\xDCV[\x93PPP\x92Pa\x0Bd\x91P`\xA0=`\xA0\x11a\x06?Wa\x06&\x81\x83a\x0E\x9DV[\x94\x91\x90\x92\x90\x92\x94\x938a\nEV[\x90`\0\x91\x82T\x81\x10\x15a\x0B\xACW`\x07\x90\x83\x80R\x02\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x01\x91\x90V[cNH{q`\xE0\x1B\x83R`2`\x04R`$\x83\xFD[4a\x015W` 6`\x03\x19\x01\x12a\x015W`\x045`\0T\x81\x10\x15a\x015Wa\x0B\xE7\x90a\x0BrV[P\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x81R\x95\x87\x16` \x87\x01R\x93\x86\x16\x93\x85\x01\x93\x90\x93R``\x84\x01\x91\x90\x91R`\x80\x83\x01R`\xA0\x82\x01\x93\x90\x93R\x91\x16`\xC0\x82\x01R`\xE0\x90\xF3[4a\x015W`\x006`\x03\x19\x01\x12a\x015W` `\0T`@Q\x90\x81R\xF3[4a\x015W`\x006`\x03\x19\x01\x12a\x015W`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x015Wa\x0C\xBE6a\0\xE8V[\x91\x90`\x02`\x01T\x14a\x01\xD2Wa\r\x07\x92`\xC0\x91`\x02`\x01Ua\x0C\xE8a\x02\xEFa\x02\xEFa\x06\x9A\x87a\x0BrV[\x90`@Q\x80\x96\x81\x94\x82\x93c\r\x17\xA7\xC7`\xE3\x1B\x84R\x883`\x04\x86\x01a\x0E\xBFV[\x03\x91Z\xFA\x91\x82\x15a\x01\xC9W`\0\x80\x93\x81\x80\x93\x81\x92a\r\xB7W[P\x15a\r\xA8W\x83\x94P`\x05a\r7a\r@\x95a\x0BrV[P\x01U\x83a\x16\xFEV[\x94\x92P\x92\x90P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED`@Q\x80a\r\x8B\x87\x873\x96\x84`@\x91\x94\x93\x92``\x82\x01\x95\x15\x15\x82R` \x82\x01R\x01RV[\x03\x90\xA3a\r\x97`\x01\x80UV[`@\x80Q\x91\x82R` \x82\x01\x92\x90\x92R\xF3[`\0\x85a\x06\0a\x08\x0F\x82a\x12\x1BV[\x93PPPPa\r\xDF\x91\x92P`\xC0=`\xC0\x11a\r\xEDW[a\r\xD7\x81\x83a\x0E\x9DV[\x81\x01\x90a\x11\xA2V[\x93\x95\x94\x90\x93\x91\x92P8a\r V[P=a\r\xCDV[4a\x015W` 6`\x03\x19\x01\x12a\x015W`\x045`\x03a\x0E\x13\x82a\x0BrV[P\x01Ta\x05\x84`\x05a\x0E2`\x04a\x0E)\x86a\x0BrV[P\x01T\x94a\x0BrV[P\x01T`@Q\x93\x84\x93\x84`@\x91\x94\x93\x92``\x82\x01\x95\x82R` \x82\x01R\x01RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x0E|W`@RV[a\x0ERV[`\xE0\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@RV[\x92\x84\x92`\x80\x95\x92`\x01\x80`\xA0\x1B\x03\x16\x85R` \x85\x01R```@\x85\x01R\x81``\x85\x01R\x84\x84\x017`\0\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[`@Q=`\0\x82>=\x90\xFD[`@Q\x90a\x0F\x13\x82a\x0E\x81V[V[\x90a\x0F\x13`@Qa\x0F%\x81a\x0E\x81V[`\xC0a\x0F\x98`\x06\x83\x96`\x01\x80`\xA0\x1B\x03\x80\x82T\x16\x86R`\x01\x82\x01T\x16` \x86\x01Ra\x0Fla\x0F\\`\x02\x83\x01T`\x01\x80`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16`@\x87\x01RV[`\x03\x81\x01T``\x86\x01R`\x04\x81\x01T`\x80\x86\x01R`\x05\x81\x01T`\xA0\x86\x01R\x01T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x91\x01RV[5a\x0F\xB0\x81a\x08cV[\x90V[\x905\x90`\x1E\x19\x816\x03\x01\x82\x12\x15a\x015W\x01\x805\x90g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x015W` \x01\x91\x816\x03\x83\x13a\x015WV[Q\x90\x81\x15\x15\x82\x03a\x015WV[\x90\x81`\xA0\x91\x03\x12a\x015Wa\x10\x07\x81a\x0F\xE6V[\x91` \x82\x01Q\x91`@\x81\x01Q\x91`\x80``\x83\x01Q\x92\x01Q\x90V[`\0[\x83\x81\x10a\x104WPP`\0\x91\x01RV[\x81\x81\x01Q\x83\x82\x01R` \x01a\x10$V[\x90` \x91a\x10]\x81Q\x80\x92\x81\x85R\x85\x80\x86\x01\x91\x01a\x10!V[`\x1F\x01`\x1F\x19\x16\x01\x01\x90V[\x90\x91a\x10\x80a\x0F\xB0\x93`@\x84R`@\x84\x01\x90a\x10DV[\x91` \x81\x84\x03\x91\x01Ra\x10DV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[a\x03\xE7\x19\x81\x01\x91\x90\x82\x11a\x10\xB4WV[a\x10\x8EV[`\0\x19\x81\x01\x91\x90\x82\x11a\x10\xB4WV[\x91\x90\x82\x03\x91\x82\x11a\x10\xB4WV[`\0Th\x01\0\0\0\0\0\0\0\0\x81\x10\x15a\x0E|W\x80`\x01a\x10\xF9\x92\x01`\0Ua\x0BrV[a\x11}W\x81Q\x81T`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x17\x83U` \x84\x01Q`\x01\x84\x01\x80T\x91\x84\x16\x91\x83\x16\x91\x90\x91\x17\x90U`@\x84\x01Q`\x02\x84\x01\x80T\x83\x16\x91\x84\x16\x91\x90\x91\x17\x90U``\x84\x01Q`\x03\x84\x01U`\x80\x84\x01Q`\x04\x84\x01U`\xA0\x84\x01Q`\x05\x84\x01U`\xC0\x90\x93\x01Q`\x06\x90\x92\x01\x80T\x90\x93\x16\x91\x16\x17\x90UV[cNH{q`\xE0\x1B`\0R`\0`\x04R`$`\0\xFD[\x90\x81` \x91\x03\x12a\x015WQ\x90V[\x91\x90\x82`\xC0\x91\x03\x12a\x015Wa\x11\xB7\x82a\x0F\xE6V[\x91` \x81\x01Q\x91`@\x82\x01Q\x91``\x81\x01Q\x91`\xA0`\x80\x83\x01Q\x92\x01Q\x90V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a\x12\tWV[`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x90\xFD[`\x01`\xFF\x1B\x81\x14a\x126W`\0\x81\x12\x15a\x0F\xB0W\x19`\x01\x01\x90V[`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x90\xFD[nZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x90v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0b\xFF\xFF\xFF\x82`\x88\x1C\x16\x17`\0R`x\x1B\x17` R`7`\t`\0\xF0\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x12\x9EWV[`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x90\xFD[` \x81\x83\x03\x12a\x015W\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x82\x82\x11a\x015W\x01\x90\x82`\x1F\x83\x01\x12\x15a\x015W\x81Q\x90\x81\x11a\x0E|W`@Q\x92a\x12\xFB`\x1F\x83\x01`\x1F\x19\x16` \x01\x85a\x0E\x9DV[\x81\x84R` \x82\x84\x01\x01\x11a\x015Wa\x0F\xB0\x91` \x80\x85\x01\x91\x01a\x10!V[a\x0F\x13\x92\x94\x93`(\x92`@Q\x96\x87\x93dDFMM-`\xD8\x1B` \x86\x01Ra\x13J\x81Q\x80\x92` `%\x89\x01\x91\x01a\x10!V[\x84\x01\x91`-`\xF8\x1B\x92\x83`%\x82\x01Ra\x13m\x82Q\x80\x93` `&\x85\x01\x91\x01a\x10!V[\x01\x82`&\x82\x01Ra\x13\x88\x82Q\x80\x93` `'\x85\x01\x91\x01a\x10!V[\x01\x90`'\x82\x01Ra\x13\xA2\x82Q\x80\x93` \x87\x85\x01\x91\x01a\x10!V[\x01\x03`\x08\x81\x01\x85R\x01\x83a\x0E\x9DV[`@Qc\x06\xFD\xDE\x03`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x93\x90\x92`\0\x92\x90\x91\x90\x83\x90\x85\x90`\x04\x90\x82\x90\x89\x16Z\xFA\x93\x84\x15a\x01\xC9W\x83\x94a\x14\x8FW[P`@Q\x90\x83\x82`\x04\x81\x89c\x95\xD8\x9BA`\xE0\x1B\x97\x88\x83R\x16Z\xFA\x92\x83\x15a\x01\xC9W\x84\x92\x83\x94a\x14pW[P`\x04\x90`@Q\x97\x88\x93\x84\x92\x83R\x16Z\xFA\x91\x82\x15a\x01\xC9Wa\x0F\xB0\x94\x81\x93a\x14HW[Pa\x14B\x90Ta\x19\x98V[\x92a\x13\x19V[a\x14B\x91\x93Pa\x14i\x90=\x80\x86\x83>a\x14a\x81\x83a\x0E\x9DV[\x81\x01\x90a\x12\xB0V[\x92\x90a\x147V[`\x04\x91\x94Pa\x14\x88\x90=\x80\x86\x83>a\x14a\x81\x83a\x0E\x9DV[\x93\x90a\x14\x14V[a\x14\xA4\x91\x94P=\x80\x85\x83>a\x14a\x81\x83a\x0E\x9DV[\x928a\x13\xEAV[G\x82\x11a\x15,WP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x80;\x15a\x015W`\0\x90`\x04`@Q\x80\x94\x81\x93c\r\x0E0\xDB`\xE4\x1B\x83RZ\xF1\x80\x15a\x01\xC9Wa\x15\x19W[PGa\x15\x0FWV[a\x0F\x13G3a\x1BEV[\x80a\x05\x95a\x15&\x92a\x0EhV[8a\x15\x07V[a\x15Ba\x0F\x13\x92a\x15<\x83a\x1A1V[\x90a\x1DWV[\x900\x903\x90`\x01`\x01`\xA0\x1B\x03\x16a\x1A\xC4V[\x90a\x15ba\x029\x83a\x0BrV[`\x01\x80`\xA0\x1B\x03\x91\x82\x82Q\x16\x91\x83` \x82\x01Q\x16\x93\x80`@\x83\x01Q\x16\x95``\x83\x01Q\x91`\xA0`\x80\x85\x01Q\x94\x01Q\x94`@Q\x96\x87R\x16` \x86\x01R`@\x85\x01R``\x84\x01R`\x80\x83\x01R`\xA0\x82\x01R\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E`\xC03\x92\xA4V[`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x93\x92\x91\x90\x81\x16\x90\x81\x85\x03a\x16fWPP\x82;\x15a\x015W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R\x92`\0\x90\x84\x90`$\x90\x82\x90\x84\x90Z\xF1\x92\x83\x15a\x01\xC9Wa\x0F\x13\x93a\x16SW[Pa\x1BEV[\x80a\x05\x95a\x16`\x92a\x0EhV[8a\x16MV[` \x92\x94P\x92a\x16ya\x16\x7F\x92\x94a\x1A1V[\x90a\x1D\x14V[`D`@Q\x94`\0\x80\x95\x81\x94\x82\x93c\xA9\x05\x9C\xBB`\xE0\x1B\x84R`\x04R`$RZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x16\xBDWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x90\xFD[\x91\x90\x82\x01\x80\x92\x11a\x10\xB4WV[\x92\x91\x90`\x03a\x17\x0C\x85a\x0BrV[P\x01T\x92a\x17\x19\x85a\x0BrV[P`\x04\x90\x81\x01T\x93\x85\x84\x11\x91\x90\x82\x15a\x19XW\x85\x81\x10\x15a\x19HW\x81a\x17~a\x17F`\x01a\x08\xA9\x8Ca\x0BrV[\x99a\x17i\x84a\x17ca\x17\\`\x02a\x08\xA9\x86a\x0BrV[\x9C\x8Ba\x10\xC8V[\x9Aa\x10\xC8V[\x97[`\x03a\x17v\x83a\x0BrV[P\x01Ua\x0BrV[P\x01U`@\x80Qcp\xA0\x821`\xE0\x1B\x80\x82R0\x84\x83\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x8B\x81\x16\x91` \x91\x90\x82\x90\x86\x90\x81\x90\x83\x01\x03\x81\x86Z\xFA\x94\x85\x15a\x01\xC9W`\0\x95a\x19)W[P\x85Q\x84\x81R0\x88\x82\x01\x90\x81R\x91\x8D\x16\x95\x90\x94\x90\x91\x83\x90\x86\x90\x81\x90` \x01\x03\x81\x89Z\xFA\x94\x85\x15a\x01\xC9W\x8D\x8F\x8E\x90\x8E\x93`\0\x99a\x18\xFCW[Pa\x18\x12\x93\x92\x91a\x18\x0B\x91a\x14\xABV[3\x90a\x15\xD8V[\x86Q\x81\x81R0\x89\x82\x01\x90\x81R\x90\x94\x84\x91\x86\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x93\x84\x15a\x01\xC9W`\0\x94a\x18\xDBW[P\x86Q\x90\x81R0\x88\x82\x01\x90\x81R\x90\x95\x83\x91\x87\x91\x90\x82\x90\x81\x90` \x01\x03\x91Z\xFA\x94\x85\x15a\x01\xC9W\x8B\x92`\0\x96a\x18\xB4W[PP\x90a\x18{\x91a\x16\xF1V[\x11a\x18\xA5W\x86a\x18\x8A\x91a\x10\xC8V[\x11a\x18\x98WPP\x94\x93\x92\x91\x90V[Qc\xF3\xCB\xBC\x87`\xE0\x1B\x81R\xFD[PPQc =\x90\x1D`\xE2\x1B\x81R\xFD[a\x18{\x93\x92\x96P\x90\x81a\x18\xD2\x92\x90=\x10a\t|Wa\tm\x81\x83a\x0E\x9DV[\x94\x90\x918a\x18oV[\x83\x91\x94Pa\x18\xF5\x90\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x93\x90a\x18?V[a\x18\x0B\x91\x99P\x91a\x19\x1Ea\x18\x12\x95\x94\x93\x89=\x8B\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x99\x91P\x91\x92\x93a\x17\xFBV[a\x19A\x91\x95P\x82=\x84\x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x938a\x17\xC3V[P`@Qc\x11\x15vg`\xE0\x1B\x81R\xFD[\x86\x85\x97\x96\x97\x10\x15a\x19HW\x81a\x17~a\x19u`\x02a\x08\xA9\x8Ca\x0BrV[\x99a\x19\x92\x88a\x17ca\x19\x8B`\x01a\x08\xA9\x86a\x0BrV[\x9C\x87a\x10\xC8V[\x97a\x17kV[\x90\x81\x15a\x19\xDBW`N\x91`@Q\x90\x83\x82R\x80`\x80\x83\x01`@R[a\x19\xC1WP\x82\x01\x91`N\x03\x82RV[\x92`\n\x90\x81\x85\x06`0\x01\x81\x84\x01R`\0\x19\x01\x93\x04\x80a\x19\xB2V[\x90P`@Q`@\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0E|W`@R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`M\x81\x11a\x10\xB4W`\n\n\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x80\x83\x02\x92\x83\x04\x03a\x10\xB4WV[`@Qc1<\xE5g`\xE0\x1B\x81R\x90` \x90\x82\x90`\x04\x90\x82\x90`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x01\xC9W`\0\x91a\x1A\x83W[P`\xFF\x16`\x12\x03`\x12\x81\x11a\x10\xB4Wa\x1A~a\x0F\xB0\x91a\x1A\x0BV[a\x1A\x19V[` \x81=` \x11a\x1A\xBCW[\x81a\x1A\x9C` \x93\x83a\x0E\x9DV[\x81\x01\x03\x12a\x1A\xB8WQ\x90`\xFF\x82\x16\x82\x03a\x05\xCBWP`\xFFa\x1AcV[P\x80\xFD[=\x91Pa\x1A\x8FV[\x91\x92`d` \x92\x94`@Q\x95`\0\x95\x86\x94\x85\x93\x84\x93c#\xB8r\xDD`\xE0\x1B\x85R`\x04R`$R`DRZ\xF1=\x15`\x1F=\x11`\x01\x84Q\x14\x16\x17\x16\x90``R\x81`@R\x15a\x1B\x0CWPV[bF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x90\xFD[`\0\x80\x80\x93\x81\x93Z\xF1\x15a\x1BUWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[a\x1B\xA4a\x02\xEFa\x02\xEF`\x06a\x08\xA9\x85a\x0BrV[\x91`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x01\xC9Wa\x1B\xE6\x93a\t7\x92`\0\x92a\x1C4W[Pa\x1B\xDD`\x05\x91a\x0BrV[P\x01T\x90a\x1D\x14V[\x90\x80;\x15a\x015W`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01[\x03\x92Z\xF1\x80\x15a\x01\xC9Wa\x1C'WPV[\x80a\x05\x95a\x0F\x13\x92a\x0EhV[`\x05\x91\x92Pa\x1CTa\x1B\xDD\x91` =` \x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x91Pa\x1B\xD1V[a\x1Cpa\x02\xEFa\x02\xEF`\x06a\x08\xA9\x85a\x0BrV[\x91`@Qc\x18\x16\r\xDD`\xE0\x1B\x81R` \x81`\x04\x81\x87Z\xFA\x80\x15a\x01\xC9Wa\x1C\xB8\x93a\x1C\xB2\x92`\0\x92a\x1C\xECW[Pa\x1C\xA9`\x05\x91a\x0BrV[P\x01T\x90a\x1DWV[\x90a\x1D\x87V[\x90\x80;\x15a\x015W`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x92\x90\x92R`\0\x90\x82\x90\x81\x83\x81`D\x81\x01a\x1C\x16V[`\x05\x91\x92Pa\x1D\x0Ca\x1C\xA9\x91` =` \x11a\t|Wa\tm\x81\x83a\x0E\x9DV[\x92\x91Pa\x1C\x9DV[g\r\xE0\xB6\xB3\xA7d\0\0\x90\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17\x82\x15\x15\x16\x15a\x015W\x04\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x015Wg\r\xE0\xB6\xB3\xA7d\0\0\x90\x04\x90V[\x90g\r\xE0\xB6\xB3\xA7d\0\0\x91\x82\x81\x02\x92\x81\x84\x04\x14\x90\x15\x17\x81\x15\x15\x16\x15a\x015W`\x01\x90`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V[\x81\x81\x02\x91\x81\x83\x04\x14\x90\x15\x17`\x01\x16\x15a\x015W`\x01g\r\xE0\xB6\xB3\xA7d\0\0`\0\x19\x83\x01\x04\x01\x90\x15\x15\x02\x90V\xFE\xA2dipfsX\"\x12 \xA0(RF\xCC\xA8i\xAE\x878\x81\xF8\x0FM\x90E\xEC\xD4L\x1C*!\xC5\x12\xE1\xA0\xAFq|\x94\x91\xFAdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct DFMM<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for DFMM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for DFMM<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for DFMM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for DFMM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(DFMM)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    DFMM_ABI.clone(),
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
                DFMM_ABI.clone(),
                DFMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `allocate` (0x2ec38188) function
        pub fn allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([46, 195, 129, 136], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deallocate` (0x9d942f9a) function
        pub fn deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([157, 148, 47, 154], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getPool` (0x068bcd8d) function
        pub fn get_pool(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pool> {
            self.0
                .method_hash([6, 139, 205, 141], pool_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getReservesAndLiquidity` (0xce153bf4) function
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
        ///Calls the contract's `init` (0x1455f1fc) function
        pub fn init(
            &self,
            params: InitParams,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([20, 85, 241, 252], (params,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `liquidityOf` (0x3be6a341) function
        pub fn liquidity_of(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 230, 163, 65], (account, pool_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lpTokenImplementation` (0xb462cd25) function
        pub fn lp_token_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([180, 98, 205, 37], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pools` (0xac4afa38) function
        pub fn pools(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::Address,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::Address,
            ),
        > {
            self.0
                .method_hash([172, 74, 250, 56], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swap` (0xbd0625ab) function
        pub fn swap(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([189, 6, 37, 171], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AllocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeallocateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Init` event
        pub fn init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitFilter> {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DFMMEvents> {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for DFMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ERC1167FailedCreateClone` with signature `ERC1167FailedCreateClone()` and selector `0xc2f868f4`
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
    #[etherror(name = "ERC1167FailedCreateClone", abi = "ERC1167FailedCreateClone()")]
    pub struct ERC1167FailedCreateClone;
    ///Custom Error type `Invalid` with signature `Invalid(bool,uint256)` and selector `0xeec0da52`
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
    #[etherror(name = "Invalid", abi = "Invalid(bool,uint256)")]
    pub struct Invalid {
        pub negative: bool,
        pub swap_constant_growth: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidSwap` with signature `InvalidSwap()` and selector `0x11157667`
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
    #[etherror(name = "InvalidSwap", abi = "InvalidSwap()")]
    pub struct InvalidSwap;
    ///Custom Error type `InvalidSwapInputTransfer` with signature `InvalidSwapInputTransfer()` and selector `0x80f64074`
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
    #[etherror(name = "InvalidSwapInputTransfer", abi = "InvalidSwapInputTransfer()")]
    pub struct InvalidSwapInputTransfer;
    ///Custom Error type `InvalidSwapOutputTransfer` with signature `InvalidSwapOutputTransfer()` and selector `0xf3cbbc87`
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
    #[etherror(name = "InvalidSwapOutputTransfer", abi = "InvalidSwapOutputTransfer()")]
    pub struct InvalidSwapOutputTransfer;
    ///Custom Error type `InvalidTokens` with signature `InvalidTokens()` and selector `0x672215de`
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
    #[etherror(name = "InvalidTokens", abi = "InvalidTokens()")]
    pub struct InvalidTokens;
    ///Custom Error type `Locked` with signature `Locked()` and selector `0x0f2e5b6c`
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
    #[etherror(name = "Locked", abi = "Locked()")]
    pub struct Locked;
    ///Custom Error type `Min` with signature `Min()` and selector `0x4d2d75b1`
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
    #[etherror(name = "Min", abi = "Min()")]
    pub struct Min;
    ///Custom Error type `OnlyWETH` with signature `OnlyWETH()` and selector `0x01f180c9`
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
    #[etherror(name = "OnlyWETH", abi = "OnlyWETH()")]
    pub struct OnlyWETH;
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
    pub enum DFMMErrors {
        ERC1167FailedCreateClone(ERC1167FailedCreateClone),
        Invalid(Invalid),
        InvalidSwap(InvalidSwap),
        InvalidSwapInputTransfer(InvalidSwapInputTransfer),
        InvalidSwapOutputTransfer(InvalidSwapOutputTransfer),
        InvalidTokens(InvalidTokens),
        Locked(Locked),
        Min(Min),
        OnlyWETH(OnlyWETH),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <ERC1167FailedCreateClone as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ERC1167FailedCreateClone(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) = <InvalidSwapInputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapInputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidSwapOutputTransfer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSwapOutputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidTokens as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokens(decoded));
            }
            if let Ok(decoded) = <Locked as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <OnlyWETH as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OnlyWETH(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ERC1167FailedCreateClone(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwap(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapInputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokens(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyWETH(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1167FailedCreateClone as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwapInputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSwapOutputTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokens as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Locked as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OnlyWETH as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1167FailedCreateClone(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwapInputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::Locked(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnlyWETH(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for DFMMErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ERC1167FailedCreateClone> for DFMMErrors {
        fn from(value: ERC1167FailedCreateClone) -> Self {
            Self::ERC1167FailedCreateClone(value)
        }
    }
    impl ::core::convert::From<Invalid> for DFMMErrors {
        fn from(value: Invalid) -> Self {
            Self::Invalid(value)
        }
    }
    impl ::core::convert::From<InvalidSwap> for DFMMErrors {
        fn from(value: InvalidSwap) -> Self {
            Self::InvalidSwap(value)
        }
    }
    impl ::core::convert::From<InvalidSwapInputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapInputTransfer) -> Self {
            Self::InvalidSwapInputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidSwapOutputTransfer> for DFMMErrors {
        fn from(value: InvalidSwapOutputTransfer) -> Self {
            Self::InvalidSwapOutputTransfer(value)
        }
    }
    impl ::core::convert::From<InvalidTokens> for DFMMErrors {
        fn from(value: InvalidTokens) -> Self {
            Self::InvalidTokens(value)
        }
    }
    impl ::core::convert::From<Locked> for DFMMErrors {
        fn from(value: Locked) -> Self {
            Self::Locked(value)
        }
    }
    impl ::core::convert::From<Min> for DFMMErrors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<OnlyWETH> for DFMMErrors {
        fn from(value: OnlyWETH) -> Self {
            Self::OnlyWETH(value)
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
    #[ethevent(
        name = "Allocate",
        abi = "Allocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct AllocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "Deallocate",
        abi = "Deallocate(address,uint256,uint256,uint256,uint256)"
    )]
    pub struct DeallocateFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub delta_x: ::ethers::core::types::U256,
        pub delta_y: ::ethers::core::types::U256,
        pub delta_l: ::ethers::core::types::U256,
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
    #[ethevent(
        name = "Init",
        abi = "Init(address,address,address,address,address,uint256,uint256,uint256,uint256)"
    )]
    pub struct InitFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub strategy: ::ethers::core::types::Address,
        pub lp_token: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_x: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub token_y: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
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
    #[ethevent(name = "Swap", abi = "Swap(address,uint256,bool,uint256,uint256)")]
    pub struct SwapFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub pool_id: ::ethers::core::types::U256,
        pub is_swap_x_for_y: bool,
        pub input_amount: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
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
    pub enum DFMMEvents {
        AllocateFilter(AllocateFilter),
        DeallocateFilter(DeallocateFilter),
        InitFilter(InitFilter),
        SwapFilter(SwapFilter),
    }
    impl ::ethers::contract::EthLogDecode for DFMMEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AllocateFilter::decode_log(log) {
                return Ok(DFMMEvents::AllocateFilter(decoded));
            }
            if let Ok(decoded) = DeallocateFilter::decode_log(log) {
                return Ok(DFMMEvents::DeallocateFilter(decoded));
            }
            if let Ok(decoded) = InitFilter::decode_log(log) {
                return Ok(DFMMEvents::InitFilter(decoded));
            }
            if let Ok(decoded) = SwapFilter::decode_log(log) {
                return Ok(DFMMEvents::SwapFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for DFMMEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AllocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeallocateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateFilter> for DFMMEvents {
        fn from(value: AllocateFilter) -> Self {
            Self::AllocateFilter(value)
        }
    }
    impl ::core::convert::From<DeallocateFilter> for DFMMEvents {
        fn from(value: DeallocateFilter) -> Self {
            Self::DeallocateFilter(value)
        }
    }
    impl ::core::convert::From<InitFilter> for DFMMEvents {
        fn from(value: InitFilter) -> Self {
            Self::InitFilter(value)
        }
    }
    impl ::core::convert::From<SwapFilter> for DFMMEvents {
        fn from(value: SwapFilter) -> Self {
            Self::SwapFilter(value)
        }
    }
    ///Container type for all input parameters for the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    #[ethcall(name = "allocate", abi = "allocate(uint256,bytes)")]
    pub struct AllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    #[ethcall(name = "deallocate", abi = "deallocate(uint256,bytes)")]
    pub struct DeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
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
    #[ethcall(name = "getPool", abi = "getPool(uint256)")]
    pub struct GetPoolCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
        name = "getReservesAndLiquidity",
        abi = "getReservesAndLiquidity(uint256)"
    )]
    pub struct GetReservesAndLiquidityCall {
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
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
    #[ethcall(name = "init", abi = "init((address,address,address,bytes))")]
    pub struct InitCall {
        pub params: InitParams,
    }
    ///Container type for all input parameters for the `liquidityOf` function with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    #[ethcall(name = "liquidityOf", abi = "liquidityOf(address,uint256)")]
    pub struct LiquidityOfCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lpTokenImplementation` function with signature `lpTokenImplementation()` and selector `0xb462cd25`
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
    #[ethcall(name = "lpTokenImplementation", abi = "lpTokenImplementation()")]
    pub struct LpTokenImplementationCall;
    ///Container type for all input parameters for the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    ///Container type for all input parameters for the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    #[ethcall(name = "swap", abi = "swap(uint256,bytes)")]
    pub struct SwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `update` function with signature `update(uint256,bytes)` and selector `0x0216b838`
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
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
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
    pub enum DFMMCalls {
        Allocate(AllocateCall),
        Deallocate(DeallocateCall),
        GetPool(GetPoolCall),
        GetReservesAndLiquidity(GetReservesAndLiquidityCall),
        Init(InitCall),
        LiquidityOf(LiquidityOfCall),
        LpTokenImplementation(LpTokenImplementationCall),
        Nonce(NonceCall),
        Pools(PoolsCall),
        Swap(SwapCall),
        Update(UpdateCall),
        Weth(WethCall),
    }
    impl ::ethers::core::abi::AbiDecode for DFMMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) = <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <LiquidityOfCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LiquidityOf(decoded));
            }
            if let Ok(decoded) = <LpTokenImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LpTokenImplementation(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Deallocate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidityOf(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LpTokenImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pools(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Swap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Update(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Weth(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for DFMMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Allocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deallocate(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetPool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetReservesAndLiquidity(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpTokenImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Nonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pools(element) => ::core::fmt::Display::fmt(element, f),
                Self::Swap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Update(element) => ::core::fmt::Display::fmt(element, f),
                Self::Weth(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AllocateCall> for DFMMCalls {
        fn from(value: AllocateCall) -> Self {
            Self::Allocate(value)
        }
    }
    impl ::core::convert::From<DeallocateCall> for DFMMCalls {
        fn from(value: DeallocateCall) -> Self {
            Self::Deallocate(value)
        }
    }
    impl ::core::convert::From<GetPoolCall> for DFMMCalls {
        fn from(value: GetPoolCall) -> Self {
            Self::GetPool(value)
        }
    }
    impl ::core::convert::From<GetReservesAndLiquidityCall> for DFMMCalls {
        fn from(value: GetReservesAndLiquidityCall) -> Self {
            Self::GetReservesAndLiquidity(value)
        }
    }
    impl ::core::convert::From<InitCall> for DFMMCalls {
        fn from(value: InitCall) -> Self {
            Self::Init(value)
        }
    }
    impl ::core::convert::From<LiquidityOfCall> for DFMMCalls {
        fn from(value: LiquidityOfCall) -> Self {
            Self::LiquidityOf(value)
        }
    }
    impl ::core::convert::From<LpTokenImplementationCall> for DFMMCalls {
        fn from(value: LpTokenImplementationCall) -> Self {
            Self::LpTokenImplementation(value)
        }
    }
    impl ::core::convert::From<NonceCall> for DFMMCalls {
        fn from(value: NonceCall) -> Self {
            Self::Nonce(value)
        }
    }
    impl ::core::convert::From<PoolsCall> for DFMMCalls {
        fn from(value: PoolsCall) -> Self {
            Self::Pools(value)
        }
    }
    impl ::core::convert::From<SwapCall> for DFMMCalls {
        fn from(value: SwapCall) -> Self {
            Self::Swap(value)
        }
    }
    impl ::core::convert::From<UpdateCall> for DFMMCalls {
        fn from(value: UpdateCall) -> Self {
            Self::Update(value)
        }
    }
    impl ::core::convert::From<WethCall> for DFMMCalls {
        fn from(value: WethCall) -> Self {
            Self::Weth(value)
        }
    }
    ///Container type for all return fields from the `allocate` function with signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    pub struct AllocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `deallocate` function with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    pub struct DeallocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `getPool` function with signature `getPool(uint256)` and selector `0x068bcd8d`
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
    pub struct GetPoolReturn(pub Pool);
    ///Container type for all return fields from the `getReservesAndLiquidity` function with signature `getReservesAndLiquidity(uint256)` and selector `0xce153bf4`
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
    pub struct GetReservesAndLiquidityReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `init` function with signature `init((address,address,address,bytes))` and selector `0x1455f1fc`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `liquidityOf` function with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    pub struct LiquidityOfReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `lpTokenImplementation` function with signature `lpTokenImplementation()` and selector `0xb462cd25`
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
    pub struct LpTokenImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `nonce` function with signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pools` function with signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn {
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `swap` function with signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    pub struct SwapReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    ///Container type for all return fields from the `weth` function with signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
    ///`Pool(address,address,address,uint256,uint256,uint256,address)`
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
    pub struct Pool {
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
}

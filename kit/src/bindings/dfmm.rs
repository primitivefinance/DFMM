pub use dfmm::*;
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
pub mod dfmm {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                    name: ::std::borrow::ToOwned::to_owned("weth_"),
                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                    internal_type: ::core::option::Option::Some(::std::borrow::ToOwned::to_owned(
                        "address"
                    ),),
                },],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allocate"),
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
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deallocate"),
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
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getPool"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getPool"),
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
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ::ethers::core::abi::ethabi::ParamType::Address,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IDFMM.Pool"),
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
                    ::std::borrow::ToOwned::to_owned("init"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("init"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("params"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Address,
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ],),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("struct IDFMM.InitParams"),
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
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("liquidityOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
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
                    ::std::borrow::ToOwned::to_owned("lpTokenImplementation"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lpTokenImplementation",),
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
                    ::std::borrow::ToOwned::to_owned("nonce"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonce"),
                        inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("pools"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pools"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("swap"),
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
                        ],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("update"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("update"),
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
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("weth"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("weth"),
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
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Allocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Allocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deallocate"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deallocate"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("deltaL"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Init"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
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
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveX"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("reserveY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("totalLiquidity"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Swap"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Swap"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("account"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("isSwapXForY"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("inputAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("outputAmount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("ERC1167FailedCreateClone",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Invalid"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
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
                                name: ::std::borrow::ToOwned::to_owned("swapConstantGrowth",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSwap"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapInputTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSwapInputTransfer",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSwapOutputTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidSwapOutputTransfer",),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTokens"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTransfer"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidTransfer"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Locked"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("Locked"),
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
                    ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("OnlyWETH"),
                        inputs: ::std::vec![],
                    },],
                ),
            ]),
            receive: true,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static DFMM_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R`\x01\x80U4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\08J8\x03\x80b\08J\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\0\xFCV[`\x01`\x01`\xA0\x1B\x03\x81\x16`\xA0R`@Qb\0\0S\x90b\0\0\xEEV[`@Q\x80\x91\x03\x90`\0\xF0\x80\x15\x80\x15b\0\0pW=`\0\x80>=`\0\xFD[P`\x01`\x01`\xA0\x1B\x03\x16`\x80\x81\x90R`@\x80Qc&lE\xBB`\xE1\x1B\x81R`\x04\x81\x01\x91\x90\x91R`\0`D\x82\x01\x81\x90R```$\x83\x01R`d\x82\x01RcL\xD8\x8Bv\x90`\x84\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15b\0\0\xCEW`\0\x80\xFD[PZ\xF1\x15\x80\x15b\0\0\xE3W=`\0\x80>=`\0\xFD[PPPPPb\0\x01.V[a\x0E\xB9\x80b\0)\x91\x839\x01\x90V[`\0` \x82\x84\x03\x12\x15b\0\x01\x0FW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x01'W`\0\x80\xFD[\x93\x92PPPV[`\x80Q`\xA0Qa(\x1Bb\0\x01v`\09`\0\x81\x81`\xBB\x01R\x81\x81a\x01\xF1\x01R\x81\x81a\x15r\x01R\x81\x81a\x1A2\x01Ra\x1A\x7F\x01R`\0\x81\x81a\x02\xD7\x01Ra\x06\xF1\x01Ra(\x1B`\0\xF3\xFE`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0dW\x80c\x9D\x94/\x9A\x14a\x02+W\x80c\xACJ\xFA8\x14a\x02KW\x80c\xAF\xFE\xD0\xE0\x14a\x02\xB0W\x80c\xB4b\xCD%\x14a\x02\xC5W\x80c\xBD\x06%\xAB\x14a\x02\xF9W\x80c\xCE\x15;\xF4\x14a\x03\x19W`\0\x80\xFD[\x80c\x02\x16\xB88\x14a\x01\0W\x80c\x06\x8B\xCD\x8D\x14a\x01 W\x80c\x14U\xF1\xFC\x14a\x01VW\x80c.\xC3\x81\x88\x14a\x01\x89W\x80c;\xE6\xA3A\x14a\x01\xB1W\x80c?\xC8\xCE\xF3\x14a\x01\xDFW`\0\x80\xFD[6a\0\xFBW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xF9W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x01\x0CW`\0\x80\xFD[Pa\0\xF9a\x01\x1B6`\x04a!\x1DV[a\x03TV[4\x80\x15a\x01,W`\0\x80\xFD[Pa\x01@a\x01;6`\x04a!\x99V[a\x043V[`@Qa\x01M\x91\x90a\"\x03V[`@Q\x80\x91\x03\x90\xF3[a\x01ia\x01d6`\x04a\"\x11V[a\x04\xF7V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01MV[a\x01\x9Ca\x01\x976`\x04a!\x1DV[a\nVV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01MV[4\x80\x15a\x01\xBDW`\0\x80\xFD[Pa\x01\xD1a\x01\xCC6`\x04a\"cV[a\x0C\xEFV[`@Q\x90\x81R` \x01a\x01MV[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x02\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01MV[4\x80\x15a\x027W`\0\x80\xFD[Pa\x01\x9Ca\x02F6`\x04a!\x1DV[a\x0EAV[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x02ka\x02f6`\x04a!\x99V[a\x10\xC5V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R``\x85\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x01MV[4\x80\x15a\x02\xBCW`\0\x80\xFD[P`\0Ta\x01\xD1V[4\x80\x15a\x02\xD1W`\0\x80\xFD[Pa\x02\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x05W`\0\x80\xFD[Pa\x01\x9Ca\x03\x146`\x04a!\x1DV[a\x11$V[4\x80\x15a\x03%W`\0\x80\xFD[Pa\x039a\x0346`\x04a!\x99V[a\x12\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01MV[`\x01T`\x02\x03a\x03wW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x03\x90Wa\x03\x90a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x18\x1C\xBA\xB4\x913\x91\x87\x91\x82\x90\x81\x10a\x03\xC8Wa\x03\xC8a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x86\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xF8\x95\x94\x93\x92\x91\x90a\"\xCCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04&W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x04\x7FWa\x04\x7Fa\"\x8DV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x01T\x90\x91\x16`\xC0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05 W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x055``\x86\x01`@\x87\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16a\x05N`@\x87\x01` \x88\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05uW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x90\x91R`\0\x90\x80a\x05\x91` \x89\x01\x89a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87` \x01` \x81\x01\x90a\x05\xB2\x91\x90a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xD0``\x89\x01`@\x8A\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90P`\0\x80`\0\x80`\0\x8A`\0\x01` \x81\x01\x90a\x06\x1E\x91\x90a#RV[`\x01`\x01`\xA0\x1B\x03\x16c2\xE3\x8303`\0\x80T\x90P\x89\x8F\x80``\x01\x90a\x06D\x91\x90a#mV[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06d\x95\x94\x93\x92\x91\x90a#\xBBV[`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA7\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x06\xEAW`\0\x84\x12a\x06\xC3\x85a\x13RV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\x91V[\x90P`\0a\x07Ta\x07)` \x8F\x01\x8Fa#RV[\x8E` \x01` \x81\x01\x90a\x07<\x91\x90a#RV[\x8F`@\x01` \x81\x01\x90a\x07O\x91\x90a#RV[a\x13\xFEV[`@Qc&lE\xBB`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cL\xD8\x8Bv\x90a\x07\x85\x90\x84\x90\x81\x90`\x04\x01a$\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xB3W=`\0\x80>=`\0\xFD[PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x86a\x07\xD5\x91\x90a$\xDFV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08/W=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x91W=`\0\x80>=`\0\xFD[PPPP\x84\x88``\x01\x81\x81RPP\x83\x88`\x80\x01\x81\x81RPP\x82\x88`\xA0\x01\x81\x81RPP\x81\x88`\xC0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\0\x88\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x07\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPP`\0`\x01`\0\x80T\x90Pa\t\xE5\x91\x90a$\xDFV[\x90Pa\n\x03\x8E` \x01` \x81\x01\x90a\t\xFD\x91\x90a#RV[\x87a\x15iV[a\n\x1F\x8E`@\x01` \x81\x01\x90a\n\x19\x91\x90a#RV[\x86a\x15iV[a\n)\x81\x84a\x17)V[\x80\x86\x86a\n8a\x03\xE8\x88a$\xDFV[\x9CP\x9CP\x9CP\x9CPPPPPPPPPP`\x01\x80U\x92\x94\x91\x93P\x91\x90V[`\0\x80`\x01T`\x02\x03a\n|W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\n\x9EWa\n\x9Ea\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x02J\xA2\x06\x913\x91\x8E\x91\x82\x90\x81\x10a\n\xD6Wa\n\xD6a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x06\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BG\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x0BcW`\0\x84\x12a\x06\xC3\x85a\x13RV[\x82`\0\x8B\x81T\x81\x10a\x0BwWa\x0Bwa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0B\x97\x91\x90a$\xF2V[\x92PP\x81\x90UP\x81`\0\x8B\x81T\x81\x10a\x0B\xB2Wa\x0B\xB2a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x82\x82Ta\x0B\xD2\x91\x90a$\xF2V[\x92PP\x81\x90UP\x80`\0\x8B\x81T\x81\x10a\x0B\xEDWa\x0B\xEDa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01`\0\x82\x82Ta\x0C\r\x91\x90a$\xF2V[\x90\x91UPa\x0C\x1F\x90P\x8A`\x01\x83a\x18.V[a\x0CX`\0\x8B\x81T\x81\x10a\x0C5Wa\x0C5a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x15iV[a\x0C\x91`\0\x8B\x81T\x81\x10a\x0CnWa\x0Cna\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83a\x15iV[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2P`\x01\x80U\x90\x98\x90\x97P\x95PPPPPPV[`\0\x80`\0\x83\x81T\x81\x10a\r\x05Wa\r\x05a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x91\x90\x91\x02\x01`\x06\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rcW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x87\x91\x90a%\x05V[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a%\x05V[\x90P`\0\x80\x86\x81T\x81\x10a\x0E\x03Wa\x0E\x03a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90Pa\x0E4a\x0E-\x83\x83a\x19\xFF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x1A\x1BV[\x94PPPPP[\x92\x91PPV[`\0\x80`\x01T`\x02\x03a\x0EgW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x0E\x89Wa\x0E\x89a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c9n>|\x913\x91\x8E\x91\x82\x90\x81\x10a\x0E\xC1Wa\x0E\xC1a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xF1\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F2\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x0FNW`\0\x84\x12a\x06\xC3\x85a\x13RV[\x82`\0\x8B\x81T\x81\x10a\x0FbWa\x0Fba\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0F\x82\x91\x90a$\xDFV[\x92PP\x81\x90UP\x81`\0\x8B\x81T\x81\x10a\x0F\x9DWa\x0F\x9Da\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x82\x82Ta\x0F\xBD\x91\x90a$\xDFV[\x92PP\x81\x90UP\x80`\0\x8B\x81T\x81\x10a\x0F\xD8Wa\x0F\xD8a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01`\0\x82\x82Ta\x0F\xF8\x91\x90a$\xDFV[\x90\x91UPa\x10\n\x90P\x8A`\0\x83a\x18.V[a\x10D`\0\x8B\x81T\x81\x10a\x10 Wa\x10 a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x1A0V[a\x10~`\0\x8B\x81T\x81\x10a\x10ZWa\x10Za\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x1A0V[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0C\xD4V[`\0\x81\x81T\x81\x10a\x10\xD5W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x97P\x93\x85\x16\x95\x92\x85\x16\x94\x91\x93\x90\x92\x91\x16\x87V[`\0\x80`\x01T`\x02\x03a\x11JW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x11lWa\x11la\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91ce\xC9\xFF\xC2\x913\x91\x8E\x91\x82\x90\x81\x10a\x11\xA4Wa\x11\xA4a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xD4\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x15\x91\x90a%\x1EV[\x95P\x95P\x95PP\x94P\x94P\x84a\x122W`\0\x84\x12a\x06\xC3\x85a\x13RV[\x80`\0\x8B\x81T\x81\x10a\x12FWa\x12Fa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UP`\0\x80`\0a\x12k\x8D\x87\x87a\x1C\x18V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x12\xE9Wa\x12\xE9a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T`\0\x85\x81T\x81\x10a\x13\x0EWa\x13\x0Ea\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T`\0\x86\x81T\x81\x10a\x133Wa\x133a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x13xW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13\x89WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\x8CW`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14f\x91\x90\x81\x01\x90a%\x85V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\xCC\x91\x90\x81\x01\x90a%\x85V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x152\x91\x90\x81\x01\x90a%\x85V[`\0Ta\x15>\x90a\x1E-V[`@Q` \x01a\x15Q\x94\x93\x92\x91\x90a&2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[\x80G\x10a\x15\xFBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x15\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xDFW=`\0\x80>=`\0\xFD[PPPPP`\0G\x11\x15a\x15\xF7Wa\x15\xF73Ga\x1E\x94V[PPV[`\0a\x16\x0F\x82a\x16\n\x85a\x1E\xE5V[a\x1F\x83V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16}\x91\x90a%\x05V[\x90Pa\x16\x8B\x8430\x85a\x1F\x8FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90a%\x05V[\x90Pa\x17\x02\x83\x83a$\xF2V[\x81\x10\x15a\x17\"W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\0\x80\x83\x81T\x81\x10a\x17=Wa\x17=a\"\x8DV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x85R`\x01\x83\x01T\x82\x16\x95\x85\x01\x86\x90R`\x02\x83\x01T\x82\x16\x85\x85\x01\x81\x90R`\x03\x84\x01T``\x87\x01\x81\x90R`\x04\x85\x01T`\x80\x88\x01\x81\x90R`\x05\x86\x01T`\xA0\x89\x01\x81\x90R`\x06\x90\x96\x01T\x90\x94\x16`\xC0\x88\x01R\x94Q\x95\x97P\x95\x943\x94\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E\x94a\x18!\x94\x8B\x93\x8D\x93\x92\x90`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x90\x95\x16` \x85\x01R`@\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x80\x84\x81T\x81\x10a\x18BWa\x18Ba\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCD\x91\x90a%\x05V[\x90P`\0\x80\x86\x81T\x81\x10a\x18\xE3Wa\x18\xE3a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90P\x84\x15a\x19|W`\0a\x19\x13a\x19\x0C\x84\x84a\x19\xFFV[\x86\x90a\x1A\x1BV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19rW=`\0\x80>=`\0\xFD[PPPPPa\x19\xF7V[`\0a\x19\x92a\x19\x8B\x84\x84a \x16V[\x86\x90a +V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xF1W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[`\0a\x1A\x14\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a @V[\x93\x92PPPV[`\0a\x1A\x14\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a @V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A\xF2W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xDFW=`\0\x80>=`\0\xFD[PPPPa\x1A\xED\x82\x82a\x1E\x94V[PPPV[`\0a\x1B\x06\x82a\x1B\x01\x86a\x1E\xE5V[a _V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bt\x91\x90a%\x05V[\x90Pa\x1B\x81\x85\x85\x84a kV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xEC\x91\x90a%\x05V[\x90Pa\x1B\xF8\x83\x83a$\xDFV[\x81\x10\x15a\x19\xF7W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x1C4Wa\x1C4a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x1C\\Wa\x1C\\a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x1D\x1CW\x80\x88\x10a\x1C\x9BW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x1C\xAEWa\x1C\xAEa\"\x8DV[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x1C\xE0Wa\x1C\xE0a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x1D\t\x82\x8Aa$\xDFV[\x93Pa\x1D\x15\x88\x82a$\xDFV[\x92Pa\x1D\xB9V[\x81\x89\x10a\x1D<W`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x1DOWa\x1DOa\"\x8DV[`\0\x91\x82R` \x82 `\x02`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x1D\x81Wa\x1D\x81a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x1D\xAA\x81\x89a$\xDFV[\x93Pa\x1D\xB6\x89\x83a$\xDFV[\x92P[\x88`\0\x8B\x81T\x81\x10a\x1D\xCDWa\x1D\xCDa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1D\xF6Wa\x1D\xF6a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01\x81\x90UPa\x1E\x15\x86\x85a\x15iV[a\x1E \x853\x85a\x1A0V[PP\x93\x97\x92\x96P\x93P\x93PV[``\x81`\0\x03a\x1ETWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`N`@Q\x91P\x80\x82R`\x80\x82\x01`@R[\x82\x15a\x1E\x85W`\n\x80\x84\x06`0\x01\x82\x84\x01R\x90\x92\x04\x91`\0\x19\x01a\x1EfV[`N\x81\x90\x03\x91\x01\x90\x81R\x91\x90PV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x1A\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x06\xE1V[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FJ\x91\x90a&\xBBV[`\xFF\x16\x90P`\0a\x1F\\\x82`\x12a$\xDFV[\x90Pa\x1Fi\x81`\na'\xC2V[a\x1F{\x90g\r\xE0\xB6\xB3\xA7d\0\0a'\xCEV[\x94\x93PPPPV[`\0a\x1A\x14\x83\x83a \x16V[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x06\xE1V[`\0a\x1A\x14\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \xEFV[`\0a\x1A\x14\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \xEFV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a XW`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x1A\x14\x83\x83a\x19\xFFV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a \xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x06\xE1V[PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!\x07W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!2W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!QW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!eW`\0\x80\xFD[\x815\x81\x81\x11\x15a!tW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\x86W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\xABW`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x82\x16\x90\x84\x01R``\x80\x83\x01Q\x90\x84\x01R`\x80\x80\x83\x01Q\x90\x84\x01R`\xA0\x82\x81\x01Q\x90\x84\x01R`\xC0\x91\x82\x01Q\x16\x91\x01RV[`\xE0\x81\x01a\x0E;\x82\x84a!\xB2V[`\0` \x82\x84\x03\x12\x15a\"#W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\":W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x1A\x14W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x8CW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\"vW`\0\x80\xFD[a\"\x7F\x83a\"LV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R` \x82\x01\x86\x90R\x84T\x81\x16`@\x83\x01R`\x01\x85\x01T\x81\x16``\x83\x01R`\x02\x85\x01T\x81\x16`\x80\x83\x01R`\x03\x85\x01T`\xA0\x83\x01R`\x04\x85\x01T`\xC0\x83\x01R`\x05\x85\x01T`\xE0\x83\x01R`\x06\x85\x01T\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R`\0\x90a#F\x81\x84\x01\x85\x87a\"\xA3V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a#dW`\0\x80\xFD[a\x1A\x14\x82a\"LV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#\x84W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#\x9FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a#\xB4W`\0\x80\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R` \x81\x01\x85\x90R`\0a\x01@a#\xE0`@\x84\x01\x87a!\xB2V[\x80a\x01 \x84\x01Ra#F\x81\x84\x01\x85\x87a\"\xA3V[\x80Q\x80\x15\x15\x81\x14a\x13\x8CW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a$\x1CW`\0\x80\xFD[a$%\x86a#\xF4V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a$fW\x81\x81\x01Q\x83\x82\x01R` \x01a$NV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\x87\x81` \x86\x01` \x86\x01a$KV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a$\xAE`@\x83\x01\x85a$oV[\x82\x81\x03` \x84\x01Ra$\xC0\x81\x85a$oV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E;Wa\x0E;a$\xC9V[\x80\x82\x01\x80\x82\x11\x15a\x0E;Wa\x0E;a$\xC9V[`\0` \x82\x84\x03\x12\x15a%\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%7W`\0\x80\xFD[a%@\x87a#\xF4V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a%\x97W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xAFW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a%\xC3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a%\xD5Wa%\xD5a%oV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a%\xFDWa%\xFDa%oV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a&\x16W`\0\x80\xFD[a&'\x83` \x83\x01` \x88\x01a$KV[\x97\x96PPPPPPPV[dDFMM-`\xD8\x1B\x81R`\0\x85Qa&R\x81`\x05\x85\x01` \x8A\x01a$KV[\x80\x83\x01\x90P`-`\xF8\x1B\x80`\x05\x83\x01R\x86Qa&u\x81`\x06\x85\x01` \x8B\x01a$KV[`\x06\x92\x01\x91\x82\x01\x81\x90R\x85Qa&\x92\x81`\x07\x85\x01` \x8A\x01a$KV[`\x07\x92\x01\x91\x82\x01R\x83Qa&\xAD\x81`\x08\x84\x01` \x88\x01a$KV[\x01`\x08\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a&\xCDW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1A\x14W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a'\x19W\x81`\0\x19\x04\x82\x11\x15a&\xFFWa&\xFFa$\xC9V[\x80\x85\x16\x15a'\x0CW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a&\xE3V[P\x92P\x92\x90PV[`\0\x82a'0WP`\x01a\x0E;V[\x81a'=WP`\0a\x0E;V[\x81`\x01\x81\x14a'SW`\x02\x81\x14a']Wa'yV[`\x01\x91PPa\x0E;V[`\xFF\x84\x11\x15a'nWa'na$\xC9V[PP`\x01\x82\x1Ba\x0E;V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a'\x9CWP\x81\x81\na\x0E;V[a'\xA6\x83\x83a&\xDEV[\x80`\0\x19\x04\x82\x11\x15a'\xBAWa'\xBAa$\xC9V[\x02\x93\x92PPPV[`\0a\x1A\x14\x83\x83a'!V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E;Wa\x0E;a$\xC9V\xFE\xA2dipfsX\"\x12 \xB2\xD9]{\xE6f\x95B\x9C\xE7\x0E\xFCj\xCF\x1D\x15k6p}\xAF\xF2\x01\x99z\x9D\0Q\x1B\xF8(\x1FdsolcC\0\x08\x16\x003`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[Pa\x0E\x99\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\x0BW`\x005`\xE0\x1C\x80cL\xD8\x8Bv\x11a\0\xA2W\x80c\x9D\xC2\x9F\xAC\x11a\0qW\x80c\x9D\xC2\x9F\xAC\x14a\x02!W\x80c\xA9\x05\x9C\xBB\x14a\x024W\x80c\xAF\xBA\x13\xC4\x14a\x02GW\x80c\xD5\x05\xAC\xCF\x14a\x02rW\x80c\xDDb\xED>\x14a\x02\x85W`\0\x80\xFD[\x80cL\xD8\x8Bv\x14a\x01\xC6W\x80cp\xA0\x821\x14a\x01\xD9W\x80c~\xCE\xBE\0\x14a\x01\xF9W\x80c\x95\xD8\x9BA\x14a\x02\x19W`\0\x80\xFD[\x80c#\xB8r\xDD\x11a\0\xDEW\x80c#\xB8r\xDD\x14a\x01|W\x80c1<\xE5g\x14a\x01\x8FW\x80c6D\xE5\x15\x14a\x01\xA9W\x80c@\xC1\x0F\x19\x14a\x01\xB1W`\0\x80\xFD[\x80c\x06\xFD\xDE\x03\x14a\x01\x10W\x80c\t^\xA7\xB3\x14a\x01.W\x80c\x15\x8E\xF9>\x14a\x01QW\x80c\x18\x16\r\xDD\x14a\x01eW[`\0\x80\xFD[a\x01\x18a\x02\xB0V[`@Qa\x01%\x91\x90a\t\xA6V[`@Q\x80\x91\x03\x90\xF3[a\x01Aa\x01<6`\x04a\n\x11V[a\x03>V[`@Q\x90\x15\x15\x81R` \x01a\x01%V[`\x08Ta\x01A\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x01n`\x02T\x81V[`@Q\x90\x81R` \x01a\x01%V[a\x01Aa\x01\x8A6`\x04a\n;V[a\x03\xABV[a\x01\x97`\x12\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x01%V[a\x01na\x04\x8BV[a\x01\xC4a\x01\xBF6`\x04a\n\x11V[a\x04\xAAV[\0[a\x01\xC4a\x01\xD46`\x04a\x0B\x1AV[a\x04\xE3V[a\x01na\x01\xE76`\x04a\x0B~V[`\x03` R`\0\x90\x81R`@\x90 T\x81V[a\x01na\x02\x076`\x04a\x0B~V[`\x07` R`\0\x90\x81R`@\x90 T\x81V[a\x01\x18a\x05_V[a\x01\xC4a\x02/6`\x04a\n\x11V[a\x05lV[a\x01Aa\x02B6`\x04a\n\x11V[a\x05\xA1V[`\x08Ta\x02Z\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01%V[a\x01\xC4a\x02\x806`\x04a\x0B\xA0V[a\x06\x07V[a\x01na\x02\x936`\x04a\x0C\x13V[`\x04` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`\0\x80Ta\x02\xBD\x90a\x0CFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xE9\x90a\x0CFV[\x80\x15a\x036W\x80`\x1F\x10a\x03\x0BWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x036V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03\x19W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[3`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 `\x01`\x01`\xA0\x1B\x03\x87\x16\x80\x85R\x92R\x80\x83 \x85\x90UQ\x91\x92\x90\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90a\x03\x99\x90\x86\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01[\x92\x91PPV[`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x81 T`\0\x19\x81\x14a\x04\x07Wa\x03\xE2\x83\x82a\x0C\x96V[`\x01`\x01`\xA0\x1B\x03\x86\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 U[`\x01`\x01`\xA0\x1B\x03\x85\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x85\x92\x90a\x04/\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x87\x01\x90UQ\x90\x91\x87\x16\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90a\x04x\x90\x87\x81R` \x01\x90V[`@Q\x80\x91\x03\x90\xA3P`\x01\x94\x93PPPPV[`\0`\x05TF\x14a\x04\xA3Wa\x04\x9Ea\x08PV[\x90P\x90V[P`\x06T\x90V[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x04\xD5W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xDF\x82\x82a\x08\xEAV[PPV[`\x08T`\x01`\xA0\x1B\x90\x04`\xFF\x16\x15a\x05\rW`@Qb\xDC\x14\x9F`\xE4\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x08\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x17\x90U`\0a\x05+\x83\x82a\x0C\xFAV[P`\x01a\x058\x82\x82a\x0C\xFAV[PF`\x05Ua\x05Ea\x08PV[`\x06UPP`\x08\x80T`\xFF`\xA0\x1B\x19\x16`\x01`\xA0\x1B\x17\x90UV[`\x01\x80Ta\x02\xBD\x90a\x0CFV[`\x08T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x05\x97W`@QchS\xCB\xA7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04\xDF\x82\x82a\tDV[3`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x91\x90\x83\x90a\x05\xC2\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x83\x16`\0\x81\x81R`\x03` R`@\x90\x81\x90 \x80T\x85\x01\x90UQ3\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90a\x03\x99\x90\x86\x81R` \x01\x90V[B\x84\x10\x15a\x06\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0`\x01a\x06ha\x04\x8BV[`\x01`\x01`\xA0\x1B\x03\x8A\x81\x16`\0\x81\x81R`\x07` \x90\x81R`@\x91\x82\x90 \x80T`\x01\x81\x01\x90\x91U\x82Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x81\x84\x01R\x80\x84\x01\x94\x90\x94R\x93\x8D\x16``\x84\x01R`\x80\x83\x01\x8C\x90R`\xA0\x83\x01\x93\x90\x93R`\xC0\x80\x83\x01\x8B\x90R\x81Q\x80\x84\x03\x90\x91\x01\x81R`\xE0\x83\x01\x90\x91R\x80Q\x92\x01\x91\x90\x91 a\x19\x01`\xF0\x1Ba\x01\0\x83\x01Ra\x01\x02\x82\x01\x92\x90\x92Ra\x01\"\x81\x01\x91\x90\x91Ra\x01B\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 `\0\x84R\x90\x83\x01\x80\x83RR`\xFF\x87\x16\x90\x82\x01R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\x07tW=`\0\x80>=`\0\xFD[PP`@Q`\x1F\x19\x01Q\x91PP`\x01`\x01`\xA0\x1B\x03\x81\x16\x15\x80\x15\x90a\x07\xAAWP\x87`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14[a\x07\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x01a\x06SV[`\x01`\x01`\xA0\x1B\x03\x90\x81\x16`\0\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x8A\x85\x16\x80\x85R\x90\x83R\x92\x81\x90 \x89\x90UQ\x88\x81R\x91\x92\x8A\x16\x91\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91\x01`@Q\x80\x91\x03\x90\xA3PPPPPPPV[`\0\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F`\0`@Qa\x08\x82\x91\x90a\r\xBAV[`@\x80Q\x91\x82\x90\x03\x82 ` \x83\x01\x93\x90\x93R\x81\x01\x91\x90\x91R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x82\x01RF`\x80\x82\x01R0`\xA0\x82\x01R`\xC0\x01`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x90V[\x80`\x02`\0\x82\x82Ta\x08\xFC\x91\x90a\x0E0V[\x90\x91UPP`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x81\x81R`\x03` \x90\x81R`@\x80\x83 \x80T\x86\x01\x90UQ\x84\x81R`\0\x80Q` a\x0ED\x839\x81Q\x91R\x91\x01[`@Q\x80\x91\x03\x90\xA3PPV[`\x01`\x01`\xA0\x1B\x03\x82\x16`\0\x90\x81R`\x03` R`@\x81 \x80T\x83\x92\x90a\tl\x90\x84\x90a\x0C\x96V[\x90\x91UPP`\x02\x80T\x82\x90\x03\x90U`@Q\x81\x81R`\0\x90`\x01`\x01`\xA0\x1B\x03\x84\x16\x90`\0\x80Q` a\x0ED\x839\x81Q\x91R\x90` \x01a\t8V[`\0` \x80\x83R\x83Q\x80` \x85\x01R`\0[\x81\x81\x10\x15a\t\xD4W\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a\t\xB8V[P`\0`@\x82\x86\x01\x01R`@`\x1F\x19`\x1F\x83\x01\x16\x85\x01\x01\x92PPP\x92\x91PPV[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\n\x0CW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\n$W`\0\x80\xFD[a\n-\x83a\t\xF5V[\x94` \x93\x90\x93\x015\x93PPPV[`\0\x80`\0``\x84\x86\x03\x12\x15a\nPW`\0\x80\xFD[a\nY\x84a\t\xF5V[\x92Pa\ng` \x85\x01a\t\xF5V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x82`\x1F\x83\x01\x12a\n\x9EW`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\n\xB9Wa\n\xB9a\nwV[`@Q`\x1F\x83\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x82\x82\x11\x81\x83\x10\x17\x15a\n\xE1Wa\n\xE1a\nwV[\x81`@R\x83\x81R\x86` \x85\x88\x01\x01\x11\x15a\n\xFAW`\0\x80\xFD[\x83` \x87\x01` \x83\x017`\0` \x85\x83\x01\x01R\x80\x94PPPPP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0B-W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x0BEW`\0\x80\xFD[a\x0BQ\x86\x83\x87\x01a\n\x8DV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a\x0BgW`\0\x80\xFD[Pa\x0Bt\x85\x82\x86\x01a\n\x8DV[\x91PP\x92P\x92\x90PV[`\0` \x82\x84\x03\x12\x15a\x0B\x90W`\0\x80\xFD[a\x0B\x99\x82a\t\xF5V[\x93\x92PPPV[`\0\x80`\0\x80`\0\x80`\0`\xE0\x88\x8A\x03\x12\x15a\x0B\xBBW`\0\x80\xFD[a\x0B\xC4\x88a\t\xF5V[\x96Pa\x0B\xD2` \x89\x01a\t\xF5V[\x95P`@\x88\x015\x94P``\x88\x015\x93P`\x80\x88\x015`\xFF\x81\x16\x81\x14a\x0B\xF6W`\0\x80\xFD[\x96\x99\x95\x98P\x93\x96\x92\x95\x94`\xA0\x84\x015\x94P`\xC0\x90\x93\x015\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x0C&W`\0\x80\xFD[a\x0C/\x83a\t\xF5V[\x91Pa\x0C=` \x84\x01a\t\xF5V[\x90P\x92P\x92\x90PV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x0CZW`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0CzWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x03\xA5Wa\x03\xA5a\x0C\x80V[`\x1F\x82\x11\x15a\x0C\xF5W`\0\x81`\0R` `\0 `\x1F\x85\x01`\x05\x1C\x81\x01` \x86\x10\x15a\x0C\xD2WP\x80[`\x1F\x85\x01`\x05\x1C\x82\x01\x91P[\x81\x81\x10\x15a\x0C\xF1W\x82\x81U`\x01\x01a\x0C\xDEV[PPP[PPPV[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\r\x14Wa\r\x14a\nwV[a\r(\x81a\r\"\x84Ta\x0CFV[\x84a\x0C\xA9V[` \x80`\x1F\x83\x11`\x01\x81\x14a\r]W`\0\x84\x15a\rEWP\x85\x83\x01Q[`\0\x19`\x03\x86\x90\x1B\x1C\x19\x16`\x01\x85\x90\x1B\x17\x85Ua\x0C\xF1V[`\0\x85\x81R` \x81 `\x1F\x19\x86\x16\x91[\x82\x81\x10\x15a\r\x8CW\x88\x86\x01Q\x82U\x94\x84\x01\x94`\x01\x90\x91\x01\x90\x84\x01a\rmV[P\x85\x82\x10\x15a\r\xAAW\x87\x85\x01Q`\0\x19`\x03\x88\x90\x1B`\xF8\x16\x1C\x19\x16\x81U[PPPPP`\x01\x90\x81\x1B\x01\x90UPV[`\0\x80\x83Ta\r\xC8\x81a\x0CFV[`\x01\x82\x81\x16\x80\x15a\r\xE0W`\x01\x81\x14a\r\xF5Wa\x0E$V[`\xFF\x19\x84\x16\x87R\x82\x15\x15\x83\x02\x87\x01\x94Pa\x0E$V[\x87`\0R` \x80`\0 `\0[\x85\x81\x10\x15a\x0E\x1BW\x81T\x8A\x82\x01R\x90\x84\x01\x90\x82\x01a\x0E\x02V[PPP\x82\x87\x01\x94P[P\x92\x96\x95PPPPPPV[\x80\x82\x01\x80\x82\x11\x15a\x03\xA5Wa\x03\xA5a\x0C\x80V\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \x1C_\xD6\xC2\x12\x1C\x14y\xF4\xB8\xFD;\xBCL\x90TB\xA6\xB5\xA9\xA1\xBE\x15\xB8)\x06&\x0B\x05\x89\xD2\xDBdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static DFMM_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\xABW`\x005`\xE0\x1C\x80c\x9D\x94/\x9A\x11a\0dW\x80c\x9D\x94/\x9A\x14a\x02+W\x80c\xACJ\xFA8\x14a\x02KW\x80c\xAF\xFE\xD0\xE0\x14a\x02\xB0W\x80c\xB4b\xCD%\x14a\x02\xC5W\x80c\xBD\x06%\xAB\x14a\x02\xF9W\x80c\xCE\x15;\xF4\x14a\x03\x19W`\0\x80\xFD[\x80c\x02\x16\xB88\x14a\x01\0W\x80c\x06\x8B\xCD\x8D\x14a\x01 W\x80c\x14U\xF1\xFC\x14a\x01VW\x80c.\xC3\x81\x88\x14a\x01\x89W\x80c;\xE6\xA3A\x14a\x01\xB1W\x80c?\xC8\xCE\xF3\x14a\x01\xDFW`\0\x80\xFD[6a\0\xFBW3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\0\xF9W`@Qc\x01\xF1\x80\xC9`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\0[`\0\x80\xFD[4\x80\x15a\x01\x0CW`\0\x80\xFD[Pa\0\xF9a\x01\x1B6`\x04a!\x1DV[a\x03TV[4\x80\x15a\x01,W`\0\x80\xFD[Pa\x01@a\x01;6`\x04a!\x99V[a\x043V[`@Qa\x01M\x91\x90a\"\x03V[`@Q\x80\x91\x03\x90\xF3[a\x01ia\x01d6`\x04a\"\x11V[a\x04\xF7V[`@\x80Q\x94\x85R` \x85\x01\x93\x90\x93R\x91\x83\x01R``\x82\x01R`\x80\x01a\x01MV[a\x01\x9Ca\x01\x976`\x04a!\x1DV[a\nVV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x01MV[4\x80\x15a\x01\xBDW`\0\x80\xFD[Pa\x01\xD1a\x01\xCC6`\x04a\"cV[a\x0C\xEFV[`@Q\x90\x81R` \x01a\x01MV[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x02\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01MV[4\x80\x15a\x027W`\0\x80\xFD[Pa\x01\x9Ca\x02F6`\x04a!\x1DV[a\x0EAV[4\x80\x15a\x02WW`\0\x80\xFD[Pa\x02ka\x02f6`\x04a!\x99V[a\x10\xC5V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x98\x89\x16\x81R\x96\x88\x16` \x88\x01R\x94\x87\x16\x94\x86\x01\x94\x90\x94R``\x85\x01\x92\x90\x92R`\x80\x84\x01R`\xA0\x83\x01R\x90\x91\x16`\xC0\x82\x01R`\xE0\x01a\x01MV[4\x80\x15a\x02\xBCW`\0\x80\xFD[P`\0Ta\x01\xD1V[4\x80\x15a\x02\xD1W`\0\x80\xFD[Pa\x02\x13\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[4\x80\x15a\x03\x05W`\0\x80\xFD[Pa\x01\x9Ca\x03\x146`\x04a!\x1DV[a\x11$V[4\x80\x15a\x03%W`\0\x80\xFD[Pa\x039a\x0346`\x04a!\x99V[a\x12\xD2V[`@\x80Q\x93\x84R` \x84\x01\x92\x90\x92R\x90\x82\x01R``\x01a\x01MV[`\x01T`\x02\x03a\x03wW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01U`\0\x80T\x84\x90\x81\x10a\x03\x90Wa\x03\x90a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x18\x1C\xBA\xB4\x913\x91\x87\x91\x82\x90\x81\x10a\x03\xC8Wa\x03\xC8a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x86\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03\xF8\x95\x94\x93\x92\x91\x90a\"\xCCV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x04\x12W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x04&W=`\0\x80>=`\0\xFD[PP`\x01\x80UPPPPPV[`@\x80Q`\xE0\x81\x01\x82R`\0\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x91\x90\x91R`\0\x82\x81T\x81\x10a\x04\x7FWa\x04\x7Fa\"\x8DV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x84R`\x01\x82\x01T\x81\x16\x94\x84\x01\x94\x90\x94R`\x02\x81\x01T\x84\x16\x91\x83\x01\x91\x90\x91R`\x03\x81\x01T``\x83\x01R`\x04\x81\x01T`\x80\x83\x01R`\x05\x81\x01T`\xA0\x83\x01R`\x06\x01T\x90\x91\x16`\xC0\x82\x01R\x92\x91PPV[`\0\x80`\0\x80`\x01T`\x02\x03a\x05 W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01Ua\x055``\x86\x01`@\x87\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16a\x05N`@\x87\x01` \x88\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x03a\x05uW`@Qc3\x91\n\xEF`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\xE0\x81\x01\x90\x91R`\0\x90\x80a\x05\x91` \x89\x01\x89a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01\x87` \x01` \x81\x01\x90a\x05\xB2\x91\x90a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01a\x05\xD0``\x89\x01`@\x8A\x01a#RV[`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0\x81R` \x01`\0`\x01`\x01`\xA0\x1B\x03\x16\x81RP\x90P`\0\x80`\0\x80`\0\x8A`\0\x01` \x81\x01\x90a\x06\x1E\x91\x90a#RV[`\x01`\x01`\xA0\x1B\x03\x16c2\xE3\x8303`\0\x80T\x90P\x89\x8F\x80``\x01\x90a\x06D\x91\x90a#mV[`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x06d\x95\x94\x93\x92\x91\x90a#\xBBV[`\xA0`@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x06\x83W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xA7\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x06\xEAW`\0\x84\x12a\x06\xC3\x85a\x13RV[`@Qcw`m)`\xE1\x1B\x81R\x91\x15\x15`\x04\x83\x01R`$\x82\x01R`D\x01[`@Q\x80\x91\x03\x90\xFD[`\0a\x07\x15\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x13\x91V[\x90P`\0a\x07Ta\x07)` \x8F\x01\x8Fa#RV[\x8E` \x01` \x81\x01\x90a\x07<\x91\x90a#RV[\x8F`@\x01` \x81\x01\x90a\x07O\x91\x90a#RV[a\x13\xFEV[`@Qc&lE\xBB`\xE1\x1B\x81R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x83\x16\x90cL\xD8\x8Bv\x90a\x07\x85\x90\x84\x90\x81\x90`\x04\x01a$\x9BV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\x9FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xB3W=`\0\x80>=`\0\xFD[PPPP\x81`\x01`\x01`\xA0\x1B\x03\x16c@\xC1\x0F\x193a\x03\xE8\x86a\x07\xD5\x91\x90a$\xDFV[`@Q`\x01`\x01`\xE0\x1B\x03\x19`\xE0\x85\x90\x1B\x16\x81R`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`\x04\x83\x01R`$\x82\x01R`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x1BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08/W=`\0\x80>=`\0\xFD[PP`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R`\0`\x04\x82\x01Ra\x03\xE8`$\x82\x01R`\x01`\x01`\xA0\x1B\x03\x85\x16\x92Pc@\xC1\x0F\x19\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08}W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\x91W=`\0\x80>=`\0\xFD[PPPP\x84\x88``\x01\x81\x81RPP\x83\x88`\x80\x01\x81\x81RPP\x82\x88`\xA0\x01\x81\x81RPP\x81\x88`\xC0\x01\x90`\x01`\x01`\xA0\x1B\x03\x16\x90\x81`\x01`\x01`\xA0\x1B\x03\x16\x81RPP`\0\x88\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90`\0R` `\0 \x90`\x07\x02\x01`\0\x90\x91\x90\x91\x90\x91P`\0\x82\x01Q\x81`\0\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP` \x82\x01Q\x81`\x01\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP`@\x82\x01Q\x81`\x02\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP``\x82\x01Q\x81`\x03\x01U`\x80\x82\x01Q\x81`\x04\x01U`\xA0\x82\x01Q\x81`\x05\x01U`\xC0\x82\x01Q\x81`\x06\x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UPPP`\0`\x01`\0\x80T\x90Pa\t\xE5\x91\x90a$\xDFV[\x90Pa\n\x03\x8E` \x01` \x81\x01\x90a\t\xFD\x91\x90a#RV[\x87a\x15iV[a\n\x1F\x8E`@\x01` \x81\x01\x90a\n\x19\x91\x90a#RV[\x86a\x15iV[a\n)\x81\x84a\x17)V[\x80\x86\x86a\n8a\x03\xE8\x88a$\xDFV[\x9CP\x9CP\x9CP\x9CPPPPPPPPPP`\x01\x80U\x92\x94\x91\x93P\x91\x90V[`\0\x80`\x01T`\x02\x03a\n|W`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\n\x9EWa\n\x9Ea\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c\x02J\xA2\x06\x913\x91\x8E\x91\x82\x90\x81\x10a\n\xD6Wa\n\xD6a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0B\x06\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0B#W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0BG\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x0BcW`\0\x84\x12a\x06\xC3\x85a\x13RV[\x82`\0\x8B\x81T\x81\x10a\x0BwWa\x0Bwa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0B\x97\x91\x90a$\xF2V[\x92PP\x81\x90UP\x81`\0\x8B\x81T\x81\x10a\x0B\xB2Wa\x0B\xB2a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x82\x82Ta\x0B\xD2\x91\x90a$\xF2V[\x92PP\x81\x90UP\x80`\0\x8B\x81T\x81\x10a\x0B\xEDWa\x0B\xEDa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01`\0\x82\x82Ta\x0C\r\x91\x90a$\xF2V[\x90\x91UPa\x0C\x1F\x90P\x8A`\x01\x83a\x18.V[a\x0CX`\0\x8B\x81T\x81\x10a\x0C5Wa\x0C5a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x84a\x15iV[a\x0C\x91`\0\x8B\x81T\x81\x10a\x0CnWa\x0Cna\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x83a\x15iV[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\x95\x97W\x7F3\x93 w^c\xD3\xFE\xD7\xD5\xDD\xE66[\xAD\xCC\x9F\xCC\xDBf\xB3H\x94c\x0C\xA9\x8Bo\x90`\x80\x01[`@Q\x80\x91\x03\x90\xA2P`\x01\x80U\x90\x98\x90\x97P\x95PPPPPPV[`\0\x80`\0\x83\x81T\x81\x10a\r\x05Wa\r\x05a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x91\x90\x91\x02\x01`\x06\x01T`@Qcp\xA0\x821`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`\x04\x83\x01R\x90\x91\x16\x92P\x82\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\rcW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\x87\x91\x90a%\x05V[\x90P`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a%\x05V[\x90P`\0\x80\x86\x81T\x81\x10a\x0E\x03Wa\x0E\x03a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90Pa\x0E4a\x0E-\x83\x83a\x19\xFF\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x84\x90a\x1A\x1BV[\x94PPPPP[\x92\x91PPV[`\0\x80`\x01T`\x02\x03a\x0EgW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x0E\x89Wa\x0E\x89a\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91c9n>|\x913\x91\x8E\x91\x82\x90\x81\x10a\x0E\xC1Wa\x0E\xC1a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x0E\xF1\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xA0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x0EW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0F2\x91\x90a$\x04V[\x94P\x94P\x94P\x94P\x94P\x84a\x0FNW`\0\x84\x12a\x06\xC3\x85a\x13RV[\x82`\0\x8B\x81T\x81\x10a\x0FbWa\x0Fba\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01`\0\x82\x82Ta\x0F\x82\x91\x90a$\xDFV[\x92PP\x81\x90UP\x81`\0\x8B\x81T\x81\x10a\x0F\x9DWa\x0F\x9Da\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01`\0\x82\x82Ta\x0F\xBD\x91\x90a$\xDFV[\x92PP\x81\x90UP\x80`\0\x8B\x81T\x81\x10a\x0F\xD8Wa\x0F\xD8a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01`\0\x82\x82Ta\x0F\xF8\x91\x90a$\xDFV[\x90\x91UPa\x10\n\x90P\x8A`\0\x83a\x18.V[a\x10D`\0\x8B\x81T\x81\x10a\x10 Wa\x10 a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x85a\x1A0V[a\x10~`\0\x8B\x81T\x81\x10a\x10ZWa\x10Za\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x163\x84a\x1A0V[`@\x80Q\x8B\x81R` \x81\x01\x85\x90R\x90\x81\x01\x83\x90R``\x81\x01\x82\x90R3\x90\x7F\xAC\xBE\x12~\x93\xA8\xA0\xB2x\xD8\xE0n' [=\xF9\xD1\xF3\x81$\x14\xBC\x89\x17\xC7t\xA87\x101n\x90`\x80\x01a\x0C\xD4V[`\0\x81\x81T\x81\x10a\x10\xD5W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 `\x07\x90\x91\x02\x01\x80T`\x01\x82\x01T`\x02\x83\x01T`\x03\x84\x01T`\x04\x85\x01T`\x05\x86\x01T`\x06\x90\x96\x01T`\x01`\x01`\xA0\x1B\x03\x95\x86\x16\x97P\x93\x85\x16\x95\x92\x85\x16\x94\x91\x93\x90\x92\x91\x16\x87V[`\0\x80`\x01T`\x02\x03a\x11JW`@Qc\x03\xCB\x96\xDB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\x01\x81\x90UP`\0\x80`\0\x80`\0\x80\x8A\x81T\x81\x10a\x11lWa\x11la\"\x8DV[`\0\x91\x82R` \x82 `\x07\x90\x91\x02\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x91ce\xC9\xFF\xC2\x913\x91\x8E\x91\x82\x90\x81\x10a\x11\xA4Wa\x11\xA4a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01\x8D\x8D`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x11\xD4\x95\x94\x93\x92\x91\x90a\"\xCCV[`\xC0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11\xF1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x12\x15\x91\x90a%\x1EV[\x95P\x95P\x95PP\x94P\x94P\x84a\x122W`\0\x84\x12a\x06\xC3\x85a\x13RV[\x80`\0\x8B\x81T\x81\x10a\x12FWa\x12Fa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01\x81\x90UP`\0\x80`\0a\x12k\x8D\x87\x87a\x1C\x18V[`@\x80Q\x86\x15\x15\x81R` \x81\x01\x84\x90R\x90\x81\x01\x82\x90R\x94\x97P\x90\x95P\x93P\x8F\x923\x92P\x7FL}\xEF\x84\xE4++\xC0\xA5\xAA\xB2\"\x86\x8D\xD7\xA0\x92\xB53w\xA4\xB57\xAB\xCD\x944Zz\x85'\xED\x91P``\x01`@Q\x80\x91\x03\x90\xA3`\x01\x80U\x90\x9C\x90\x9BP\x99PPPPPPPPPPV[`\0\x80`\0\x80\x84\x81T\x81\x10a\x12\xE9Wa\x12\xE9a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T`\0\x85\x81T\x81\x10a\x13\x0EWa\x13\x0Ea\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T`\0\x86\x81T\x81\x10a\x133Wa\x133a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x92P\x92P\x92P\x91\x93\x90\x92PV[`\0`\x01`\xFF\x1B\x82\x03a\x13xW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x13\x89WP\x19`\x01\x01\x90V[P\x80[\x91\x90PV[`\0v=`-\x80`\n=9\x81\xF36==7===6=s\0\0\0\x82``\x1B`\xE8\x1C\x17`\0RnZ\xF4=\x82\x80>\x90=\x91`+W\xFD[\xF3\x82`x\x1B\x17` R`7`\t`\0\xF0\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x13\x8CW`@Qc0\xBE\x1A=`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[``\x83`\x01`\x01`\xA0\x1B\x03\x16c\x06\xFD\xDE\x03`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14>W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14f\x91\x90\x81\x01\x90a%\x85V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14\xA4W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x14\xCC\x91\x90\x81\x01\x90a%\x85V[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x95\xD8\x9BA`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x15\nW=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x152\x91\x90\x81\x01\x90a%\x85V[`\0Ta\x15>\x90a\x1E-V[`@Q` \x01a\x15Q\x94\x93\x92\x91\x90a&2V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x93\x92PPPV[\x80G\x10a\x15\xFBW\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xD0\xE3\r\xB0\x82`@Q\x82c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x15\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\xDFW=`\0\x80>=`\0\xFD[PPPPP`\0G\x11\x15a\x15\xF7Wa\x15\xF73Ga\x1E\x94V[PPV[`\0a\x16\x0F\x82a\x16\n\x85a\x1E\xE5V[a\x1F\x83V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16}\x91\x90a%\x05V[\x90Pa\x16\x8B\x8430\x85a\x1F\x8FV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x16\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x16\xF6\x91\x90a%\x05V[\x90Pa\x17\x02\x83\x83a$\xF2V[\x81\x10\x15a\x17\"W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\0\x80\x83\x81T\x81\x10a\x17=Wa\x17=a\"\x8DV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q`\xE0\x81\x01\x82R`\x07\x90\x93\x02\x90\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x80\x85R`\x01\x83\x01T\x82\x16\x95\x85\x01\x86\x90R`\x02\x83\x01T\x82\x16\x85\x85\x01\x81\x90R`\x03\x84\x01T``\x87\x01\x81\x90R`\x04\x85\x01T`\x80\x88\x01\x81\x90R`\x05\x86\x01T`\xA0\x89\x01\x81\x90R`\x06\x90\x96\x01T\x90\x94\x16`\xC0\x88\x01R\x94Q\x95\x97P\x95\x943\x94\x7FF\x0B?F\x8A\xE9\xCC\x90\xB3\xD7w\x08\x15\xDEW\n\x18w\xE2\x19\xD9\x9C\x9C\xDD\nf\xB4\x04\x10\xFF\x81\x8E\x94a\x18!\x94\x8B\x93\x8D\x93\x92\x90`\x01`\x01`\xA0\x1B\x03\x96\x87\x16\x81R\x94\x90\x95\x16` \x85\x01R`@\x84\x01\x92\x90\x92R``\x83\x01R`\x80\x82\x01R`\xA0\x81\x01\x91\x90\x91R`\xC0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPPV[`\0\x80\x84\x81T\x81\x10a\x18BWa\x18Ba\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x06\x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x90P`\0\x81`\x01`\x01`\xA0\x1B\x03\x16c\x18\x16\r\xDD`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\xA9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18\xCD\x91\x90a%\x05V[\x90P`\0\x80\x86\x81T\x81\x10a\x18\xE3Wa\x18\xE3a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x05\x01T\x90P\x84\x15a\x19|W`\0a\x19\x13a\x19\x0C\x84\x84a\x19\xFFV[\x86\x90a\x1A\x1BV[`@Qc@\xC1\x0F\x19`\xE0\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19^W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19rW=`\0\x80>=`\0\xFD[PPPPPa\x19\xF7V[`\0a\x19\x92a\x19\x8B\x84\x84a \x16V[\x86\x90a +V[`@Qc'p\xA7\xEB`\xE2\x1B\x81R3`\x04\x82\x01R`$\x81\x01\x82\x90R\x90\x91P`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c\x9D\xC2\x9F\xAC\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x19\xDDW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x19\xF1W=`\0\x80>=`\0\xFD[PPPPP[PPPPPPV[`\0a\x1A\x14\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a @V[\x93\x92PPPV[`\0a\x1A\x14\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a @V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x03a\x1A\xF2W`@Qc.\x1A}M`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x90c.\x1A}M\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1A\xCBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1A\xDFW=`\0\x80>=`\0\xFD[PPPPa\x1A\xED\x82\x82a\x1E\x94V[PPPV[`\0a\x1B\x06\x82a\x1B\x01\x86a\x1E\xE5V[a _V[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R\x90\x91P`\0\x90`\x01`\x01`\xA0\x1B\x03\x86\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1BPW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1Bt\x91\x90a%\x05V[\x90Pa\x1B\x81\x85\x85\x84a kV[`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\0\x90`\x01`\x01`\xA0\x1B\x03\x87\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1B\xC8W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1B\xEC\x91\x90a%\x05V[\x90Pa\x1B\xF8\x83\x83a$\xDFV[\x81\x10\x15a\x19\xF7W`@Qc/5%1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x80`\0\x80`\0\x89\x81T\x81\x10a\x1C4Wa\x1C4a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01T\x90P`\0\x80\x8A\x81T\x81\x10a\x1C\\Wa\x1C\\a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01T\x90P\x81\x89\x11\x96P\x86\x15a\x1D\x1CW\x80\x88\x10a\x1C\x9BW`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x1C\xAEWa\x1C\xAEa\"\x8DV[`\0\x91\x82R` \x82 `\x01`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x1C\xE0Wa\x1C\xE0a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x02`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x1D\t\x82\x8Aa$\xDFV[\x93Pa\x1D\x15\x88\x82a$\xDFV[\x92Pa\x1D\xB9V[\x81\x89\x10a\x1D<W`@Qc\x11\x15vg`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x8A\x81T\x81\x10a\x1DOWa\x1DOa\"\x8DV[`\0\x91\x82R` \x82 `\x02`\x07\x90\x92\x02\x01\x01T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x97P\x8B\x90\x81\x10a\x1D\x81Wa\x1D\x81a\"\x8DV[`\0\x91\x82R` \x90\x91 `\x01`\x07\x90\x92\x02\x01\x01T`\x01`\x01`\xA0\x1B\x03\x16\x94Pa\x1D\xAA\x81\x89a$\xDFV[\x93Pa\x1D\xB6\x89\x83a$\xDFV[\x92P[\x88`\0\x8B\x81T\x81\x10a\x1D\xCDWa\x1D\xCDa\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x03\x01\x81\x90UP\x87`\0\x8B\x81T\x81\x10a\x1D\xF6Wa\x1D\xF6a\"\x8DV[\x90`\0R` `\0 \x90`\x07\x02\x01`\x04\x01\x81\x90UPa\x1E\x15\x86\x85a\x15iV[a\x1E \x853\x85a\x1A0V[PP\x93\x97\x92\x96P\x93P\x93PV[``\x81`\0\x03a\x1ETWPP`@\x80Q\x80\x82\x01\x90\x91R`\x01\x81R`\x03`\xFC\x1B` \x82\x01R\x90V[`N`@Q\x91P\x80\x82R`\x80\x82\x01`@R[\x82\x15a\x1E\x85W`\n\x80\x84\x06`0\x01\x82\x84\x01R\x90\x92\x04\x91`\0\x19\x01a\x1EfV[`N\x81\x90\x03\x91\x01\x90\x81R\x91\x90PV[`\0\x80`\0\x80`\0\x85\x87Z\xF1\x90P\x80a\x1A\xEDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x01a\x06\xE1V[`\0\x80\x82`\x01`\x01`\xA0\x1B\x03\x16c1<\xE5g`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1F&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1FJ\x91\x90a&\xBBV[`\xFF\x16\x90P`\0a\x1F\\\x82`\x12a$\xDFV[\x90Pa\x1Fi\x81`\na'\xC2V[a\x1F{\x90g\r\xE0\xB6\xB3\xA7d\0\0a'\xCEV[\x94\x93PPPPV[`\0a\x1A\x14\x83\x83a \x16V[`\0`@Qc#\xB8r\xDD`\xE0\x1B`\0R\x84`\x04R\x83`$R\x82`DR` `\0`d`\0\x80\x8AZ\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a\x17\"W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x14`$\x82\x01Rs\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x94\x93\xD3W\xD1\x90RS\x11Q`b\x1B`D\x82\x01R`d\x01a\x06\xE1V[`\0a\x1A\x14\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a \xEFV[`\0a\x1A\x14\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a \xEFV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a XW`\0\x80\xFD[\x04\x92\x91PPV[`\0a\x1A\x14\x83\x83a\x19\xFFV[`\0`@Qc\xA9\x05\x9C\xBB`\xE0\x1B`\0R\x83`\x04R\x82`$R` `\0`D`\0\x80\x89Z\xF1=\x15`\x1F=\x11`\x01`\0Q\x14\x16\x17\x16\x91P`\0``R\x80`@RP\x80a \xE9W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x15\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`\x8A\x1B`D\x82\x01R`d\x01a\x06\xE1V[PPPPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a!\x07W`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a!2W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a!QW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a!eW`\0\x80\xFD[\x815\x81\x81\x11\x15a!tW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a!\x86W`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15a!\xABW`\0\x80\xFD[P5\x91\x90PV[\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x83R` \x80\x83\x01Q\x82\x16\x90\x84\x01R`@\x80\x83\x01Q\x82\x16\x90\x84\x01R``\x80\x83\x01Q\x90\x84\x01R`\x80\x80\x83\x01Q\x90\x84\x01R`\xA0\x82\x81\x01Q\x90\x84\x01R`\xC0\x91\x82\x01Q\x16\x91\x01RV[`\xE0\x81\x01a\x0E;\x82\x84a!\xB2V[`\0` \x82\x84\x03\x12\x15a\"#W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\":W`\0\x80\xFD[\x82\x01`\x80\x81\x85\x03\x12\x15a\x1A\x14W`\0\x80\xFD[\x805`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x13\x8CW`\0\x80\xFD[`\0\x80`@\x83\x85\x03\x12\x15a\"vW`\0\x80\xFD[a\"\x7F\x83a\"LV[\x94` \x93\x90\x93\x015\x93PPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x86\x81\x16\x82R` \x82\x01\x86\x90R\x84T\x81\x16`@\x83\x01R`\x01\x85\x01T\x81\x16``\x83\x01R`\x02\x85\x01T\x81\x16`\x80\x83\x01R`\x03\x85\x01T`\xA0\x83\x01R`\x04\x85\x01T`\xC0\x83\x01R`\x05\x85\x01T`\xE0\x83\x01R`\x06\x85\x01T\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01\x81\x90R`\0\x90a#F\x81\x84\x01\x85\x87a\"\xA3V[\x98\x97PPPPPPPPV[`\0` \x82\x84\x03\x12\x15a#dW`\0\x80\xFD[a\x1A\x14\x82a\"LV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a#\x84W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a#\x9FW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a#\xB4W`\0\x80\xFD[\x92P\x92\x90PV[`\x01`\x01`\xA0\x1B\x03\x86\x16\x81R` \x81\x01\x85\x90R`\0a\x01@a#\xE0`@\x84\x01\x87a!\xB2V[\x80a\x01 \x84\x01Ra#F\x81\x84\x01\x85\x87a\"\xA3V[\x80Q\x80\x15\x15\x81\x14a\x13\x8CW`\0\x80\xFD[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15a$\x1CW`\0\x80\xFD[a$%\x86a#\xF4V[` \x87\x01Q`@\x88\x01Q``\x89\x01Q`\x80\x90\x99\x01Q\x92\x9A\x91\x99P\x97\x96P\x90\x94P\x92PPPV[`\0[\x83\x81\x10\x15a$fW\x81\x81\x01Q\x83\x82\x01R` \x01a$NV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra$\x87\x81` \x86\x01` \x86\x01a$KV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[`@\x81R`\0a$\xAE`@\x83\x01\x85a$oV[\x82\x81\x03` \x84\x01Ra$\xC0\x81\x85a$oV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0E;Wa\x0E;a$\xC9V[\x80\x82\x01\x80\x82\x11\x15a\x0E;Wa\x0E;a$\xC9V[`\0` \x82\x84\x03\x12\x15a%\x17W`\0\x80\xFD[PQ\x91\x90PV[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a%7W`\0\x80\xFD[a%@\x87a#\xF4V[\x95P` \x87\x01Q\x94P`@\x87\x01Q\x93P``\x87\x01Q\x92P`\x80\x87\x01Q\x91P`\xA0\x87\x01Q\x90P\x92\x95P\x92\x95P\x92\x95V[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a%\x97W`\0\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a%\xAFW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a%\xC3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a%\xD5Wa%\xD5a%oV[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a%\xFDWa%\xFDa%oV[\x81`@R\x82\x81R\x87` \x84\x87\x01\x01\x11\x15a&\x16W`\0\x80\xFD[a&'\x83` \x83\x01` \x88\x01a$KV[\x97\x96PPPPPPPV[dDFMM-`\xD8\x1B\x81R`\0\x85Qa&R\x81`\x05\x85\x01` \x8A\x01a$KV[\x80\x83\x01\x90P`-`\xF8\x1B\x80`\x05\x83\x01R\x86Qa&u\x81`\x06\x85\x01` \x8B\x01a$KV[`\x06\x92\x01\x91\x82\x01\x81\x90R\x85Qa&\x92\x81`\x07\x85\x01` \x8A\x01a$KV[`\x07\x92\x01\x91\x82\x01R\x83Qa&\xAD\x81`\x08\x84\x01` \x88\x01a$KV[\x01`\x08\x01\x96\x95PPPPPPV[`\0` \x82\x84\x03\x12\x15a&\xCDW`\0\x80\xFD[\x81Q`\xFF\x81\x16\x81\x14a\x1A\x14W`\0\x80\xFD[`\x01\x81\x81[\x80\x85\x11\x15a'\x19W\x81`\0\x19\x04\x82\x11\x15a&\xFFWa&\xFFa$\xC9V[\x80\x85\x16\x15a'\x0CW\x91\x81\x02\x91[\x93\x84\x1C\x93\x90\x80\x02\x90a&\xE3V[P\x92P\x92\x90PV[`\0\x82a'0WP`\x01a\x0E;V[\x81a'=WP`\0a\x0E;V[\x81`\x01\x81\x14a'SW`\x02\x81\x14a']Wa'yV[`\x01\x91PPa\x0E;V[`\xFF\x84\x11\x15a'nWa'na$\xC9V[PP`\x01\x82\x1Ba\x0E;V[P` \x83\x10a\x013\x83\x10\x16`N\x84\x10`\x0B\x84\x10\x16\x17\x15a'\x9CWP\x81\x81\na\x0E;V[a'\xA6\x83\x83a&\xDEV[\x80`\0\x19\x04\x82\x11\x15a'\xBAWa'\xBAa$\xC9V[\x02\x93\x92PPPV[`\0a\x1A\x14\x83\x83a'!V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0E;Wa\x0E;a$\xC9V\xFE\xA2dipfsX\"\x12 \xB2\xD9]{\xE6f\x95B\x9C\xE7\x0E\xFCj\xCF\x1D\x15k6p}\xAF\xF2\x01\x99z\x9D\0Q\x1B\xF8(\x1FdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static DFMM_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            f.debug_tuple(::core::stringify!(DFMM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> DFMM<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                DFMM_ABI.clone(),
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
                DFMM_ABI.clone(),
                DFMM_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `allocate` (0x2ec38188) function
        pub fn allocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([46, 195, 129, 136], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `deallocate` (0x9d942f9a) function
        pub fn deallocate(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([157, 148, 47, 154], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getPool` (0x068bcd8d) function
        pub fn get_pool(
            &self,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, Pool> {
            self.0
                .method_hash([6, 139, 205, 141], pool_id)
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
        /// Calls the contract's `init` (0x1455f1fc) function
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
        /// Calls the contract's `liquidityOf` (0x3be6a341) function
        pub fn liquidity_of(
            &self,
            account: ::ethers::core::types::Address,
            pool_id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([59, 230, 163, 65], (account, pool_id))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `lpTokenImplementation` (0xb462cd25) function
        pub fn lp_token_implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([180, 98, 205, 37], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `nonce` (0xaffed0e0) function
        pub fn nonce(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([175, 254, 208, 224], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `pools` (0xac4afa38) function
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
        /// Calls the contract's `swap` (0xbd0625ab) function
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
        /// Calls the contract's `update` (0x0216b838) function
        pub fn update(
            &self,
            pool_id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([2, 22, 184, 56], (pool_id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `weth` (0x3fc8cef3) function
        pub fn weth(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([63, 200, 206, 243], ())
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Allocate` event
        pub fn allocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AllocateFilter> {
            self.0.event()
        }
        /// Gets the contract's `Deallocate` event
        pub fn deallocate_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DeallocateFilter> {
            self.0.event()
        }
        /// Gets the contract's `Init` event
        pub fn init_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitFilter> {
            self.0.event()
        }
        /// Gets the contract's `Swap` event
        pub fn swap_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, SwapFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DFMMEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for DFMM<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `ERC1167FailedCreateClone` with signature
    /// `ERC1167FailedCreateClone()` and selector `0xc2f868f4`
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
    #[etherror(name = "ERC1167FailedCreateClone", abi = "ERC1167FailedCreateClone()")]
    pub struct ERC1167FailedCreateClone;
    /// Custom Error type `Invalid` with signature `Invalid(bool,uint256)` and
    /// selector `0xeec0da52`
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
    #[etherror(name = "Invalid", abi = "Invalid(bool,uint256)")]
    pub struct Invalid {
        pub negative: bool,
        pub swap_constant_growth: ::ethers::core::types::U256,
    }
    /// Custom Error type `InvalidSwap` with signature `InvalidSwap()` and
    /// selector `0x11157667`
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
    #[etherror(name = "InvalidSwap", abi = "InvalidSwap()")]
    pub struct InvalidSwap;
    /// Custom Error type `InvalidSwapInputTransfer` with signature
    /// `InvalidSwapInputTransfer()` and selector `0x80f64074`
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
    #[etherror(name = "InvalidSwapInputTransfer", abi = "InvalidSwapInputTransfer()")]
    pub struct InvalidSwapInputTransfer;
    /// Custom Error type `InvalidSwapOutputTransfer` with signature
    /// `InvalidSwapOutputTransfer()` and selector `0xf3cbbc87`
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
        name = "InvalidSwapOutputTransfer",
        abi = "InvalidSwapOutputTransfer()"
    )]
    pub struct InvalidSwapOutputTransfer;
    /// Custom Error type `InvalidTokens` with signature `InvalidTokens()` and
    /// selector `0x672215de`
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
    #[etherror(name = "InvalidTokens", abi = "InvalidTokens()")]
    pub struct InvalidTokens;
    /// Custom Error type `InvalidTransfer` with signature `InvalidTransfer()`
    /// and selector `0x2f352531`
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
    #[etherror(name = "InvalidTransfer", abi = "InvalidTransfer()")]
    pub struct InvalidTransfer;
    /// Custom Error type `Locked` with signature `Locked()` and selector
    /// `0x0f2e5b6c`
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
    #[etherror(name = "Locked", abi = "Locked()")]
    pub struct Locked;
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
    /// Custom Error type `OnlyWETH` with signature `OnlyWETH()` and selector
    /// `0x01f180c9`
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
    #[etherror(name = "OnlyWETH", abi = "OnlyWETH()")]
    pub struct OnlyWETH;
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
    pub enum DFMMErrors {
        ERC1167FailedCreateClone(ERC1167FailedCreateClone),
        Invalid(Invalid),
        InvalidSwap(InvalidSwap),
        InvalidSwapInputTransfer(InvalidSwapInputTransfer),
        InvalidSwapOutputTransfer(InvalidSwapOutputTransfer),
        InvalidTokens(InvalidTokens),
        InvalidTransfer(InvalidTransfer),
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
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) =
                <ERC1167FailedCreateClone as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ERC1167FailedCreateClone(decoded));
            }
            if let Ok(decoded) = <Invalid as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Invalid(decoded));
            }
            if let Ok(decoded) = <InvalidSwap as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSwap(decoded));
            }
            if let Ok(decoded) =
                <InvalidSwapInputTransfer as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidSwapInputTransfer(decoded));
            }
            if let Ok(decoded) =
                <InvalidSwapOutputTransfer as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InvalidSwapOutputTransfer(decoded));
            }
            if let Ok(decoded) = <InvalidTokens as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTokens(decoded));
            }
            if let Ok(decoded) = <InvalidTransfer as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTransfer(decoded));
            }
            if let Ok(decoded) = <Locked as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Locked(decoded));
            }
            if let Ok(decoded) = <Min as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Min(decoded));
            }
            if let Ok(decoded) = <OnlyWETH as ::ethers::core::abi::AbiDecode>::decode(data) {
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
                Self::InvalidSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidSwapInputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSwapOutputTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokens(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InvalidTransfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Locked(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnlyWETH(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for DFMMErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ERC1167FailedCreateClone as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Invalid as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <InvalidSwap as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSwapInputTransfer as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InvalidSwapOutputTransfer as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <InvalidTokens as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <InvalidTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Locked as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <OnlyWETH as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for DFMMErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ERC1167FailedCreateClone(element) => ::core::fmt::Display::fmt(element, f),
                Self::Invalid(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwapInputTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSwapOutputTransfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTokens(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTransfer(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<InvalidTransfer> for DFMMErrors {
        fn from(value: InvalidTransfer) -> Self {
            Self::InvalidTransfer(value)
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
    /// Container type for all of the contract's events
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
    /// Container type for all input parameters for the `allocate` function with
    /// signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    #[ethcall(name = "allocate", abi = "allocate(uint256,bytes)")]
    pub struct AllocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `deallocate` function
    /// with signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    #[ethcall(name = "deallocate", abi = "deallocate(uint256,bytes)")]
    pub struct DeallocateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `getPool` function with
    /// signature `getPool(uint256)` and selector `0x068bcd8d`
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
    #[ethcall(name = "getPool", abi = "getPool(uint256)")]
    pub struct GetPoolCall {
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
    /// Container type for all input parameters for the `init` function with
    /// signature `init((address,address,address,bytes))` and selector
    /// `0x1455f1fc`
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
    #[ethcall(name = "init", abi = "init((address,address,address,bytes))")]
    pub struct InitCall {
        pub params: InitParams,
    }
    /// Container type for all input parameters for the `liquidityOf` function
    /// with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    #[ethcall(name = "liquidityOf", abi = "liquidityOf(address,uint256)")]
    pub struct LiquidityOfCall {
        pub account: ::ethers::core::types::Address,
        pub pool_id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `lpTokenImplementation`
    /// function with signature `lpTokenImplementation()` and selector
    /// `0xb462cd25`
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
    #[ethcall(name = "lpTokenImplementation", abi = "lpTokenImplementation()")]
    pub struct LpTokenImplementationCall;
    /// Container type for all input parameters for the `nonce` function with
    /// signature `nonce()` and selector `0xaffed0e0`
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
    #[ethcall(name = "nonce", abi = "nonce()")]
    pub struct NonceCall;
    /// Container type for all input parameters for the `pools` function with
    /// signature `pools(uint256)` and selector `0xac4afa38`
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
    #[ethcall(name = "pools", abi = "pools(uint256)")]
    pub struct PoolsCall(pub ::ethers::core::types::U256);
    /// Container type for all input parameters for the `swap` function with
    /// signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    #[ethcall(name = "swap", abi = "swap(uint256,bytes)")]
    pub struct SwapCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `update` function with
    /// signature `update(uint256,bytes)` and selector `0x0216b838`
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
    #[ethcall(name = "update", abi = "update(uint256,bytes)")]
    pub struct UpdateCall {
        pub pool_id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `weth` function with
    /// signature `weth()` and selector `0x3fc8cef3`
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
    #[ethcall(name = "weth", abi = "weth()")]
    pub struct WethCall;
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
            if let Ok(decoded) = <AllocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allocate(decoded));
            }
            if let Ok(decoded) = <DeallocateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deallocate(decoded));
            }
            if let Ok(decoded) = <GetPoolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetPool(decoded));
            }
            if let Ok(decoded) =
                <GetReservesAndLiquidityCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::GetReservesAndLiquidity(decoded));
            }
            if let Ok(decoded) = <InitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Init(decoded));
            }
            if let Ok(decoded) = <LiquidityOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LiquidityOf(decoded));
            }
            if let Ok(decoded) =
                <LpTokenImplementationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LpTokenImplementation(decoded));
            }
            if let Ok(decoded) = <NonceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonce(decoded));
            }
            if let Ok(decoded) = <PoolsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pools(decoded));
            }
            if let Ok(decoded) = <SwapCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Swap(decoded));
            }
            if let Ok(decoded) = <UpdateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Update(decoded));
            }
            if let Ok(decoded) = <WethCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Weth(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for DFMMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Allocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deallocate(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetPool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetReservesAndLiquidity(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Init(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LiquidityOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::GetReservesAndLiquidity(element) => ::core::fmt::Display::fmt(element, f),
                Self::Init(element) => ::core::fmt::Display::fmt(element, f),
                Self::LiquidityOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::LpTokenImplementation(element) => ::core::fmt::Display::fmt(element, f),
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
    /// Container type for all return fields from the `allocate` function with
    /// signature `allocate(uint256,bytes)` and selector `0x2ec38188`
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
    pub struct AllocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `deallocate` function with
    /// signature `deallocate(uint256,bytes)` and selector `0x9d942f9a`
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
    pub struct DeallocateReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `getPool` function with
    /// signature `getPool(uint256)` and selector `0x068bcd8d`
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
    pub struct GetPoolReturn(pub Pool);
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
    /// Container type for all return fields from the `init` function with
    /// signature `init((address,address,address,bytes))` and selector
    /// `0x1455f1fc`
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
    pub struct InitReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `liquidityOf` function
    /// with signature `liquidityOf(address,uint256)` and selector `0x3be6a341`
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
    pub struct LiquidityOfReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `lpTokenImplementation`
    /// function with signature `lpTokenImplementation()` and selector
    /// `0xb462cd25`
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
    pub struct LpTokenImplementationReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `nonce` function with
    /// signature `nonce()` and selector `0xaffed0e0`
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
    pub struct NonceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `pools` function with
    /// signature `pools(uint256)` and selector `0xac4afa38`
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
    pub struct PoolsReturn {
        pub strategy: ::ethers::core::types::Address,
        pub token_x: ::ethers::core::types::Address,
        pub token_y: ::ethers::core::types::Address,
        pub reserve_x: ::ethers::core::types::U256,
        pub reserve_y: ::ethers::core::types::U256,
        pub total_liquidity: ::ethers::core::types::U256,
        pub liquidity_token: ::ethers::core::types::Address,
    }
    /// Container type for all return fields from the `swap` function with
    /// signature `swap(uint256,bytes)` and selector `0xbd0625ab`
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
    pub struct SwapReturn(
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
    );
    /// Container type for all return fields from the `weth` function with
    /// signature `weth()` and selector `0x3fc8cef3`
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
    pub struct WethReturn(pub ::ethers::core::types::Address);
}

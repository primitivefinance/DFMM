pub use atomic_v2::*;
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
pub mod atomic_v2 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("solverAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("exchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("liquidExchangeAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("assetAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("quoteAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("XTOY"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("XTOY"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("YTOX"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("YTOX"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("asset"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("asset"),
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
                    ::std::borrow::ToOwned::to_owned("cdf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cdf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("cumulativeProfit"),
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
                    ::std::borrow::ToOwned::to_owned("divWadDown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("divWadDown"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("divWadUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("divWadUp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("exchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("exchange"),
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenXBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenXBalance",),
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYEndBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenYEndBalance",),
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
                    ::std::borrow::ToOwned::to_owned("intermediateTokenYStartBalance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("intermediateTokenYStartBalance",),
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
                    ::std::borrow::ToOwned::to_owned("liquidExchange"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("liquidExchange"),
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
                    ::std::borrow::ToOwned::to_owned("log"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("log"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lower_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lower_exchange_price",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("input"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadDown"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulWadDown"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("mulWadUp"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("mulWadUp"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("x"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("y"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pdf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("pdf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ppf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ppf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("input"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("output"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quote"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("quote"),
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
                    ::std::borrow::ToOwned::to_owned("raise_exchange_price"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("raise_exchange_price",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("poolId"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("input"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
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
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("estimatedOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("estimatedPrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
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
                    ::std::borrow::ToOwned::to_owned("solver"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("solver"),
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
                    ::std::borrow::ToOwned::to_owned("sqrt"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("sqrt"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("x"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("z"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Loss"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Loss"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("loss"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Price"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Price"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("price"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("timestamp"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Profit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Profit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("profit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("AttemptedProfit"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("profit"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("int256"),
                            ),
                        },],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("DexSwapFailure"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("DexSwapFailure"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("reason"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("err"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
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
                    ::std::borrow::ToOwned::to_owned("InsufficientApprovalY"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientApprovalY",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("allowance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceX"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalanceX",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InsufficientBalanceY"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InsufficientBalanceY",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payment"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
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
                (
                    ::std::borrow::ToOwned::to_owned("SimulatedSwapFailure"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("SimulatedSwapFailure",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("valid"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("estimatedOut"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("estimatedPrice"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("payload"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes"),
                                ),
                            },
                        ],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("UnprofitableArbitrage"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("UnprofitableArbitrage",),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("start_y_balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("end_y_balance"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("absolute_difference",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static ATOMICV2_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R`\t\x80Ta\xFF\xFF\x19\x16`\x01\x17\x90U4\x80\x15b\0\0\x1FW`\0\x80\xFD[P`@Qb\0 R8\x03\x80b\0 R\x839\x81\x01`@\x81\x90Rb\0\0B\x91b\0\0\xC2V[`\x02\x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16`\x01`\x01`\xA0\x1B\x03\x97\x88\x16\x17\x90\x91U`\x01\x80T\x82\x16\x95\x87\x16\x95\x90\x95\x17\x90\x94U`\0\x80T\x85\x16\x93\x86\x16\x93\x90\x93\x17\x90\x92U`\x03\x80T\x84\x16\x91\x85\x16\x91\x90\x91\x17\x90U`\x04\x80T\x90\x92\x16\x92\x16\x91\x90\x91\x17\x90Ub\0\x012V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xBDW`\0\x80\xFD[\x91\x90PV[`\0\x80`\0\x80`\0`\xA0\x86\x88\x03\x12\x15b\0\0\xDBW`\0\x80\xFD[b\0\0\xE6\x86b\0\0\xA5V[\x94Pb\0\0\xF6` \x87\x01b\0\0\xA5V[\x93Pb\0\x01\x06`@\x87\x01b\0\0\xA5V[\x92Pb\0\x01\x16``\x87\x01b\0\0\xA5V[\x91Pb\0\x01&`\x80\x87\x01b\0\0\xA5V[\x90P\x92\x95P\x92\x95\x90\x93PV[a\x1F\x10\x80b\0\x01B`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\x96\xFB\xEE\x1D\x11a\0\xC3W\x80c\xD2L\xE6\xE5\x11a\0|W\x80c\xD2L\xE6\xE5\x14a\x02\xC3W\x80c\xD2\xF7&Z\x14a\x02\xD6W\x80c\xE5$\xF8I\x14a\x02\xE9W\x80c\xF3\xC9s\xCF\x14a\x02\xFCW\x80c\xF9\0^\xB5\x14a\x03\tW\x80c\xFA.Y\x94\x14a\x03\x1CW`\0\x80\xFD[\x80c\x96\xFB\xEE\x1D\x14a\x02OW\x80c\x99\x9B\x93\xAF\x14a\x02dW\x80c\x9F'\xEFO\x14a\x02wW\x80c\xAE\x97h\xA8\x14a\x02\x8AW\x80c\xBD%-\x06\x14a\x02\x9DW\x80c\xD0\xB7\x1B\x1E\x14a\x02\xB0W`\0\x80\xFD[\x80cI\xA7\xA2m\x11a\x01\x15W\x80cI\xA7\xA2m\x14a\x01\xECW\x80cdI\xFCW\x14a\x01\xFFW\x80cgsB\xCE\x14a\x02!W\x80cr\xB9\x82F\x14a\x024W\x80c\x85\xB3\x19\xFF\x14a\x02=W\x80c\x93e \xC3\x14a\x02FW`\0\x80\xFD[\x80c-[l\xB9\x14a\x01RW\x80c6yr:\x14a\x01xW\x80c7\xC6\xA4J\x14a\x01\x8BW\x80c8\xD5.\x0F\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\xC9W[`\0\x80\xFD[a\x01ea\x01`6`\x04a\x1BVV[a\x03%V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01ea\x01\x866`\x04a\x1BVV[a\x036V[a\x01ea\x01\x996`\x04a\x1BoV[a\x03AV[`\x03Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01oV[a\x01\xDCa\x01\xD76`\x04a\x1B\xA2V[a\x03TV[`@Qa\x01o\x94\x93\x92\x91\x90a\x1C*V[`\x02Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tTa\x02\x11\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x01ea\x02/6`\x04a\x1BVV[a\x03\xEFV[a\x01e`\x07T\x81V[a\x01e`\x08T\x81V[a\x01e`\x05T\x81V[a\x02ba\x02]6`\x04a\x1BoV[a\x03\xFAV[\0[`\x04Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ea\x02\x986`\x04a\x1BoV[a\x05NV[a\x01ea\x02\xAB6`\x04a\x1BoV[a\x05ZV[a\x01ea\x02\xBE6`\x04a\x1BVV[a\x05fV[a\x01ea\x02\xD16`\x04a\x1BVV[a\x05qV[`\x01Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ea\x02\xF76`\x04a\x1BoV[a\x05|V[`\tTa\x02\x11\x90`\xFF\x16\x81V[a\x02ba\x03\x176`\x04a\x1BoV[a\x05\x88V[a\x01e`\x06T\x81V[`\0a\x030\x82a\x06\xCBV[\x92\x91PPV[`\0a\x030\x82a\x08\xABV[`\0a\x03M\x83\x83a\tQV[\x93\x92PPPV[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xDE\x91\x90\x81\x01\x90a\x1CqV[\x93P\x93P\x93P\x93P\x93P\x93P\x93P\x93V[`\0a\x030\x82a\tfV[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04h\x91\x90a\x1DFV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x04\xAC\x82a\n\nV[`\tTa\x04\xC1\x90a\x01\0\x90\x04`\xFF\x16\x83a\x0C.V[`\tT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x05A\x92\x86\x92`\xFF\x90\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05<\x91\x90a\x1DFV[a\x0EQV[a\x05Ia\x11\x10V[PPPV[`\0a\x03M\x83\x83a\x12\xCFV[`\0a\x03M\x83\x83a\x12\xE4V[`\0a\x030\x82a\x12\xF9V[`\0a\x030\x82a\x13bV[`\0a\x03M\x83\x83a\x13\xBEV[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF6\x91\x90a\x1DFV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x06:\x82a\n\nV[`\tTa\x06Q\x90\x84\x90a\x01\0\x90\x04`\xFF\x16\x84a\x0EQV[`\tT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x05A\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC6\x91\x90a\x1DFV[a\x0C.V[`\0\x80\x82\x13a\x07\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x07\x1A\x84a\x13\xD3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x08\xC4WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x08\xECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\t\rW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\x1A\x83`\x02a\x1DuV[\x90P`\0a\t'\x82a\x14{V[\x90P`\0a\t=g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x16\xFCV[\x90Pa\tH\x81a\x1D\xA5V[\x95\x94PPPPPV[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17\x11V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\t\x7FW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\t\x9BW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\t\xB3W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\t\xC9W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n|\x91\x90a\x1DFV[\x90P\x81\x81\x10\x15a\n\xA9W`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x07\x04V[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B!\x91\x90a\x1DFV[\x90P\x82\x81\x10\x15a\x0BNW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x07\x04V[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C&\x91\x90a\x1DFV[`\x06UPPPV[\x81\x15a\r\x08W`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x98W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\0W=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rnW=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xDAW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EJ\x91\x90a\x1DFV[`\x05UPPV[\x81\x15a\x0E\xC4W`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xBBW=`\0\x80>=`\0\xFD[PPPPa\x0F/V[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F*W=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xB8\x91\x90\x81\x01\x90a\x1CqV[\x93P\x93P\x93P\x93P\x83a\x0F\xE6W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x07\x04\x94\x93\x92\x91\x90a\x1C*V[`\x01T`@Qc\xBD\x06%\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBD\x06%\xAB\x90a\x10\x18\x90\x8A\x90\x85\x90`\x04\x01a\x1D\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x102W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x10CWP`\x01[a\x10\x92W=\x80\x80\x15a\x10qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10vV[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x07\x04\x91\x90a\x1D\xE2V[\x85a\x11\x07W`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x03\x91\x90a\x1DFV[`\x05U[PPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11}\x91\x90a\x1DFV[`\x07\x81\x90U`\x06T\x11\x15a\x12\x08W\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x07T`\x06Ta\x11\xBC\x91\x90a\x1E)V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x06T`\x07Ta\x11\xDD\x81\x83a\x1E<V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x07\x04V[`\0`\x06T`\x07Ta\x12\x1A\x91\x90a\x1E)V[\x90P\x80`\x08`\0\x82\x82Ta\x12.\x91\x90a\x1EcV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\x07T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xC8W=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x170V[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x170V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x13\x17g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1DuV[a\x13!\x91\x90a\x1EvV[\x90P`\0a\x13.\x82a\x1D\xA5V[\x90P`\0a\x13;\x82a\x17^V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x13Xg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DuV[a\tH\x91\x90a\x1EvV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x13x\x81a\x1D\xA5V[a\x13\x82\x91\x90a\x1DuV[a\x13\x8C\x91\x90a\x1EvV[\x90Pa\x13\x97\x81a\x19BV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x13\xB4g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DuV[a\x03M\x91\x90a\x1EvV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x11V[`\0\x80\x82\x11a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\x04V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a\x14\x92WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x14\xB0W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xD1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x14\xF9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x15\x04W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x15,Wa\x15'\x83g\x1B\xC1mgN\xC8\0\0a\x1E<V[a\x15.V[\x82[\x90P`\0a\x15D\x82g\x1B\xC1mgN\xC8\0\0a\x1A\xEBV[\x90P\x80`\0\x03a\x15gW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15r\x82a\x06\xCBV[\x90P`\0c;\x9A\xCA\0a\x15\x9Da\x15\x98a\x15\x92g\x1B\xC1mgN\xC8\0\0a\x1D\xA5V[\x85a\x16\xFCV[a\tfV[a\x15\xA7\x91\x90a\x1DuV[\x90P`\0\x80a\x15\xBE\x83g\x03\xC1f\\z\xAB \0a\x16\xFCV[a\x15\xD0\x90g \x05\xFEO&\x8E\xA0\0a\x1E\xB2V[\x90P`\0a\x16\0\x84a\x15\xE9\x86f\x9F2u$b\xA0\0a\x16\xFCV[a\x15\xFB\x90g\r\xC5R\x7Fd, \0a\x1E\xB2V[a\x16\xFCV[a\x16\x12\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[\x90Pa\x166g\t\xD0(\xCCo _\xFF\x19\x85a\x16,\x85\x85a\x1A\xEBV[a\x15\xFB\x91\x90a\x1E<V[\x92PPP`\0[`\x02\x81\x10\x15a\x16\xD1W`\0\x86a\x16R\x84a\x17^V[a\x16\\\x91\x90a\x1E<V[\x90P`\0a\x16j\x84\x85a\x16\xFCV[a\x16s\x90a\x1D\xA5V[\x90P`\0a\x16\x80\x82a\x19BV[\x90P`\0a\x16\x8E\x86\x85a\x16\xFCV[a\x16\xA0g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x16\xFCV[a\x16\xAA\x91\x90a\x1E<V[\x90Pa\x16\xB6\x84\x82a\x1A\xEBV[a\x16\xC0\x90\x87a\x1E\xB2V[\x95P\x84`\x01\x01\x94PPPPPa\x16=V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x16\xEEWa\x16\xE9\x82a\x1D\xA5V[a\x16\xF0V[\x81[\x98\x97PPPPPPPPV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xFCV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17)W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17HW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x17wWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x17\x8EWP`\0\x91\x90PV[a\x17\x9FgV\x98\xEE\xF0fp\0\0a\x1D\xA5V[\x82\x13a\x17\xB4WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x17\xBF\x83a\x1B\x1BV[\x90P`\0a\x17\xF8g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xE1\x84g\x1B\xC1mgN\xC8\0\0a\tQV[a\x17\xF3\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[a\x1A\xEBV[\x90P`\0\x80\x82a\x18T\x81a\x18A\x81a\x18/\x81a\x18\x1C\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x16\xFCV[a\x15\xFB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1E\xB2V[a\x15\xFB\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1E\xB2V[a\x15\xFB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1E\xB2V[a\x18f\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1E\xB2V[\x91P\x83\x90Pa\x18\xCE\x81a\x18\xBC\x81a\x18\xAA\x81a\x18\x98\x81a\x18\x85\x81\x8Ba\x16\xFCV[a\x15\xFB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1E\xB2V[a\x15\xFB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1E\xB2V[a\x15\xFB\x90g\x051\n\xA7\xD5!0\0a\x1E\xB2V[a\x15\xFB\x90g\r\xE0\xCC=\x15a\0\0a\x1E\xB2V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x18\xE4\x87\x88a\x16\xFCV[a\x18\xF0\x90`\0\x19a\x1DuV[a\x18\xFA\x91\x90a\x1E<V[a\x19\x04\x91\x90a\x1E\xB2V[\x92PP`\0a\x19\x12\x83a\x19BV[\x90P`\0a\x19 \x85\x83a\x16\xFCV[\x90P`\0\x88\x12a\x190W\x80a\x16\xF0V[a\x16\xF0\x81g\x1B\xC1mgN\xC8\0\0a\x1E<V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x19]WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x19\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07\x04V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\x14W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1BAW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1BRWP\x19`\x01\x01\x90V[P\x90V[`\0` \x82\x84\x03\x12\x15a\x1BhW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x82W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x1B\x9FW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\xB7W`\0\x80\xFD[\x835\x92P` \x84\x015a\x1B\xC9\x81a\x1B\x91V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\x1B\xF5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1B\xDDV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1C\x16\x81` \x86\x01` \x86\x01a\x1B\xDAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x1CQ`\x80\x83\x01\x84a\x1B\xFEV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C\x87W`\0\x80\xFD[\x84Qa\x1C\x92\x81a\x1B\x91V[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\xBFW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\xD3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1C\xE5Wa\x1C\xE5a\x1C[V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x1D\rWa\x1D\ra\x1C[V[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x1D&W`\0\x80\xFD[a\x1D7\x83` \x83\x01` \x88\x01a\x1B\xDAV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1DXW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1D\x91Wa\x1D\x91a\x1D_V[\x81\x81\x05\x83\x14\x82\x15\x17a\x030Wa\x030a\x1D_V[`\0`\x01`\xFF\x1B\x82\x01a\x1D\xBAWa\x1D\xBAa\x1D_V[P`\0\x03\x90V[\x82\x81R`@` \x82\x01R`\0a\x1D\xDA`@\x83\x01\x84a\x1B\xFEV[\x94\x93PPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x03M`\x80\x83\x01\x84a\x1B\xFEV[\x81\x81\x03\x81\x81\x11\x15a\x030Wa\x030a\x1D_V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1E\\Wa\x1E\\a\x1D_V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x030Wa\x030a\x1D_V[`\0\x82a\x1E\x93WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1E\xADWa\x1E\xADa\x1D_V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1E\xD2Wa\x1E\xD2a\x1D_V[PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 )x\xE1&\xC1\x04d\xBA\xA5ee\x13\x03\x04\xCFl\xF5\x1C\xED\xFE\xBE92\xC7\x16\xF9\x90o\xD3\x8C\xF6\x1AdsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static ATOMICV2_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01MW`\x005`\xE0\x1C\x80c\x96\xFB\xEE\x1D\x11a\0\xC3W\x80c\xD2L\xE6\xE5\x11a\0|W\x80c\xD2L\xE6\xE5\x14a\x02\xC3W\x80c\xD2\xF7&Z\x14a\x02\xD6W\x80c\xE5$\xF8I\x14a\x02\xE9W\x80c\xF3\xC9s\xCF\x14a\x02\xFCW\x80c\xF9\0^\xB5\x14a\x03\tW\x80c\xFA.Y\x94\x14a\x03\x1CW`\0\x80\xFD[\x80c\x96\xFB\xEE\x1D\x14a\x02OW\x80c\x99\x9B\x93\xAF\x14a\x02dW\x80c\x9F'\xEFO\x14a\x02wW\x80c\xAE\x97h\xA8\x14a\x02\x8AW\x80c\xBD%-\x06\x14a\x02\x9DW\x80c\xD0\xB7\x1B\x1E\x14a\x02\xB0W`\0\x80\xFD[\x80cI\xA7\xA2m\x11a\x01\x15W\x80cI\xA7\xA2m\x14a\x01\xECW\x80cdI\xFCW\x14a\x01\xFFW\x80cgsB\xCE\x14a\x02!W\x80cr\xB9\x82F\x14a\x024W\x80c\x85\xB3\x19\xFF\x14a\x02=W\x80c\x93e \xC3\x14a\x02FW`\0\x80\xFD[\x80c-[l\xB9\x14a\x01RW\x80c6yr:\x14a\x01xW\x80c7\xC6\xA4J\x14a\x01\x8BW\x80c8\xD5.\x0F\x14a\x01\x9EW\x80c9(\xFF\x97\x14a\x01\xC9W[`\0\x80\xFD[a\x01ea\x01`6`\x04a\x1BVV[a\x03%V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01ea\x01\x866`\x04a\x1BVV[a\x036V[a\x01ea\x01\x996`\x04a\x1BoV[a\x03AV[`\x03Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x01oV[a\x01\xDCa\x01\xD76`\x04a\x1B\xA2V[a\x03TV[`@Qa\x01o\x94\x93\x92\x91\x90a\x1C*V[`\x02Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\tTa\x02\x11\x90a\x01\0\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x01oV[a\x01ea\x02/6`\x04a\x1BVV[a\x03\xEFV[a\x01e`\x07T\x81V[a\x01e`\x08T\x81V[a\x01e`\x05T\x81V[a\x02ba\x02]6`\x04a\x1BoV[a\x03\xFAV[\0[`\x04Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\0Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ea\x02\x986`\x04a\x1BoV[a\x05NV[a\x01ea\x02\xAB6`\x04a\x1BoV[a\x05ZV[a\x01ea\x02\xBE6`\x04a\x1BVV[a\x05fV[a\x01ea\x02\xD16`\x04a\x1BVV[a\x05qV[`\x01Ta\x01\xB1\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x01ea\x02\xF76`\x04a\x1BoV[a\x05|V[`\tTa\x02\x11\x90`\xFF\x16\x81V[a\x02ba\x03\x176`\x04a\x1BoV[a\x05\x88V[a\x01e`\x06T\x81V[`\0a\x030\x82a\x06\xCBV[\x92\x91PPV[`\0a\x030\x82a\x08\xABV[`\0a\x03M\x83\x83a\tQV[\x93\x92PPPV[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91``\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x03\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x03\xDE\x91\x90\x81\x01\x90a\x1CqV[\x93P\x93P\x93P\x93P\x93P\x93P\x93P\x93V[`\0a\x030\x82a\tfV[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x04DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x04h\x91\x90a\x1DFV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x04\xAC\x82a\n\nV[`\tTa\x04\xC1\x90a\x01\0\x90\x04`\xFF\x16\x83a\x0C.V[`\tT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x05A\x92\x86\x92`\xFF\x90\x91\x16\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\x18W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05<\x91\x90a\x1DFV[a\x0EQV[a\x05Ia\x11\x10V[PPPV[`\0a\x03M\x83\x83a\x12\xCFV[`\0a\x03M\x83\x83a\x12\xE4V[`\0a\x030\x82a\x12\xF9V[`\0a\x030\x82a\x13bV[`\0a\x03M\x83\x83a\x13\xBEV[`\x02T`@Qc\x03\xB4\xD1\x03`\xE4\x1B\x81R`\x04\x81\x01\x84\x90R`\0\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c;M\x100\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xD2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05\xF6\x91\x90a\x1DFV[`@\x80Q\x82\x81RB` \x82\x01R\x91\x92P\x7F\xD15<h\xE7\x9E\xF7\r\xE8N\xE9\r/\xAC\xF8E\xEC$\x89Q\x16\xD4\xA05\x05\xAAAxZ\xF7\x1FZ\x91\x01`@Q\x80\x91\x03\x90\xA1a\x06:\x82a\n\nV[`\tTa\x06Q\x90\x84\x90a\x01\0\x90\x04`\xFF\x16\x84a\x0EQV[`\tT`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01Ra\x05A\x92`\xFF\x16\x91`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\xA2W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC6\x91\x90a\x1DFV[a\x0C.V[`\0\x80\x82\x13a\x07\rW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\0``a\x07\x1A\x84a\x13\xD3V[\x03`\x9F\x81\x81\x03\x94\x90\x94\x1B\x90\x93\x1ClFWr\xB2\xBB\xBB_\x82K\x15 z0\x81\x01\x81\x02``\x90\x81\x1Dm\x03\x88\xEA\xA2t\x12\xD5\xAC\xA0&\x81]cn\x01\x82\x02\x81\x1Dm\r\xF9\x9A\xC5\x02\x03\x1B\xF9S\xEF\xF4r\xFD\xCC\x01\x82\x02\x81\x1Dm\x13\xCD\xFF\xB2\x9DQ\xD9\x93\"\xBD\xFF_\"\x11\x01\x82\x02\x81\x1Dm\n\x0Ft #\xDE\xF7\x83\xA3\x07\xA9\x86\x91.\x01\x82\x02\x81\x1Dm\x01\x92\r\x80C\xCA\x89\xB5#\x92S(NB\x01\x82\x02\x81\x1Dl\x0Bz\x86\xD77Th\xFA\xC6g\xA0\xA5'\x01l)P\x8EE\x85C\xD8\xAAM\xF2\xAB\xEEx\x83\x01\x83\x02\x82\x1Dm\x019`\x1A.\xFA\xBEq~`L\xBBH\x94\x01\x83\x02\x82\x1Dm\x02$\x7Fz{e\x942\x06I\xAA\x03\xAB\xA1\x01\x83\x02\x82\x1Dl\x8C?8\xE9Zk\x1F\xF2\xAB\x1C;46\x19\x01\x83\x02\x82\x1Dm\x028Gs\xBD\xF1\xACVv\xFA\xCC\xED`\x90\x19\x01\x83\x02\x90\x91\x1Dl\xB9\xA0%\xD8\x14\xB2\x9C!+\x8B\x1A\x07\xCD\x19\x01\x90\x91\x02x\n\tPp\x84\xCCi\x9B\xB0\xE7\x1E\xA8i\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x01\x05q\x13@\xDA\xA0\xD5\xF7i\xDB\xA1\x91\\\xEFY\xF0\x81ZU\x06\x02}\x02g\xA3l\x0C\x95\xB3\x97Z\xB3\xEE[ :v\x14\xA3\xF7Ss\xF0G\xD8\x03\xAE{f\x87\xF2\xB3\x93\x90\x93\x02\x92\x90\x92\x01}W\x11^G\x01\x8Cqw\xEE\xBF|\xD3p\xA35j\x1Bxc\0\x8AZ\xE8\x02\x8Cr\xB8\x86B\x84\x01`\xAE\x1D\x92\x91PPV[`\0g\x06\xF0[Y\xD3\xB2\0\0\x82\x03a\x08\xC4WP`\0\x91\x90PV[g\r\xE0\xB6\xB3\xA7d\0\0\x82\x12a\x08\xECW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\t\rW`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\t\x1A\x83`\x02a\x1DuV[\x90P`\0a\t'\x82a\x14{V[\x90P`\0a\t=g\x13\xA0K\xBD\xFD\xC9\xBE\x88\x83a\x16\xFCV[\x90Pa\tH\x81a\x1D\xA5V[\x95\x94PPPPPV[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x17\x11V[`\xB5\x81`\x01`\x88\x1B\x81\x10a\t\x7FW`@\x91\x90\x91\x1B\x90`\x80\x1C[i\x01\0\0\0\0\0\0\0\0\0\x81\x10a\t\x9BW` \x91\x90\x91\x1B\x90`@\x1C[e\x01\0\0\0\0\0\x81\x10a\t\xB3W`\x10\x91\x90\x91\x1B\x90` \x1C[c\x01\0\0\0\x81\x10a\t\xC9W`\x08\x91\x90\x91\x1B\x90`\x10\x1C[b\x01\0\0\x01\x02`\x12\x1C\x80\x82\x04\x01`\x01\x90\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x81\x1C\x80\x83\x04\x01\x90\x1C\x90\x81\x90\x04\x81\x11\x90\x03\x90V[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\nXW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\n|\x91\x90a\x1DFV[\x90P\x81\x81\x10\x15a\n\xA9W`@Qc\n\xBEZ\x89`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x83\x90R`D\x01a\x07\x04V[`\x04\x80T`@Qcn\xB1v\x9F`\xE1\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`\0\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xDDb\xED>\x90`D\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\n\xFDW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0B!\x91\x90a\x1DFV[\x90P\x82\x81\x10\x15a\x0BNW`@Qc\xDAV\xD3\xC5`\xE0\x1B\x81R`\x04\x81\x01\x82\x90R`$\x81\x01\x84\x90R`D\x01a\x07\x04V[`\x04\x80T`@Qc#\xB8r\xDD`\xE0\x1B\x81R3\x92\x81\x01\x92\x90\x92R0`$\x83\x01R`D\x82\x01\x85\x90R`\x01`\x01`\xA0\x1B\x03\x16\x90c#\xB8r\xDD\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0B\xA1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0B\xB5W=`\0\x80>=`\0\xFD[PP`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0C\x02W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0C&\x91\x90a\x1DFV[`\x06UPPPV[\x81\x15a\r\x08W`\x03T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\x84W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0C\x98W=`\0\x80>=`\0\xFD[PP`\0T`\x03T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x86\x90R\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0C\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\0W=`\0\x80>=`\0\xFD[PPPPPPV[`\x04\x80T`\0T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\rZW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\rnW=`\0\x80>=`\0\xFD[PP`\0T`\x04\x80T`@Qc\xD0\x04\xF0\xF7`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x81\x01\x92\x90\x92R`$\x82\x01\x86\x90R\x90\x91\x16\x92Pc\xD0\x04\xF0\xF7\x91P`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\r\xC6W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\r\xDAW=`\0\x80>=`\0\xFD[PP`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x92Pcp\xA0\x821\x91P`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0E&W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x0EJ\x91\x90a\x1DFV[`\x05UPPV[\x81\x15a\x0E\xC4W`\x03T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x04\x82\x01R`$\x81\x01\x84\x90R\x91\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0E\xA7W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0E\xBBW=`\0\x80>=`\0\xFD[PPPPa\x0F/V[`\x04\x80T`\x01T`@Qc\t^\xA7\xB3`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x93\x81\x01\x93\x90\x93R`$\x83\x01\x84\x90R\x16\x90c\t^\xA7\xB3\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x0F\x16W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x0F*W=`\0\x80>=`\0\xFD[PPPP[`\x02T`@Qc9(\xFF\x97`\xE0\x1B\x81R`\x04\x81\x01\x85\x90R\x83\x15\x15`$\x82\x01R`D\x81\x01\x83\x90R`\0\x91\x82\x91\x82\x91\x82\x91`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c9(\xFF\x97\x90`d\x01`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x0F\x90W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x0F\xB8\x91\x90\x81\x01\x90a\x1CqV[\x93P\x93P\x93P\x93P\x83a\x0F\xE6W\x83\x83\x83\x83`@Qc\x03\x14\xE6#`\xE3\x1B\x81R`\x04\x01a\x07\x04\x94\x93\x92\x91\x90a\x1C*V[`\x01T`@Qc\xBD\x06%\xAB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90c\xBD\x06%\xAB\x90a\x10\x18\x90\x8A\x90\x85\x90`\x04\x01a\x1D\xC1V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x102W`\0\x80\xFD[PZ\xF1\x92PPP\x80\x15a\x10CWP`\x01[a\x10\x92W=\x80\x80\x15a\x10qW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x10vV[``\x91P[P\x80`@Qcg\xA1k\x8D`\xE1\x1B\x81R`\x04\x01a\x07\x04\x91\x90a\x1D\xE2V[\x85a\x11\x07W`\x03T`@Qcp\xA0\x821`\xE0\x1B\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x10\xDFW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11\x03\x91\x90a\x1DFV[`\x05U[PPPPPPPV[`\x04\x80T`@Qcp\xA0\x821`\xE0\x1B\x81R0\x92\x81\x01\x92\x90\x92R`\x01`\x01`\xA0\x1B\x03\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x11YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x11}\x91\x90a\x1DFV[`\x07\x81\x90U`\x06T\x11\x15a\x12\x08W\x7F\xB6[.\x08]}\x04\x0C1?}N\x1A\xC9\x0FY7\x02o\xEEI~\x0E$\xA7\xEF\xF1jU\xE1\xC5\xEA`\x07T`\x06Ta\x11\xBC\x91\x90a\x1E)V[`@Q\x90\x81R` \x01`@Q\x80\x91\x03\x90\xA1`\x06T`\x07Ta\x11\xDD\x81\x83a\x1E<V[`@Qc\xB1n7\x83`\xE0\x1B\x81R`\x04\x81\x01\x93\x90\x93R`$\x83\x01\x91\x90\x91R\x19`D\x82\x01R`d\x01a\x07\x04V[`\0`\x06T`\x07Ta\x12\x1A\x91\x90a\x1E)V[\x90P\x80`\x08`\0\x82\x82Ta\x12.\x91\x90a\x1EcV[\x90\x91UPP`@Q\x81\x81R\x7F5}\x90_\x181 \x97\x97\xDFMU\xD7\x9C\\[\xF1\xD9\xF71\x1C\x97j\xFD\x05\xE1=\x88\x1E\xAB\x9B\xC8\x90` \x01`@Q\x80\x91\x03\x90\xA1`\x04\x80T`\x07T`@Qc\xA9\x05\x9C\xBB`\xE0\x1B\x81R3\x93\x81\x01\x93\x90\x93R`$\x83\x01R`\x01`\x01`\xA0\x1B\x03\x16\x90c\xA9\x05\x9C\xBB\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x12\xB4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x12\xC8W=`\0\x80>=`\0\xFD[PPPPPV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x170V[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84a\x170V[`\0\x80g\x13\xA0K\xBD\xFD\xC9\xBE\x88a\x13\x17g\r\xE0\xB6\xB3\xA7d\0\0\x85a\x1DuV[a\x13!\x91\x90a\x1EvV[\x90P`\0a\x13.\x82a\x1D\xA5V[\x90P`\0a\x13;\x82a\x17^V[\x90Pg\x1B\xC1mgN\xC8\0\0a\x13Xg\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DuV[a\tH\x91\x90a\x1EvV[`\0\x80g\x1B\xC1mgN\xC8\0\0\x83a\x13x\x81a\x1D\xA5V[a\x13\x82\x91\x90a\x1DuV[a\x13\x8C\x91\x90a\x1EvV[\x90Pa\x13\x97\x81a\x19BV[\x90Pg\"\xC9U\"\x95T\xC1\xB6a\x13\xB4g\r\xE0\xB6\xB3\xA7d\0\0\x83a\x1DuV[a\x03M\x91\x90a\x1EvV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x17\x11V[`\0\x80\x82\x11a\x14\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\t`$\x82\x01Rh\x15S\x91\x11Q\x92S\x91Q`\xBA\x1B`D\x82\x01R`d\x01a\x07\x04V[P`\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11`\x07\x1B\x82\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x82\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x82\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x82\x81\x1C`\xFF\x10`\x03\x90\x81\x1B\x90\x91\x17\x83\x81\x1C`\x0F\x10`\x02\x1B\x17\x83\x81\x1C\x90\x91\x10\x82\x1B\x17\x91\x82\x1C\x11\x17\x90V[`\0\x80\x82\x12\x80a\x14\x92WPg\x1B\xC1mgN\xC8\0\0\x82\x13[\x15a\x14\xB0W`@Qc-\x04\x83\xC5`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81`\0\x03a\x14\xD1W`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81g\x1B\xC1mgN\xC8\0\0\x03a\x14\xF9W`@Qc\"\xEDY\x85`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80\x15a\x15\x04W\x91\x90PV[`\0g\r\xE0\xB6\xB3\xA7d\0\0\x83\x12a\x15,Wa\x15'\x83g\x1B\xC1mgN\xC8\0\0a\x1E<V[a\x15.V[\x82[\x90P`\0a\x15D\x82g\x1B\xC1mgN\xC8\0\0a\x1A\xEBV[\x90P\x80`\0\x03a\x15gW`@Qc\x07\xA0!'`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a\x15r\x82a\x06\xCBV[\x90P`\0c;\x9A\xCA\0a\x15\x9Da\x15\x98a\x15\x92g\x1B\xC1mgN\xC8\0\0a\x1D\xA5V[\x85a\x16\xFCV[a\tfV[a\x15\xA7\x91\x90a\x1DuV[\x90P`\0\x80a\x15\xBE\x83g\x03\xC1f\\z\xAB \0a\x16\xFCV[a\x15\xD0\x90g \x05\xFEO&\x8E\xA0\0a\x1E\xB2V[\x90P`\0a\x16\0\x84a\x15\xE9\x86f\x9F2u$b\xA0\0a\x16\xFCV[a\x15\xFB\x90g\r\xC5R\x7Fd, \0a\x1E\xB2V[a\x16\xFCV[a\x16\x12\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[\x90Pa\x166g\t\xD0(\xCCo _\xFF\x19\x85a\x16,\x85\x85a\x1A\xEBV[a\x15\xFB\x91\x90a\x1E<V[\x92PPP`\0[`\x02\x81\x10\x15a\x16\xD1W`\0\x86a\x16R\x84a\x17^V[a\x16\\\x91\x90a\x1E<V[\x90P`\0a\x16j\x84\x85a\x16\xFCV[a\x16s\x90a\x1D\xA5V[\x90P`\0a\x16\x80\x82a\x19BV[\x90P`\0a\x16\x8E\x86\x85a\x16\xFCV[a\x16\xA0g\x0F\xA8\xCE\xDF\xC2\xAD\xDD\xFA\x84a\x16\xFCV[a\x16\xAA\x91\x90a\x1E<V[\x90Pa\x16\xB6\x84\x82a\x1A\xEBV[a\x16\xC0\x90\x87a\x1E\xB2V[\x95P\x84`\x01\x01\x94PPPPPa\x16=V[g\r\xE0\xB6\xB3\xA7d\0\0\x88\x12a\x16\xEEWa\x16\xE9\x82a\x1D\xA5V[a\x16\xF0V[\x81[\x98\x97PPPPPPPPV[`\0a\x03M\x83\x83g\r\xE0\xB6\xB3\xA7d\0\0a\x1A\xFCV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17)W`\0\x80\xFD[\x04\x92\x91PPV[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x04\x85\x14\x17\x16a\x17HW`\0\x80\xFD[`\x01\x82`\x01\x83\x03\x04\x01\x81\x15\x15\x02\x90P\x93\x92PPPV[`\0\x81`\0\x03a\x17wWPg\r\xE0\xB6\xB3\xA7d\0\0\x91\x90PV[gV\x98\xEE\xF0fp\0\0\x82\x12a\x17\x8EWP`\0\x91\x90PV[a\x17\x9FgV\x98\xEE\xF0fp\0\0a\x1D\xA5V[\x82\x13a\x17\xB4WPg\x1B\xC1mgN\xC8\0\0\x91\x90PV[`\0a\x17\xBF\x83a\x1B\x1BV[\x90P`\0a\x17\xF8g\r\xE0\xB6\xB3\xA7d\0\0a\x17\xE1\x84g\x1B\xC1mgN\xC8\0\0a\tQV[a\x17\xF3\x90g\r\xE0\xB6\xB3\xA7d\0\0a\x1E\xB2V[a\x1A\xEBV[\x90P`\0\x80\x82a\x18T\x81a\x18A\x81a\x18/\x81a\x18\x1C\x81g\x02_\x0F\xE1\x05\xA3\x14\0a\x16\xFCV[a\x15\xFB\x90g\x0Bh\xDF\x18\xE4q\xFB\xFF\x19a\x1E\xB2V[a\x15\xFB\x90g\x14\xA8EL\x19\xE1\xAC\0a\x1E\xB2V[a\x15\xFB\x90g\x0F\xC1\x0E\x01W\x82w\xFF\x19a\x1E\xB2V[a\x18f\x90g\x03\xDE\xBD\x08;\x8C|\0a\x1E\xB2V[\x91P\x83\x90Pa\x18\xCE\x81a\x18\xBC\x81a\x18\xAA\x81a\x18\x98\x81a\x18\x85\x81\x8Ba\x16\xFCV[a\x15\xFB\x90g\x02\x95\xD4\0\xEA2W\xFF\x19a\x1E\xB2V[a\x15\xFB\x90g\x01W\xD8\xB2\xEC\xC7\x08\0a\x1E\xB2V[a\x15\xFB\x90g\x051\n\xA7\xD5!0\0a\x1E\xB2V[a\x15\xFB\x90g\r\xE0\xCC=\x15a\0\0a\x1E\xB2V[\x91P\x81g\x11\x90\0\xAB\x10\x0F\xFC\0a\x18\xE4\x87\x88a\x16\xFCV[a\x18\xF0\x90`\0\x19a\x1DuV[a\x18\xFA\x91\x90a\x1E<V[a\x19\x04\x91\x90a\x1E\xB2V[\x92PP`\0a\x19\x12\x83a\x19BV[\x90P`\0a\x19 \x85\x83a\x16\xFCV[\x90P`\0\x88\x12a\x190W\x80a\x16\xF0V[a\x16\xF0\x81g\x1B\xC1mgN\xC8\0\0a\x1E<V[`\0h\x02H\xCE6\xA7\x0C\xB2k>\x19\x82\x13a\x19]WP`\0\x91\x90PV[h\x07U\xBFy\x8BJ\x1B\xF1\xE5\x82\x12a\x19\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkEXP_OVERFLOW`\xA0\x1B`D\x82\x01R`d\x01a\x07\x04V[e\x03x-\xAC\xE9\xD9`N\x83\x90\x1B\x05\x91P`\0``k\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x84\x82\x1B\x05`\x01`_\x1B\x01\x90\x1Dk\xB1r\x17\xF7\xD1\xCFy\xAB\xC9\xE3\xB3\x98\x81\x02\x90\x93\x03l$\x0C3\x0E\x9F\xB2\xD9\xCB\xAF\x0F\xD5\xAA\xFB\x19\x81\x01\x81\x02``\x90\x81\x1Dm\x02wYI\x91\xCF\xC8_n$a\x83|\xD9\x01\x82\x02\x81\x1Dm\x1AR\x12U\xE3OjPa\xB2^\xF1\xC9\xC3\x19\x01\x82\x02\x81\x1Dm\xB1\xBB\xB2\x01\xF4C\xCF\x96/\x1A\x1D=\xB4\xA5\x01\x82\x02\x81\x1Dn\x02\xC7#\x88\xD9\xF7OQ\xA93\x1F\xEDi?\x14\x19\x01\x82\x02\x81\x1Dn\x05\x18\x0B\xB1G\x99\xABG\xA8\xA8\xCB*R}W\x01m\x02\xD1g W{\xD1\x9B\xF6\x14\x17o\xE9\xEAl\x10\xFEh\xE7\xFD7\xD0\0{q?vP\x84\x01\x84\x02\x83\x1D\x90\x81\x01\x90\x84\x01m\x01\xD3\x96~\xD3\x0F\xC4\xF8\x9C\x02\xBA\xB5p\x81\x19\x01\x02\x90\x91\x1Dn\x05\x87\xF5\x03\xBBn\xA2\x9D%\xFC\xB7@\x19dP\x01\x90\x91\x02m6\rz\xEE\xA0\x93&>\xCCn\x0E\xCB)\x17`b\x1B\x01\x05t\x02\x9D\x9D\xC3\x85c\xC3.\\/m\xC1\x92\xEEp\xEFe\xF9\x97\x8A\xF3\x02`\xC3\x93\x90\x93\x03\x92\x90\x92\x1C\x92\x91PPV[`\0a\x03M\x83g\r\xE0\xB6\xB3\xA7d\0\0\x84[\x82\x82\x02\x81\x15\x15\x84\x15\x85\x83\x05\x85\x14\x17\x16a\x1B\x14W`\0\x80\xFD[\x05\x92\x91PPV[`\0`\x01`\xFF\x1B\x82\x03a\x1BAW`@QcM-u\xB1`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82\x12\x15a\x1BRWP\x19`\x01\x01\x90V[P\x90V[`\0` \x82\x84\x03\x12\x15a\x1BhW`\0\x80\xFD[P5\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a\x1B\x82W`\0\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[\x80\x15\x15\x81\x14a\x1B\x9FW`\0\x80\xFD[PV[`\0\x80`\0``\x84\x86\x03\x12\x15a\x1B\xB7W`\0\x80\xFD[\x835\x92P` \x84\x015a\x1B\xC9\x81a\x1B\x91V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[`\0[\x83\x81\x10\x15a\x1B\xF5W\x81\x81\x01Q\x83\x82\x01R` \x01a\x1B\xDDV[PP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra\x1C\x16\x81` \x86\x01` \x86\x01a\x1B\xDAV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[\x84\x15\x15\x81R\x83` \x82\x01R\x82`@\x82\x01R`\x80``\x82\x01R`\0a\x1CQ`\x80\x83\x01\x84a\x1B\xFEV[\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\x1C\x87W`\0\x80\xFD[\x84Qa\x1C\x92\x81a\x1B\x91V[\x80\x94PP` \x85\x01Q\x92P`@\x85\x01Q\x91P``\x85\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x1C\xBFW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x1C\xD3W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x1C\xE5Wa\x1C\xE5a\x1C[V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x1D\rWa\x1D\ra\x1C[V[\x81`@R\x82\x81R\x8A` \x84\x87\x01\x01\x11\x15a\x1D&W`\0\x80\xFD[a\x1D7\x83` \x83\x01` \x88\x01a\x1B\xDAV[\x97\x9A\x96\x99P\x94\x97PPPPPPV[`\0` \x82\x84\x03\x12\x15a\x1DXW`\0\x80\xFD[PQ\x91\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x80\x82\x02`\0\x82\x12`\x01`\xFF\x1B\x84\x14\x16\x15a\x1D\x91Wa\x1D\x91a\x1D_V[\x81\x81\x05\x83\x14\x82\x15\x17a\x030Wa\x030a\x1D_V[`\0`\x01`\xFF\x1B\x82\x01a\x1D\xBAWa\x1D\xBAa\x1D_V[P`\0\x03\x90V[\x82\x81R`@` \x82\x01R`\0a\x1D\xDA`@\x83\x01\x84a\x1B\xFEV[\x94\x93PPPPV[`@\x81R`\x19`@\x82\x01R\x7FDEX swap failed with data\0\0\0\0\0\0\0``\x82\x01R`\x80` \x82\x01R`\0a\x03M`\x80\x83\x01\x84a\x1B\xFEV[\x81\x81\x03\x81\x81\x11\x15a\x030Wa\x030a\x1D_V[\x81\x81\x03`\0\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x1E\\Wa\x1E\\a\x1D_V[P\x92\x91PPV[\x80\x82\x01\x80\x82\x11\x15a\x030Wa\x030a\x1D_V[`\0\x82a\x1E\x93WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[`\x01`\xFF\x1B\x82\x14`\0\x19\x84\x14\x16\x15a\x1E\xADWa\x1E\xADa\x1D_V[P\x05\x90V[\x80\x82\x01\x82\x81\x12`\0\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15a\x1E\xD2Wa\x1E\xD2a\x1D_V[PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 )x\xE1&\xC1\x04d\xBA\xA5ee\x13\x03\x04\xCFl\xF5\x1C\xED\xFE\xBE92\xC7\x16\xF9\x90o\xD3\x8C\xF6\x1AdsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static ATOMICV2_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct AtomicV2<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AtomicV2<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AtomicV2<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AtomicV2<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AtomicV2<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AtomicV2))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AtomicV2<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                ATOMICV2_ABI.clone(),
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
                ATOMICV2_ABI.clone(),
                ATOMICV2_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `XTOY` (0xf3c973cf) function
        pub fn xtoy(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([243, 201, 115, 207], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `YTOX` (0x6449fc57) function
        pub fn ytox(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([100, 73, 252, 87], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `asset` (0x38d52e0f) function
        pub fn asset(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([56, 213, 46, 15], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `cdf` (0xd0b71b1e) function
        pub fn cdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([208, 183, 27, 30], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `cumulativeProfit` (0x85b319ff) function
        pub fn cumulative_profit(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([133, 179, 25, 255], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `divWadDown` (0x37c6a44a) function
        pub fn div_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([55, 198, 164, 74], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `divWadUp` (0xbd252d06) function
        pub fn div_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 37, 45, 6], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `exchange` (0xd2f7265a) function
        pub fn exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([210, 247, 38, 90], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenXBalance` (0x936520c3)
        /// function
        pub fn intermediate_token_x_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([147, 101, 32, 195], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenYEndBalance` (0x72b98246)
        /// function
        pub fn intermediate_token_y_end_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([114, 185, 130, 70], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `intermediateTokenYStartBalance` (0xfa2e5994)
        /// function
        pub fn intermediate_token_y_start_balance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([250, 46, 89, 148], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `liquidExchange` (0x9f27ef4f) function
        pub fn liquid_exchange(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([159, 39, 239, 79], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log(
            &self,
            x: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([45, 91, 108, 185], x)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `lower_exchange_price` (0x96fbee1d) function
        pub fn lower_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([150, 251, 238, 29], (pool_id, input))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `mulWadDown` (0xe524f849) function
        pub fn mul_wad_down(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([229, 36, 248, 73], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `mulWadUp` (0xae9768a8) function
        pub fn mul_wad_up(
            &self,
            x: ::ethers::core::types::U256,
            y: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([174, 151, 104, 168], (x, y))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `pdf` (0xd24ce6e5) function
        pub fn pdf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([210, 76, 230, 229], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `ppf` (0x3679723a) function
        pub fn ppf(
            &self,
            input: ::ethers::core::types::I256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([54, 121, 114, 58], input)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `quote` (0x999b93af) function
        pub fn quote(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([153, 155, 147, 175], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `raise_exchange_price` (0xf9005eb5) function
        pub fn raise_exchange_price(
            &self,
            pool_id: ::ethers::core::types::U256,
            input: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 0, 94, 181], (pool_id, input))
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
                ::ethers::core::types::U256,
                ::ethers::core::types::Bytes,
            ),
        > {
            self.0
                .method_hash([57, 40, 255, 151], (pool_id, swap_x_in, amount_in))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `solver` (0x49a7a26d) function
        pub fn solver(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([73, 167, 162, 109], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `sqrt` (0x677342ce) function
        pub fn sqrt(
            &self,
            x: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([103, 115, 66, 206], x)
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Loss` event
        pub fn loss_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, LossFilter> {
            self.0.event()
        }
        /// Gets the contract's `Price` event
        pub fn price_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PriceFilter> {
            self.0.event()
        }
        /// Gets the contract's `Profit` event
        pub fn profit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ProfitFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, AtomicV2Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for AtomicV2<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    /// Custom Error type `AttemptedProfit` with signature
    /// `AttemptedProfit(int256)` and selector `0x85aba8de`
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
    #[etherror(name = "AttemptedProfit", abi = "AttemptedProfit(int256)")]
    pub struct AttemptedProfit {
        pub profit: ::ethers::core::types::I256,
    }
    /// Custom Error type `DexSwapFailure` with signature
    /// `DexSwapFailure(string,bytes)` and selector `0xcf42d71a`
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
    #[etherror(name = "DexSwapFailure", abi = "DexSwapFailure(string,bytes)")]
    pub struct DexSwapFailure {
        pub reason: ::std::string::String,
        pub err: ::ethers::core::types::Bytes,
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
    /// Custom Error type `InsufficientApprovalY` with signature
    /// `InsufficientApprovalY(uint256,uint256)` and selector `0xda56d3c5`
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
        name = "InsufficientApprovalY",
        abi = "InsufficientApprovalY(uint256,uint256)"
    )]
    pub struct InsufficientApprovalY {
        pub allowance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    /// Custom Error type `InsufficientBalanceX` with signature
    /// `InsufficientBalanceX(uint256,uint256)` and selector `0x0295b09c`
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
        name = "InsufficientBalanceX",
        abi = "InsufficientBalanceX(uint256,uint256)"
    )]
    pub struct InsufficientBalanceX {
        pub balance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
    /// Custom Error type `InsufficientBalanceY` with signature
    /// `InsufficientBalanceY(uint256,uint256)` and selector `0x0abe5a89`
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
        name = "InsufficientBalanceY",
        abi = "InsufficientBalanceY(uint256,uint256)"
    )]
    pub struct InsufficientBalanceY {
        pub balance: ::ethers::core::types::U256,
        pub payment: ::ethers::core::types::U256,
    }
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
    /// Custom Error type `SimulatedSwapFailure` with signature
    /// `SimulatedSwapFailure(bool,uint256,uint256,bytes)` and selector
    /// `0x18a73118`
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
        name = "SimulatedSwapFailure",
        abi = "SimulatedSwapFailure(bool,uint256,uint256,bytes)"
    )]
    pub struct SimulatedSwapFailure {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub estimated_price: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    /// Custom Error type `UnprofitableArbitrage` with signature
    /// `UnprofitableArbitrage(uint256,uint256,uint256)` and selector
    /// `0xb16e3783`
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
        name = "UnprofitableArbitrage",
        abi = "UnprofitableArbitrage(uint256,uint256,uint256)"
    )]
    pub struct UnprofitableArbitrage {
        pub start_y_balance: ::ethers::core::types::U256,
        pub end_y_balance: ::ethers::core::types::U256,
        pub absolute_difference: ::ethers::core::types::U256,
    }
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
    pub enum AtomicV2Errors {
        AttemptedProfit(AttemptedProfit),
        DexSwapFailure(DexSwapFailure),
        Infinity(Infinity),
        InsufficientApprovalY(InsufficientApprovalY),
        InsufficientBalanceX(InsufficientBalanceX),
        InsufficientBalanceY(InsufficientBalanceY),
        Min(Min),
        NegativeInfinity(NegativeInfinity),
        OutOfBounds(OutOfBounds),
        SimulatedSwapFailure(SimulatedSwapFailure),
        UnprofitableArbitrage(UnprofitableArbitrage),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicV2Errors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <AttemptedProfit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AttemptedProfit(decoded));
            }
            if let Ok(decoded) = <DexSwapFailure as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DexSwapFailure(decoded));
            }
            if let Ok(decoded) = <Infinity as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Infinity(decoded));
            }
            if let Ok(decoded) =
                <InsufficientApprovalY as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientApprovalY(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalanceX as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalanceX(decoded));
            }
            if let Ok(decoded) =
                <InsufficientBalanceY as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::InsufficientBalanceY(decoded));
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
            if let Ok(decoded) =
                <SimulatedSwapFailure as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulatedSwapFailure(decoded));
            }
            if let Ok(decoded) =
                <UnprofitableArbitrage as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::UnprofitableArbitrage(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicV2Errors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::AttemptedProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DexSwapFailure(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Infinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::InsufficientApprovalY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceX(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientBalanceY(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Min(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NegativeInfinity(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OutOfBounds(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SimulatedSwapFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnprofitableArbitrage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AtomicV2Errors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <AttemptedProfit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <DexSwapFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <Infinity as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InsufficientApprovalY as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalanceX as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <InsufficientBalanceY as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector == <Min as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NegativeInfinity as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector == <OutOfBounds as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SimulatedSwapFailure as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ if selector
                    == <UnprofitableArbitrage as ::ethers::contract::EthError>::selector() =>
                {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AtomicV2Errors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AttemptedProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DexSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::Infinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientApprovalY(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceX(element) => ::core::fmt::Display::fmt(element, f),
                Self::InsufficientBalanceY(element) => ::core::fmt::Display::fmt(element, f),
                Self::Min(element) => ::core::fmt::Display::fmt(element, f),
                Self::NegativeInfinity(element) => ::core::fmt::Display::fmt(element, f),
                Self::OutOfBounds(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulatedSwapFailure(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnprofitableArbitrage(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AtomicV2Errors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<AttemptedProfit> for AtomicV2Errors {
        fn from(value: AttemptedProfit) -> Self {
            Self::AttemptedProfit(value)
        }
    }
    impl ::core::convert::From<DexSwapFailure> for AtomicV2Errors {
        fn from(value: DexSwapFailure) -> Self {
            Self::DexSwapFailure(value)
        }
    }
    impl ::core::convert::From<Infinity> for AtomicV2Errors {
        fn from(value: Infinity) -> Self {
            Self::Infinity(value)
        }
    }
    impl ::core::convert::From<InsufficientApprovalY> for AtomicV2Errors {
        fn from(value: InsufficientApprovalY) -> Self {
            Self::InsufficientApprovalY(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceX> for AtomicV2Errors {
        fn from(value: InsufficientBalanceX) -> Self {
            Self::InsufficientBalanceX(value)
        }
    }
    impl ::core::convert::From<InsufficientBalanceY> for AtomicV2Errors {
        fn from(value: InsufficientBalanceY) -> Self {
            Self::InsufficientBalanceY(value)
        }
    }
    impl ::core::convert::From<Min> for AtomicV2Errors {
        fn from(value: Min) -> Self {
            Self::Min(value)
        }
    }
    impl ::core::convert::From<NegativeInfinity> for AtomicV2Errors {
        fn from(value: NegativeInfinity) -> Self {
            Self::NegativeInfinity(value)
        }
    }
    impl ::core::convert::From<OutOfBounds> for AtomicV2Errors {
        fn from(value: OutOfBounds) -> Self {
            Self::OutOfBounds(value)
        }
    }
    impl ::core::convert::From<SimulatedSwapFailure> for AtomicV2Errors {
        fn from(value: SimulatedSwapFailure) -> Self {
            Self::SimulatedSwapFailure(value)
        }
    }
    impl ::core::convert::From<UnprofitableArbitrage> for AtomicV2Errors {
        fn from(value: UnprofitableArbitrage) -> Self {
            Self::UnprofitableArbitrage(value)
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
    #[ethevent(name = "Loss", abi = "Loss(uint256)")]
    pub struct LossFilter {
        pub loss: ::ethers::core::types::U256,
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
    #[ethevent(name = "Price", abi = "Price(uint256,uint256)")]
    pub struct PriceFilter {
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
        Hash,
    )]
    #[ethevent(name = "Profit", abi = "Profit(uint256)")]
    pub struct ProfitFilter {
        pub profit: ::ethers::core::types::U256,
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
    pub enum AtomicV2Events {
        LossFilter(LossFilter),
        PriceFilter(PriceFilter),
        ProfitFilter(ProfitFilter),
    }
    impl ::ethers::contract::EthLogDecode for AtomicV2Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = LossFilter::decode_log(log) {
                return Ok(AtomicV2Events::LossFilter(decoded));
            }
            if let Ok(decoded) = PriceFilter::decode_log(log) {
                return Ok(AtomicV2Events::PriceFilter(decoded));
            }
            if let Ok(decoded) = ProfitFilter::decode_log(log) {
                return Ok(AtomicV2Events::ProfitFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AtomicV2Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LossFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PriceFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProfitFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LossFilter> for AtomicV2Events {
        fn from(value: LossFilter) -> Self {
            Self::LossFilter(value)
        }
    }
    impl ::core::convert::From<PriceFilter> for AtomicV2Events {
        fn from(value: PriceFilter) -> Self {
            Self::PriceFilter(value)
        }
    }
    impl ::core::convert::From<ProfitFilter> for AtomicV2Events {
        fn from(value: ProfitFilter) -> Self {
            Self::ProfitFilter(value)
        }
    }
    /// Container type for all input parameters for the `XTOY` function with
    /// signature `XTOY()` and selector `0xf3c973cf`
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
    #[ethcall(name = "XTOY", abi = "XTOY()")]
    pub struct XtoyCall;
    /// Container type for all input parameters for the `YTOX` function with
    /// signature `YTOX()` and selector `0x6449fc57`
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
    #[ethcall(name = "YTOX", abi = "YTOX()")]
    pub struct YtoxCall;
    /// Container type for all input parameters for the `asset` function with
    /// signature `asset()` and selector `0x38d52e0f`
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
    #[ethcall(name = "asset", abi = "asset()")]
    pub struct AssetCall;
    /// Container type for all input parameters for the `cdf` function with
    /// signature `cdf(int256)` and selector `0xd0b71b1e`
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
    #[ethcall(name = "cdf", abi = "cdf(int256)")]
    pub struct CdfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `cumulativeProfit`
    /// function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    #[ethcall(name = "cumulativeProfit", abi = "cumulativeProfit()")]
    pub struct CumulativeProfitCall;
    /// Container type for all input parameters for the `divWadDown` function
    /// with signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    #[ethcall(name = "divWadDown", abi = "divWadDown(uint256,uint256)")]
    pub struct DivWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `divWadUp` function with
    /// signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    #[ethcall(name = "divWadUp", abi = "divWadUp(uint256,uint256)")]
    pub struct DivWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `exchange` function with
    /// signature `exchange()` and selector `0xd2f7265a`
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
    #[ethcall(name = "exchange", abi = "exchange()")]
    pub struct ExchangeCall;
    /// Container type for all input parameters for the
    /// `intermediateTokenXBalance` function with signature
    /// `intermediateTokenXBalance()` and selector `0x936520c3`
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
        name = "intermediateTokenXBalance",
        abi = "intermediateTokenXBalance()"
    )]
    pub struct IntermediateTokenXBalanceCall;
    /// Container type for all input parameters for the
    /// `intermediateTokenYEndBalance` function with signature
    /// `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
        name = "intermediateTokenYEndBalance",
        abi = "intermediateTokenYEndBalance()"
    )]
    pub struct IntermediateTokenYEndBalanceCall;
    /// Container type for all input parameters for the
    /// `intermediateTokenYStartBalance` function with signature
    /// `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
        name = "intermediateTokenYStartBalance",
        abi = "intermediateTokenYStartBalance()"
    )]
    pub struct IntermediateTokenYStartBalanceCall;
    /// Container type for all input parameters for the `liquidExchange`
    /// function with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    #[ethcall(name = "liquidExchange", abi = "liquidExchange()")]
    pub struct LiquidExchangeCall;
    /// Container type for all input parameters for the `log` function with
    /// signature `log(int256)` and selector `0x2d5b6cb9`
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
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct LogCall {
        pub x: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `lower_exchange_price`
    /// function with signature `lower_exchange_price(uint256,uint256)` and
    /// selector `0x96fbee1d`
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
        name = "lower_exchange_price",
        abi = "lower_exchange_price(uint256,uint256)"
    )]
    pub struct LowerExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub input: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `mulWadDown` function
    /// with signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    #[ethcall(name = "mulWadDown", abi = "mulWadDown(uint256,uint256)")]
    pub struct MulWadDownCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `mulWadUp` function with
    /// signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    #[ethcall(name = "mulWadUp", abi = "mulWadUp(uint256,uint256)")]
    pub struct MulWadUpCall {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `pdf` function with
    /// signature `pdf(int256)` and selector `0xd24ce6e5`
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
    #[ethcall(name = "pdf", abi = "pdf(int256)")]
    pub struct PdfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `ppf` function with
    /// signature `ppf(int256)` and selector `0x3679723a`
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
    #[ethcall(name = "ppf", abi = "ppf(int256)")]
    pub struct PpfCall {
        pub input: ::ethers::core::types::I256,
    }
    /// Container type for all input parameters for the `quote` function with
    /// signature `quote()` and selector `0x999b93af`
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
    #[ethcall(name = "quote", abi = "quote()")]
    pub struct QuoteCall;
    /// Container type for all input parameters for the `raise_exchange_price`
    /// function with signature `raise_exchange_price(uint256,uint256)` and
    /// selector `0xf9005eb5`
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
        name = "raise_exchange_price",
        abi = "raise_exchange_price(uint256,uint256)"
    )]
    pub struct RaiseExchangePriceCall {
        pub pool_id: ::ethers::core::types::U256,
        pub input: ::ethers::core::types::U256,
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
    /// Container type for all input parameters for the `solver` function with
    /// signature `solver()` and selector `0x49a7a26d`
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
    #[ethcall(name = "solver", abi = "solver()")]
    pub struct SolverCall;
    /// Container type for all input parameters for the `sqrt` function with
    /// signature `sqrt(uint256)` and selector `0x677342ce`
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
    #[ethcall(name = "sqrt", abi = "sqrt(uint256)")]
    pub struct SqrtCall {
        pub x: ::ethers::core::types::U256,
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
    pub enum AtomicV2Calls {
        Xtoy(XtoyCall),
        Ytox(YtoxCall),
        Asset(AssetCall),
        Cdf(CdfCall),
        CumulativeProfit(CumulativeProfitCall),
        DivWadDown(DivWadDownCall),
        DivWadUp(DivWadUpCall),
        Exchange(ExchangeCall),
        IntermediateTokenXBalance(IntermediateTokenXBalanceCall),
        IntermediateTokenYEndBalance(IntermediateTokenYEndBalanceCall),
        IntermediateTokenYStartBalance(IntermediateTokenYStartBalanceCall),
        LiquidExchange(LiquidExchangeCall),
        Log(LogCall),
        LowerExchangePrice(LowerExchangePriceCall),
        MulWadDown(MulWadDownCall),
        MulWadUp(MulWadUpCall),
        Pdf(PdfCall),
        Ppf(PpfCall),
        Quote(QuoteCall),
        RaiseExchangePrice(RaiseExchangePriceCall),
        SimulateSwap(SimulateSwapCall),
        Solver(SolverCall),
        Sqrt(SqrtCall),
    }
    impl ::ethers::core::abi::AbiDecode for AtomicV2Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <XtoyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Xtoy(decoded));
            }
            if let Ok(decoded) = <YtoxCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ytox(decoded));
            }
            if let Ok(decoded) = <AssetCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Asset(decoded));
            }
            if let Ok(decoded) = <CdfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Cdf(decoded));
            }
            if let Ok(decoded) =
                <CumulativeProfitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CumulativeProfit(decoded));
            }
            if let Ok(decoded) = <DivWadDownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivWadDown(decoded));
            }
            if let Ok(decoded) = <DivWadUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::DivWadUp(decoded));
            }
            if let Ok(decoded) = <ExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Exchange(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenXBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenXBalance(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenYEndBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenYEndBalance(decoded));
            }
            if let Ok(decoded) =
                <IntermediateTokenYStartBalanceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IntermediateTokenYStartBalance(decoded));
            }
            if let Ok(decoded) =
                <LiquidExchangeCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LiquidExchange(decoded));
            }
            if let Ok(decoded) = <LogCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log(decoded));
            }
            if let Ok(decoded) =
                <LowerExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LowerExchangePrice(decoded));
            }
            if let Ok(decoded) = <MulWadDownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulWadDown(decoded));
            }
            if let Ok(decoded) = <MulWadUpCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MulWadUp(decoded));
            }
            if let Ok(decoded) = <PdfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pdf(decoded));
            }
            if let Ok(decoded) = <PpfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ppf(decoded));
            }
            if let Ok(decoded) = <QuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Quote(decoded));
            }
            if let Ok(decoded) =
                <RaiseExchangePriceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RaiseExchangePrice(decoded));
            }
            if let Ok(decoded) = <SimulateSwapCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SimulateSwap(decoded));
            }
            if let Ok(decoded) = <SolverCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Solver(decoded));
            }
            if let Ok(decoded) = <SqrtCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Sqrt(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AtomicV2Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Xtoy(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ytox(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Asset(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Cdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CumulativeProfit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::DivWadUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Exchange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IntermediateTokenXBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYEndBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LiquidExchange(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Log(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LowerExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MulWadDown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::MulWadUp(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pdf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Ppf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Quote(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RaiseExchangePrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SimulateSwap(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Solver(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Sqrt(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AtomicV2Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Xtoy(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ytox(element) => ::core::fmt::Display::fmt(element, f),
                Self::Asset(element) => ::core::fmt::Display::fmt(element, f),
                Self::Cdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::CumulativeProfit(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::DivWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Exchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenXBalance(element) => ::core::fmt::Display::fmt(element, f),
                Self::IntermediateTokenYEndBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IntermediateTokenYStartBalance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LiquidExchange(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowerExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadDown(element) => ::core::fmt::Display::fmt(element, f),
                Self::MulWadUp(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pdf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ppf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Quote(element) => ::core::fmt::Display::fmt(element, f),
                Self::RaiseExchangePrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SimulateSwap(element) => ::core::fmt::Display::fmt(element, f),
                Self::Solver(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sqrt(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<XtoyCall> for AtomicV2Calls {
        fn from(value: XtoyCall) -> Self {
            Self::Xtoy(value)
        }
    }
    impl ::core::convert::From<YtoxCall> for AtomicV2Calls {
        fn from(value: YtoxCall) -> Self {
            Self::Ytox(value)
        }
    }
    impl ::core::convert::From<AssetCall> for AtomicV2Calls {
        fn from(value: AssetCall) -> Self {
            Self::Asset(value)
        }
    }
    impl ::core::convert::From<CdfCall> for AtomicV2Calls {
        fn from(value: CdfCall) -> Self {
            Self::Cdf(value)
        }
    }
    impl ::core::convert::From<CumulativeProfitCall> for AtomicV2Calls {
        fn from(value: CumulativeProfitCall) -> Self {
            Self::CumulativeProfit(value)
        }
    }
    impl ::core::convert::From<DivWadDownCall> for AtomicV2Calls {
        fn from(value: DivWadDownCall) -> Self {
            Self::DivWadDown(value)
        }
    }
    impl ::core::convert::From<DivWadUpCall> for AtomicV2Calls {
        fn from(value: DivWadUpCall) -> Self {
            Self::DivWadUp(value)
        }
    }
    impl ::core::convert::From<ExchangeCall> for AtomicV2Calls {
        fn from(value: ExchangeCall) -> Self {
            Self::Exchange(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenXBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenXBalanceCall) -> Self {
            Self::IntermediateTokenXBalance(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenYEndBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenYEndBalanceCall) -> Self {
            Self::IntermediateTokenYEndBalance(value)
        }
    }
    impl ::core::convert::From<IntermediateTokenYStartBalanceCall> for AtomicV2Calls {
        fn from(value: IntermediateTokenYStartBalanceCall) -> Self {
            Self::IntermediateTokenYStartBalance(value)
        }
    }
    impl ::core::convert::From<LiquidExchangeCall> for AtomicV2Calls {
        fn from(value: LiquidExchangeCall) -> Self {
            Self::LiquidExchange(value)
        }
    }
    impl ::core::convert::From<LogCall> for AtomicV2Calls {
        fn from(value: LogCall) -> Self {
            Self::Log(value)
        }
    }
    impl ::core::convert::From<LowerExchangePriceCall> for AtomicV2Calls {
        fn from(value: LowerExchangePriceCall) -> Self {
            Self::LowerExchangePrice(value)
        }
    }
    impl ::core::convert::From<MulWadDownCall> for AtomicV2Calls {
        fn from(value: MulWadDownCall) -> Self {
            Self::MulWadDown(value)
        }
    }
    impl ::core::convert::From<MulWadUpCall> for AtomicV2Calls {
        fn from(value: MulWadUpCall) -> Self {
            Self::MulWadUp(value)
        }
    }
    impl ::core::convert::From<PdfCall> for AtomicV2Calls {
        fn from(value: PdfCall) -> Self {
            Self::Pdf(value)
        }
    }
    impl ::core::convert::From<PpfCall> for AtomicV2Calls {
        fn from(value: PpfCall) -> Self {
            Self::Ppf(value)
        }
    }
    impl ::core::convert::From<QuoteCall> for AtomicV2Calls {
        fn from(value: QuoteCall) -> Self {
            Self::Quote(value)
        }
    }
    impl ::core::convert::From<RaiseExchangePriceCall> for AtomicV2Calls {
        fn from(value: RaiseExchangePriceCall) -> Self {
            Self::RaiseExchangePrice(value)
        }
    }
    impl ::core::convert::From<SimulateSwapCall> for AtomicV2Calls {
        fn from(value: SimulateSwapCall) -> Self {
            Self::SimulateSwap(value)
        }
    }
    impl ::core::convert::From<SolverCall> for AtomicV2Calls {
        fn from(value: SolverCall) -> Self {
            Self::Solver(value)
        }
    }
    impl ::core::convert::From<SqrtCall> for AtomicV2Calls {
        fn from(value: SqrtCall) -> Self {
            Self::Sqrt(value)
        }
    }
    /// Container type for all return fields from the `XTOY` function with
    /// signature `XTOY()` and selector `0xf3c973cf`
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
    pub struct XtoyReturn(pub bool);
    /// Container type for all return fields from the `YTOX` function with
    /// signature `YTOX()` and selector `0x6449fc57`
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
    pub struct YtoxReturn(pub bool);
    /// Container type for all return fields from the `asset` function with
    /// signature `asset()` and selector `0x38d52e0f`
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
    pub struct AssetReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `cdf` function with
    /// signature `cdf(int256)` and selector `0xd0b71b1e`
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
    pub struct CdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `cumulativeProfit`
    /// function with signature `cumulativeProfit()` and selector `0x85b319ff`
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
    pub struct CumulativeProfitReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `divWadDown` function with
    /// signature `divWadDown(uint256,uint256)` and selector `0x37c6a44a`
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
    pub struct DivWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `divWadUp` function with
    /// signature `divWadUp(uint256,uint256)` and selector `0xbd252d06`
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
    pub struct DivWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `exchange` function with
    /// signature `exchange()` and selector `0xd2f7265a`
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
    pub struct ExchangeReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the
    /// `intermediateTokenXBalance` function with signature
    /// `intermediateTokenXBalance()` and selector `0x936520c3`
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
    pub struct IntermediateTokenXBalanceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the
    /// `intermediateTokenYEndBalance` function with signature
    /// `intermediateTokenYEndBalance()` and selector `0x72b98246`
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
    pub struct IntermediateTokenYEndBalanceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the
    /// `intermediateTokenYStartBalance` function with signature
    /// `intermediateTokenYStartBalance()` and selector `0xfa2e5994`
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
    pub struct IntermediateTokenYStartBalanceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `liquidExchange` function
    /// with signature `liquidExchange()` and selector `0x9f27ef4f`
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
    pub struct LiquidExchangeReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `log` function with
    /// signature `log(int256)` and selector `0x2d5b6cb9`
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
    pub struct LogReturn {
        pub z: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `mulWadDown` function with
    /// signature `mulWadDown(uint256,uint256)` and selector `0xe524f849`
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
    pub struct MulWadDownReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `mulWadUp` function with
    /// signature `mulWadUp(uint256,uint256)` and selector `0xae9768a8`
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
    pub struct MulWadUpReturn {
        pub z: ::ethers::core::types::U256,
    }
    /// Container type for all return fields from the `pdf` function with
    /// signature `pdf(int256)` and selector `0xd24ce6e5`
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
    pub struct PdfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `ppf` function with
    /// signature `ppf(int256)` and selector `0x3679723a`
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
    pub struct PpfReturn {
        pub output: ::ethers::core::types::I256,
    }
    /// Container type for all return fields from the `quote` function with
    /// signature `quote()` and selector `0x999b93af`
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
    pub struct QuoteReturn(pub ::ethers::core::types::Address);
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
    pub struct SimulateSwapReturn {
        pub valid: bool,
        pub estimated_out: ::ethers::core::types::U256,
        pub estimated_price: ::ethers::core::types::U256,
        pub payload: ::ethers::core::types::Bytes,
    }
    /// Container type for all return fields from the `solver` function with
    /// signature `solver()` and selector `0x49a7a26d`
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
    pub struct SolverReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `sqrt` function with
    /// signature `sqrt(uint256)` and selector `0x677342ce`
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
    pub struct SqrtReturn {
        pub z: ::ethers::core::types::U256,
    }
}

pub use weth::*;
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
pub mod weth {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("DOMAIN_SEPARATOR"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes32"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allowance"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("allowance"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::string::String::new(),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
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
                    ::std::borrow::ToOwned::to_owned("approve"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("approve"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("decimals"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("decimals"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint8"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("deposit"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
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
                    ::std::borrow::ToOwned::to_owned("nonces"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("nonces"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("address"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("permit"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("permit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("value"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("deadline"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("v"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint8"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("r"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("s"),
                                kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bytes32"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("symbol"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("totalSupply"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("totalSupply"),
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
                    ::std::borrow::ToOwned::to_owned("transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferFrom"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("transferFrom"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("withdraw"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("withdraw"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("amount"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("Approval"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Approval"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("spender"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Deposit"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Deposit"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Transfer"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Transfer"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("from"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Withdrawal"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("Withdrawal"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("to"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("amount"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: false,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: true,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static WETH_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@\x81\x81R4b\0\x04(Wb\0\0\x17\x82b\0\x04-V[`\r\x82R` l+\xB90\xB882\xB2\x10\"\xBA42\xB9`\x99\x1B\x81\x84\x01R\x81Q\x92b\0\0@\x84b\0\x04-V[`\x04\x84Rc\n\xE8\xAA\x89`\xE3\x1B\x82\x85\x01R\x80Q`\x01`\x01`@\x1B\x03\x93\x90\x84\x81\x11b\0\x04\x12W`\0\x90\x80b\0\0t\x83Tb\0\x04IV[\x94`\x1F\x95\x86\x81\x11b\0\x03\xC1W[P\x86\x90\x86\x83\x11`\x01\x14b\0\x03YW\x84\x92b\0\x03MW[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x81U[\x85Q\x85\x81\x11b\0\x039W`\x01\x90b\0\0\xC5\x82Tb\0\x04IV[\x85\x81\x11b\0\x02\xF1W[P\x85\x85\x82\x11`\x01\x14b\0\x02\x8CW\x83\x94\x95\x96\x97\x98\x82\x93\x94\x92b\0\x02\x80W[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\x12`\x80RF`\xA0R\x82Q\x93\x82\x90\x83T\x92b\0\x01\x1B\x84b\0\x04IV[\x90\x81\x88R\x88\x88\x01\x94\x89\x82\x82\x16\x91\x82`\0\x14b\0\x02cWPP`\x01\x14b\0\x02'W[PP\x85`\x1F\x19\x92\x03\x01\x16\x84\x01\x93\x80\x85\x10\x87\x86\x11\x17b\0\x02\x13W\x84\x84RQ\x90 \x93\x83\x01\x93\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x85R\x82\x84\x01R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6``\x84\x01RF`\x80\x84\x01R0`\xA0\x84\x01R`\xA0\x83R`\xC0\x83\x01\x94\x83\x86\x10\x90\x86\x11\x17b\0\x01\xFFWP\x83\x90RQ\x90 `\xC0Ra\x0C\xEF\x90\x81b\0\x04\x87\x829`\x80Q\x81a\x05\x97\x01R`\xA0Q\x81a\t\xFE\x01R`\xC0Q\x81a\n%\x01R\xF3[cNH{q`\xE0\x1B\x81R`A`\x04R`$\x90\xFD[cNH{q`\xE0\x1B\x83R`A`\x04R`$\x83\xFD[\x90\x88\x92\x93P\x85\x80R\x82\x86 \x91\x86\x92[\x82\x84\x10b\0\x02MWPPP\x86\x01\x01\x908\x80b\0\x01<V[\x80T\x8A\x85\x01\x86\x01R\x8A\x94\x90\x93\x01\x92\x81\x01b\0\x026V[\x92P\x93\x94PP`\xFF\x19\x16\x84R\x15\x15`\x05\x1B\x86\x01\x01\x908\x80b\0\x01<V[\x01Q\x90P8\x80b\0\0\xEBV[\x82\x84R\x86\x84 \x90`\x1F\x19\x83\x16\x85[\x81\x81\x10b\0\x02\xDBWP\x99\x83\x85\x96\x97\x98\x99\x9A\x9B\x10b\0\x02\xC1W[PPP\x81\x1B\x01\x81Ub\0\0\xFFV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x02\xB3V[\x8B\x83\x01Q\x84U\x92\x85\x01\x92\x91\x89\x01\x91\x89\x01b\0\x02\x9AV[\x82\x84R\x86\x84 \x86\x80\x84\x01`\x05\x1C\x82\x01\x92\x89\x85\x10b\0\x03/W[\x01`\x05\x1C\x01\x90\x83\x90[\x82\x81\x10b\0\x03#WPPb\0\0\xCEV[\x85\x81U\x01\x83\x90b\0\x03\x13V[\x92P\x81\x92b\0\x03\nV[cNH{q`\xE0\x1B\x82R`A`\x04R`$\x82\xFD[\x01Q\x90P8\x80b\0\0\x97V[\x84\x80R\x87\x85 \x92P`\x1F\x19\x84\x16\x85[\x89\x82\x82\x10b\0\x03\xAAWPP\x90\x84`\x01\x95\x94\x93\x92\x10b\0\x03\x90W[PPP\x81\x1B\x01\x81Ub\0\0\xACV[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U8\x80\x80b\0\x03\x82V[`\x01\x85\x96\x82\x93\x96\x86\x01Q\x81U\x01\x95\x01\x93\x01b\0\x03hV[\x90\x91P\x83\x80R\x86\x84 \x86\x80\x85\x01`\x05\x1C\x82\x01\x92\x89\x86\x10b\0\x04\x08W[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10b\0\x03\xF9WPb\0\0\x81V[\x85\x81U\x84\x93P`\x01\x01b\0\x03\xEAV[\x92P\x81\x92b\0\x03\xDDV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0\x80\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17b\0\x04\x12W`@RV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15b\0\x04{W[` \x83\x10\x14b\0\x04eWV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91b\0\x04YV\xFE`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a\x0B\x96V[\0[`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x01\x03W\x80c\t^\xA7\xB3\x14a\0\xFEW\x80c\x18\x16\r\xDD\x14a\0\xF9W\x80c#\xB8r\xDD\x14a\0\xF4W\x80c.\x1A}M\x14a\0\xEFW\x80c1<\xE5g\x14a\0\xEAW\x80c6D\xE5\x15\x14a\0\xE5W\x80cp\xA0\x821\x14a\0\xE0W\x80c~\xCE\xBE\0\x14a\0\xDBW\x80c\x95\xD8\x9BA\x14a\0\xD6W\x80c\xA9\x05\x9C\xBB\x14a\0\xD1W\x80c\xD0\xE3\r\xB0\x14a\0\xCCW\x80c\xD5\x05\xAC\xCF\x14a\0\xC7Wc\xDDb\xED>\x03a\0\x0EWa\t~V[a\x07\x88V[a\x07tV[a\x06\xF9V[a\x06RV[a\x06\x18V[a\x05\xDEV[a\x05\xBBV[a\x05}V[a\x04\xAFV[a\x03\x84V[a\x03fV[a\x02\xDBV[a\x01\xC3V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x018W[` \x83\x10\x14a\x01\"WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x01\x17V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01dW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\x01\xAFWPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\x01\x8DV[4a\x02\xAAW`\0\x80`\x03\x196\x01\x12a\x02\xA7W`@Q\x90\x80\x80T\x90a\x01\xE6\x82a\x01\x08V[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x02zWP`\x01\x14a\x02#W[a\x02\x1F\x86a\x02\x13\x81\x88\x03\x82a\x01BV[`@Q\x91\x82\x91\x82a\x01zV[\x03\x90\xF3[\x80\x80\x95PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x83\x85\x10a\x02gWPPPP\x81\x01` \x01a\x02\x13\x82a\x02\x1F8a\x02\x03V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x02JV[\x90P\x86\x95Pa\x02\x1F\x96\x93P` \x92Pa\x02\x13\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x938a\x02\x03V[\x80\xFD[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xAAWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xAAWV[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAWa\x02\xF4a\x02\xAFV[`$5\x903`\0R`\x04` R\x81a\x03\"\x82`@`\0 \x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x903\x90\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90` \x90\xA3` `@Q`\x01\x81R\xF3[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` `\x02T`@Q\x90\x81R\xF3[4a\x02\xAAW``6`\x03\x19\x01\x12a\x02\xAAWa\x03\x9Da\x02\xAFV[a\x03\xA5a\x02\xC5V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 `D5\x94\x91\x93\x91\x92\x91\x90T`\x01\x81\x01a\x04MW[Pa\x04\t`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R\x93`\x01\x80`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[a\x04\x14\x86\x82Ta\t\xECV[\x90U`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x88\x01\x90U\x90Q\x95\x86R\x91\x16\x93\xA3`@Q`\x01\x81R` \x90\xF3[\x85\x81\x03\x90\x81\x11a\x04\xAAW`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R\x93a\x04\t\x91a\x04\xA23a\x04\x8B\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\x04` R`@`\0 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U\x93Pa\x03\xDDV[a\t\xD6V[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x045`\0\x903\x82R`\x03` R`@\x82 \x90\x81T\x81\x81\x03\x90\x81\x11a\x04\xAAW\x83\x80\x80\x80\x94\x81\x94\x87U\x80`\x02T\x03`\x02U\x81`@Q\x82\x81R`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x92\xA3`@Q\x81\x81R\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be` 3\x92\xA23Z\xF1\x15a\x05BW\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` a\x05\xD6a\t\xF9V[`@Q\x90\x81R\xF3[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x01`\x01`\xA0\x1B\x03a\x05\xFFa\x02\xAFV[\x16`\0R`\x03` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x01`\x01`\xA0\x1B\x03a\x069a\x02\xAFV[\x16`\0R`\x05` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xAAW`\0\x80`\x03\x196\x01\x12a\x02\xA7W`@Q\x90\x80`\x01\x80T\x90a\x06w\x82a\x01\x08V[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x02zWP`\x01\x14a\x06\xA1Wa\x02\x1F\x86a\x02\x13\x81\x88\x03\x82a\x01BV[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x06\xE6WPPPP\x81\x01` \x01a\x02\x13\x82a\x02\x1F8a\x02\x03V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x06\xC9V[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAWa\x07\x12a\x02\xAFV[`$5\x903`\0R`\x03` R`@`\0 \x80T\x90\x83\x82\x03\x91\x82\x11a\x04\xAAWU`\x01\x80`\xA0\x1B\x03\x16\x90\x81`\0R`\x03` R`@`\0 \x81\x81T\x01\x90U`@Q\x90\x81R`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x92\xA3` `@Q`\x01\x81R\xF3[`\x006`\x03\x19\x01\x12a\x02\xAAWa\0!a\x0B\x96V[4a\x02\xAAW`\xE06`\x03\x19\x01\x12a\x02\xAAWa\x07\xA1a\x02\xAFV[a\x07\xA9a\x02\xC5V[\x90`D5`d5\x92`\x845\x93`\xFF\x85\x16\x85\x03a\x02\xAAWa\x08\xDD` \x91a\x07\xD1B\x82\x10\x15a\x0C\x04V[a\x08\xA4a\x08\xB0a\x07\xDFa\t\xF9V[\x92\x88a\x07\xFD\x81`\x01\x80`\xA0\x1B\x03\x16`\0R`\x05` R`@`\0 \x90V[\x80T`\x01\x81\x01\x90\x91U`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x82\x01R\x93\x8B\x16\x91\x84\x01\x91\x90\x91R``\x83\x01\x8B\x90R`\x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x92\x90\x92R\x81`\xC0\x82\x01\x03\x91a\x08{`\x1F\x19\x93\x84\x81\x01\x83R\x82a\x01BV[Q\x90 `@Q\x93\x84\x91\x88\x83\x01\x96\x87\x90\x91`B\x92a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x01BV[Q\x90 `@\x80Q\x91\x82R`\xFF\x90\x97\x16` \x82\x01R`\xA45\x96\x81\x01\x96\x90\x96R`\xC45``\x87\x01R`\x80\x86\x01\x90V[\x85`\0\x96\x87\x92\x83\x80R\x03\x90`\x01Z\xFA\x15a\tyW\x83Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91`\x01`\x01`\xA0\x1B\x03\x91\x84\x90a\tZ\x90\x83\x90a\x04\x8B\x90a\t@\x87\x82\x16\x80\x15\x15\x90\x81a\tmW[Pa\x0C\\V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 \x90V[U`@Q\x93\x84R\x81\x16\x93\x16\x91` \x90\xA3\x80\xF3[\x90P\x88\x8C\x16\x148a\t:V[a\x0CPV[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAW` a\t\xCDa\t\x9Ca\x02\xAFV[a\t\xA4a\x02\xC5V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x04\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x04\xAAWV[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\nGWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x91\x90\x81a\nX\x84a\x01\x08V[\x80\x83R` \x94\x85\x84\x01\x94`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x0BqWPP`\x01\x14a\x0B\x19W[PPP\x91\x81a\n\x94a\x0B\x13\x93a\x0B\x05\x95\x03\x82a\x01BV[Q\x90 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x95\x81\x01\x95\x86R` \x86\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x85\x01RF``\x85\x01R0`\x80\x85\x01R\x91\x82\x90`\xA0\x85\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01BV[Q\x90 \x90V[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\x0B\\WPPP\x82\x01\x01\x81a\n\x94a\x0B\x13a\n}V[\x80T\x86\x85\x01\x86\x01R\x87\x94\x90\x93\x01\x92\x81\x01a\x0BCV[`\xFF\x19\x16\x88R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\n\x94\x91Pa\x0B\x13\x90Pa\n}V[`\x02T4\x81\x01\x80\x91\x11a\x04\xAAW`\x02U3`\0R`\x03` R`@`\0 4\x81T\x01\x90U`@Q4\x81R`\0`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x93\xA3`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[\x15a\x0C\x0BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x15a\x0CcWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x90\xFD\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xE2\xC8\x9F \xA3\x18k\xD0\n\x15\xB6\xA9\xDB\x0Ce\x17\xD9\xBDo\x92\xDF\x97\x03\xDD\x9E\xE2\x86\xA7\\\x8E\x0BldsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static WETH_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10\x15a\0#W[6\x15a\0\x19W`\0\x80\xFD[a\0!a\x0B\x96V[\0[`\x005`\xE0\x1C\x80c\x06\xFD\xDE\x03\x14a\x01\x03W\x80c\t^\xA7\xB3\x14a\0\xFEW\x80c\x18\x16\r\xDD\x14a\0\xF9W\x80c#\xB8r\xDD\x14a\0\xF4W\x80c.\x1A}M\x14a\0\xEFW\x80c1<\xE5g\x14a\0\xEAW\x80c6D\xE5\x15\x14a\0\xE5W\x80cp\xA0\x821\x14a\0\xE0W\x80c~\xCE\xBE\0\x14a\0\xDBW\x80c\x95\xD8\x9BA\x14a\0\xD6W\x80c\xA9\x05\x9C\xBB\x14a\0\xD1W\x80c\xD0\xE3\r\xB0\x14a\0\xCCW\x80c\xD5\x05\xAC\xCF\x14a\0\xC7Wc\xDDb\xED>\x03a\0\x0EWa\t~V[a\x07\x88V[a\x07tV[a\x06\xF9V[a\x06RV[a\x06\x18V[a\x05\xDEV[a\x05\xBBV[a\x05}V[a\x04\xAFV[a\x03\x84V[a\x03fV[a\x02\xDBV[a\x01\xC3V[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x018W[` \x83\x10\x14a\x01\"WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x01\x17V[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x01dW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[` \x80\x82R\x82Q\x81\x83\x01\x81\x90R\x90\x93\x92`\0[\x82\x81\x10a\x01\xAFWPP`@\x92\x93P`\0\x83\x82\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[\x81\x81\x01\x86\x01Q\x84\x82\x01`@\x01R\x85\x01a\x01\x8DV[4a\x02\xAAW`\0\x80`\x03\x196\x01\x12a\x02\xA7W`@Q\x90\x80\x80T\x90a\x01\xE6\x82a\x01\x08V[\x80\x85R\x91` \x91`\x01\x91\x82\x81\x16\x90\x81\x15a\x02zWP`\x01\x14a\x02#W[a\x02\x1F\x86a\x02\x13\x81\x88\x03\x82a\x01BV[`@Q\x91\x82\x91\x82a\x01zV[\x03\x90\xF3[\x80\x80\x95PR\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x83\x85\x10a\x02gWPPPP\x81\x01` \x01a\x02\x13\x82a\x02\x1F8a\x02\x03V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x02JV[\x90P\x86\x95Pa\x02\x1F\x96\x93P` \x92Pa\x02\x13\x94\x91P`\xFF\x19\x16\x82\x84\x01R\x15\x15`\x05\x1B\x82\x01\x01\x92\x938a\x02\x03V[\x80\xFD[`\0\x80\xFD[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xAAWV[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\xAAWV[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAWa\x02\xF4a\x02\xAFV[`$5\x903`\0R`\x04` R\x81a\x03\"\x82`@`\0 \x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U`@Q\x91\x82R`\x01`\x01`\xA0\x1B\x03\x16\x903\x90\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x90` \x90\xA3` `@Q`\x01\x81R\xF3[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` `\x02T`@Q\x90\x81R\xF3[4a\x02\xAAW``6`\x03\x19\x01\x12a\x02\xAAWa\x03\x9Da\x02\xAFV[a\x03\xA5a\x02\xC5V[`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\0\x81\x81R`\x04` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 `D5\x94\x91\x93\x91\x92\x91\x90T`\x01\x81\x01a\x04MW[Pa\x04\t`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R\x93`\x01\x80`\xA0\x1B\x03\x16`\0R`\x03` R`@`\0 \x90V[a\x04\x14\x86\x82Ta\t\xECV[\x90U`\x01`\x01`\xA0\x1B\x03\x81\x16`\0\x90\x81R`\x03` \x90\x81R`@\x91\x82\x90 \x80T\x88\x01\x90U\x90Q\x95\x86R\x91\x16\x93\xA3`@Q`\x01\x81R` \x90\xF3[\x85\x81\x03\x90\x81\x11a\x04\xAAW`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R\x93a\x04\t\x91a\x04\xA23a\x04\x8B\x84`\x01\x80`\xA0\x1B\x03\x16`\0R`\x04` R`@`\0 \x90V[\x90`\x01\x80`\xA0\x1B\x03\x16`\0R` R`@`\0 \x90V[U\x93Pa\x03\xDDV[a\t\xD6V[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x045`\0\x903\x82R`\x03` R`@\x82 \x90\x81T\x81\x81\x03\x90\x81\x11a\x04\xAAW\x83\x80\x80\x80\x94\x81\x94\x87U\x80`\x02T\x03`\x02U\x81`@Q\x82\x81R`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x92\xA3`@Q\x81\x81R\x7F\x7F\xCFS,\x15\xF0\xA6\xDB\x0B\xD6\xD0\xE08\xBE\xA7\x1D0\xD8\x08\xC7\xD9\x8C\xB3\xBFrh\xA9[\xF5\x08\x1Be` 3\x92\xA23Z\xF1\x15a\x05BW\x80\xF3[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x13`$\x82\x01Rr\x11U\x12\x17\xD5\x14\x90S\x94\xD1\x91T\x97\xD1\x90RS\x11Q`j\x1B`D\x82\x01R`d\x90\xFD[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` `@Q`\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x81R\xF3[4a\x02\xAAW`\x006`\x03\x19\x01\x12a\x02\xAAW` a\x05\xD6a\t\xF9V[`@Q\x90\x81R\xF3[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x01`\x01`\xA0\x1B\x03a\x05\xFFa\x02\xAFV[\x16`\0R`\x03` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xAAW` 6`\x03\x19\x01\x12a\x02\xAAW`\x01`\x01`\xA0\x1B\x03a\x069a\x02\xAFV[\x16`\0R`\x05` R` `@`\0 T`@Q\x90\x81R\xF3[4a\x02\xAAW`\0\x80`\x03\x196\x01\x12a\x02\xA7W`@Q\x90\x80`\x01\x80T\x90a\x06w\x82a\x01\x08V[\x80\x86R\x92` \x92`\x01\x81\x16\x90\x81\x15a\x02zWP`\x01\x14a\x06\xA1Wa\x02\x1F\x86a\x02\x13\x81\x88\x03\x82a\x01BV[\x93P`\x01\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x83\x85\x10a\x06\xE6WPPPP\x81\x01` \x01a\x02\x13\x82a\x02\x1F8a\x02\x03V[\x80T\x86\x86\x01\x84\x01R\x93\x82\x01\x93\x81\x01a\x06\xC9V[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAWa\x07\x12a\x02\xAFV[`$5\x903`\0R`\x03` R`@`\0 \x80T\x90\x83\x82\x03\x91\x82\x11a\x04\xAAWU`\x01\x80`\xA0\x1B\x03\x16\x90\x81`\0R`\x03` R`@`\0 \x81\x81T\x01\x90U`@Q\x90\x81R`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x92\xA3` `@Q`\x01\x81R\xF3[`\x006`\x03\x19\x01\x12a\x02\xAAWa\0!a\x0B\x96V[4a\x02\xAAW`\xE06`\x03\x19\x01\x12a\x02\xAAWa\x07\xA1a\x02\xAFV[a\x07\xA9a\x02\xC5V[\x90`D5`d5\x92`\x845\x93`\xFF\x85\x16\x85\x03a\x02\xAAWa\x08\xDD` \x91a\x07\xD1B\x82\x10\x15a\x0C\x04V[a\x08\xA4a\x08\xB0a\x07\xDFa\t\xF9V[\x92\x88a\x07\xFD\x81`\x01\x80`\xA0\x1B\x03\x16`\0R`\x05` R`@`\0 \x90V[\x80T`\x01\x81\x01\x90\x91U`@\x80Q\x7Fnq\xED\xAE\x12\xB1\xB9\x7FM\x1F`7\x0F\xEF\x10\x10_\xA2\xFA\xAE\x01&\x11J\x16\x9Cd\x84]a&\xC9\x8A\x82\x01\x90\x81R`\x01`\x01`\xA0\x1B\x03\x94\x85\x16` \x82\x01R\x93\x8B\x16\x91\x84\x01\x91\x90\x91R``\x83\x01\x8B\x90R`\x80\x83\x01\x91\x90\x91R`\xA0\x82\x01\x92\x90\x92R\x81`\xC0\x82\x01\x03\x91a\x08{`\x1F\x19\x93\x84\x81\x01\x83R\x82a\x01BV[Q\x90 `@Q\x93\x84\x91\x88\x83\x01\x96\x87\x90\x91`B\x92a\x19\x01`\xF0\x1B\x83R`\x02\x83\x01R`\"\x82\x01R\x01\x90V[\x03\x90\x81\x01\x83R\x82a\x01BV[Q\x90 `@\x80Q\x91\x82R`\xFF\x90\x97\x16` \x82\x01R`\xA45\x96\x81\x01\x96\x90\x96R`\xC45``\x87\x01R`\x80\x86\x01\x90V[\x85`\0\x96\x87\x92\x83\x80R\x03\x90`\x01Z\xFA\x15a\tyW\x83Q\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x91`\x01`\x01`\xA0\x1B\x03\x91\x84\x90a\tZ\x90\x83\x90a\x04\x8B\x90a\t@\x87\x82\x16\x80\x15\x15\x90\x81a\tmW[Pa\x0C\\V[`\x01`\x01`\xA0\x1B\x03\x16`\0\x90\x81R`\x04` R`@\x90 \x90V[U`@Q\x93\x84R\x81\x16\x93\x16\x91` \x90\xA3\x80\xF3[\x90P\x88\x8C\x16\x148a\t:V[a\x0CPV[4a\x02\xAAW`@6`\x03\x19\x01\x12a\x02\xAAW` a\t\xCDa\t\x9Ca\x02\xAFV[a\t\xA4a\x02\xC5V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\0\x90\x81R`\x04\x85R`@\x80\x82 \x92\x90\x93\x16\x81R` \x91\x90\x91R \x90V[T`@Q\x90\x81R\xF3[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x91\x90\x82\x03\x91\x82\x11a\x04\xAAWV[`\0F\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x03a\nGWP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90V[`@Q\x81T\x91\x90\x81a\nX\x84a\x01\x08V[\x80\x83R` \x94\x85\x84\x01\x94`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x0BqWPP`\x01\x14a\x0B\x19W[PPP\x91\x81a\n\x94a\x0B\x13\x93a\x0B\x05\x95\x03\x82a\x01BV[Q\x90 `@\x80Q\x7F\x8Bs\xC3\xC6\x9B\xB8\xFE=Q.\xCCL\xF7Y\xCCy#\x9F{\x17\x9B\x0F\xFA\xCA\xA9\xA7]R+9@\x0F\x95\x81\x01\x95\x86R` \x86\x01\x92\x90\x92R\x7F\xC8\x9E\xFD\xAAT\xC0\xF2\x0Cz\xDFa(\x82\xDF\tP\xF5\xA9Qc~\x03\x07\xCD\xCBLg/)\x8B\x8B\xC6\x90\x85\x01RF``\x85\x01R0`\x80\x85\x01R\x91\x82\x90`\xA0\x85\x01\x90V[\x03`\x1F\x19\x81\x01\x83R\x82a\x01BV[Q\x90 \x90V[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\x0B\\WPPP\x82\x01\x01\x81a\n\x94a\x0B\x13a\n}V[\x80T\x86\x85\x01\x86\x01R\x87\x94\x90\x93\x01\x92\x81\x01a\x0BCV[`\xFF\x19\x16\x88R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\n\x94\x91Pa\x0B\x13\x90Pa\n}V[`\x02T4\x81\x01\x80\x91\x11a\x04\xAAW`\x02U3`\0R`\x03` R`@`\0 4\x81T\x01\x90U`@Q4\x81R`\0`\0\x80Q` a\x0C\x9A\x839\x81Q\x91R` 3\x93\xA3`@Q4\x81R\x7F\xE1\xFF\xFC\xC4\x92=\x04\xB5Y\xF4\xD2\x9A\x8B\xFCl\xDA\x04\xEB[\r<F\x07Q\xC2@,\\\\\xC9\x10\x9C` 3\x92\xA2V[\x15a\x0C\x0BWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x17`$\x82\x01R\x7FPERMIT_DEADLINE_EXPIRED\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x90\xFD[`@Q=`\0\x82>=\x90\xFD[\x15a\x0CcWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm$\xA7+ \xA6$\xA2/\xA9\xA4\xA3\xA7\"\xA9`\x91\x1B`D\x82\x01R`d\x90\xFD\xFE\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\xA2dipfsX\"\x12 \xE2\xC8\x9F \xA3\x18k\xD0\n\x15\xB6\xA9\xDB\x0Ce\x17\xD9\xBDo\x92\xDF\x97\x03\xDD\x9E\xE2\x86\xA7\\\x8E\x0BldsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static WETH_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct WETH<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for WETH<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for WETH<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for WETH<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for WETH<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(WETH))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> WETH<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                WETH_ABI.clone(),
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
                WETH_ABI.clone(),
                WETH_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `DOMAIN_SEPARATOR` (0x3644e515) function
        pub fn domain_separator(&self) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([54, 68, 229, 21], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `allowance` (0xdd62ed3e) function
        pub fn allowance(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([221, 98, 237, 62], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, amount))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], p0)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `decimals` (0x313ce567) function
        pub fn decimals(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([49, 60, 229, 103], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `deposit` (0xd0e30db0) function
        pub fn deposit(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 227, 13, 176], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `nonces` (0x7ecebe00) function
        pub fn nonces(
            &self,
            p0: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([126, 206, 190, 0], p0)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `permit` (0xd505accf) function
        pub fn permit(
            &self,
            owner: ::ethers::core::types::Address,
            spender: ::ethers::core::types::Address,
            value: ::ethers::core::types::U256,
            deadline: ::ethers::core::types::U256,
            v: u8,
            r: [u8; 32],
            s: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [213, 5, 172, 207],
                    (owner, spender, value, deadline, v, r, s),
                )
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `symbol` (0x95d89b41) function
        pub fn symbol(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([149, 216, 155, 65], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `totalSupply` (0x18160ddd) function
        pub fn total_supply(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([24, 22, 13, 221], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `transfer` (0xa9059cbb) function
        pub fn transfer(
            &self,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([169, 5, 156, 187], (to, amount))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, amount))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `withdraw` (0x2e1a7d4d) function
        pub fn withdraw(
            &self,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([46, 26, 125, 77], amount)
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        /// Gets the contract's `Deposit` event
        pub fn deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, DepositFilter> {
            self.0.event()
        }
        /// Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Gets the contract's `Withdrawal` event
        pub fn withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WithdrawalFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, WETHEvents> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for WETH<M> {
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
        Hash,
    )]
    #[ethevent(name = "Approval", abi = "Approval(address,address,uint256)")]
    pub struct ApprovalFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Transfer", abi = "Transfer(address,address,uint256)")]
    pub struct TransferFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Withdrawal", abi = "Withdrawal(address,uint256)")]
    pub struct WithdrawalFilter {
        #[ethevent(indexed)]
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
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
    pub enum WETHEvents {
        ApprovalFilter(ApprovalFilter),
        DepositFilter(DepositFilter),
        TransferFilter(TransferFilter),
        WithdrawalFilter(WithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for WETHEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(WETHEvents::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(WETHEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(WETHEvents::TransferFilter(decoded));
            }
            if let Ok(decoded) = WithdrawalFilter::decode_log(log) {
                return Ok(WETHEvents::WithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for WETHEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::DepositFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawalFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for WETHEvents {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<DepositFilter> for WETHEvents {
        fn from(value: DepositFilter) -> Self {
            Self::DepositFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for WETHEvents {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
    impl ::core::convert::From<WithdrawalFilter> for WETHEvents {
        fn from(value: WithdrawalFilter) -> Self {
            Self::WithdrawalFilter(value)
        }
    }
    /// Container type for all input parameters for the `DOMAIN_SEPARATOR`
    /// function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    #[ethcall(name = "DOMAIN_SEPARATOR", abi = "DOMAIN_SEPARATOR()")]
    pub struct DomainSeparatorCall;
    /// Container type for all input parameters for the `allowance` function
    /// with signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    #[ethcall(name = "allowance", abi = "allowance(address,address)")]
    pub struct AllowanceCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
    /// Container type for all input parameters for the `approve` function with
    /// signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    #[ethcall(name = "approve", abi = "approve(address,uint256)")]
    pub struct ApproveCall {
        pub spender: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `balanceOf` function
    /// with signature `balanceOf(address)` and selector `0x70a08231`
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
    #[ethcall(name = "balanceOf", abi = "balanceOf(address)")]
    pub struct BalanceOfCall(pub ::ethers::core::types::Address);
    /// Container type for all input parameters for the `decimals` function with
    /// signature `decimals()` and selector `0x313ce567`
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
    #[ethcall(name = "decimals", abi = "decimals()")]
    pub struct DecimalsCall;
    /// Container type for all input parameters for the `deposit` function with
    /// signature `deposit()` and selector `0xd0e30db0`
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
    #[ethcall(name = "deposit", abi = "deposit()")]
    pub struct DepositCall;
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
    /// Container type for all input parameters for the `nonces` function with
    /// signature `nonces(address)` and selector `0x7ecebe00`
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
    #[ethcall(name = "nonces", abi = "nonces(address)")]
    pub struct NoncesCall(pub ::ethers::core::types::Address);
    /// Container type for all input parameters for the `permit` function with
    /// signature `permit(address,address,uint256,uint256,uint8,bytes32,
    /// bytes32)` and selector `0xd505accf`
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
        name = "permit",
        abi = "permit(address,address,uint256,uint256,uint8,bytes32,bytes32)"
    )]
    pub struct PermitCall {
        pub owner: ::ethers::core::types::Address,
        pub spender: ::ethers::core::types::Address,
        pub value: ::ethers::core::types::U256,
        pub deadline: ::ethers::core::types::U256,
        pub v: u8,
        pub r: [u8; 32],
        pub s: [u8; 32],
    }
    /// Container type for all input parameters for the `symbol` function with
    /// signature `symbol()` and selector `0x95d89b41`
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
    #[ethcall(name = "symbol", abi = "symbol()")]
    pub struct SymbolCall;
    /// Container type for all input parameters for the `totalSupply` function
    /// with signature `totalSupply()` and selector `0x18160ddd`
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
    #[ethcall(name = "totalSupply", abi = "totalSupply()")]
    pub struct TotalSupplyCall;
    /// Container type for all input parameters for the `transfer` function with
    /// signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    #[ethcall(name = "transfer", abi = "transfer(address,uint256)")]
    pub struct TransferCall {
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `transferFrom` function
    /// with signature `transferFrom(address,address,uint256)` and selector
    /// `0x23b872dd`
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
    #[ethcall(name = "transferFrom", abi = "transferFrom(address,address,uint256)")]
    pub struct TransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `withdraw` function with
    /// signature `withdraw(uint256)` and selector `0x2e1a7d4d`
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
    #[ethcall(name = "withdraw", abi = "withdraw(uint256)")]
    pub struct WithdrawCall {
        pub amount: ::ethers::core::types::U256,
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
    pub enum WETHCalls {
        DomainSeparator(DomainSeparatorCall),
        Allowance(AllowanceCall),
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        Decimals(DecimalsCall),
        Deposit(DepositCall),
        Name(NameCall),
        Nonces(NoncesCall),
        Permit(PermitCall),
        Symbol(SymbolCall),
        TotalSupply(TotalSupplyCall),
        Transfer(TransferCall),
        TransferFrom(TransferFromCall),
        Withdraw(WithdrawCall),
    }
    impl ::ethers::core::abi::AbiDecode for WETHCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <DomainSeparatorCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::DomainSeparator(decoded));
            }
            if let Ok(decoded) = <AllowanceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Allowance(decoded));
            }
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <DecimalsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Decimals(decoded));
            }
            if let Ok(decoded) = <DepositCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Deposit(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <NoncesCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nonces(decoded));
            }
            if let Ok(decoded) = <PermitCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Permit(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TotalSupplyCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TotalSupply(decoded));
            }
            if let Ok(decoded) = <TransferCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Transfer(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            if let Ok(decoded) = <WithdrawCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Withdraw(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for WETHCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::DomainSeparator(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Allowance(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Decimals(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Deposit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Nonces(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Permit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TotalSupply(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Transfer(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Withdraw(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for WETHCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DomainSeparator(element) => ::core::fmt::Display::fmt(element, f),
                Self::Allowance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::Decimals(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deposit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nonces(element) => ::core::fmt::Display::fmt(element, f),
                Self::Permit(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TotalSupply(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transfer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::Withdraw(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<DomainSeparatorCall> for WETHCalls {
        fn from(value: DomainSeparatorCall) -> Self {
            Self::DomainSeparator(value)
        }
    }
    impl ::core::convert::From<AllowanceCall> for WETHCalls {
        fn from(value: AllowanceCall) -> Self {
            Self::Allowance(value)
        }
    }
    impl ::core::convert::From<ApproveCall> for WETHCalls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for WETHCalls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<DecimalsCall> for WETHCalls {
        fn from(value: DecimalsCall) -> Self {
            Self::Decimals(value)
        }
    }
    impl ::core::convert::From<DepositCall> for WETHCalls {
        fn from(value: DepositCall) -> Self {
            Self::Deposit(value)
        }
    }
    impl ::core::convert::From<NameCall> for WETHCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<NoncesCall> for WETHCalls {
        fn from(value: NoncesCall) -> Self {
            Self::Nonces(value)
        }
    }
    impl ::core::convert::From<PermitCall> for WETHCalls {
        fn from(value: PermitCall) -> Self {
            Self::Permit(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for WETHCalls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TotalSupplyCall> for WETHCalls {
        fn from(value: TotalSupplyCall) -> Self {
            Self::TotalSupply(value)
        }
    }
    impl ::core::convert::From<TransferCall> for WETHCalls {
        fn from(value: TransferCall) -> Self {
            Self::Transfer(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for WETHCalls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
    impl ::core::convert::From<WithdrawCall> for WETHCalls {
        fn from(value: WithdrawCall) -> Self {
            Self::Withdraw(value)
        }
    }
    /// Container type for all return fields from the `DOMAIN_SEPARATOR`
    /// function with signature `DOMAIN_SEPARATOR()` and selector `0x3644e515`
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
    pub struct DomainSeparatorReturn(pub [u8; 32]);
    /// Container type for all return fields from the `allowance` function with
    /// signature `allowance(address,address)` and selector `0xdd62ed3e`
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
    pub struct AllowanceReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `approve` function with
    /// signature `approve(address,uint256)` and selector `0x095ea7b3`
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
    pub struct ApproveReturn(pub bool);
    /// Container type for all return fields from the `balanceOf` function with
    /// signature `balanceOf(address)` and selector `0x70a08231`
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
    pub struct BalanceOfReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `decimals` function with
    /// signature `decimals()` and selector `0x313ce567`
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
    pub struct DecimalsReturn(pub u8);
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
    /// Container type for all return fields from the `nonces` function with
    /// signature `nonces(address)` and selector `0x7ecebe00`
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
    pub struct NoncesReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `symbol` function with
    /// signature `symbol()` and selector `0x95d89b41`
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
    pub struct SymbolReturn(pub ::std::string::String);
    /// Container type for all return fields from the `totalSupply` function
    /// with signature `totalSupply()` and selector `0x18160ddd`
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
    pub struct TotalSupplyReturn(pub ::ethers::core::types::U256);
    /// Container type for all return fields from the `transfer` function with
    /// signature `transfer(address,uint256)` and selector `0xa9059cbb`
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
    pub struct TransferReturn(pub bool);
    /// Container type for all return fields from the `transferFrom` function
    /// with signature `transferFrom(address,address,uint256)` and selector
    /// `0x23b872dd`
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
    pub struct TransferFromReturn(pub bool);
}

pub use mock_erc721::*;
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
pub mod mock_erc721 {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
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
                    ::std::borrow::ToOwned::to_owned("balanceOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("balanceOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("getApproved"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("getApproved"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("initialize"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_name"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("_symbol"),
                                kind: ::ethers::core::abi::ethabi::ParamType::String,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("string"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("isApprovedForAll"),
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
                    ::std::borrow::ToOwned::to_owned("ownerOf"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("ownerOf"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
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
                    ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("safeTransferFrom"),
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
                                    name: ::std::borrow::ToOwned::to_owned("id"),
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
                            state_mutability:
                                ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("setApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("address"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("bool"),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("supportsInterface"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("supportsInterface"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("interfaceId"),
                            kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(4usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bytes4"),
                            ),
                        },],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("bool"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
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
                    ::std::borrow::ToOwned::to_owned("tokenURI"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("tokenURI"),
                        inputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::borrow::ToOwned::to_owned("id"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                    ::std::vec![::ethers::core::abi::ethabi::Event {
                        name: ::std::borrow::ToOwned::to_owned("ApprovalForAll"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("owner"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("operator"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                indexed: true,
                            },
                            ::ethers::core::abi::ethabi::EventParam {
                                name: ::std::borrow::ToOwned::to_owned("approved"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Bool,
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
                                name: ::std::borrow::ToOwned::to_owned("id"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                indexed: true,
                            },
                        ],
                        anonymous: false,
                    },],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    /// The parsed JSON ABI of the contract.
    pub static MOCKERC721_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80\x80`@R4a\0\x16Wa\x0E\xDC\x90\x81a\0\x1C\x829\xF3[`\0\x80\xFD\xFE`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x01\xFF\xC9\xA7\x14a\n\xA0WP\x81c\x06\xFD\xDE\x03\x14a\t\xEFW\x81c\x08\x18\x12\xFC\x14a\t\xBDW\x81c\t^\xA7\xB3\x14a\t\x08W\x81c#\xB8r\xDD\x14a\x08\xF0W\x81cB\x84.\x0E\x14a\x080W\x81cL\xD8\x8Bv\x14a\x04\xF5W\x81ccR!\x1E\x14a\x04\x8AW\x81cp\xA0\x821\x14a\x04\x17W\x81c\x95\xD8\x9BA\x14a\x03%W\x81c\xA2,\xB4e\x14a\x02\xA0W\x81c\xB8\x8DO\xDE\x14a\x01nWP\x80c\xC8{V\xDD\x14a\x01\x0FWc\xE9\x85\xE9\xC5\x14a\0\xBFW`\0\x80\xFD[4a\x01\x0BW\x80`\x03\x196\x01\x12a\x01\x0BW`\xFF\x81` \x93a\0\xDDa\x0B\xBDV[a\0\xE5a\x0B\xD8V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x83R`\x05\x87R\x83\x83 \x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[P\x80\xFD[P4a\x01\x0BW` \x80`\x03\x196\x01\x12a\x01jW\x91\x81Q\x92\x83\x91` \x83R``Q\x91\x82` \x85\x01R\x81[\x83\x81\x10a\x01UWPP\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[`\x80\x81\x01Q\x87\x82\x01\x87\x01R\x86\x94P\x81\x01a\x018V[\x82\x80\xFD[\x90P4a\x01jW`\x806`\x03\x19\x01\x12a\x01jWa\x01\x89a\x0B\xBDV[\x90a\x01\x92a\x0B\xD8V[`D5`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\x9CW6`#\x82\x01\x12\x15a\x02\x9CWa\x01\xC5\x906\x90`$\x81\x87\x015\x91\x01a\x0C#V[\x91a\x01\xD1\x82\x82\x87a\x0C\xC5V[\x80;\x15\x94\x85\x15a\x01\xE9W[\x87a\x01\xE6\x87a\x0EgV[\x80\xF3[` \x93\x94\x95P\x87`\x01\x80`\xA0\x1B\x03\x80\x92a\x022\x8AQ\x98\x89\x97\x88\x96\x87\x94c\n\x85\xBD\x01`\xE1\x1B\x9D\x8E\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x0B}V[\x03\x93\x16Z\xF1\x90\x81\x15a\x02\x8FWa\x01\xE6\x93P\x84\x91a\x02`W[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80\x80a\x01\xDCV[a\x02\x82\x91P` =` \x11a\x02\x88W[a\x02z\x81\x83a\x0BEV[\x81\x01\x90a\x0EGV[8a\x02JV[P=a\x02pV[PPPQ\x90=\x90\x82>=\x90\xFD[\x86\x80\xFD[PP4a\x01\x0BW\x80`\x03\x196\x01\x12a\x01\x0BWa\x02\xBAa\x0B\xBDV[\x90`$5\x90\x81\x15\x15\x80\x92\x03a\x03!W3\x84R`\x05` R\x80\x84 \x92`\x01\x80`\xA0\x1B\x03\x16\x92\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83\x80\xFD[\x82\x844a\x04\x14W\x80`\x03\x196\x01\x12a\x04\x14W\x81Q\x91\x82\x82`\x01\x93`\x01T\x94a\x03L\x86a\x0B\x0BV[\x91\x82\x85R` \x96\x87`\x01\x82\x16\x91\x82`\0\x14a\x03\xEDWPP`\x01\x14a\x03\x91W[PPPa\x03\x8D\x92\x91a\x03~\x91\x03\x85a\x0BEV[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x0B}V[\x03\x90\xF3[\x91\x90\x86\x93P`\x01\x83R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x84\x10a\x03\xD5WPPP\x82\x01\x01\x81a\x03~a\x03\x8Da\x03kV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x03\xBCV[`\xFF\x19\x16\x87\x82\x01R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\x03~\x91Pa\x03\x8D\x90Pa\x03kV[\x80\xFD[\x83\x91P4a\x01\x0BW` 6`\x03\x19\x01\x12a\x01\x0BW`\x01`\x01`\xA0\x1B\x03a\x04;a\x0B\xBDV[\x16\x90\x81\x15a\x04XW` \x84\x80\x85\x85\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[`d\x90` \x85Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0C`$\x82\x01RkZERO_ADDRESS`\xA0\x1B`D\x82\x01R\xFD[\x90P\x824a\x04\x14W` 6`\x03\x19\x01\x12a\x04\x14W\x815\x81R`\x02` R\x82\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x04\xC5WP` \x91Q\x90\x81R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD3RS\x95\x11Q`\xB2\x1B`D\x82\x01R\xFD[\x83\x91P4a\x01\x0BW\x82`\x03\x196\x01\x12a\x01\x0BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x03!Wa\x05'\x906\x90\x84\x01a\x0CjV[\x91`$5\x82\x81\x11a\x08,Wa\x05?\x906\x90\x83\x01a\x0CjV[\x94`\xFF`\x06T\x16a\x07\xF3WP\x82Q\x82\x81\x11a\x07\xE0W\x80a\x05_\x86Ta\x0B\x0BV[\x94`\x1F\x95\x86\x81\x11a\x07uW[P` \x90\x86\x83\x11`\x01\x14a\x06\xF4W\x87\x92a\x06\xE9W[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x06\xD6WP`\x01\x91a\x05\xAC\x83Ta\x0B\x0BV[\x81\x81\x11a\x06tW[P` \x90\x82\x11`\x01\x14a\x05\xF9W\x83\x94\x82\x93\x94\x92a\x05\xEEW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\xFF\x19`\x06T\x16\x17`\x06U\x80\xF3[\x01Q\x90P\x84\x80a\x05\xCCV[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x06^WP\x95\x83\x85\x96\x97\x10a\x06EW[PPP\x81\x1B\x01\x81Ua\x05\xE0V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x068V[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x06%V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x06\xCDW[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x06\xC2WPPa\x05\xB4V[\x86\x81U\x01\x84\x90a\x06\xB4V[\x92P\x81\x92a\x06\xABV[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x05\x80V[\x87\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x07]WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07DW[PPP\x81\x1B\x01\x84Ua\x05\x95V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x077V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07!V[\x90\x91P\x86\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x07\xD7W[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\xC9WPa\x05kV[\x88\x81U\x84\x93P`\x01\x01a\x07\xBCV[\x92P\x81\x92a\x07\xAFV[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R\xFD[\x84\x80\xFD[\x90P4a\x01jWa\x08@6a\x0B\xEEV[\x90a\x08N\x82\x82\x85\x96\x95a\x0C\xC5V[\x80;\x15\x93\x84\x15a\x08cW[\x86a\x01\xE6\x86a\x0EgV[` \x92\x93\x94P`\xA4\x90\x87`\x01\x80`\xA0\x1B\x03\x80\x94\x89Q\x97\x88\x96\x87\x95c\n\x85\xBD\x01`\xE1\x1B\x9B\x8C\x88R3\x90\x88\x01R\x16`$\x86\x01R`D\x85\x01R`\x80`d\x85\x01R\x82`\x84\x85\x01R\x16Z\xF1\x90\x81\x15a\x02\x8FWa\x01\xE6\x93P\x84\x91a\x08\xD1W[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80a\x08YV[a\x08\xEA\x91P` =` \x11a\x02\x88Wa\x02z\x81\x83a\x0BEV[8a\x08\xBCV[\x834a\x04\x14Wa\x01\xE6a\t\x026a\x0B\xEEV[\x91a\x0C\xC5V[\x90P4a\x01jW\x81`\x03\x196\x01\x12a\x01jWa\t\"a\x0B\xBDV[`$5\x80\x85R`\x02` R\x83\x85 T\x90\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x903\x84\x14\x80\x15a\t\x9EW[a\tU\x90a\x0C\x88V[\x85\x87R` R\x85 \x92\x16\x91\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84\x80\xA4\x80\xF3[P\x83\x87R`\x05` \x90\x81R\x82\x88 3\x89R\x90R\x81\x87 T`\xFF\x16a\tLV[\x90P4a\x01jW` 6`\x03\x19\x01\x12a\x01jW\x805\x83R` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x82\x844a\x04\x14W\x80`\x03\x196\x01\x12a\x04\x14W\x81Q\x91\x82\x82\x83Ta\n\x11\x81a\x0B\x0BV[\x90\x81\x84R` \x95`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x03\xEDWPP`\x01\x14a\nEWPPPa\x03\x8D\x92\x91a\x03~\x91\x03\x85a\x0BEV[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\n\x88WPPP\x82\x01\x01\x81a\x03~a\x03\x8Da\x03kV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\noV[\x84\x914a\x01jW` 6`\x03\x19\x01\x12a\x01jW5c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01jW` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\n\xFAW[\x81\x15a\n\xE9W[P\x15\x15\x81R\xF3[c[^\x13\x9F`\xE0\x1B\x14\x90P\x83a\n\xE2V[c\x80\xACX\xCD`\xE0\x1B\x81\x14\x91Pa\n\xDBV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B;W[` \x83\x10\x14a\x0B%WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0B\x1AV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0BgW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x0B\xA9WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0B\x88V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xD3WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xD3WV[``\x90`\x03\x19\x01\x12a\x0B\xD3W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x0B\xD3W\x91`$5\x90\x81\x16\x81\x03a\x0B\xD3W\x90`D5\x90V[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0BgW`@Q\x91a\x0CM`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x0BEV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x0B\xD3W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xD3W\x81` a\x0C\x85\x935\x91\x01a\x0C#V[\x90V[\x15a\x0C\x8FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x95\x94\x86\x16\x94\x90\x86\x16\x85\x03a\x0E\x16W\x85\x16\x94\x85\x15a\r\xDEWa\r\x12\x90\x853\x14\x90\x81\x15a\r\xC1W[\x81\x15a\r\xABW[Pa\x0C\x88V[\x83\x83R`\x03\x82R\x80\x83 \x80T\x90\x81\x15a\r\x97W`\0\x19\x91\x82\x01\x90U\x85\x84R`\x03\x83R\x81\x84 \x80T\x90\x91\x81\x14a\r\x97W`\x01\x01\x90U\x85\x83R`\x02\x82R\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x04\x90\x92R\x82 \x80T\x90\x91\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x80\xA4V[cNH{q`\xE0\x1B\x85R`\x11`\x04R`$\x85\xFD[\x90P\x87\x85R`\x04\x84R\x82\x85 T\x163\x148a\r\x0CV[\x86\x86R`\x05\x85R\x83\x86 3\x87R\x85R\x83\x86 T`\xFF\x16\x91Pa\r\x05V[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\n`$\x82\x01RiWRONG_FROM`\xB0\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x0B\xD3WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x0B\xD3W\x90V[\x15a\x0EnWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \xDB\x0C\xBC\xBE\x06\x83\xD5\x165\x86\x12E\x04\x1B\x03\xB2(kBg\xB7\xA9\xEAgA,\x95>\x84\xEB\x0F\xC7dsolcC\0\x08\x16\x003";
    /// The bytecode of the contract.
    pub static MOCKERC721_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@\x81\x81R`\x04\x91\x826\x10\x15a\0\x16W`\0\x80\xFD[`\0\x92\x835`\xE0\x1C\x91\x82c\x01\xFF\xC9\xA7\x14a\n\xA0WP\x81c\x06\xFD\xDE\x03\x14a\t\xEFW\x81c\x08\x18\x12\xFC\x14a\t\xBDW\x81c\t^\xA7\xB3\x14a\t\x08W\x81c#\xB8r\xDD\x14a\x08\xF0W\x81cB\x84.\x0E\x14a\x080W\x81cL\xD8\x8Bv\x14a\x04\xF5W\x81ccR!\x1E\x14a\x04\x8AW\x81cp\xA0\x821\x14a\x04\x17W\x81c\x95\xD8\x9BA\x14a\x03%W\x81c\xA2,\xB4e\x14a\x02\xA0W\x81c\xB8\x8DO\xDE\x14a\x01nWP\x80c\xC8{V\xDD\x14a\x01\x0FWc\xE9\x85\xE9\xC5\x14a\0\xBFW`\0\x80\xFD[4a\x01\x0BW\x80`\x03\x196\x01\x12a\x01\x0BW`\xFF\x81` \x93a\0\xDDa\x0B\xBDV[a\0\xE5a\x0B\xD8V[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x83R`\x05\x87R\x83\x83 \x91\x16\x82R\x85R T\x91Q\x91\x16\x15\x15\x81R\xF3[P\x80\xFD[P4a\x01\x0BW` \x80`\x03\x196\x01\x12a\x01jW\x91\x81Q\x92\x83\x91` \x83R``Q\x91\x82` \x85\x01R\x81[\x83\x81\x10a\x01UWPP\x82\x82\x01\x84\x01R`\x1F\x01`\x1F\x19\x16\x81\x01\x03\x01\x90\xF3[`\x80\x81\x01Q\x87\x82\x01\x87\x01R\x86\x94P\x81\x01a\x018V[\x82\x80\xFD[\x90P4a\x01jW`\x806`\x03\x19\x01\x12a\x01jWa\x01\x89a\x0B\xBDV[\x90a\x01\x92a\x0B\xD8V[`D5`d5g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11a\x02\x9CW6`#\x82\x01\x12\x15a\x02\x9CWa\x01\xC5\x906\x90`$\x81\x87\x015\x91\x01a\x0C#V[\x91a\x01\xD1\x82\x82\x87a\x0C\xC5V[\x80;\x15\x94\x85\x15a\x01\xE9W[\x87a\x01\xE6\x87a\x0EgV[\x80\xF3[` \x93\x94\x95P\x87`\x01\x80`\xA0\x1B\x03\x80\x92a\x022\x8AQ\x98\x89\x97\x88\x96\x87\x94c\n\x85\xBD\x01`\xE1\x1B\x9D\x8E\x87R3\x90\x87\x01R\x16`$\x85\x01R`D\x84\x01R`\x80`d\x84\x01R`\x84\x83\x01\x90a\x0B}V[\x03\x93\x16Z\xF1\x90\x81\x15a\x02\x8FWa\x01\xE6\x93P\x84\x91a\x02`W[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80\x80a\x01\xDCV[a\x02\x82\x91P` =` \x11a\x02\x88W[a\x02z\x81\x83a\x0BEV[\x81\x01\x90a\x0EGV[8a\x02JV[P=a\x02pV[PPPQ\x90=\x90\x82>=\x90\xFD[\x86\x80\xFD[PP4a\x01\x0BW\x80`\x03\x196\x01\x12a\x01\x0BWa\x02\xBAa\x0B\xBDV[\x90`$5\x90\x81\x15\x15\x80\x92\x03a\x03!W3\x84R`\x05` R\x80\x84 \x92`\x01\x80`\xA0\x1B\x03\x16\x92\x83\x85R` R\x80\x84 `\xFF\x19\x81T\x16`\xFF\x84\x16\x17\x90UQ\x90\x81R\x7F\x170~\xAB9\xABa\x07\xE8\x89\x98E\xAD=Y\xBD\x96S\xF2\0\xF2 \x92\x04\x89\xCA+Y7il1` 3\x92\xA3\x80\xF3[\x83\x80\xFD[\x82\x844a\x04\x14W\x80`\x03\x196\x01\x12a\x04\x14W\x81Q\x91\x82\x82`\x01\x93`\x01T\x94a\x03L\x86a\x0B\x0BV[\x91\x82\x85R` \x96\x87`\x01\x82\x16\x91\x82`\0\x14a\x03\xEDWPP`\x01\x14a\x03\x91W[PPPa\x03\x8D\x92\x91a\x03~\x91\x03\x85a\x0BEV[Q\x92\x82\x84\x93\x84R\x83\x01\x90a\x0B}V[\x03\x90\xF3[\x91\x90\x86\x93P`\x01\x83R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6[\x82\x84\x10a\x03\xD5WPPP\x82\x01\x01\x81a\x03~a\x03\x8Da\x03kV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\x03\xBCV[`\xFF\x19\x16\x87\x82\x01R\x93\x15\x15`\x05\x1B\x86\x01\x90\x93\x01\x93P\x84\x92Pa\x03~\x91Pa\x03\x8D\x90Pa\x03kV[\x80\xFD[\x83\x91P4a\x01\x0BW` 6`\x03\x19\x01\x12a\x01\x0BW`\x01`\x01`\xA0\x1B\x03a\x04;a\x0B\xBDV[\x16\x90\x81\x15a\x04XW` \x84\x80\x85\x85\x81R`\x03\x84R T\x90Q\x90\x81R\xF3[`d\x90` \x85Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x0C`$\x82\x01RkZERO_ADDRESS`\xA0\x1B`D\x82\x01R\xFD[\x90P\x824a\x04\x14W` 6`\x03\x19\x01\x12a\x04\x14W\x815\x81R`\x02` R\x82\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90\x81\x15a\x04\xC5WP` \x91Q\x90\x81R\xF3[`d\x90` \x84Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\n`$\x82\x01Ri\x13\x93\xD5\x17\xD3RS\x95\x11Q`\xB2\x1B`D\x82\x01R\xFD[\x83\x91P4a\x01\x0BW\x82`\x03\x196\x01\x12a\x01\x0BWg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x815\x81\x81\x11a\x03!Wa\x05'\x906\x90\x84\x01a\x0CjV[\x91`$5\x82\x81\x11a\x08,Wa\x05?\x906\x90\x83\x01a\x0CjV[\x94`\xFF`\x06T\x16a\x07\xF3WP\x82Q\x82\x81\x11a\x07\xE0W\x80a\x05_\x86Ta\x0B\x0BV[\x94`\x1F\x95\x86\x81\x11a\x07uW[P` \x90\x86\x83\x11`\x01\x14a\x06\xF4W\x87\x92a\x06\xE9W[PP\x81`\x01\x1B\x91`\0\x19\x90`\x03\x1B\x1C\x19\x16\x17\x84U[\x84Q\x91\x82\x11a\x06\xD6WP`\x01\x91a\x05\xAC\x83Ta\x0B\x0BV[\x81\x81\x11a\x06tW[P` \x90\x82\x11`\x01\x14a\x05\xF9W\x83\x94\x82\x93\x94\x92a\x05\xEEW[PP`\0\x19`\x03\x83\x90\x1B\x1C\x19\x16\x90\x82\x1B\x17\x81U[`\xFF\x19`\x06T\x16\x17`\x06U\x80\xF3[\x01Q\x90P\x84\x80a\x05\xCCV[\x82\x84R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x90`\x1F\x19\x83\x16\x85[\x81\x81\x10a\x06^WP\x95\x83\x85\x96\x97\x10a\x06EW[PPP\x81\x1B\x01\x81Ua\x05\xE0V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x84\x80\x80a\x068V[\x87\x83\x01Q\x84U\x92\x85\x01\x92` \x92\x83\x01\x92\x01a\x06%V[\x83\x85R\x7F\xB1\x0E-Rv\x12\x07;&\xEE\xCD\xFDq~j2\x0C\xF4KJ\xFA\xC2\xB0s-\x9F\xCB\xE2\xB7\xFA\x0C\xF6\x82\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x06\xCDW[\x01`\x05\x1C\x01\x90\x84\x90[\x82\x81\x10a\x06\xC2WPPa\x05\xB4V[\x86\x81U\x01\x84\x90a\x06\xB4V[\x92P\x81\x92a\x06\xABV[cNH{q`\xE0\x1B\x84R`A\x90R`$\x83\xFD[\x01Q\x90P\x87\x80a\x05\x80V[\x87\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x92P`\x1F\x19\x84\x16\x88[\x81\x81\x10a\x07]WP\x90\x84`\x01\x95\x94\x93\x92\x10a\x07DW[PPP\x81\x1B\x01\x84Ua\x05\x95V[\x01Q`\0\x19`\xF8\x84`\x03\x1B\x16\x1C\x19\x16\x90U\x87\x80\x80a\x077V[\x92\x93` `\x01\x81\x92\x87\x86\x01Q\x81U\x01\x95\x01\x93\x01a\x07!V[\x90\x91P\x86\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c\x86\x80\x85\x01`\x05\x1C\x82\x01\x92` \x86\x10a\x07\xD7W[\x90\x85\x94\x93\x92\x91\x01`\x05\x1C\x01\x90[\x81\x81\x10a\x07\xC9WPa\x05kV[\x88\x81U\x84\x93P`\x01\x01a\x07\xBCV[\x92P\x81\x92a\x07\xAFV[cNH{q`\xE0\x1B\x85R`A\x82R`$\x85\xFD[\x90` `d\x92Q\x91bF\x1B\xCD`\xE5\x1B\x83R\x82\x01R`\x13`$\x82\x01Rr\x10S\x14\x91PQ\x16W\xD2S\x92U\x12PS\x12V\x91Q`j\x1B`D\x82\x01R\xFD[\x84\x80\xFD[\x90P4a\x01jWa\x08@6a\x0B\xEEV[\x90a\x08N\x82\x82\x85\x96\x95a\x0C\xC5V[\x80;\x15\x93\x84\x15a\x08cW[\x86a\x01\xE6\x86a\x0EgV[` \x92\x93\x94P`\xA4\x90\x87`\x01\x80`\xA0\x1B\x03\x80\x94\x89Q\x97\x88\x96\x87\x95c\n\x85\xBD\x01`\xE1\x1B\x9B\x8C\x88R3\x90\x88\x01R\x16`$\x86\x01R`D\x85\x01R`\x80`d\x85\x01R\x82`\x84\x85\x01R\x16Z\xF1\x90\x81\x15a\x02\x8FWa\x01\xE6\x93P\x84\x91a\x08\xD1W[P`\x01`\x01`\xE0\x1B\x03\x19\x16\x148\x80\x80\x80a\x08YV[a\x08\xEA\x91P` =` \x11a\x02\x88Wa\x02z\x81\x83a\x0BEV[8a\x08\xBCV[\x834a\x04\x14Wa\x01\xE6a\t\x026a\x0B\xEEV[\x91a\x0C\xC5V[\x90P4a\x01jW\x81`\x03\x196\x01\x12a\x01jWa\t\"a\x0B\xBDV[`$5\x80\x85R`\x02` R\x83\x85 T\x90\x93\x91\x92`\x01`\x01`\xA0\x1B\x03\x91\x82\x16\x92\x903\x84\x14\x80\x15a\t\x9EW[a\tU\x90a\x0C\x88V[\x85\x87R` R\x85 \x92\x16\x91\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xA0\x1B\x82T\x16\x17\x90U\x7F\x8C[\xE1\xE5\xEB\xEC}[\xD1OqB}\x1E\x84\xF3\xDD\x03\x14\xC0\xF7\xB2)\x1E[ \n\xC8\xC7\xC3\xB9%\x84\x80\xA4\x80\xF3[P\x83\x87R`\x05` \x90\x81R\x82\x88 3\x89R\x90R\x81\x87 T`\xFF\x16a\tLV[\x90P4a\x01jW` 6`\x03\x19\x01\x12a\x01jW\x805\x83R` \x90\x81R\x91\x81\x90 T\x90Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xF3[\x82\x844a\x04\x14W\x80`\x03\x196\x01\x12a\x04\x14W\x81Q\x91\x82\x82\x83Ta\n\x11\x81a\x0B\x0BV[\x90\x81\x84R` \x95`\x01\x91\x87`\x01\x82\x16\x91\x82`\0\x14a\x03\xEDWPP`\x01\x14a\nEWPPPa\x03\x8D\x92\x91a\x03~\x91\x03\x85a\x0BEV[\x91\x90\x86\x93P\x82\x80R\x7F)\r\xEC\xD9T\x8Bb\xA8\xD6\x03E\xA9\x888o\xC8K\xA6\xBC\x95H@\x08\xF66/\x93\x16\x0E\xF3\xE5c[\x82\x84\x10a\n\x88WPPP\x82\x01\x01\x81a\x03~a\x03\x8Da\x03kV[\x80T\x84\x8A\x01\x86\x01R\x88\x95P\x87\x94\x90\x93\x01\x92\x81\x01a\noV[\x84\x914a\x01jW` 6`\x03\x19\x01\x12a\x01jW5c\xFF\xFF\xFF\xFF`\xE0\x1B\x81\x16\x80\x91\x03a\x01jW` \x92Pc\x01\xFF\xC9\xA7`\xE0\x1B\x81\x14\x90\x81\x15a\n\xFAW[\x81\x15a\n\xE9W[P\x15\x15\x81R\xF3[c[^\x13\x9F`\xE0\x1B\x14\x90P\x83a\n\xE2V[c\x80\xACX\xCD`\xE0\x1B\x81\x14\x91Pa\n\xDBV[\x90`\x01\x82\x81\x1C\x92\x16\x80\x15a\x0B;W[` \x83\x10\x14a\x0B%WV[cNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[\x91`\x7F\x16\x91a\x0B\x1AV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17a\x0BgW`@RV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x91\x90\x82Q\x92\x83\x82R`\0[\x84\x81\x10a\x0B\xA9WPP\x82`\0` \x80\x94\x95\x84\x01\x01R`\x1F\x80\x19\x91\x01\x16\x01\x01\x90V[` \x81\x83\x01\x81\x01Q\x84\x83\x01\x82\x01R\x01a\x0B\x88V[`\x045\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xD3WV[`\0\x80\xFD[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x0B\xD3WV[``\x90`\x03\x19\x01\x12a\x0B\xD3W`\x01`\x01`\xA0\x1B\x03\x90`\x045\x82\x81\x16\x81\x03a\x0B\xD3W\x91`$5\x90\x81\x16\x81\x03a\x0B\xD3W\x90`D5\x90V[\x92\x91\x92g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11a\x0BgW`@Q\x91a\x0CM`\x1F\x82\x01`\x1F\x19\x16` \x01\x84a\x0BEV[\x82\x94\x81\x84R\x81\x83\x01\x11a\x0B\xD3W\x82\x81` \x93\x84`\0\x96\x017\x01\x01RV[\x90\x80`\x1F\x83\x01\x12\x15a\x0B\xD3W\x81` a\x0C\x85\x935\x91\x01a\x0C#V[\x90V[\x15a\x0C\x8FWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0E`$\x82\x01Rm\x13\x93\xD5\x17\xD0UU\x12\x13\xD4\x92V\x91Q`\x92\x1B`D\x82\x01R`d\x90\xFD[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 T`\x01`\x01`\xA0\x1B\x03\x95\x94\x86\x16\x94\x90\x86\x16\x85\x03a\x0E\x16W\x85\x16\x94\x85\x15a\r\xDEWa\r\x12\x90\x853\x14\x90\x81\x15a\r\xC1W[\x81\x15a\r\xABW[Pa\x0C\x88V[\x83\x83R`\x03\x82R\x80\x83 \x80T\x90\x81\x15a\r\x97W`\0\x19\x91\x82\x01\x90U\x85\x84R`\x03\x83R\x81\x84 \x80T\x90\x91\x81\x14a\r\x97W`\x01\x01\x90U\x85\x83R`\x02\x82R\x80\x83 \x80T`\x01`\x01`\xA0\x1B\x03\x19\x90\x81\x16\x87\x17\x90\x91U`\x04\x90\x92R\x82 \x80T\x90\x91\x16\x90U\x7F\xDD\xF2R\xAD\x1B\xE2\xC8\x9Bi\xC2\xB0h\xFC7\x8D\xAA\x95+\xA7\xF1c\xC4\xA1\x16(\xF5ZM\xF5#\xB3\xEF\x90\x80\xA4V[cNH{q`\xE0\x1B\x85R`\x11`\x04R`$\x85\xFD[\x90P\x87\x85R`\x04\x84R\x82\x85 T\x163\x148a\r\x0CV[\x86\x86R`\x05\x85R\x83\x86 3\x87R\x85R\x83\x86 T`\xFF\x16\x91Pa\r\x05V[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\x11`$\x82\x01Rp\x12S\x95\x90S\x12Q\x17\xD4\x91P\xD2T\x12QS\x95`z\x1B`D\x82\x01R`d\x90\xFD[\x81QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x81\x01\x84\x90R`\n`$\x82\x01RiWRONG_FROM`\xB0\x1B`D\x82\x01R`d\x90\xFD[\x90\x81` \x91\x03\x12a\x0B\xD3WQ`\x01`\x01`\xE0\x1B\x03\x19\x81\x16\x81\x03a\x0B\xD3W\x90V[\x15a\x0EnWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Ro\x15S\x94\xD0Q\x91W\xD4\x91P\xD2T\x12QS\x95`\x82\x1B`D\x82\x01R`d\x90\xFD\xFE\xA2dipfsX\"\x12 \xDB\x0C\xBC\xBE\x06\x83\xD5\x165\x86\x12E\x04\x1B\x03\xB2(kBg\xB7\xA9\xEAgA,\x95>\x84\xEB\x0F\xC7dsolcC\0\x08\x16\x003";
    /// The deployed bytecode of the contract.
    pub static MOCKERC721_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct MockERC721<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MockERC721<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MockERC721<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MockERC721<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MockERC721<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MockERC721))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MockERC721<M> {
        /// Creates a new contract instance with the specified `ethers` client
        /// at `address`. The contract derefs to a `ethers::Contract`
        /// object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                MOCKERC721_ABI.clone(),
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
                MOCKERC721_ABI.clone(),
                MOCKERC721_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        /// Calls the contract's `approve` (0x095ea7b3) function
        pub fn approve(
            &self,
            spender: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 94, 167, 179], (spender, id))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `balanceOf` (0x70a08231) function
        pub fn balance_of(
            &self,
            owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([112, 160, 130, 49], owner)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `getApproved` (0x081812fc) function
        pub fn get_approved(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([8, 24, 18, 252], p0)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `initialize` (0x4cd88b76) function
        pub fn initialize(
            &self,
            name: ::std::string::String,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 216, 139, 118], (name, symbol))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `isApprovedForAll` (0xe985e9c5) function
        pub fn is_approved_for_all(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([233, 133, 233, 197], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `name` (0x06fdde03) function
        pub fn name(&self) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `ownerOf` (0x6352211e) function
        pub fn owner_of(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([99, 82, 33, 30], id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `safeTransferFrom` (0x42842e0e) function
        pub fn safe_transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 132, 46, 14], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `safeTransferFrom` (0xb88d4fde) function
        pub fn safe_transfer_from_with_from_and_to_and_data(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 141, 79, 222], (from, to, id, data))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `setApprovalForAll` (0xa22cb465) function
        pub fn set_approval_for_all(
            &self,
            operator: ::ethers::core::types::Address,
            approved: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([162, 44, 180, 101], (operator, approved))
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
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
        /// Calls the contract's `tokenURI` (0xc87b56dd) function
        pub fn token_uri(
            &self,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([200, 123, 86, 221], id)
                .expect("method not found (this should never happen)")
        }
        /// Calls the contract's `transferFrom` (0x23b872dd) function
        pub fn transfer_from(
            &self,
            from: ::ethers::core::types::Address,
            to: ::ethers::core::types::Address,
            id: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([35, 184, 114, 221], (from, to, id))
                .expect("method not found (this should never happen)")
        }
        /// Gets the contract's `Approval` event
        pub fn approval_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalFilter> {
            self.0.event()
        }
        /// Gets the contract's `ApprovalForAll` event
        pub fn approval_for_all_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, ApprovalForAllFilter>
        {
            self.0.event()
        }
        /// Gets the contract's `Transfer` event
        pub fn transfer_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, TransferFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, MockERC721Events> {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>> for MockERC721<M> {
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
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    #[ethevent(name = "ApprovalForAll", abi = "ApprovalForAll(address,address,bool)")]
    pub struct ApprovalForAllFilter {
        #[ethevent(indexed)]
        pub owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
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
        #[ethevent(indexed)]
        pub id: ::ethers::core::types::U256,
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
    pub enum MockERC721Events {
        ApprovalFilter(ApprovalFilter),
        ApprovalForAllFilter(ApprovalForAllFilter),
        TransferFilter(TransferFilter),
    }
    impl ::ethers::contract::EthLogDecode for MockERC721Events {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ApprovalFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalFilter(decoded));
            }
            if let Ok(decoded) = ApprovalForAllFilter::decode_log(log) {
                return Ok(MockERC721Events::ApprovalForAllFilter(decoded));
            }
            if let Ok(decoded) = TransferFilter::decode_log(log) {
                return Ok(MockERC721Events::TransferFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for MockERC721Events {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ApprovalFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApprovalForAllFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApprovalFilter> for MockERC721Events {
        fn from(value: ApprovalFilter) -> Self {
            Self::ApprovalFilter(value)
        }
    }
    impl ::core::convert::From<ApprovalForAllFilter> for MockERC721Events {
        fn from(value: ApprovalForAllFilter) -> Self {
            Self::ApprovalForAllFilter(value)
        }
    }
    impl ::core::convert::From<TransferFilter> for MockERC721Events {
        fn from(value: TransferFilter) -> Self {
            Self::TransferFilter(value)
        }
    }
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
        pub id: ::ethers::core::types::U256,
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
    pub struct BalanceOfCall {
        pub owner: ::ethers::core::types::Address,
    }
    /// Container type for all input parameters for the `getApproved` function
    /// with signature `getApproved(uint256)` and selector `0x081812fc`
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
    #[ethcall(name = "getApproved", abi = "getApproved(uint256)")]
    pub struct GetApprovedCall(pub ::ethers::core::types::U256);
    /// Container type for all input parameters for the `initialize` function
    /// with signature `initialize(string,string)` and selector `0x4cd88b76`
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
    #[ethcall(name = "initialize", abi = "initialize(string,string)")]
    pub struct InitializeCall {
        pub name: ::std::string::String,
        pub symbol: ::std::string::String,
    }
    /// Container type for all input parameters for the `isApprovedForAll`
    /// function with signature `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
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
    #[ethcall(name = "isApprovedForAll", abi = "isApprovedForAll(address,address)")]
    pub struct IsApprovedForAllCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
    );
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
    /// Container type for all input parameters for the `ownerOf` function with
    /// signature `ownerOf(uint256)` and selector `0x6352211e`
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
    #[ethcall(name = "ownerOf", abi = "ownerOf(uint256)")]
    pub struct OwnerOfCall {
        pub id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `safeTransferFrom`
    /// function with signature `safeTransferFrom(address,address,uint256)` and
    /// selector `0x42842e0e`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256)"
    )]
    pub struct SafeTransferFromCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
    }
    /// Container type for all input parameters for the `safeTransferFrom`
    /// function with signature
    /// `safeTransferFrom(address,address,uint256,bytes)` and selector
    /// `0xb88d4fde`
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
        name = "safeTransferFrom",
        abi = "safeTransferFrom(address,address,uint256,bytes)"
    )]
    pub struct SafeTransferFromWithFromAndToAndDataCall {
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub id: ::ethers::core::types::U256,
        pub data: ::ethers::core::types::Bytes,
    }
    /// Container type for all input parameters for the `setApprovalForAll`
    /// function with signature `setApprovalForAll(address,bool)` and selector
    /// `0xa22cb465`
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
    #[ethcall(name = "setApprovalForAll", abi = "setApprovalForAll(address,bool)")]
    pub struct SetApprovalForAllCall {
        pub operator: ::ethers::core::types::Address,
        pub approved: bool,
    }
    /// Container type for all input parameters for the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
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
    /// Container type for all input parameters for the `tokenURI` function with
    /// signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    #[ethcall(name = "tokenURI", abi = "tokenURI(uint256)")]
    pub struct TokenURICall {
        pub id: ::ethers::core::types::U256,
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
        pub id: ::ethers::core::types::U256,
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
    pub enum MockERC721Calls {
        Approve(ApproveCall),
        BalanceOf(BalanceOfCall),
        GetApproved(GetApprovedCall),
        Initialize(InitializeCall),
        IsApprovedForAll(IsApprovedForAllCall),
        Name(NameCall),
        OwnerOf(OwnerOfCall),
        SafeTransferFrom(SafeTransferFromCall),
        SafeTransferFromWithFromAndToAndData(SafeTransferFromWithFromAndToAndDataCall),
        SetApprovalForAll(SetApprovalForAllCall),
        SupportsInterface(SupportsInterfaceCall),
        Symbol(SymbolCall),
        TokenURI(TokenURICall),
        TransferFrom(TransferFromCall),
    }
    impl ::ethers::core::abi::AbiDecode for MockERC721Calls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ApproveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Approve(decoded));
            }
            if let Ok(decoded) = <BalanceOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BalanceOf(decoded));
            }
            if let Ok(decoded) = <GetApprovedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetApproved(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsApprovedForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::IsApprovedForAll(decoded));
            }
            if let Ok(decoded) = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded) = <OwnerOfCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SafeTransferFrom(decoded));
            }
            if let Ok(decoded) =
                <SafeTransferFromWithFromAndToAndDataCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::SafeTransferFromWithFromAndToAndData(decoded));
            }
            if let Ok(decoded) =
                <SetApprovalForAllCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetApprovalForAll(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded) = <SymbolCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Symbol(decoded));
            }
            if let Ok(decoded) = <TokenURICall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::TokenURI(decoded));
            }
            if let Ok(decoded) = <TransferFromCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferFrom(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MockERC721Calls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Approve(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::BalanceOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetApproved(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::IsApprovedForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OwnerOf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetApprovalForAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SupportsInterface(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Symbol(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenURI(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferFrom(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for MockERC721Calls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Approve(element) => ::core::fmt::Display::fmt(element, f),
                Self::BalanceOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetApproved(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsApprovedForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnerOf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFrom(element) => ::core::fmt::Display::fmt(element, f),
                Self::SafeTransferFromWithFromAndToAndData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetApprovalForAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::Symbol(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenURI(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferFrom(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ApproveCall> for MockERC721Calls {
        fn from(value: ApproveCall) -> Self {
            Self::Approve(value)
        }
    }
    impl ::core::convert::From<BalanceOfCall> for MockERC721Calls {
        fn from(value: BalanceOfCall) -> Self {
            Self::BalanceOf(value)
        }
    }
    impl ::core::convert::From<GetApprovedCall> for MockERC721Calls {
        fn from(value: GetApprovedCall) -> Self {
            Self::GetApproved(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for MockERC721Calls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<IsApprovedForAllCall> for MockERC721Calls {
        fn from(value: IsApprovedForAllCall) -> Self {
            Self::IsApprovedForAll(value)
        }
    }
    impl ::core::convert::From<NameCall> for MockERC721Calls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<OwnerOfCall> for MockERC721Calls {
        fn from(value: OwnerOfCall) -> Self {
            Self::OwnerOf(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromCall> for MockERC721Calls {
        fn from(value: SafeTransferFromCall) -> Self {
            Self::SafeTransferFrom(value)
        }
    }
    impl ::core::convert::From<SafeTransferFromWithFromAndToAndDataCall> for MockERC721Calls {
        fn from(value: SafeTransferFromWithFromAndToAndDataCall) -> Self {
            Self::SafeTransferFromWithFromAndToAndData(value)
        }
    }
    impl ::core::convert::From<SetApprovalForAllCall> for MockERC721Calls {
        fn from(value: SetApprovalForAllCall) -> Self {
            Self::SetApprovalForAll(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for MockERC721Calls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SymbolCall> for MockERC721Calls {
        fn from(value: SymbolCall) -> Self {
            Self::Symbol(value)
        }
    }
    impl ::core::convert::From<TokenURICall> for MockERC721Calls {
        fn from(value: TokenURICall) -> Self {
            Self::TokenURI(value)
        }
    }
    impl ::core::convert::From<TransferFromCall> for MockERC721Calls {
        fn from(value: TransferFromCall) -> Self {
            Self::TransferFrom(value)
        }
    }
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
    /// Container type for all return fields from the `getApproved` function
    /// with signature `getApproved(uint256)` and selector `0x081812fc`
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
    pub struct GetApprovedReturn(pub ::ethers::core::types::Address);
    /// Container type for all return fields from the `isApprovedForAll`
    /// function with signature `isApprovedForAll(address,address)` and selector
    /// `0xe985e9c5`
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
    pub struct IsApprovedForAllReturn(pub bool);
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
    /// Container type for all return fields from the `ownerOf` function with
    /// signature `ownerOf(uint256)` and selector `0x6352211e`
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
    pub struct OwnerOfReturn {
        pub owner: ::ethers::core::types::Address,
    }
    /// Container type for all return fields from the `supportsInterface`
    /// function with signature `supportsInterface(bytes4)` and selector
    /// `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
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
    /// Container type for all return fields from the `tokenURI` function with
    /// signature `tokenURI(uint256)` and selector `0xc87b56dd`
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
    pub struct TokenURIReturn(pub ::std::string::String);
}

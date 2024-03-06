# Contribution guidelines

### General

A few rules:
1. Follow our [style guide](#style-guide).
2. Format on save using Foundry formatter.

## Style Guide

### Imports

1. Use named imports as much as possible.
2. Import external dependencies first (libs, etc).
3. Import from root `src/DFMM.sol` instead of `../DFMM.sol`.

### Tests

1. Name the tests accordingly using the following format:
`test_{name of the contract}_{name of the function}_{what should happen}`

For example:
`test_DFMM_init_IncrementsPoolId`.
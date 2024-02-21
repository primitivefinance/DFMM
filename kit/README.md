# DFMM Kit
The `dfmm-kit` is a Rust crate designed to be used as a library for the development of DFMM applications, namely with Arbiter and Excalibur.
Let's look at the modules and what they're for.

## Usage
This repository has been set to be a Rust workspace and the `dfmm-kit` crate is the only crate.
In the `kit/Cargo.toml`, `dfmm-kit` exposes a binary that is used for testing the crate.
You can access this binary by running:
```bash
cargo run --bin kit
```
from the root of the repository.
Running this command will show the help menu for the `#[arbiter_macros::main]` macro. 

If you want to run a specific simulation, you can do something like:
```bash
cargo run --bin kit simulation kit/configs/example.toml
```

## Pool
The `pool` module provides a generic `Pool<P: PoolType>` struct along with the `PoolType` trait.
The `PoolType` trait should be implemented for any of the distinct types of pools that DFMM offers. 
For instance, we have `G3M` (GeometricMean), `LogNormal`, and `ConstantSum` pool types.
These pools come along with their parameters and solvers.
The `Pool` struct wraps up a `PoolType` and places metadata alongside it such as the pool's ID and the tokens it holds. 
The `Pool` struct also provides methods to interact with the pool, such as `swap`.

## Behaviors
The `behaviors` module provides implementations of the `arbiter-engine` `Behavior` trait.
These are the behaviors that the `arbiter-engine` will use to interact with the `dfmm-kit`.
Any implementations here can be used in simulation with Arbiter and eventually with Excalibur.

## Bindings
The `bindings` module contains the Rust bindings for the Solidity contracts that are used in the DFMM system.
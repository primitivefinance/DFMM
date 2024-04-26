# Pools
The pool module contains a trait `PoolType` that is generic over dfmm strategies.
The `PoolType` trait in the `pool` module is a central abstraction designed for different DFMM strategies. 
It defines the necessary operations and types required to interact with various pool strategies. 
Here's a brief overview of its capabilities and structure:

- **Generic Types**:
  - `Parameters`: Configuration parameters for the pool, which are serializable, deserializable, and can be used in Ethereum ABI decoding.
  - `StrategyContract`: Represents the contract that implements the strategy logic.
  - `SolverContract`: Represents the contract that handles the solving logic for the pool.
  - `AllocationData`: Data related to resource allocation within the pool, which is also serializable and deserializable.

- **Asynchronous Methods**:
  - `swap_data`: Asynchronously fetches data required for token swaps within the pool.
  - `update_data`: Asynchronously updates the pool's parameters.
  - `change_allocation_data`: Asynchronously modifies the allocation data of the pool.

- **Synchronous Methods**:
  - `get_contracts`: Retrieves instances of the strategy and solver contracts.
  - `get_strategy_address`: Fetches the address of the strategy contract.
  - `get_init_data`: Gathers initial configuration data for pool setup.
  - `set_controller`: Sets the controller address in the parameters.
  - `create_instance`: Instantiates a new pool with the specified contracts and parameters.
  - `get_params`: Retrieves the parameters of a pool from its strategy contract.

This trait is crucial for ensuring that different pool strategies can be managed uniformly, providing a consistent interface for operations like swaps, parameter updates, and resource allocations across any pool on any strategy. 
Each strategy implements the pool trait in the rest of the pool module.

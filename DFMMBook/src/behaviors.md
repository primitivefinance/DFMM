# Behaviors
The `behaviors` module contains functionality for agent behaviors for these contracts.
The `behaviors` module in the DFMM kit is designed to encapsulate the various agent behaviors that interact with the DFMM contracts. This module is crucial for automating and managing the interactions between users (or agents) and the different pool strategies defined in the `pool` module. The behaviors are structured around a series of sub-modules, each tailored to specific tasks and operations within the DFMM ecosystem.

## Behaviors Module
Note that we use the unstable Rust [RFC-1210](https://rust-lang.github.io/rfcs/1210-impl-specialization.html) for impl-specialization. 
This provides some flexibility in the items each behavior streams and reacts to.
In particular, it allows us to define the process behavior trait for a subset of types to easily specify the type `E` in the stream you would like the behavior to react to.

### 1. Allocate
This `allocate` behavior handles the allocation of resources within the DFMM pools. It defines behaviors for adjusting liquidity allocation based on various strategies and conditions.

### 2. Creator
The `creator` is responsible for the creation of new pool instances. It uses configuration data to set up pools according to specified parameters and strategies.

### 3. Deploy
The `deploy` behavior manages the deployment of contracts related to the DFMM. It handles the initialization and setup of contracts on the blockchain, ensuring that they are ready for interaction.

### 4. Swap
The `swap` behavior facilitates token swaps within the pools. It defines the logic for exchanging tokens based on the current pool state and market conditions.
The swap behavior, in particular, has a `SwapType` trait that can be implemented by any specific instance of a swap type for the swap behavior. 
This trait enables users to configure particular triggers for swaps and specific methods to calculate the swap size.

### 5. Token
The `token` sub-module manages token-related operations, such as minting and querying token states. It interacts with the token contracts to perform administrative tasks and queries.

### 6. Update
Finally, the `update` sub-module is responsible for updating the state and configuration of pools. It handles changes to the pool parameters and ensures that the pools adapt to new settings effectively.

Each of these sub-modules is designed to handle specific aspects of the DFMM ecosystem, working together to provide a comprehensive set of behaviors that manage and automate interactions with the DFMM contracts.

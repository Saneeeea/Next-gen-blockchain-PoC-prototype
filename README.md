# My Blockchain PoC
This is a lightweight proof-of-concept for a next-gen blockchain architecture. I put this together to explore how we can handle asynchronous state transitions with DAGs, implement native Account Abstraction, and verify cross-chain states using ZK primitives.

## Why this project?
Standard linear blockchains often struggle with throughput bottlenecks. This project experiments with a DAG-based structure to allow for more parallel, concurrent transaction processing. It also includes an abstraction layer for smart contracts to keep things flexible and modular.

## Key Features
 Async DAG Core: Handles transactions in a non-linear graph structure to optimize for concurrency.
 Modular Smart Contracts: A simple, type-safe contract environment designed with Wasm portability in mind.
 Native Account Abstraction: Built-in support for social recovery (via ZK-like verification) and gas sponsorship (meta-transactions).
 ZK Light Client: A foundational layer to verify state transitions from external chains cryptographically.

## Getting Started
### Prerequisites
 Rust (latest stable)
 Tokio (async runtime)
 
## Building and Running
```bash
# Clone the repository
git clone [https://github.com/Saneeeea/Next-gen-blockchain-PoC-prototype)
# Build the project
cargo build
# Run the node
cargo run
```

## Status
This is currently a rough PoC for architectural exploration. The consensus logic and ZK circuits are mocks, but the core async pipeline and state management are functional.

## License
MIT. See ⁠LICENSE⁠ for more details.

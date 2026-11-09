# Holospaces Architecture

## Overview
Holospaces is an open, proof-carrying compilation system for reproducible quantum software. It represents programs, transformations, executable artifacts, formal proofs, test evidence, and benchmark results as one immutable content-addressed graph (Merkle DAG).

## Core Architecture
```
holospaces/
├── Cargo.toml
├── LICENSE
├── README.md
├── crates/
│   ├── holo-cli/          # build, verify, inspect, diff, reproduce, explain
│   ├── holo-core/         # IDs, nodes, manifests, claims, canonical JSON
│   ├── holo-store/        # content-addressed graph storage (.holospace/objects/)
│   ├── holo-qasm/         # OpenQASM parser and AST generator
│   ├── holo-ir/           # typed quantum and topological IRs
│   ├── holo-compiler/     # circuit-to-braid transformations
│   ├── holo-simulator/    # exact/symbolic reference execution (num-rational)
│   ├── holo-assurance/    # evidence and verification policies
│   └── holo-view/         # graph export & terminal visualizer
├── proofs/lean/           # Lean 4 formal certificates
├── validators/            # Independent differential validators (Q#, Fortran)
├── specifications/        # Gherkin features & JSON schemas
├── examples/              # Grover-2Q, Bell-State
├── benchmarks/            # Reproducible benchmarks
└── docs/                  # System documentation
```

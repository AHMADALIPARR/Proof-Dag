# ProofDag

ProofDag is an open, proof-carrying compilation system for reproducible quantum software. It represents programs, transformations, executable artifacts, formal proofs, test evidence, and benchmark results as one immutable content-addressed graph.

Every node in the graph represents a typed scientific claim:
- "This parser produced this AST."
- "This transformation preserves semantics under these assumptions."
- "This circuit is equivalent to this exact reference model."
- "This braid satisfies these topological constraints."
- "This claim was proved, tested, simulated, or merely asserted."

## Repository Architecture

```
holospaces/
├── Cargo.toml
├── LICENSE
├── README.md
├── crates/
│   ├── holo-cli/          # build, verify, inspect, diff, reproduce, explain
│   ├── holo-core/         # IDs, nodes, manifests, claims, canonical serialization
│   ├── holo-store/        # content-addressed graph storage (.holospace/objects/)
│   ├── holo-qasm/         # OpenQASM 3 parser and canonicalizer
│   ├── holo-ir/           # typed quantum and topological IRs
│   ├── holo-compiler/     # circuit-to-braid transformations
│   ├── holo-simulator/    # exact/symbolic reference execution (num-rational)
│   ├── holo-assurance/    # evidence and verification policies
│   └── holo-view/         # graph export & terminal visualizer
├── proofs/lean/           # Lean 4 formal certificates
├── validators/            # Independent validators (Q#, Fortran)
├── specifications/        # Gherkin features & JSON schemas
├── examples/              # Grover-2Q, Bell-State
├── benchmarks/            # Reproducible benchmarks
├── docs/                  # System documentation
└── attic/                 # Preserved historical research code
```

## Quickstart: The CLI Experience

```bash
# Build proof-carrying graph from OpenQASM 3 source
cargo run -p holo-cli -- build examples/grover-2q/main.qasm

# Verify graph integrity and cryptographic hashes
cargo run -p holo-cli -- verify <root-id>

# Inspect root manifest and DAG topology
cargo run -p holo-cli -- inspect <root-id>

# Explain a claim with its assumptions and supporting evidence
cargo run -p holo-cli -- explain <claim-id>

# Reproduce the graph deterministically
cargo run -p holo-cli -- reproduce <root-id>
```

## Assurance Levels

1. **Level 0: Assertion**
2. **Level 1: Reproduced**
3. **Level 2: Property Tested**
4. **Level 3: Fuzz Tested**
5. **Level 4: Differentially Validated**
6. **Level 5: Exact Reference Checked**
7. **Level 6: Model Checked**
8. **Level 7: Formally Proved**

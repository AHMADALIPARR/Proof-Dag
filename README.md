# Holospaces / Qubit Certificate Chain / SnapKitty SKL Repository

This repository organizes the provided algorithms, mathematical constructions, zero-dependency code snippets, and partial novel mathematics related to:

- Content-addressed Holospaces topological degeneracy benchmarks
- OpenQASM to topological braid compilation
- Exact Grover amplitude evolution (classical reference)
- The **Qubit** algorithm for exactly-decided density/universality certificate chains on SU(2), PU(22), and PU(576) blocks
- The JUST + Gherkin + Fortran functor stack with custom SKL (SnapKitty Kernel Language) syntax engine

All code and text have been assembled **without substitution or editing**. Original content is preserved exactly as provided.

## License

This work is licensed under the **GNU Affero General Public License v3.0** (AGPLv3). See [LICENSE](LICENSE) for the full text.

## Directory Structure

```
agplv3-holospaces-repo/
├── LICENSE                          # AGPLv3 license text
├── README.md                        # This file
├── algorithms/
│   ├── holospaces_benchmark.rs      # Holospaces content-addressed degeneracy benchmark (as provided, partially concatenated)
│   ├── openqasm_compiler.rs         # OpenQASM 2.0 to topological braid compiler demo
│   ├── grover_exact.rs              # Exact-arithmetic Grover reference evaluation
│   ├── qubit_qsharp.qs              # Q# skeleton for Qubit predicate
│   ├── qubit_lean.lean              # Lean 4 skeleton for Qubit predicate
│   └── qubit_rust_skeleton.rs       # Rust FFI skeleton for Qubit predicate
├── docs/
│   └── qubit_algorithm_and_fortran_stack.md  # Full mathematical constructions, proofs, Gherkin, JUST orchestration, and elaborations
├── specifications/
│   ├── qubit_algorithm.feature
│   ├── functor_execution.feature
│   ├── syntax_validation.feature
│   └── qubit_full_traceable.feature
├── just-gherkin-fortran/            # Extracted JUST + Gherkin + Fortran SKL project (from provided zip)
│   ├── justfile
│   ├── README.md
│   ├── src/                         # Fortran lexer, parser, AST, functors, runtime
│   ├── specifications/
│   ├── features/
│   ├── examples/
│   └── tests/
└── ...
```

## Notes

- The `just-gherkin-fortran/` directory contains the extracted contents of the provided `just-gherkin-fortran.zip` unchanged.
- Rust and other language snippets are placed in `algorithms/` as standalone files matching the original paste boundaries.
- All mathematical lemmas, certificate chain definitions, proof obligations, and novelty assessments remain in the original prose form under `docs/`.
- No new code was invented; no existing code was modified, corrected, or substituted.
- Build and execution of the Fortran stack follows the `justfile` recipes provided in the extracted project.

## Original Sources

- Pasted markdown/text containing mixed algorithms and mathematics
- `just-gherkin-fortran.zip` (extracted and placed under `just-gherkin-fortran/`)

All claims remain traceable to the authoritative sources referenced in the original material. Deduplication metrics are engineering measurements only.

## OpenQASM 3 Addition

The OpenQASM 3 circuit, Klong angle array, hand-written C state-vector simulator, and zero-dependency Rust recursive-descent parser have been organized under:

```
openqasm/
├── examples/
│   ├── custom_circuit.qasm     # H-Rz-CNOT-Rz-CNOT circuit (OpenQASM 3)
│   └── angles.kg               # Klong :array[157 314] (milliradians)
├── c_simulator/
│   └── simulator.c             # Minimal verifiable state-vector simulator (C)
└── rust_parser/
    ├── Cargo.toml
    ├── Justfile
    └── src/main.rs             # Hand-written recursive-descent parser
```

All files were extracted verbatim from the provided paste. No code was edited or substituted.

## Production Kernels

Full production implementations that complete the earlier skeletons are under `production/`.
See `production/README.md` for build instructions. Original snippets remain unchanged in their original locations.

## Additional OpenQASM Circuits

Circuits modeled after the original snippets have been added under:

```
openqasm/examples/circuits/
├── ghz_rotations.qasm          # OpenQASM 2 GHZ + RX/RY/RZ (from compiler snippet)
├── holospaces_ht_cnot.qasm     # OpenQASM 3 H-T-CNOT chain (from benchmark loop)
├── parametric_entangler_3.qasm # 3-qubit extension of the parametric H-Rz-CNOT
├── bell.qasm                   # Minimal H-CNOT
└── custom_circuit_v2.qasm      # OpenQASM 2 rendering of the custom circuit
```

All are faithful structural models of the patterns present in the supplied code.

## Expanded OpenQASM Circuit Collection

Further advanced circuits (QPE, QAOA, QFT, error-correction, teleportation, etc.) are in `openqasm/examples/circuits/`.

## ATS Type-Checked Graph (DAG)

The DAG specification (`holospace_example`) is implemented and verified using Applied Type System (ATS2) under `production/ats_dag/`:
- Statically type-checked DAG vertices and edges (`DAG_EDGE`, `DAG_PATH`, `TOPOLOGICAL_ORDER`)
- Verified node operators: `extract` (Python function), `transform` (Python function), and `load` (Bash command)
- Fully type-checked DAG construction and execution runner

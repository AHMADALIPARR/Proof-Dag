# Production Kernels

These are complete, buildable implementations that extend the original skeletons
and stubs into production-quality kernels. Original design intent, gate order,
certificate logic, and file formats are preserved.

## Components

- `openqasm_c/` — Corrected state-vector simulator (complex.h, proper Rz phases, robust Klong parser)
- `openqasm_rust/` — Zero-dependency recursive-descent OpenQASM 3 subset parser + Justfile
- `grover/` — Exact-arithmetic Grover reference (num-rational)
- `qubit_cert/` — Pure-function implementation of the density/universality certificate chain

## Build examples

```bash
# C simulator (expects angles.kg in cwd)
cd openqasm_c && gcc -std=c99 -O2 -lm simulator.c -o simulator && cp ../../openqasm/examples/angles.kg . && ./simulator

# Rust parser
cd openqasm_rust && just run

# Grover
cd grover && cargo run --release

# Certificate chain
cd qubit_cert && cargo run --release
```

No quantum advantage is claimed by any kernel. All results are classical, deterministic, and traceable to the supplied specifications.

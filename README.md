# ProofDag

**Proof-carrying compilation for reproducible quantum software**

[![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL%203.0-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![Rust](https://img.shields.io/badge/Rust-Workspace-orange.svg)](https://www.rust-lang.org/)
[![Lean 4](https://img.shields.io/badge/Lean%204-Proofs-purple.svg)](https://lean-lang.org/)

ProofDag is an open, proof-carrying compilation system that treats every artifact in the quantum software lifecycle as a **typed scientific claim** inside one immutable, content-addressed Directed Acyclic Graph (DAG).

Programs, transformations, executable kernels, formal proofs, test evidence, and benchmark results are all first-class nodes. Every edge records a precise dependency or justification. The result is a fully reproducible, auditable, and explainable quantum software stack.

> “This parser produced this AST.”  
> “This transformation preserves semantics under these assumptions.”  
> “This circuit is equivalent to this exact reference model.”  
> “This claim was proved, tested, simulated, or merely asserted.”

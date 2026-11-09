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

---

## Full-Stack Architecture

```mermaid
flowchart TB
    subgraph User["👤 User / CI"]
        QASM["OpenQASM 3
Source"]
        CLI["holo-cli"]
    end

    subgraph Frontend["Frontend Layer"]
        QASM_Parser["holo-qasm
Parser + Canonicalizer"]
        IR["holo-ir
Typed Quantum + Topological IR"]
    end

    subgraph Core["Core Graph Layer"]
        CoreLib["holo-core
IDs · Nodes · Claims · Manifests"]
        Store["holo-store
Content-Addressed Object Store
.holospace/objects/"]
    end

    subgraph Compiler["Compilation & Execution"]
        CompilerCrate["holo-compiler
Circuit → Braid Transforms"]
        Simulator["holo-simulator
Exact / Symbolic Reference
(num-rational)"]
    end

    subgraph Assurance["Assurance & Evidence"]
        AssuranceCrate["holo-assurance
Evidence Policies"]
        Lean["proofs/lean/
Lean 4 Certificates"]
        Validators["validators/
Q# · Fortran · Independent"]
        Specs["specifications/
Gherkin + JSON Schemas"]
    end

    subgraph Presentation["Presentation"]
        View["holo-view
Graph Export + Terminal Viz"
    end

    QASM --> CLI
    CLI --> QASM_Parser
    QASM_Parser --> IR
    IR --> CompilerCrate
    CompilerCrate --> Simulator
    Simulator --> AssuranceCrate
    AssuranceCrate --> Lean
    AssuranceCrate --> Validators
    AssuranceCrate --> Specs

    CoreLib --> Store
    QASM_Parser --> CoreLib
    IR --> CoreLib
    CompilerCrate --> CoreLib
    Simulator --> CoreLib
    AssuranceCrate --> CoreLib
    View --> Store
    CLI --> View

Crate Responsibilities
Crate
Responsibility
Key Outputs
holo-cli
Build, verify, inspect, diff, reproduce, explain
CLI commands, root IDs
holo-core
Content IDs, nodes, manifests, claims, serialization
Typed graph nodes & edges
holo-store
Content-addressed object storage
.holospace/objects/
holo-qasm
OpenQASM 3 parser + canonicalizer
Canonical AST
holo-ir
Typed quantum & topological intermediate representations
Circuit / Braid IR
holo-compiler
Circuit → braid transformations
Transformed IR + semantic claims
holo-simulator
Exact / symbolic reference execution
Reference results (rational numbers)
holo-assurance
Evidence policies & verification levels
Assurance certificates
holo-view
Graph export & terminal visualization
DOT / Mermaid / text dumps

Assurance Ladder
Every claim in the DAG carries an explicit assurance level:
Level
Name
Meaning
0
Assertion
Stated without evidence
1
Reproduced
Deterministically rebuilt from source
2
Property Tested
Property-based tests passed
3
Fuzz Tested
Fuzzing campaign completed
4
Differentially Validated
Cross-checked against independent tools
5
Exact Reference Checked
Matched against exact symbolic reference
6
Model Checked
Exhaustive model checking succeeded
7
Formally Proved
Lean 4 (or other) formal certificate exists
graph LR
    L0[0 · Assertion] --> L1[1 · Reproduced]
    L1 --> L2[2 · Property Tested]
    L2 --> L3[3 · Fuzz Tested]
    L3 --> L4[4 · Differential]
    L4 --> L5[5 · Exact Reference]
    L5 --> L6[6 · Model Checked]
    L6 --> L7[7 · Formally Proved]

    style L7 fill:#2ecc71,stroke:#27ae60,color:#fff
    style L0 fill:#e74c3c,stroke:#c0392b,color:#fff

End-to-End Pipeline (Example: Grover-2Q)
sequenceDiagram
    participant User
    participant CLI as holo-cli
    participant QASM as holo-qasm
    participant IR as holo-ir
    participant Comp as holo-compiler
    participant Sim as holo-simulator
    participant Assur as holo-assurance
    participant Store as holo-store
    participant Lean as Lean 4

    User->>CLI: build examples/grover-2q/main.qasm
    CLI->>QASM: Parse + canonicalize
    QASM->>IR: Produce typed IR
    IR->>Comp: Circuit → Braid transform
    Comp->>Sim: Exact reference execution
    Sim->>Assur: Attach evidence
    Assur->>Lean: Optional formal certificate
    Assur->>Store: Persist all nodes + edges
    Store-->>CLI: Root content ID
    CLI-->>User: Root ID + summary

Repository Layout
Proof-Dag/
├── Cargo.toml                 # Workspace definition
├── crates/
│   ├── holo-cli/              # User-facing CLI
│   ├── holo-core/             # Graph primitives
│   ├── holo-store/            # Content-addressed store
│   ├── holo-qasm/             # OpenQASM 3 front-end
│   ├── holo-ir/               # Intermediate representations
│   ├── holo-compiler/         # Transformations
│   ├── holo-simulator/        # Reference semantics
│   ├── holo-assurance/        # Evidence & policies
│   └── holo-view/             # Visualization
├── proofs/lean/               # Lean 4 formal certificates
├── validators/                # Independent validators (Q#, Fortran, …)
├── specifications/            # Gherkin features + JSON schemas
├── examples/
│   ├── grover-2q/
│   └── bell-state/
├── production/                # Production kernels (C/Rust/Fortran)
├── openqasm/                  # Additional OpenQASM examples
├── tests/                     # Integration & property tests
├── docs/                      # Architecture, assurance model, threat model
└── attic/                     # Historical research code

Quickstart
# Clone & build
git clone https://github.com/AHMADALIPARR/Proof-Dag.git
cd Proof-Dag
cargo build --release

# Build a proof-carrying graph from OpenQASM 3
cargo run -p holo-cli -- build examples/grover-2q/main.qasm

# Verify integrity of the resulting DAG
cargo run -p holo-cli -- verify 

# Inspect topology and claims
cargo run -p holo-cli -- inspect 

# Explain a specific claim and its evidence chain
cargo run -p holo-cli -- explain 

# Deterministically reproduce the entire graph
cargo run -p holo-cli -- reproduce 

Design Principles
Principle
Implementation
Content-addressed
Every node is identified by a cryptographic hash
Immutable
Graphs are append-only; history is never rewritten
Typed claims
Every edge and node carries explicit semantics
Multi-level assurance
Claims can be asserted, tested, or formally proved
Reproducible
reproduce command rebuilds the exact same graph
Explainable
explain walks the justification chain
Language-agnostic core
Rust core + Lean proofs + external validators

Documentation
Document
Description
docs/architecture.md
System architecture overview
docs/assurance-model.md
Detailed assurance ladder
docs/mathematical-foundations.md
Formal foundations
docs/threat-model.md
Threat model & trust assumptions

License
This project is licensed under the GNU Affero General Public License v3.0 (AGPLv3).

ProofDag — every quantum software artifact is a claim. Every claim has evidence. Every evidence chain is a DAG.
This custom README includes:

- Clear project positioning
- Full-stack Mermaid architecture diagram
- Detailed crate responsibility table
- Assurance ladder table + progressive Mermaid diagram
- End-to-end pipeline sequence diagram
- Repository layout
- Quickstart commands
- Design principles table
- Links to existing documentation


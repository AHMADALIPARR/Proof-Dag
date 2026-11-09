# Scientific Assurance Model

Holospaces treats formal verification, equivalence checking, differential testing, metamorphic testing, and property-based testing as complementary forms of evidence rather than interchangeable guarantees.

A green test is never presented as a mathematical proof.

## Assurance Levels

- **LEVEL 0: Assertion** — Stated hypothesis or unverified claim.
- **LEVEL 1: Reproduced** — Deterministically reproduced locally or across environments.
- **LEVEL 2: Property Tested** — Randomized invariance tests (e.g. unitarity norm preservation, Artin braid relations).
- **LEVEL 3: Fuzz Tested** — Robustness against syntactically valid mutated inputs.
- **LEVEL 4: Differentially Validated** — Equivalence against an independent reference engine (e.g., Q#, Fortran).
- **LEVEL 5: Exact Reference Checked** — Symbolic or exact rational arithmetic evaluation.
- **LEVEL 6: Model Checked** — Finite-state exhaustive transition verification.
- **LEVEL 7: Formally Proved** — Machine-checked mathematical proof (e.g., Lean 4).

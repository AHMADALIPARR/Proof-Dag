# Threat Model

## Assumptions and Boundaries
1. **Cryptographic Integrity**: SHA3-256 is assumed collision-resistant for node content addressing.
2. **Deterministic Canonicalization**: JSON payloads and AST structures are sorted and escaped canonical representations.
3. **Evidence Distinction**: Assurance levels are explicitly typed; test results cannot masquerade as formal proofs.
4. **Environment Reproduction**: All transformation passes record tool identifier, version, and configuration parameters.

Feature: Holospaces Genesis Compilation and Assurance

  Scenario: Complete end-to-end compilation of Grover 2-qubit circuit
    Given an OpenQASM 3 program "examples/grover-2q/main.qasm"
    When the program is compiled with the Holospaces compiler
    Then a canonical AST node is produced with parent pointing to source
    And a canonical Quantum IR node is generated
    And a topological braid representation is synthesized
    And exact semantic evaluation confirms probability 1 for target state |11>
    And formal Lean proof certificate is verified
    And differential execution matches the independent reference model
    And an assurance manifest is created with a deterministic Holospace root ID

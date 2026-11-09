OPENQASM 3.0;
include "stdgates.inc";

// Qubit register (2 qubits)
qubit[2] q;

// Classical angle array (supplied by host via FFI)
// Values are in MILLRADIANS; converted to radians in host code
float[2] angles;

// Circuit: H - Rz(α) - CNOT - Rz(β) - CNOT
h q[0];
rz(angles[0]) q[0]; // α on qubit 0
cx q[0], q[1]; // CNOT 0→1
rz(angles[1]) q[1]; // β on qubit 1
cx q[0], q[1]; // CNOT 0→1

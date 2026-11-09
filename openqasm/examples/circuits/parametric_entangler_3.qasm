OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;
float[3] angles;  // milliradians supplied by host

// H - Rz - CNOT chain with per-qubit rotations
h q[0];
rz(angles[0]) q[0];
cx q[0], q[1];

h q[1];
rz(angles[1]) q[1];
cx q[1], q[2];

rz(angles[2]) q[2];
cx q[0], q[2];

OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;  // ancilla, system A, system B

h q[0];
cswap q[0], q[1], q[2];
h q[0];

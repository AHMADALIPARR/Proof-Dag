OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;  // 3 controls + 1 target

// Approximate multi-controlled X using ancilla-free style (partial)
h q[3];
ccx q[0], q[1], q[3];
h q[3];
ccx q[1], q[2], q[3];
h q[3];

OPENQASM 3.0;
include "stdgates.inc";

qubit[2] q;
float[1] phase;

h q[0];
cp(phase[0]) q[0], q[1];
h q[0];

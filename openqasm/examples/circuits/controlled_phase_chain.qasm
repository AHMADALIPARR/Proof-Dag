OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;
float[3] phases;

h q[0];
cp(phases[0]) q[0], q[1];
h q[1];
cp(phases[1]) q[1], q[2];
h q[2];
cp(phases[2]) q[2], q[3];
h q[3];

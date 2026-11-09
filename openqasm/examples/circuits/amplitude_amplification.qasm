OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;

// State preparation
h q[0];
h q[1];
h q[2];

// Oracle (mark |111>)
ccz q[0], q[1], q[2];

// Diffusion
h q[0]; h q[1]; h q[2];
x q[0]; x q[1]; x q[2];
ccz q[0], q[1], q[2];
x q[0]; x q[1]; x q[2];
h q[0]; h q[1]; h q[2];

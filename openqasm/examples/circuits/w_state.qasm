OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;

// Approximate W state preparation
ry(1.910633) q[0];
ch q[0], q[1];
ry(0.955317) q[1];
cx q[1], q[2];
cx q[0], q[1];
x q[0];

OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;
float[4] angles;

rz(angles[0]) q[0];
rx(angles[1]) q[1];
ry(angles[2]) q[2];
rz(angles[3]) q[3];

cx q[0], q[1];
cx q[1], q[2];
cx q[2], q[3];

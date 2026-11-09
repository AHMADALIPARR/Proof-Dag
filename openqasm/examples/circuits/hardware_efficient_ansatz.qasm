OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;
float[12] theta;  // 3 layers × 4 params

// Layer 1
ry(theta[0]) q[0];
ry(theta[1]) q[1];
ry(theta[2]) q[2];
ry(theta[3]) q[3];
cx q[0], q[1];
cx q[1], q[2];
cx q[2], q[3];

// Layer 2
ry(theta[4]) q[0];
ry(theta[5]) q[1];
ry(theta[6]) q[2];
ry(theta[7]) q[3];
cx q[0], q[1];
cx q[1], q[2];
cx q[2], q[3];

// Layer 3
ry(theta[8]) q[0];
ry(theta[9]) q[1];
ry(theta[10]) q[2];
ry(theta[11]) q[3];

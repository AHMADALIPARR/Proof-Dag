OPENQASM 2.0;
include "qelib1.inc";
qreg q[3];
creg c[3];

// Prepare GHZ state
h q[0];
cx q[0], q[1];
cx q[1], q[2];

// Parametrized rotations
rx(pi/2) q[0];
ry(pi/4) q[1];
rz(pi/8) q[2];

OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;  // 3 counting + 1 system
float[1] phase;

// Initialize system in eigenstate |1>
x q[3];

// Hadamards on counting register
h q[0];
h q[1];
h q[2];

// Controlled-U^{2^k}
cp(phase[0]) q[0], q[3];
cp(phase[0]*2) q[1], q[3];
cp(phase[0]*4) q[2], q[3];

// Inverse QFT on counting register
h q[2];
cp(-pi/2) q[1], q[2];
h q[1];
cp(-pi/4) q[0], q[2];
cp(-pi/2) q[0], q[1];
h q[0];
swap q[0], q[2];

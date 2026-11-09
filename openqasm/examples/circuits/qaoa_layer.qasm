OPENQASM 3.0;
include "stdgates.inc";

qubit[4] q;
float[2] gamma_beta;

// Cost layer (MaxCut-style ZZ)
rzz(gamma_beta[0]) q[0], q[1];
rzz(gamma_beta[0]) q[1], q[2];
rzz(gamma_beta[0]) q[2], q[3];
rzz(gamma_beta[0]) q[3], q[0];

// Mixer layer
rx(gamma_beta[1]) q[0];
rx(gamma_beta[1]) q[1];
rx(gamma_beta[1]) q[2];
rx(gamma_beta[1]) q[3];

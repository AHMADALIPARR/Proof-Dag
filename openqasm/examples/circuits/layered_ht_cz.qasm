OPENQASM 3.0;
include "stdgates.inc";

qubit[5] q;

// Layer 1: H + T
h q[0]; t q[0];
h q[1]; t q[1];
h q[2]; t q[2];
h q[3]; t q[3];
h q[4]; t q[4];

// Layer 2: nearest-neighbor CZ
cz q[0], q[1];
cz q[1], q[2];
cz q[2], q[3];
cz q[3], q[4];

// Layer 3: more H + T
h q[0]; t q[0];
h q[1]; t q[1];
h q[2]; t q[2];
h q[3]; t q[3];
h q[4]; t q[4];

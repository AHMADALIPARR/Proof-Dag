OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;

// Encode logical |ψ> on q[0] into bit-flip code
cx q[0], q[1];
cx q[0], q[2];

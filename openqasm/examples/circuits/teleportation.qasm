OPENQASM 3.0;
include "stdgates.inc";

qubit[3] q;  // msg, Alice, Bob
bit[2] c;

// Prepare Bell pair
h q[1];
cx q[1], q[2];

// Alice's operations
cx q[0], q[1];
h q[0];

// Measurements (classical feed-forward omitted in pure circuit)
measure q[0] -> c[0];
measure q[1] -> c[1];

// Bob's corrections (conditional, shown unconditionally for circuit form)
cx q[1], q[2];
cz q[0], q[2];

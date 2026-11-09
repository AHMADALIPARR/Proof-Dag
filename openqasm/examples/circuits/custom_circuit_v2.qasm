OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
creg c[2];

// Equivalent to the OpenQASM 3 parametric circuit
// (angles supplied externally; here shown with symbolic placeholders)
h q[0];
rz(0.157) q[0];
cx q[0], q[1];
rz(0.314) q[1];
cx q[0], q[1];

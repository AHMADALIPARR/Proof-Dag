namespace Holospaces.Validators.QSharp {
    open Microsoft.Quantum.Intrinsic;
    open Microsoft.Quantum.Canon;

    // Independent reference execution of 2-qubit Grover search
    operation RunGrover2Q() : Result[] {
        use q = Qubit[2];

        // Superposition
        H(q[0]);
        H(q[1]);

        // Oracle for |11>
        CZ(q[0], q[1]);

        // Diffusion
        H(q[0]);
        H(q[1]);
        X(q[0]);
        X(q[1]);
        CZ(q[0], q[1]);
        X(q[0]);
        X(q[1]);
        H(q[0]);
        H(q[1]);

        let res = [M(q[0]), M(q[1])];
        ResetAll(q);
        return res;
    }
}

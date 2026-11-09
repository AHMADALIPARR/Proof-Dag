use holo_simulator::simulate_circuit;
use holo_ir::QuantumCircuitIR;

#[test]
fn test_differential_statevector_vs_analytical() {
    let mut circuit = QuantumCircuitIR::new(2, 0);
    circuit.add_gate("H", vec![0], vec![]);
    circuit.add_gate("H", vec![1], vec![]);
    circuit.add_gate("CZ", vec![0, 1], vec![]);
    circuit.add_gate("H", vec![0], vec![]);
    circuit.add_gate("H", vec![1], vec![]);
    circuit.add_gate("X", vec![0], vec![]);
    circuit.add_gate("X", vec![1], vec![]);
    circuit.add_gate("CZ", vec![0, 1], vec![]);
    circuit.add_gate("X", vec![0], vec![]);
    circuit.add_gate("X", vec![1], vec![]);
    circuit.add_gate("H", vec![0], vec![]);
    circuit.add_gate("H", vec![1], vec![]);

    let probs = simulate_circuit(&circuit).unwrap();

    // Verify differential equality with independent analytical baseline
    assert!((probs.get("00").unwrap() - 0.0).abs() < 1e-12);
    assert!((probs.get("01").unwrap() - 0.0).abs() < 1e-12);
    assert!((probs.get("10").unwrap() - 0.0).abs() < 1e-12);
    assert!((probs.get("11").unwrap() - 1.0).abs() < 1e-12);
}

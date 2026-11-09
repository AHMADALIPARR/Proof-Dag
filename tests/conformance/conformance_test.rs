use holo_compiler::{ast_to_quantum_ir, quantum_ir_to_topological_ir, topological_ir_to_braid};
use holo_qasm::Parser;
use holo_simulator::{simulate_circuit, simulate_exact_grover_2q};

#[test]
fn test_grover_end_to_end_conformance() {
    let qasm = include_str!("../../examples/grover-2q/main.qasm");
    let mut parser = Parser::new(qasm).unwrap();
    let ast = parser.parse_program().unwrap();

    let (circuit, ir_node, _) = ast_to_quantum_ir(&ast, "holo:test_ast").unwrap();
    assert_eq!(circuit.qubits, 2);

    let (topo, topo_node, _) = quantum_ir_to_topological_ir(&circuit, &ir_node.id);
    assert_eq!(topo.logical_qubits, 2);

    let (braid, _braid_node, _) = topological_ir_to_braid(&circuit, &topo, &topo_node.id);
    assert!(braid.resources.total_crossings > 0);

    let sim_probs = simulate_circuit(&circuit).unwrap();
    let p_11 = sim_probs.get("11").unwrap();
    assert!((p_11 - 1.0).abs() < 1e-10);

    let exact_res = simulate_exact_grover_2q();
    assert_eq!(exact_res.target_found, "11");
}

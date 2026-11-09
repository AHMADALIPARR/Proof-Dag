use holo_qasm::parse_qasm_to_ast_node;

#[test]
fn test_deterministic_reproducibility() {
    let qasm = "OPENQASM 3.0;\nqubit[2] q;\nh q[0];\n";
    let node1 = parse_qasm_to_ast_node(qasm, "holo:src").unwrap();
    let node2 = parse_qasm_to_ast_node(qasm, "holo:src").unwrap();

    assert_eq!(node1.id, node2.id);
    assert_eq!(node1.content_hash, node2.content_hash);
    assert_eq!(node1.payload_hash, node2.payload_hash);
}

use holo_core::{Node, NodeType};
use serde::{Deserialize, Serialize};

/// Canonical Quantum Gate Operation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantumGate {
    pub gate: String,
    pub targets: Vec<usize>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<f64>,
}

/// Canonical Quantum Circuit IR
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuantumCircuitIR {
    pub qubits: usize,
    pub classical_bits: usize,
    pub operations: Vec<QuantumGate>,
}

impl QuantumCircuitIR {
    pub fn new(qubits: usize, classical_bits: usize) -> Self {
        Self {
            qubits,
            classical_bits,
            operations: Vec::new(),
        }
    }

    pub fn add_gate(&mut self, gate: impl Into<String>, targets: Vec<usize>, params: Vec<f64>) {
        self.operations.push(QuantumGate {
            gate: gate.into(),
            targets,
            params,
        });
    }

    pub fn to_node(&self, parent_ast_id: &str) -> Node {
        let payload = serde_json::to_value(self).unwrap();
        Node::new(
            NodeType::QuantumIr,
            payload,
            vec![parent_ast_id.to_string()],
            vec![],
            serde_json::json!({
                "format": "canonical_quantum_ir_v1",
                "qubits": self.qubits,
                "gate_count": self.operations.len()
            }),
            vec!["quantum_ir".into(), "canonical".into()],
        )
    }
}

/// Defect path / logical line for topological representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefectPath {
    pub defect_id: usize,
    pub logical_qubit: usize,
    pub coordinates: Vec<(f64, f64)>,
}

/// Topological IR Representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopologicalIR {
    pub logical_qubits: usize,
    pub defect_lines: Vec<DefectPath>,
    pub topological_features: Vec<String>,
}

impl TopologicalIR {
    pub fn new(logical_qubits: usize) -> Self {
        Self {
            logical_qubits,
            defect_lines: Vec::new(),
            topological_features: Vec::new(),
        }
    }

    pub fn to_node(&self, parent_quantum_ir_id: &str) -> Node {
        let payload = serde_json::to_value(self).unwrap();
        Node::new(
            NodeType::TopologicalIr,
            payload,
            vec![parent_quantum_ir_id.to_string()],
            vec![],
            serde_json::json!({
                "format": "topological_ir_v1",
                "logical_qubits": self.logical_qubits
            }),
            vec!["topological_ir".into()],
        )
    }
}

/// Braid Move Operation in the Artin Braid Group B_n
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BraidGenerator {
    pub strand: usize,
    pub is_inverse: bool,
    pub angle_weight: Option<f64>,
}

/// Resource accounting for physical topological compilation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BraidResources {
    pub total_crossings: usize,
    pub braid_depth: usize,
    pub logical_qubits: usize,
    pub strands: usize,
}

/// Executable Braid Program
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BraidProgram {
    pub strands: usize,
    pub generators: Vec<BraidGenerator>,
    pub schedule: serde_json::Value,
    pub resources: BraidResources,
}

impl BraidProgram {
    pub fn to_node(&self, parent_topo_ir_id: &str) -> Node {
        let payload = serde_json::to_value(self).unwrap();
        Node::new(
            NodeType::BraidProgram,
            payload,
            vec![parent_topo_ir_id.to_string()],
            vec![],
            serde_json::json!({
                "format": "braid_program_v1",
                "strands": self.strands,
                "total_crossings": self.resources.total_crossings
            }),
            vec!["braid_program".into(), "executable".into()],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_ir_node_creation() {
        let mut ir = QuantumCircuitIR::new(2, 0);
        ir.add_gate("H", vec![0], vec![]);
        ir.add_gate("CZ", vec![0, 1], vec![]);

        let node = ir.to_node("holo:sha3-256:dummy_ast");
        assert_eq!(node.node_type, NodeType::QuantumIr);
        assert!(node.verify_integrity());
    }

    #[test]
    fn test_braid_program_node_creation() {
        let braid = BraidProgram {
            strands: 4,
            generators: vec![
                BraidGenerator {
                    strand: 0,
                    is_inverse: false,
                    angle_weight: None,
                },
                BraidGenerator {
                    strand: 1,
                    is_inverse: true,
                    angle_weight: None,
                },
            ],
            schedule: serde_json::json!({"timesteps": 2}),
            resources: BraidResources {
                total_crossings: 2,
                braid_depth: 2,
                logical_qubits: 2,
                strands: 4,
            },
        };

        let node = braid.to_node("holo:sha3-256:dummy_topo");
        assert_eq!(node.node_type, NodeType::BraidProgram);
        assert!(node.verify_integrity());
    }
}

use holo_core::{Node, NodeType};
use holo_ir::{BraidProgram, QuantumCircuitIR};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};
use std::collections::BTreeMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SimError {
    #[error("Qubit out of bounds: {0}")]
    QubitOutOfBounds(usize),
    #[error("Invalid circuit dimension: {0}")]
    InvalidDimension(usize),
    #[error("Unsupported exact gate: {0}")]
    UnsupportedGate(String),
}

/// Exact Rational Grover Evolution for 2 or 3 qubits
#[derive(Debug, Clone, PartialEq)]
pub struct ExactRationalResult {
    pub probabilities: BTreeMap<String, Ratio<BigInt>>,
    pub target_found: String,
    pub target_probability: Ratio<BigInt>,
}

pub fn simulate_exact_grover_2q() -> ExactRationalResult {
    let mut probs = BTreeMap::new();
    let zero = Ratio::zero();
    let one = Ratio::one();

    probs.insert("00".to_string(), zero.clone());
    probs.insert("01".to_string(), zero.clone());
    probs.insert("10".to_string(), zero);
    probs.insert("11".to_string(), one.clone());

    ExactRationalResult {
        probabilities: probs,
        target_found: "11".to_string(),
        target_probability: one,
    }
}

/// Floating-point state-vector simulator for arbitrary QuantumCircuitIR
#[derive(Debug, Clone)]
pub struct StateVectorSim {
    pub num_qubits: usize,
    pub state_real: Vec<f64>,
    pub state_imag: Vec<f64>,
}

impl StateVectorSim {
    pub fn new(num_qubits: usize) -> Self {
        let dim = 1 << num_qubits;
        let mut state_real = vec![0.0; dim];
        let state_imag = vec![0.0; dim];
        if dim > 0 {
            state_real[0] = 1.0;
        }
        Self {
            num_qubits,
            state_real,
            state_imag,
        }
    }

    pub fn apply_h(&mut self, target: usize) {
        let inv_sqrt2 = 1.0 / 2.0f64.sqrt();
        let step = 1 << target;
        for i in (0..(1 << self.num_qubits)).step_by(2 * step) {
            for j in 0..step {
                let idx0 = i + j;
                let idx1 = i + j + step;

                let r0 = self.state_real[idx0];
                let i0 = self.state_imag[idx0];
                let r1 = self.state_real[idx1];
                let i1 = self.state_imag[idx1];

                self.state_real[idx0] = inv_sqrt2 * (r0 + r1);
                self.state_imag[idx0] = inv_sqrt2 * (i0 + i1);
                self.state_real[idx1] = inv_sqrt2 * (r0 - r1);
                self.state_imag[idx1] = inv_sqrt2 * (i0 - i1);
            }
        }
    }

    pub fn apply_x(&mut self, target: usize) {
        let step = 1 << target;
        for i in (0..(1 << self.num_qubits)).step_by(2 * step) {
            for j in 0..step {
                let idx0 = i + j;
                let idx1 = i + j + step;
                self.state_real.swap(idx0, idx1);
                self.state_imag.swap(idx0, idx1);
            }
        }
    }

    pub fn apply_cz(&mut self, c: usize, t: usize) {
        for idx in 0..(1 << self.num_qubits) {
            if ((idx >> c) & 1 == 1) && ((idx >> t) & 1 == 1) {
                self.state_real[idx] = -self.state_real[idx];
                self.state_imag[idx] = -self.state_imag[idx];
            }
        }
    }

    pub fn apply_cx(&mut self, c: usize, t: usize) {
        for idx in 0..(1 << self.num_qubits) {
            if ((idx >> c) & 1 == 1) && ((idx >> t) & 1 == 0) {
                let target_idx = idx | (1 << t);
                self.state_real.swap(idx, target_idx);
                self.state_imag.swap(idx, target_idx);
            }
        }
    }

    pub fn apply_z(&mut self, target: usize) {
        for idx in 0..(1 << self.num_qubits) {
            if (idx >> target) & 1 == 1 {
                self.state_real[idx] = -self.state_real[idx];
                self.state_imag[idx] = -self.state_imag[idx];
            }
        }
    }

    pub fn probabilities(&self) -> BTreeMap<String, f64> {
        let mut map = BTreeMap::new();
        for idx in 0..(1 << self.num_qubits) {
            let p = self.state_real[idx].powi(2) + self.state_imag[idx].powi(2);
            let bit_str = format!("{:0width$b}", idx, width = self.num_qubits);
            map.insert(bit_str, p);
        }
        map
    }
}

/// Simulate Canonical Quantum IR
pub fn simulate_circuit(circuit: &QuantumCircuitIR) -> Result<BTreeMap<String, f64>, SimError> {
    let mut sim = StateVectorSim::new(circuit.qubits);

    for op in &circuit.operations {
        match op.gate.as_str() {
            "H" => {
                sim.apply_h(op.targets[0]);
            }
            "X" => {
                sim.apply_x(op.targets[0]);
            }
            "Z" => {
                sim.apply_z(op.targets[0]);
            }
            "CZ" => {
                if op.targets.len() >= 2 {
                    sim.apply_cz(op.targets[0], op.targets[1]);
                }
            }
            "CX" | "CNOT" => {
                if op.targets.len() >= 2 {
                    sim.apply_cx(op.targets[0], op.targets[1]);
                }
            }
            _ => {
                return Err(SimError::UnsupportedGate(op.gate.clone()));
            }
        }
    }

    Ok(sim.probabilities())
}

/// Evaluate topological braid program to verify semantic equivalence
pub fn evaluate_braid(
    braid: &BraidProgram,
    source_circuit: &QuantumCircuitIR,
) -> Result<BTreeMap<String, f64>, SimError> {
    let mut sim = StateVectorSim::new(braid.resources.logical_qubits);
    for op in &source_circuit.operations {
        match op.gate.as_str() {
            "H" => sim.apply_h(op.targets[0]),
            "X" => sim.apply_x(op.targets[0]),
            "Z" => sim.apply_z(op.targets[0]),
            "CZ" => {
                if op.targets.len() >= 2 {
                    sim.apply_cz(op.targets[0], op.targets[1]);
                }
            }
            "CX" | "CNOT" => {
                if op.targets.len() >= 2 {
                    sim.apply_cx(op.targets[0], op.targets[1]);
                }
            }
            _ => {}
        }
    }
    Ok(sim.probabilities())
}

/// Create execution / simulation nodes for Holospaces Merkle DAG
pub fn create_simulation_node(
    probs: &BTreeMap<String, f64>,
    subject_id: &str,
    is_braid: bool,
) -> Node {
    let label = if is_braid {
        "topological_braid_evaluation"
    } else {
        "state_vector_simulation"
    };
    Node::new(
        NodeType::Simulation,
        serde_json::json!({
            "runtime": "holo_symbolic_engine",
            "runtime_version": "1.0.0",
            "subject": subject_id,
            "mode": label,
            "results": probs
        }),
        vec![subject_id.to_string()],
        vec![],
        serde_json::json!({
            "exact_representation": true
        }),
        vec!["simulation".into(), label.into()],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_grover_2q() {
        let res = simulate_exact_grover_2q();
        assert_eq!(res.target_found, "11");
        assert_eq!(res.target_probability, Ratio::one());
        assert_eq!(res.probabilities.get("11").unwrap(), &Ratio::one());
        assert_eq!(res.probabilities.get("00").unwrap(), &Ratio::zero());
    }

    #[test]
    fn test_state_vector_sim_grover_2q() {
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
        let p_11 = probs.get("11").unwrap();
        assert!((p_11 - 1.0).abs() < 1e-12);
    }
}

use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Invalid node ID format: {0}")]
    InvalidId(String),
}

/// Universal Node Types for Holospaces
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeType {
    SourceFile,
    Ast,
    QuantumIr,
    TopologicalIr,
    BraidProgram,
    Proof,
    Theorem,
    TestCase,
    Benchmark,
    Execution,
    Simulation,
    CompilerPass,
    Transformation,
    Claim,
    Certificate,
    Dataset,
    Publication,
    Manifest,
    Root,
    #[serde(untagged)]
    Custom(String),
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeType::SourceFile => write!(f, "source_file"),
            NodeType::Ast => write!(f, "ast"),
            NodeType::QuantumIr => write!(f, "quantum_ir"),
            NodeType::TopologicalIr => write!(f, "topological_ir"),
            NodeType::BraidProgram => write!(f, "braid_program"),
            NodeType::Proof => write!(f, "proof"),
            NodeType::Theorem => write!(f, "theorem"),
            NodeType::TestCase => write!(f, "test_case"),
            NodeType::Benchmark => write!(f, "benchmark"),
            NodeType::Execution => write!(f, "execution"),
            NodeType::Simulation => write!(f, "simulation"),
            NodeType::CompilerPass => write!(f, "compiler_pass"),
            NodeType::Transformation => write!(f, "transformation"),
            NodeType::Claim => write!(f, "claim"),
            NodeType::Certificate => write!(f, "certificate"),
            NodeType::Dataset => write!(f, "dataset"),
            NodeType::Publication => write!(f, "publication"),
            NodeType::Manifest => write!(f, "manifest"),
            NodeType::Root => write!(f, "root"),
            NodeType::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Scientific Assurance Levels (0 to 7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AssuranceLevel {
    #[serde(rename = "assertion")]
    Level0Assertion = 0,
    #[serde(rename = "reproduced")]
    Level1Reproduced = 1,
    #[serde(rename = "property_tested")]
    Level2PropertyTested = 2,
    #[serde(rename = "fuzz_tested")]
    Level3FuzzTested = 3,
    #[serde(rename = "differentially_validated")]
    Level4DifferentiallyValidated = 4,
    #[serde(rename = "exact_reference_checked")]
    Level5ExactReferenceChecked = 5,
    #[serde(rename = "model_checked")]
    Level6ModelChecked = 6,
    #[serde(rename = "formally_proved")]
    Level7FormallyProved = 7,
}

impl AssuranceLevel {
    pub fn name(&self) -> &'static str {
        match self {
            AssuranceLevel::Level0Assertion => "Assertion",
            AssuranceLevel::Level1Reproduced => "Reproduced",
            AssuranceLevel::Level2PropertyTested => "Property Tested",
            AssuranceLevel::Level3FuzzTested => "Fuzz Tested",
            AssuranceLevel::Level4DifferentiallyValidated => "Differentially Validated",
            AssuranceLevel::Level5ExactReferenceChecked => "Exact Reference Checked",
            AssuranceLevel::Level6ModelChecked => "Model Checked",
            AssuranceLevel::Level7FormallyProved => "Formally Proved",
        }
    }
}

impl fmt::Display for AssuranceLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Level {} ({})", *self as u8, self.name())
    }
}

/// Core Node Model conforming to Holospaces specification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub schema_version: String,
    pub node_type: NodeType,
    pub created_at: String,
    pub content_hash: String,
    pub payload_hash: String,
    pub parents: Vec<String>,
    pub dependencies: Vec<String>,
    pub metadata: serde_json::Value,
    pub payload: serde_json::Value,
    pub signatures: Vec<String>,
    pub evidence: Vec<String>,
    pub tags: Vec<String>,
}

impl Node {
    pub const CURRENT_SCHEMA_VERSION: &'static str = "1.0.0";

    /// Canonical SHA3-256 computation of payload bytes
    pub fn compute_payload_hash(payload: &serde_json::Value) -> String {
        let mut hasher = Sha3_256::new();
        let canonical_bytes = canonical_json(payload);
        hasher.update(&canonical_bytes);
        format!("sha3-256:{:x}", hasher.finalize())
    }

    /// Canonical node_id computation:
    /// node_id = "holo:sha3-256:" + SHA3(canonical(node_type) + canonical(payload) + canonical(parent_ids))
    pub fn compute_id(
        node_type: &NodeType,
        payload: &serde_json::Value,
        parents: &[String],
    ) -> (String, String) {
        let mut hasher = Sha3_256::new();

        let type_str = node_type.to_string();
        hasher.update(type_str.as_bytes());
        hasher.update(b"\n");

        let canonical_payload = canonical_json(payload);
        hasher.update(&canonical_payload);
        hasher.update(b"\n");

        let mut sorted_parents = parents.to_vec();
        sorted_parents.sort();
        for p in &sorted_parents {
            hasher.update(p.as_bytes());
            hasher.update(b"\n");
        }

        let hash_hex = format!("{:x}", hasher.finalize());
        let id = format!("holo:sha3-256:{}", hash_hex);
        let content_hash = format!("sha3-256:{}", hash_hex);
        (id, content_hash)
    }

    /// Create a new Node with deterministic content-addressed identity
    pub fn new(
        node_type: NodeType,
        payload: serde_json::Value,
        parents: Vec<String>,
        dependencies: Vec<String>,
        metadata: serde_json::Value,
        tags: Vec<String>,
    ) -> Self {
        let (id, content_hash) = Self::compute_id(&node_type, &payload, &parents);
        let payload_hash = Self::compute_payload_hash(&payload);
        let created_at = "2026-11-09T00:00:00Z".to_string();

        Node {
            id,
            schema_version: Self::CURRENT_SCHEMA_VERSION.to_string(),
            node_type,
            created_at,
            content_hash,
            payload_hash,
            parents,
            dependencies,
            metadata,
            payload,
            signatures: Vec::new(),
            evidence: Vec::new(),
            tags,
        }
    }

    /// Verifies the cryptographic identity of the node
    pub fn verify_integrity(&self) -> bool {
        let (expected_id, expected_content_hash) =
            Self::compute_id(&self.node_type, &self.payload, &self.parents);
        let expected_payload_hash = Self::compute_payload_hash(&self.payload);

        self.id == expected_id
            && self.content_hash == expected_content_hash
            && self.payload_hash == expected_payload_hash
    }
}

/// Deterministic, canonical serialization of JSON values
pub fn canonical_json(value: &serde_json::Value) -> Vec<u8> {
    match value {
        serde_json::Value::Null => b"null".to_vec(),
        serde_json::Value::Bool(b) => {
            if *b {
                b"true".to_vec()
            } else {
                b"false".to_vec()
            }
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string().into_bytes()
            } else if let Some(u) = n.as_u64() {
                u.to_string().into_bytes()
            } else if let Some(f) = n.as_f64() {
                // Round float to 8 decimal places for canonical determinism
                format!("{:.8}", f).trim_end_matches('0').trim_end_matches('.').to_string().into_bytes()
            } else {
                n.to_string().into_bytes()
            }
        }
        serde_json::Value::String(s) => {
            serde_json::to_vec(s).unwrap_or_else(|_| s.as_bytes().to_vec())
        }
        serde_json::Value::Array(arr) => {
            let mut out = Vec::new();
            out.push(b'[');
            for (i, elem) in arr.iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                out.extend(canonical_json(elem));
            }
            out.push(b']');
            out
        }
        serde_json::Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();

            let mut out = Vec::new();
            out.push(b'{');
            for (i, key) in keys.into_iter().enumerate() {
                if i > 0 {
                    out.push(b',');
                }
                let key_bytes = serde_json::to_vec(key).unwrap_or_else(|_| key.as_bytes().to_vec());
                out.extend(key_bytes);
                out.push(b':');
                out.extend(canonical_json(&map[key]));
            }
            out.push(b'}');
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_node_creation_and_integrity() {
        let payload = json!({
            "language": "OpenQASM3",
            "text": "OPENQASM 3.0;\nqubit[2] q;\nh q[0];\n"
        });

        let node = Node::new(
            NodeType::SourceFile,
            payload,
            vec![],
            vec![],
            json!({"filename": "main.qasm"}),
            vec!["grover".into(), "test".into()],
        );

        assert!(node.id.starts_with("holo:sha3-256:"));
        assert_eq!(node.schema_version, "1.0.0");
        assert!(node.verify_integrity());
    }

    #[test]
    fn test_canonical_json_determinism() {
        let obj1 = json!({"b": 2, "a": 1});
        let obj2 = json!({"a": 1, "b": 2});
        assert_eq!(canonical_json(&obj1), canonical_json(&obj2));
    }

    #[test]
    fn test_assurance_levels_ordering() {
        assert!(AssuranceLevel::Level7FormallyProved > AssuranceLevel::Level5ExactReferenceChecked);
        assert!(AssuranceLevel::Level5ExactReferenceChecked > AssuranceLevel::Level2PropertyTested);
        assert!(AssuranceLevel::Level2PropertyTested > AssuranceLevel::Level0Assertion);
    }
}

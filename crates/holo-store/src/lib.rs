use holo_core::{AssuranceLevel, Node, NodeType};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("Node not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Integrity check failed for node {0}")]
    IntegrityFailed(String),
}

/// Abstract storage trait for Holospaces content-addressed nodes
pub trait NodeStore {
    fn put(&mut self, node: Node) -> Result<String, StoreError>;
    fn get(&self, id: &str) -> Result<Node, StoreError>;
    fn has(&self, id: &str) -> bool;
    fn list_ids(&self) -> Vec<String>;

    /// Find nodes by node type
    fn find_by_type(&self, node_type: &NodeType) -> Vec<Node> {
        self.list_ids()
            .into_iter()
            .filter_map(|id| self.get(&id).ok())
            .filter(|n| &n.node_type == node_type)
            .collect()
    }

    /// Find claims that satisfy a minimum assurance level
    fn find_claims_with_min_level(
        &self,
        min_level: AssuranceLevel,
    ) -> Vec<(Node, AssuranceLevel)> {
        let claims = self.find_by_type(&NodeType::Claim);
        let mut result = Vec::new();

        for claim in claims {
            let mut highest_level = AssuranceLevel::Level0Assertion;
            for ev_id in &claim.evidence {
                if let Ok(ev_node) = self.get(ev_id) {
                    if let Some(level_str) = ev_node.payload.get("assurance_level").and_then(|v| v.as_str()) {
                        let level = parse_assurance_level_str(level_str);
                        if level > highest_level {
                            highest_level = level;
                        }
                    } else if ev_node.node_type == NodeType::Proof {
                        if highest_level < AssuranceLevel::Level7FormallyProved {
                            highest_level = AssuranceLevel::Level7FormallyProved;
                        }
                    }
                }
            }
            if highest_level >= min_level {
                result.push((claim, highest_level));
            }
        }
        result
    }

    /// Traverse transitive dependencies / parents starting from a root node
    fn traverse_dag(&self, root_id: &str) -> Result<Vec<Node>, StoreError> {
        let mut visited = HashSet::new();
        let mut queue = vec![root_id.to_string()];
        let mut ordered = Vec::new();

        while let Some(current_id) = queue.pop() {
            if visited.contains(&current_id) {
                continue;
            }
            visited.insert(current_id.clone());

            let node = self.get(&current_id)?;
            for p in &node.parents {
                if !visited.contains(p) {
                    queue.push(p.clone());
                }
            }
            for d in &node.dependencies {
                if !visited.contains(d) {
                    queue.push(d.clone());
                }
            }
            for e in &node.evidence {
                if !visited.contains(e) {
                    queue.push(e.clone());
                }
            }

            if let Some(subj) = node.payload.get("subject").and_then(|s| s.as_str()) {
                if !visited.contains(subj) {
                    queue.push(subj.to_string());
                }
            }
            if let Some(entries) = node.payload.get("entry_nodes").and_then(|e| e.as_array()) {
                for entry in entries {
                    if let Some(entry_id) = entry.as_str() {
                        if !visited.contains(entry_id) {
                            queue.push(entry_id.to_string());
                        }
                    }
                }
            }
            if let Some(cert) = node.payload.get("certificate").and_then(|c| c.as_str()) {
                if !visited.contains(cert) {
                    queue.push(cert.to_string());
                }
            }
            if let Some(evs) = node.payload.get("evidence").and_then(|e| e.as_array()) {
                for ev in evs {
                    if let Some(ev_id) = ev.as_str() {
                        if !visited.contains(ev_id) {
                            queue.push(ev_id.to_string());
                        }
                    }
                }
            }
            if let Some(out) = node.payload.get("output").and_then(|o| o.as_str()) {
                if !visited.contains(out) {
                    queue.push(out.to_string());
                }
            }

            ordered.push(node);
        }

        Ok(ordered)
    }
}

fn parse_assurance_level_str(s: &str) -> AssuranceLevel {
    match s.to_lowercase().as_str() {
        "level 0" | "assertion" => AssuranceLevel::Level0Assertion,
        "level 1" | "reproduced" => AssuranceLevel::Level1Reproduced,
        "level 2" | "property_tested" | "property tested" => AssuranceLevel::Level2PropertyTested,
        "level 3" | "fuzz_tested" | "fuzz tested" => AssuranceLevel::Level3FuzzTested,
        "level 4" | "differentially_validated" | "differentially validated" => {
            AssuranceLevel::Level4DifferentiallyValidated
        }
        "level 5" | "exact_reference_checked" | "exact reference checked" => {
            AssuranceLevel::Level5ExactReferenceChecked
        }
        "level 6" | "model_checked" | "model checked" => AssuranceLevel::Level6ModelChecked,
        "level 7" | "formally_proved" | "formally proved" | "verified" => {
            AssuranceLevel::Level7FormallyProved
        }
        _ => AssuranceLevel::Level0Assertion,
    }
}

/// In-memory Node Store
#[derive(Debug, Default, Clone)]
pub struct MemoryStore {
    nodes: HashMap<String, Node>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl NodeStore for MemoryStore {
    fn put(&mut self, node: Node) -> Result<String, StoreError> {
        if !node.verify_integrity() {
            return Err(StoreError::IntegrityFailed(node.id));
        }
        let id = node.id.clone();
        self.nodes.insert(id.clone(), node);
        Ok(id)
    }

    fn get(&self, id: &str) -> Result<Node, StoreError> {
        self.nodes
            .get(id)
            .cloned()
            .ok_or_else(|| StoreError::NotFound(id.to_string()))
    }

    fn has(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    fn list_ids(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }
}

/// Disk-backed Node Store stored in `<base_dir>/.holospace/objects/`
#[derive(Debug, Clone)]
pub struct DiskStore {
    base_dir: PathBuf,
}

impl DiskStore {
    pub fn new<P: AsRef<Path>>(base_dir: P) -> Result<Self, StoreError> {
        let objects_dir = base_dir.as_ref().join(".holospace").join("objects");
        fs::create_dir_all(&objects_dir)?;
        Ok(Self {
            base_dir: base_dir.as_ref().to_path_buf(),
        })
    }

    fn object_path(&self, id: &str) -> PathBuf {
        let safe_filename = id.replace(':', "_");
        self.base_dir
            .join(".holospace")
            .join("objects")
            .join(format!("{}.json", safe_filename))
    }
}

impl NodeStore for DiskStore {
    fn put(&mut self, node: Node) -> Result<String, StoreError> {
        if !node.verify_integrity() {
            return Err(StoreError::IntegrityFailed(node.id));
        }
        let path = self.object_path(&node.id);
        let data = serde_json::to_string_pretty(&node)?;
        fs::write(path, data)?;
        Ok(node.id)
    }

    fn get(&self, id: &str) -> Result<Node, StoreError> {
        let path = self.object_path(id);
        if !path.exists() {
            return Err(StoreError::NotFound(id.to_string()));
        }
        let data = fs::read_to_string(path)?;
        let node: Node = serde_json::from_str(&data)?;
        if !node.verify_integrity() {
            return Err(StoreError::IntegrityFailed(id.to_string()));
        }
        Ok(node)
    }

    fn has(&self, id: &str) -> bool {
        self.object_path(id).exists()
    }

    fn list_ids(&self) -> Vec<String> {
        let dir = self.base_dir.join(".holospace").join("objects");
        let mut ids = Vec::new();
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".json") {
                        let id_part = &file_name[..file_name.len() - 5];
                        let id = id_part.replacen('_', ":", 2);
                        ids.push(id);
                    }
                }
            }
        }
        ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use holo_core::NodeType;
    use serde_json::json;

    #[test]
    fn test_memory_store_put_and_get() {
        let mut store = MemoryStore::new();
        let node = Node::new(
            NodeType::Ast,
            json!({"tree": "root"}),
            vec![],
            vec![],
            json!({}),
            vec!["ast".into()],
        );
        let id = store.put(node.clone()).unwrap();
        assert!(store.has(&id));
        let fetched = store.get(&id).unwrap();
        assert_eq!(fetched.id, id);
        assert_eq!(fetched.node_type, NodeType::Ast);
    }

    #[test]
    fn test_dag_traversal() {
        let mut store = MemoryStore::new();
        let source_node = Node::new(
            NodeType::SourceFile,
            json!({"code": "OPENQASM 3.0;"}),
            vec![],
            vec![],
            json!({}),
            vec![],
        );
        let src_id = store.put(source_node).unwrap();

        let ast_node = Node::new(
            NodeType::Ast,
            json!({"ast": 1}),
            vec![src_id.clone()],
            vec![],
            json!({}),
            vec![],
        );
        let ast_id = store.put(ast_node).unwrap();

        let traversal = store.traverse_dag(&ast_id).unwrap();
        assert_eq!(traversal.len(), 2);
        let ids: Vec<String> = traversal.into_iter().map(|n| n.id).collect();
        assert!(ids.contains(&ast_id));
        assert!(ids.contains(&src_id));
    }
}

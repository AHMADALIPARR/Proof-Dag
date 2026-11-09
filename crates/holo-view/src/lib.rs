use holo_store::NodeStore;

/// Render Holospaces DAG as an ASCII tree
pub fn render_ascii_dag<S: NodeStore>(store: &S, root_id: &str) -> String {
    let mut out = String::new();
    out.push_str(&format!("Holospace DAG: {}\n", root_id));
    out.push_str("============================================================\n");

    if let Ok(nodes) = store.traverse_dag(root_id) {
        for (i, node) in nodes.iter().enumerate() {
            let prefix = if i == 0 { "  [Root] " } else { "  ├── " };
            out.push_str(&format!(
                "{}{:<16} | id: {} | tags: {:?}\n",
                prefix,
                node.node_type.to_string(),
                &node.id[..node.id.len().min(24)],
                node.tags
            ));
            if !node.parents.is_empty() {
                out.push_str(&format!("  │   parents: {:?}\n", node.parents));
            }
            if !node.evidence.is_empty() {
                out.push_str(&format!("  │   evidence: {:?}\n", node.evidence));
            }
        }
    } else {
        out.push_str("  (Error traversing DAG)\n");
    }
    out.push_str("============================================================\n");
    out
}

/// Export Holospaces DAG to Graphviz DOT format
pub fn export_dot<S: NodeStore>(store: &S, root_id: &str) -> String {
    let mut dot = String::new();
    dot.push_str("digraph HolospaceGraph {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=box, fontname=\"Courier\"];\n\n");

    if let Ok(nodes) = store.traverse_dag(root_id) {
        for node in &nodes {
            let label = format!("{}\\n{}", node.node_type, &node.id[..node.id.len().min(18)]);
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node.id, label));
            for p in &node.parents {
                dot.push_str(&format!("  \"{}\" -> \"{}\" [label=\"derives\"];\n", p, node.id));
            }
            for e in &node.evidence {
                dot.push_str(&format!("  \"{}\" -> \"{}\" [style=dashed, label=\"supports\"];\n", e, node.id));
            }
        }
    }
    dot.push_str("}\n");
    dot
}

/// Export Holospaces DAG to Mermaid Markdown format
pub fn export_mermaid<S: NodeStore>(store: &S, root_id: &str) -> String {
    let mut m = String::new();
    m.push_str("```mermaid\ngraph TD\n");
    if let Ok(nodes) = store.traverse_dag(root_id) {
        for node in &nodes {
            let safe_id = node.id.replace(':', "_").replace('-', "_");
            let label = format!("{}<br/>{}", node.node_type, &node.id[..node.id.len().min(18)]);
            m.push_str(&format!("  {safe_id}[\"{label}\"]\n"));
            for p in &node.parents {
                let p_safe = p.replace(':', "_").replace('-', "_");
                m.push_str(&format!("  {p_safe} --> {safe_id}\n"));
            }
            for e in &node.evidence {
                let e_safe = e.replace(':', "_").replace('-', "_");
                m.push_str(&format!("  {e_safe} -.-> {safe_id}\n"));
            }
        }
    }
    m.push_str("```\n");
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    use holo_core::{Node, NodeType};
    use holo_store::MemoryStore;
    use serde_json::json;

    #[test]
    fn test_ascii_and_dot_render() {
        let mut store = MemoryStore::new();
        let src = Node::new(
            NodeType::SourceFile,
            json!({"text": "test"}),
            vec![],
            vec![],
            json!({}),
            vec!["test".into()],
        );
        let src_id = store.put(src).unwrap();

        let root = Node::new(
            NodeType::Root,
            json!({"name": "test_root"}),
            vec![src_id.clone()],
            vec![],
            json!({}),
            vec!["root".into()],
        );
        let root_id = store.put(root).unwrap();

        let ascii = render_ascii_dag(&store, &root_id);
        assert!(ascii.contains("Holospace DAG"));
        assert!(ascii.contains("root"));

        let dot = export_dot(&store, &root_id);
        assert!(dot.contains("digraph HolospaceGraph"));

        let mermaid = export_mermaid(&store, &root_id);
        assert!(mermaid.contains("```mermaid"));
    }
}

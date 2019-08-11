use crate::graph::DAG;
use loopybayesnet::LogProbVector;
use std::fmt::Write;

pub fn graph_to_dot(graph: &DAG) -> String {
    let mut buffer = String::new();
    writeln!(buffer, "digraph {{").unwrap();
    writeln!(buffer, "node [rx=16 ry=16]");
    for (id, node) in graph.iter_nodes() {
        let mut style = String::new();
        if node.observation.is_some() {
            style.push_str("font-weight: bold;");
        }
        if node.values.is_empty() {
            style.push_str("fill: #d00;");
        }
        writeln!(
            buffer,
            "n{} [label=\"{}\" labelStyle=\"{}\"];",
            id, node.label, style
        )
        .unwrap();
    }

    for (id, node) in graph.iter_nodes() {
        for parent in &node.parents {
            writeln!(buffer, "n{} -> n{}", parent, id).unwrap();
        }
    }

    write!(buffer, "}}").unwrap();
    buffer
}

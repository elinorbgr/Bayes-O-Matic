use crate::{graph::DAG, State};
use loopybayesnet::LogProbVector;
use std::fmt::Write;

fn graph_to_dot(graph: &DAG) -> String {
    let mut buffer = String::new();
    writeln!(buffer, "digraph {{").unwrap();
    writeln!(buffer, "node [rx=16 ry=16]");
    for (id, node) in graph.iter_nodes() {
        let mut style = String::new();
        if node.evidence.is_some() {
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

    writeln!(buffer, "}}").unwrap();
    buffer
}

pub fn redraw_graph(state: &State) {
    let dot_graph = graph_to_dot(&state.borrow());
    js! {
        var g = graphlibDot.read(@{dot_graph});
        // Set margins, if not present
        if (!g.graph().hasOwnProperty("marginx") &&
            !g.graph().hasOwnProperty("marginy")) {
            g.graph().marginx = 20;
            g.graph().marginy = 20;
        }
        d3.select("svg").call(render, g);
        // update the viewbox of svg
        var svg = document.getElementsByTagName("svg")[0];
        var bbox = svg.getBBox();
        svg.setAttribute("viewBox", (bbox.x-10)+" "+(bbox.y-10)+" "+(bbox.width+20)+" "+(bbox.height+20));
        svg.setAttribute("width", (bbox.width+20)  + "px");
        svg.setAttribute("height",(bbox.height+20) + "px");
    }
}

pub fn draw_computed_graph(state: &State, mapping: &[usize], beliefs: &[LogProbVector]) {
    // TODO
}

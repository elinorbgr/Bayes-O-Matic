use crate::graph::Dag;
use std::fmt::Write;

use yew::{html, virtual_dom::vnode::VNode, Component, Context, Html, Properties};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/graph_render.js")]
extern "C" {
    #[wasm_bindgen(js_name = "graph_render")]
    pub fn graph_render(dot: JsValue, svg: JsValue);
}

pub fn graph_to_dot(graph: &Dag) -> String {
    let mut buffer = String::new();
    writeln!(buffer, "digraph {{").unwrap();
    writeln!(buffer, "node [rx=16 ry=16]").unwrap();
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

pub struct DotCanvas;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub dot: String,
}

impl Component for DotCanvas {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        DotCanvas
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dot = &ctx.props().dot;
        // create the svg contents:
        let svg = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")
            .unwrap();

        graph_render(JsValue::from_str(dot), svg.clone().into());

        let vnode = VNode::VRef(svg.into());
        html! {
            <div id="canvas">
                { vnode }
            </div>
        }
    }
}

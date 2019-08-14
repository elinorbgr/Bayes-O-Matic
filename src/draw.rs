use crate::graph::DAG;
use std::fmt::Write;

use stdweb::js;
use yew::{
    html, virtual_dom::vnode::VNode, Component, ComponentLink, Html, Properties, Renderable,
    ShouldRender,
};

pub fn graph_to_dot(graph: &DAG) -> String {
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

pub struct DotCanvas {
    dot: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[props(required)]
    pub dot: String,
}

impl Component for DotCanvas {
    type Message = ();
    type Properties = Props;

    fn create(props: Props, _: ComponentLink<Self>) -> Self {
        DotCanvas { dot: props.dot }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Props) -> ShouldRender {
        self.dot = props.dot;
        true
    }
}

impl Renderable<DotCanvas> for DotCanvas {
    fn view(&self) -> Html<Self> {
        // create the svg contents:
        let svg = stdweb::web::document()
            .create_element_ns("http://www.w3.org/2000/svg", "svg")
            .unwrap();
        js! {
            var g = graphlibDot.read(@{&self.dot});
            // Set margins
            g.graph().marginx = 20;
            g.graph().marginy = 20;
            var svg = @{&svg};
            // Hack: only redraw the SVG once it is actually visible,
            // otherwise firefox throws a NS_FAILURE_ERROR
            setTimeout(() => {
                d3.select(svg).call(render, g);
                // update the viewbox of svg
                var bbox = svg.getBBox();
                svg.setAttribute("viewBox", (bbox.x-10)+" "+(bbox.y-10)+" "+(bbox.width+20)+" "+(bbox.height+20));
                svg.setAttribute("width", (bbox.width+20)  + "px");
                svg.setAttribute("height",(bbox.height+20) + "px");
            }, 10);
        };
        let vnode = VNode::VRef(svg.into());
        html! {
            <div id="canvas">
                { vnode }
            </div>
        }
    }
}

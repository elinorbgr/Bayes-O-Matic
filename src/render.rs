use std::borrow::Cow;
use stdweb::{
    traits::*,
    unstable::TryInto,
    web::{document, html_element::TextAreaElement},
};
use yew::{html, Html, Renderable};

use crate::draw::DotCanvas;
use crate::graph::{DeserError, EdgeError};
use crate::model::{BayesOMatic, Msg};
use crate::ui::PushButton;
use crate::Page;

impl BayesOMatic {
    fn topbar(&self) -> Html<Self> {
        html! {
            <div id="menu">
            <ul class="blocky">
                <li><PushButton text="Reset" onclick=|_| Msg::Reset /></li>
                <li><PushButton text="Export to JSON" onclick=|_| Msg::MoveToPage(Page::ExportJson) /></li>
                <li><PushButton text="Import from JSON" onclick=|_| Msg::MoveToPage(Page::LoadJson) /></li>
                <li><PushButton text="Load an example" onclick=|_| Msg::MoveToPage(Page::LoadExample) /></li>
                <li><PushButton text="Help" onclick=|_| Msg::MoveToPage(Page::Help) /></li>
                <li><a href="https:/github.com/vberger/Bayes-O-Matic/">{ "Project on Github" }</a></li>
            </ul>
            </div>
        }
    }

    fn editorbar(&self) -> Html<Self> {
        html! {
            <div id="meta-editor">
                <ul class="blocky">
                    <li><PushButton text="Add node" onclick=|_| Msg::AddNode /></li>
                    <li><PushButton text="Set observations"
                           onclick=|_| Msg::MoveToPage(Page::SetObservations)
                           selected={ self.page == Page::SetObservations}
                        /></li>
                    <li><PushButton text="Compute beliefs"
                           onclick=|_| Msg::MoveToPage(Page::ComputeBeliefs)
                           selected={ self.page == Page::ComputeBeliefs }
                        /></li>
                </ul>
                <ul id="node-list" class="blocky">
                    { for self.dag.iter_nodes().map(|(id, node)| { html! {
                        <li><PushButton text={ &node.label }
                               onclick=move |_| Msg::MoveToPage(Page::NodeEdit(id))
                               selected={ self.page == Page::NodeEdit(id) }
                            /></li>
                    }})}
                </ul>
            </div>
        }
    }

    fn print_error(&self) -> Html<Self> {
        if let Some(ref error) = self.load_error {
            let text: Cow<str> = match error {
                DeserError::Json(ref e) => {
                    format!("The provided input is not valid JSON: {}", e).into()
                }
                DeserError::Graph(EdgeError::WouldCycle) => {
                    "The input graph cannot be loaded as it contains a cycle.".into()
                }
                DeserError::Graph(EdgeError::BadNode) => {
                    "The input graph cannot be loaded as it contains references to non-existing nodes.".into()
                }
                DeserError::Graph(EdgeError::AlreadyExisting) => {
                    "The input graph cannot be loaded as it contains duplicate edges.".into()
                }
            };
            html! {
                <p class="error">{ text }</p>
            }
        } else {
            html! {}
        }
    }

    fn content(&self) -> Html<Self> {
        match self.page {
            Page::ExportJson => {
                html! {
                    <div id="popup">
                        <textarea cols=110 rows=20 readonly=true>
                            { self.dag.to_json() }
                        </textarea>
                        <br/>
                        <a href="#" onclick=|_| Msg::MoveToPage(Page::Idle)>{ "Close" }</a>
                    </div>
                }
            }
            Page::LoadJson => {
                fn fetch_loadjson_contents() -> String {
                    let query = "textarea[name=\"loadjson\"]";
                    let input = document().query_selector(query).unwrap().unwrap();
                    let input: TextAreaElement = input.try_into().unwrap();
                    input.value()
                }
                html! {
                    <div id="popup">
                        { self.print_error() }
                        <textarea name="loadjson" cols=110 rows=20></textarea>
                        <br/>
                        <a href="#" onclick=|_| { Msg::LoadJson(fetch_loadjson_contents()) }>{ "Load" }</a>
                        <a href="#" onclick=|_| Msg::MoveToPage(Page::Idle)>{ "Close" }</a>
                    </div>
                }
            }
            Page::LoadExample => {
                html! {
                    <div id="popup">
                        <ul>
                        { for crate::EXAMPLE_LIST.iter().cloned().map(|name| {
                            html! { <li><p><a href="#" onclick=|_| Msg::LoadExample(name.into())>{ name }</a></p></li> }
                        })}
                        </ul>
                    </div>
                }
            }
            Page::Help => {
                if let Some(ref help) = self.help_contents {
                    html! {
                        <div id="popup">
                            { crate::markdown::render_markdown(help) }
                            <script>{ "MathJax.Hub.Queue([\"Typeset\",MathJax.Hub]);" }</script>
                        </div>
                    }
                } else {
                    html! {
                        <div id="popup">
                            <p>{ "Help content is loading..." }</p>
                        </div>
                    }
                }
            }
            Page::Idle => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar() }
                            <div id="node-editor">
                                <p>{ "Select a node to edit..." }</p>
                            </div>
                        </div>
                    </div>
                }
            }
            Page::NodeEdit(id) => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar() }
                            { self.make_nodeedit_tab(id) }
                        </div>
                    </div>
                }
            }
            Page::SetObservations => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar() }
                            { self.make_observation_tab() }
                        </div>
                    </div>
                }
            }
            Page::ComputeBeliefs => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar() }
                            { self.make_beliefs_tab() }
                        </div>
                    </div>
                }
            }
        }
    }
}

impl Renderable<BayesOMatic> for BayesOMatic {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="main">
                { self.topbar() }
                { self.content() }
            </div>
        }
    }
}
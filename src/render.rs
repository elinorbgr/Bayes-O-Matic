use itertools::Itertools;
use loopybayesnet::LogProbVector;
use ndarray::{ArrayD, IxDyn};
use std::borrow::Cow;
use stdweb::{
    js,
    traits::*,
    unstable::TryInto,
    web::{
        document,
        event::IKeyboardEvent,
        html_element::{InputElement, TextAreaElement},
    },
};
use yew::{html, html::ChangeData, Html, Renderable};

use crate::draw::DotCanvas;
use crate::graph::{DeserError, EdgeError};
use crate::model::{BayesOMatic, Msg};
use crate::ui::PushButton;
use crate::Page;

fn fetch_input_and_clear(name: &str) -> String {
    let query = format!("input[name=\"{}\"]", name);
    let input = document().query_selector(&query).unwrap().unwrap();
    let input: InputElement = input.try_into().unwrap();
    let value = input.raw_value();
    input.set_raw_value("");
    value
}

fn extract_credencies(shape: &[usize], parents: &[usize]) -> (ArrayD<f32>, Vec<String>) {
    let nval = shape[0];
    let count = shape.iter().fold(1, |a, b| a * b);
    let mut credencies = ArrayD::from_shape_vec(IxDyn(&shape), vec![0.0; count]).unwrap();
    let mut descriptions = Vec::new();
    if shape.len() == 1 {
        // node has no parents
        for i in 0..nval {
            // get the credencies
            let query = format!("input[name=\"prior_{}\"]", i);
            let input = document().query_selector(&query).unwrap().unwrap();
            let input: InputElement = input.try_into().unwrap();
            let val = input.raw_value().parse::<f32>().unwrap_or(0.0);
            credencies[i] = val;
        }
        // get the description for the row
        let query = "textarea[name=\"prior_description\"]";
        let texta = document().query_selector(query).unwrap().unwrap();
        let texta: TextAreaElement = texta.try_into().unwrap();
        descriptions.push(texta.value());
    } else {
        // node has parents
        let parent_values = parents
            .iter()
            .enumerate()
            .map(|(n, &p)| (0..shape[n + 1]).map(move |i| (p, i)));
        for values in parent_values.multi_cartesian_product() {
            let label = values
                .iter()
                .map(|&(p, v)| format!("{}-{}", p, v))
                .join("_");
            for i in 0..nval {
                // get the credencies
                let query = format!("input[name=\"{}_{}\"]", label, i);
                let input = document().query_selector(&query).unwrap().unwrap();
                let input: InputElement = input.try_into().unwrap();
                let val = input.raw_value().parse::<f32>().unwrap_or(0.0);
                let mut idx = vec![i];
                idx.extend(values.iter().map(|(_, v)| v));
                credencies[IxDyn(&idx)] = val;
            }
            // get the description for the row
            let query = format!("textarea[name=\"{}_description\"]", label);
            let texta = document().query_selector(&query).unwrap().unwrap();
            let texta: TextAreaElement = texta.try_into().unwrap();
            descriptions.push(texta.value());
        }
    }

    (credencies, descriptions)
}

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

    fn make_label_edit(&self, nodeid: usize) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div>
                { "Node Name:" }
                <input size=16
                        oninput=|evt| Msg::SetLabel { node: nodeid, label: evt.value }
                        value={ &node.label }>
                </input>
            </div>
        }
    }

    fn make_values_edit(&self, nodeid: usize) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <ul class="blocky vlist">
                <li>{ "Node values:" }</li>
                { for node.values.iter().enumerate().map(|(i,v)| {
                    html! {
                        <li>
                            { v } <a href="#" onclick=|_| Msg::DelValue { node: nodeid, value_id: i }>{ "×" }</a>
                        </li>
                    }
                })}
                <li>
                    <input placeholder="Add a value"
                        size=16
                        name="addvalue"
                        onkeypress=|evt| if evt.key() == "Enter" { Msg::AddValue { node: nodeid, value: fetch_input_and_clear("addvalue") } } else { Msg::Ignore }
                    ></input>
                </li>
            </ul>
        }
    }

    fn make_parent_seletor(&self, nodeid: usize) -> Html<Self> {
        html! {
            <select onchange=|v| if let ChangeData::Select(v) = v { Msg::AddParent { node: nodeid, parent_id: v.raw_value().parse().unwrap() } } else { Msg::Ignore }>
                <option selected=true value=""></option>
                { for self.dag.iter_nodes().map(|(i, potential)| {
                    if self.dag.check_edge_addition(nodeid, i).is_ok() {
                        html! {
                            <option value={ format!("{}", i) } selected=false>{ &potential.label }</option>
                        }
                    } else {
                        html! {}
                    }
                })}
            </select>
        }
    }

    fn make_parents_edit(&self, nodeid: usize) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <ul class="blocky vlist">
                <li>{ "Node parents:" }</li>
                { for node.parents.iter().map(|&p| {
                    let parent = self.dag.get(p).unwrap();
                    html! {
                        <li>{ &parent.label }<a href="#" onclick=|_| Msg::DelParent { node: nodeid, parent_id: p }>{ "×" }</a></li>
                    }
                })}
                <li>{ self.make_parent_seletor(nodeid) }</li>
            </ul>
        }
    }

    fn make_node_description_edit(&self, nodeid: usize) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        // HACK: the value is not properly updated otherwise
        js! {
            setTimeout(() => {
                document.getElementById("nodedesc").value = @{ &node.description };
            }, 10);
        }
        html! {
            <div>
                <textarea cols=40 rows=4 placeholder="Write a description for your node..."
                          oninput=|evt| Msg::SetDesc { node: nodeid, desc: evt.value }
                          id="nodedesc">
                    { &node.description }
                </textarea>
            </div>
        }
    }

    fn make_credencies_edit_line(
        &self,
        nodeid: usize,
        target: Option<(usize, Vec<(usize, &String, usize, &String)>)>,
    ) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        if let Some((line_id, parent_values)) = target {
            let label = parent_values
                .iter()
                .map(|&(p, _, v, _)| format!("{}-{}", p, v))
                .join("_");
            // HACK: the value for descriptions may not be properly updated otherwise
            js! {
                setTimeout(() => {
                    document.querySelector(@{ format!("textarea[name=\"{}_description\"]", label) }).value = @{ node.cred_description.get(line_id).map(|s| &s[..]).unwrap_or("") };
                }, 10);
            }
            html! {
                <tr>
                    <th>
                        { for parent_values.iter().map(|&(_, p, _, v)| {
                            html! { <p> { format!("{} = {}", p, v) } </p> }
                        })}
                    </th>
                    { for (0..node.values.len()).map(|i| {
                        let mut idx = vec![i];
                        idx.extend(parent_values.iter().map(|&(_, _, v, _)| v));
                        html! {
                            <td>
                                <input name={ format!("{}_{}", label, i) }
                                       size=4
                                       value={
                                    node.credencies.as_ref()
                                        .map(|array| array[IxDyn(&idx)])
                                        .unwrap_or(0.0).to_string()
                                } />
                            </td>
                        }
                    })}
                    <td>
                        <textarea cols=20 rows=2 name={ format!("{}_description", label) }
                                  placeholder="Descripton for this row...">
                        </textarea>
                    </td>
                </tr>
            }
        } else {
            // HACK: the value for descriptions may not be properly updated otherwise
            js! {
                setTimeout(() => {
                    document.querySelector("textarea[name=prior_description]").value = @{ node.cred_description.get(0).map(|s| &s[..]).unwrap_or("") };
                }, 10);
            }
            html! {
                <tr>
                    <th>{ "Prior" }</th>
                    { for (0..node.values.len()).map(|i| {
                        html! {
                        <td>
                            <input name={ format!("prior_{}", i) } size=4 value={
                                node.credencies
                                    .as_ref()
                                    .map(|array| array[i])
                                    .unwrap_or(0.0)
                                    .to_string()
                            } />
                        </td>
                        }
                    })}
                    <td>
                        <textarea cols=20 rows=2 name="prior_description"
                                  placeholder="Descripton for this row...">
                        </textarea>
                    </td>
                </tr>
            }
        }
    }

    fn make_credencies_edit(&self, nodeid: usize) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        // one line in the table for all possible combination of parent values
        let values_iterator = node
            .parents
            .iter()
            .map(|&p| {
                let pnode = self.dag.get(p).unwrap();
                pnode
                    .values
                    .iter()
                    .enumerate()
                    .map(move |(i, v)| (p, &pnode.label, i, v))
            })
            .multi_cartesian_product()
            .enumerate();

        // prepare the metadata for the extraction function
        let mut shape = vec![node.values.len()];
        shape.extend(
            node.parents
                .iter()
                .map(|&p| self.dag.get(p).unwrap().values.len()),
        );
        let parents = node.parents.clone();

        let extract_credencies = move || {
            let (credencies, descriptions) = extract_credencies(&shape, &parents);
            Msg::UpdateCredencies {
                node: nodeid,
                credencies,
                descriptions,
            }
        };

        html! {
            <div>
            <table>
                <tr>
                    <th>
                    { if !node.parents.is_empty() { "Parent values" } else { "" } }
                    </th>
                    { for node.values.iter().map(|v| {
                        html! {
                            <th>{ format!("C({})", v) }</th>
                        }
                    })}
                    <th>{ "Explanation" }</th>
                </tr>
                { if node.parents.is_empty() { self.make_credencies_edit_line(nodeid, None) } else { html!{} }}
                { for values_iterator.map(|(iv, values)| self.make_credencies_edit_line(nodeid, Some((iv, values)))) }
            </table>
            <a href="#" onclick=move |_| extract_credencies()> { "Save credencies" }</a>
            </div>
        }
    }

    fn make_nodeedit_tab(&self, nodeid: usize) -> Html<Self> {
        html! {
            <div id="node-editor">
                { self.make_label_edit(nodeid) }
                { self.make_values_edit(nodeid) }
                { self.make_parents_edit(nodeid) }
                { self.make_node_description_edit(nodeid) }
                { self.make_credencies_edit(nodeid) }
            </div>
        }
    }

    fn make_observation_select(&self, id: usize, node: &crate::graph::Node) -> Html<Self> {
        html! {
            <select onchange=|v| if let ChangeData::Select(v) = v { Msg::SetObs { node: id, obs: v.raw_value().parse().ok() }} else { Msg::Ignore }>
                <option selected={ node.observation.is_none() } value=""></option>
                { for node.values.iter().enumerate().map(|(i,v)| {
                    html! { <option selected={ node.observation == Some(i) } value={ i }>{ v }</option> }
                })}
            </select>
        }
    }

    fn make_observation_tab(&self) -> Html<Self> {
        html! {
            <div id="node-editor">
                <p>{ "Observations for nodes:" }</p>
                <ul class="silentlist">
                    { for self.dag.iter_nodes().map(|(id, node)| {
                        html! {
                            <li>
                            { format!("Node \"{}\":", node.label) }
                            { self.make_observation_select(id, node) }
                            </li>
                        }
                    })}
                </ul>
            </div>
        }
    }

    fn make_belief_node(&self, nodeid: usize, beliefs: &LogProbVector) -> Html<Self> {
        let node = self.dag.get(nodeid).unwrap();
        if let Some(obs) = node.observation {
            html! {
                <li>
                    { format!("Node \"{}\" is observed to be: \"{}\"", node.label, node.values[obs]) }
                </li>
            }
        } else {
            let log10 = 10f32.ln();
            let log_beliefs = beliefs.log_probabilities();
            let iter = node.values.iter().zip(log_beliefs.iter());
            html! {
                <li>
                    <p>{ format!("Beliefs for node \"{}\":", node.label) }</p>
                    <ul class="posterior">
                        { for iter.map(|(name, belief)| {
                            html! {
                                <li>
                                    { format!("{}: {:.2}", name, belief / log10) }
                                </li>
                            }
                        })}
                    </ul>
                </li>
            }
        }
    }

    fn make_beliefs_tab(&self) -> Html<Self> {
        if let Some(ref results) = self.beliefs {
            html! {
                <div id="node-editor">
                    <p>{ "Results of the inference:" }</p>
                    <ul class="silentlist widelist">
                        { for results.iter().map(|&(ref beliefs, id)| {
                            self.make_belief_node(id, beliefs)
                        })}
                    </ul>
                </div>
            }
        } else {
            html! {
                <div id="node-editor">
                    <p>{ "Inference cannot be done if a node has no valid value." }</p>
                </div>
            }
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

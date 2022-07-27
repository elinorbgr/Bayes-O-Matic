use itertools::Itertools;
use ndarray::{ArrayD, IxDyn};
use wasm_bindgen::JsCast;
use web_sys::{
    window, Event, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement, InputEvent,
    KeyboardEvent,
};
use yew::{
    html,
    html::{Scope, TargetCast},
    Html,
};

use crate::{
    lang,
    model::{BayesOMatic, Msg},
};

pub fn fetch_input_and_clear(name: &str) -> String {
    let query = format!("input[name=\"{}\"]", name);
    let input = window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector(&query)
        .unwrap()
        .unwrap();
    let input: HtmlInputElement = input.dyn_into().unwrap();
    let value = input.value();
    input.set_value("");
    value
}

fn extract_credencies(shape: &[usize], parents: &[usize]) -> (ArrayD<f32>, Vec<String>) {
    let nval = shape[0];
    let count = shape.iter().product();
    let mut credencies = ArrayD::from_shape_vec(IxDyn(shape), vec![0.0; count]).unwrap();
    let mut descriptions = Vec::new();
    if shape.len() == 1 {
        // node has no parents
        for i in 0..nval {
            // get the credencies
            let query = format!("input[name=\"prior_{}\"]", i);
            let input = window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(&query)
                .unwrap()
                .unwrap();
            let input: HtmlInputElement = input.dyn_into().unwrap();
            let val = input.value().parse::<f32>().unwrap_or(0.0);
            credencies[i] = val;
        }
        // get the description for the row
        let query = "textarea[name=\"prior_description\"]";
        let texta = window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector(query)
            .unwrap()
            .unwrap();
        let texta: HtmlTextAreaElement = texta.dyn_into().unwrap();
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
                let input = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .query_selector(&query)
                    .unwrap()
                    .unwrap();
                let input: HtmlInputElement = input.dyn_into().unwrap();
                let val = input.value().parse::<f32>().unwrap_or(0.0);
                let mut idx = vec![i];
                idx.extend(values.iter().map(|(_, v)| v));
                credencies[IxDyn(&idx)] = val;
            }
            // get the description for the row
            let query = format!("textarea[name=\"{}_description\"]", label);
            let texta = window()
                .unwrap()
                .document()
                .unwrap()
                .query_selector(&query)
                .unwrap()
                .unwrap();
            let texta: HtmlTextAreaElement = texta.dyn_into().unwrap();
            descriptions.push(texta.value());
        }
    }

    (credencies, descriptions)
}

impl BayesOMatic {
    fn make_label_edit(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div class="field">
                <label class="label">{ lang!(self.lang, "node-name") }</label>
                <div class="control">
                <input class="input"
                        oninput={ link.callback(move |evt: InputEvent| Msg::SetLabel {
                            node: nodeid,
                            label: evt.target_dyn_into::<HtmlInputElement>().unwrap().value()
                        }) }
                        value={ node.label.clone() } />
                </div>
            </div>
        }
    }

    fn make_values_edit(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div class="field">
                <label class="label">{ lang!(self.lang, "node-values") }</label>
                <ul class="blocky vlist">
                    { for node.values.iter().enumerate().map(|(i,v)| {
                        html! {
                            <li>
                                { v } <a href="#" class="delete-button" onclick={ link.callback(move |_| Msg::DelValue { node: nodeid, value_id: i })}>{ "×" }</a>
                            </li>
                        }
                    })}
                </ul>
                <div class="control">
                <input placeholder={ lang!(self.lang, "add-value") }
                        class="input"
                        name="addvalue"
                        onkeypress={ link.callback(move |evt: KeyboardEvent| if evt.key() == "Enter" {
                            Msg::AddValue { node: nodeid, value: fetch_input_and_clear("addvalue") }
                        } else { Msg::Ignore }) } />
                    { format!("({})", lang!(self.lang, "press-enter")) }
                </div>
            </div>
        }
    }

    fn make_parent_seletor(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        html! {
            <select onchange={link.callback(move |e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                    Msg::AddParent { node: nodeid, parent_id: select.value().parse().unwrap() }
                } else {
                    Msg::Ignore
                })}>
                <option selected=true value="">{ format!("({})", lang!(self.lang, "add-parent")) }</option>
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

    fn make_parents_edit(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div class="field">
                <label class="label">{ lang!(self.lang, "node-parents") }</label>
                <ul class="blocky vlist">
                    { for node.parents.iter().map(|&p| {
                        let parent = self.dag.get(p).unwrap();
                        html! {
                            <li>{ &parent.label }<a href="#" class="delete-button" onclick={ link.callback(move |_| Msg::DelParent { node: nodeid, parent_id: p })}>{ "×" }</a></li>
                        }
                    })}
                </ul>
                <div class="control select">{ self.make_parent_seletor(nodeid, link) }</div>
            </div>
        }
    }

    fn make_node_description_edit(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div class="field">
                <label class="label">{ lang!(self.lang, "node-desc") }</label>
                <textarea class="textarea" cols=40 rows=4 placeholder={ lang!(self.lang, "write-desc") }
                          oninput={ link.callback(move |evt: InputEvent| Msg::SetDesc {
                              node: nodeid,
                              desc: evt.target_dyn_into::<HtmlTextAreaElement>().unwrap().value()
                          }) }
                          id="nodedesc"
                          value={ node.description.clone() }>
                </textarea>
            </div>
        }
    }

    #[allow(clippy::type_complexity)]
    fn make_credencies_edit_line(
        &self,
        nodeid: usize,
        target: Option<(usize, Vec<(usize, &String, usize, &String)>)>,
    ) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        if let Some((line_id, parent_values)) = target {
            let label = parent_values
                .iter()
                .map(|&(p, _, v, _)| format!("{}-{}", p, v))
                .join("_");
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
                                <input class="input"
                                       name={ format!("{}_{}", label, i) }
                                       size=2
                                       value={
                                    node.credencies.as_ref()
                                        .map(|array| array[IxDyn(&idx)])
                                        .unwrap_or(1.0).to_string()
                                } />
                            </td>
                        }
                    })}
                    <td>
                        <textarea class="textarea" cols=20 rows=1 name={ format!("{}_description", label) }
                                  placeholder={ lang!(self.lang, "row-desc") }
                                  value={ node.cred_description.get(line_id).cloned().unwrap_or_default() }>
                        </textarea>
                    </td>
                </tr>
            }
        } else {
            html! {
                <tr>
                    <th>{ "Prior" }</th>
                    { for (0..node.values.len()).map(|i| {
                        html! {
                        <td>
                            <input class="input" name={ format!("prior_{}", i) } size=2 value={
                                node.credencies
                                    .as_ref()
                                    .map(|array| array[i])
                                    .unwrap_or(1.0)
                                    .to_string()
                            } />
                        </td>
                        }
                    })}
                    <td>
                        <textarea class="textarea" cols=20 rows=1 name="prior_description"
                                  placeholder={ lang!(self.lang, "row-desc") }
                                  value={ node.cred_description.get(0).cloned().unwrap_or_default() }>
                        </textarea>
                    </td>
                </tr>
            }
        }
    }

    fn make_credencies_edit(&self, nodeid: usize, link: &Scope<Self>) -> Html {
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
            <table class="table">
                <tr>
                    <th>
                    { if !node.parents.is_empty() { lang!(self.lang, "parent-values") } else { "".into() } }
                    </th>
                    { for node.values.iter().map(|v| {
                        html! {
                            <th>{ format!("𝒫({})", v) }</th>
                        }
                    })}
                    <th>{ lang!(self.lang, "explanation") }</th>
                </tr>
                { if node.parents.is_empty() { self.make_credencies_edit_line(nodeid, None) } else { html!{} }}
                { for values_iterator.map(|(iv, values)| self.make_credencies_edit_line(nodeid, Some((iv, values)))) }
            </table>
            <a href="#" class="button" onclick={ link.callback(move |_| extract_credencies())}>{ lang!(self.lang, "save-credencies") }</a>
            </div>
        }
    }

    fn make_observation_select(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        html! {
            <div class="field">
                <label class="label">{ lang!(self.lang, "obs-for-node") }</label>
            <div class="control select">
                <select id="node-obs" onchange={ link.callback(move |e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                        Msg::SetObs { node: nodeid, obs: select.value().parse().ok() }
                    } else {
                        Msg::Ignore
                    })
                }>
                    <option selected={ node.observation.is_none() } value="none"></option>
                    { for node.values.iter().enumerate().map(|(i,v)| {
                        html! { <option selected={ node.observation == Some(i) } value={ i.to_string() }>{ v }</option> }
                    })}
                </select>
            </div>
            </div>
        }
    }

    pub fn make_nodeedit_tab(&self, nodeid: usize, link: &Scope<Self>) -> Html {
        html! {
            <div id="node-editor" class="box">
                <ul class="blocky">
                <li class="button" onclick={ link.callback(move |_| Msg::DuplicateNode(nodeid)) }>{ lang!(self.lang, "duplicate-node") }</li>
                <li class="button" onclick={ link.callback(move |_| Msg::RemoveNode(nodeid)) }>{ lang!(self.lang, "remove-node") }</li>
                </ul>
                { self.make_label_edit(nodeid, link) }
                { self.make_values_edit(nodeid, link) }
                { self.make_observation_select(nodeid, link) }
                { self.make_parents_edit(nodeid, link) }
                { self.make_node_description_edit(nodeid, link) }
                { self.make_credencies_edit(nodeid, link) }
            </div>
        }
    }
}

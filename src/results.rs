use loopybayesnet::LogProbVector;
use yew::{html, html::ChangeData, Html};

use crate::model::{BayesOMatic, Msg};

impl BayesOMatic {
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

    pub fn make_observation_tab(&self) -> Html<Self> {
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

    pub fn make_beliefs_tab(&self) -> Html<Self> {
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
}

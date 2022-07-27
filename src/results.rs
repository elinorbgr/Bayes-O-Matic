use loopybayesnet::LogProbVector;
use ndarray::ArrayView1;
use yew::{
    html,
    html::{Scope, TargetCast},
    Html,
};

use web_sys::{Event, HtmlSelectElement};

use crate::{
    lang,
    model::{BayesOMatic, BeliefsDisplay, Msg},
    Page,
};

fn log_sum_exp_vec(x: ArrayView1<f32>) -> f32 {
    let max_log = x.fold(std::f32::NEG_INFINITY, |old_max, &v| f32::max(old_max, v));
    if !max_log.is_finite() {
        // if max_log is +inf, result will be +inf anyway
        // if max_log is -inf, then all log values are -inf, and the result of the log_sum_exp is too
        max_log
    } else {
        max_log + x.mapv(|v| (v - max_log).exp()).sum().ln()
    }
}

impl BayesOMatic {
    fn make_belief_node(&self, nodeid: usize, beliefs: &LogProbVector) -> Html {
        let node = self.dag.get(nodeid).unwrap();
        if let Some(obs) = node.observation {
            html! {
                <div class="block">
                    <h3>{ lang!(self.lang, "node", name=&node.label[..]) }</h3>
                    <p>{ lang!(self.lang, "obs-as", value=&node.values[obs][..]) }</p>
                </div>
            }
        } else {
            let log_beliefs = beliefs.log_probabilities();
            if self.beliefs_display == BeliefsDisplay::OddsRatio {
                let logodds_iter =
                    node.values
                        .iter()
                        .zip(log_beliefs.iter().enumerate().map(|(i, &belief)| {
                            let mut all_beliefs = log_beliefs.to_owned();
                            all_beliefs[i] = std::f32::NEG_INFINITY;
                            let lse = log_sum_exp_vec(all_beliefs.view());
                            belief - lse
                        }));
                html! {
                    <div class="block">
                        <h3>{ lang!(self.lang, "node", name=&node.label[..]) }</h3>
                        <ul class="vlist blocky">
                            { for logodds_iter.map(|(name, belief)| {
                                html! {
                                    <li>
                                        { format!("{}: {:.2}", name, belief.exp()) }
                                    </li>
                                }
                            })}
                        </ul>
                    </div>
                }
            } else {
                let raw_iter = node.values.iter().zip(log_beliefs.iter());
                let min_log_belief = log_beliefs
                    .iter()
                    .copied()
                    .filter(|v| v.is_finite())
                    .fold(0.0, f32::min);
                html! {
                    <div class="block">
                        <h3>{ lang!(self.lang, "node", name=&node.label[..]) }</h3>
                        <ul class="vlist blocky">
                            { for raw_iter.map(|(name, belief)| {
                                if self.beliefs_display == BeliefsDisplay::Probabilities {
                                    html! {
                                        <li>
                                            { format!("{}: {:.1}%", name, belief.exp()*100.0) }
                                        </li>
                                    }
                                } else {
                                    html! {
                                        <li>
                                            { format!("{}: {:.2}", name, (belief - min_log_belief).exp()) }
                                        </li>
                                    }
                                }
                            })}
                        </ul>
                    </div>
                }
            }
        }
    }

    pub fn make_beliefs_tab(&self, link: &Scope<Self>) -> Html {
        if let Some(ref results) = self.beliefs {
            html! {
                <div id="node-editor" class="box content">
                    <h2>{ lang!(self.lang, "inference-results") }</h2>
                    <div class="field">
                    <label class="label">{ lang!(self.lang, "result-format") }</label>
                    <div class="control select">
                    <select onchange={ link.callback(|e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                                Msg::SetBeliefsDisplay(BeliefsDisplay::from_str(&select.value()).unwrap())
                            } else {
                                Msg::Ignore
                            }
                        ) }>
                        <option selected={ self.beliefs_display == BeliefsDisplay::RawBeliefs }
                                value="raw-beliefs">{ lang!(self.lang, "raw-beliefs") }</option>
                        <option selected={ self.beliefs_display == BeliefsDisplay::OddsRatio }
                                value="odds-ratios">{ lang!(self.lang, "odds-ratios") }</option>
                        <option selected={ self.beliefs_display == BeliefsDisplay::Probabilities }
                                value="probabilities">{ lang!(self.lang, "probabilities") }</option>
                    </select>
                    </div>
                    </div>
                    { for results.iter().map(|&(ref beliefs, id)| {
                        self.make_belief_node(id, beliefs)
                    })}
                </div>
            }
        } else {
            html! {
                <div id="node-editor" class="box content">
                    <p>{ lang!(self.lang, "inference-no-value") }</p>
                </div>
            }
        }
    }

    pub fn make_mutualinfo_tab(&self, link: &Scope<Self>) -> Html {
        if let Some(ref results) = self.mutual_info {
            html! {
                <div id="node-editor" class="box content">
                    <h2> { lang!(self.lang, "mutual-info-result") }</h2>
                    <div class="field">
                    <label class="label">{ lang!(self.lang, "target-node") }</label>
                    <div class="control select">
                    <select onchange={ link.callback(|e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                            Msg::MoveToPage(Page::MutualInformation(Some(select.value().parse().unwrap())))
                        } else {
                            Msg::Ignore
                        }
                    )}>
                    { for self.dag.iter_nodes().filter(|&(_, node)| node.observation.is_none()).map(|(id, node)| {
                        html! {
                            <option selected={ self.page == Page::MutualInformation(Some(id)) } value={ format!("{}", id) }>{ &node.label }</option>
                        }
                    }) }
                    </select>
                    </div>
                    </div>
                    { for results.iter().map(|&(id, mi)| {
                        html! {
                            <div class="block">{ format!("{} {:.2} bits", lang!(self.lang, "with-node", name=&self.dag.get(id).unwrap().label[..]), mi) }</div>
                        }
                    })}
                </div>
            }
        } else {
            html! {
                <div id="node-editor" class="box">
                    <p>{ lang!(self.lang, "mi-no-value") }</p>
                </div>
            }
        }
    }
}

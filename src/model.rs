use failure::Error;
use loopybayesnet::LogProbVector;
use ndarray::ArrayD;
use stdweb::{console, web::document};
use yew::{
    format::Nothing,
    services::fetch::{FetchService, FetchTask, Request, Response},
    Component, ComponentLink, ShouldRender,
};

use crate::{
    graph::{DeserError, DAG},
    i18n::Lang,
    lang, Page,
};

#[derive(Clone, Debug)]
pub enum Msg {
    Ignore,
    AddNode,
    DuplicateNode(usize),
    RemoveNode(usize),
    SetLabel {
        node: usize,
        label: String,
    },
    AddValue {
        node: usize,
        value: String,
    },
    DelValue {
        node: usize,
        value_id: usize,
    },
    AddParent {
        node: usize,
        parent_id: usize,
    },
    DelParent {
        node: usize,
        parent_id: usize,
    },
    SetDesc {
        node: usize,
        desc: String,
    },
    SetObs {
        node: usize,
        obs: Option<usize>,
    },
    UpdateCredencies {
        node: usize,
        credencies: ArrayD<f32>,
        descriptions: Vec<String>,
    },
    MoveToPage(Page),
    Reset,
    LoadJson(String),
    LoadExample(String),
    ShowHelp(String),
    SetLogOdds(bool),
    SetLang(String),
}

pub struct BayesOMatic {
    pub(crate) dag: DAG,
    pub(crate) page: Page,
    pub(crate) load_error: Option<DeserError>,
    fetch_service: FetchService,
    task: Option<FetchTask>,
    link: ComponentLink<BayesOMatic>,
    pub(crate) beliefs: Option<Vec<(LogProbVector, usize)>>,
    pub(crate) mutual_info: Option<Vec<(usize, f32)>>,
    pub(crate) logodds: bool,
    pub help_contents: Option<String>,
    pub(crate) lang: Lang,
}

impl BayesOMatic {
    fn compute_beliefs(&self) -> Option<Vec<(LogProbVector, usize)>> {
        let (mut bayesnet, mapping) = match self.dag.make_bayesnet() {
            Ok(v) => v,
            Err(()) => {
                // beliefs cannnot be computed,
                return None;
            }
        };

        for _ in 0..(self.dag.estimate_iteration_number()) {
            bayesnet.step();
        }
        let mut beliefs = bayesnet.beliefs();

        for b in &mut beliefs {
            b.renormalize();
        }

        Some(beliefs.into_iter().zip(mapping.into_iter()).collect())
    }

    fn compute_mutual_info(&mut self, id: usize) -> Option<Vec<(usize, f32)>> {
        if self.dag.get(id).unwrap().observation.is_some() {
            return None;
        }
        let non_observed_nodes: Vec<_> = self
            .dag
            .iter_nodes()
            .filter(|&(_, node)| node.observation.is_none())
            .map(|(id, _)| id)
            .collect();
        // retreive the base beliefs
        let mut base_belief: Vec<_> = self
            .compute_beliefs()
            .unwrap()
            .into_iter()
            .filter(|&(_, id)| non_observed_nodes.contains(&id))
            .collect();
        base_belief.sort_by_key(|&(_, id)| id);

        // compute the beliefs for all possible value of current node
        let values_count = self.dag.get(id).unwrap().values.len();
        let mut kl_tems = Vec::new();
        for i in 0..values_count {
            self.dag.set_observation(id, Some(i));
            let mut belief: Vec<_> = self
                .compute_beliefs()
                .unwrap()
                .into_iter()
                .filter(|&(_, id)| non_observed_nodes.contains(&id))
                .collect();
            belief.sort_by_key(|&(_, id)| id);
            let kl_term: Vec<_> = belief
                .iter()
                .zip(base_belief.iter())
                .map(|((cond_belief, _), (base_belief, _))| {
                    let log_ratio = (&cond_belief.log_probabilities()
                        - &base_belief.log_probabilities())
                        * cond_belief.as_probabilities();
                    log_ratio.sum()
                })
                .collect();
            kl_tems.push(kl_term);
        }
        self.dag.set_observation(id, None);
        // conditional_beliefs contains a vec of KL(P(Y|x) || P(Y))
        // first dimension runs accross the values of x, second dimension accross the nodes Y
        // we need to multiply by P(X) & sum accross the first dimension
        let pxs = base_belief
            .iter()
            .find(|&(_, nid)| *nid == id)
            .unwrap()
            .0
            .as_probabilities();
        let kls = kl_tems.into_iter().zip(pxs.into_iter()).fold(
            vec![0f32; non_observed_nodes.len()],
            |mut acc, (kl_term, &px)| {
                for (a, kl) in acc.iter_mut().zip(kl_term.iter()) {
                    if px > 0.0001 {
                        // numerical stability px=0 should crush a log's infinity
                        // so if px is too close to 0 we just stip this term
                        *a += kl * px / 2f32.ln();
                    }
                }
                acc
            },
        );

        Some(
            kls.into_iter()
                .zip(base_belief.iter())
                // if kl gets < 0.0 it is a numerical instability issue, just clamp it to 0
                .map(|(kl, &(_, id))| (id, if kl < 0.0 { 0.0 } else { kl }))
                .filter(|&(nid, _)| nid != id)
                .collect(),
        )
    }

    fn load_help(&mut self) {
        let location = document().location().unwrap();
        let origin = location.origin().unwrap();
        let pathname = location.pathname().unwrap();
        let url = format!("{}{}/help/{}.md", origin, pathname, self.lang.name);
        let callback = self
            .link
            .send_back(move |response: Response<Result<String, Error>>| {
                let (_, data) = response.into_parts();
                match data {
                    Ok(data) => {
                        console!(log, "Loading help.");
                        Msg::ShowHelp(data)
                    }
                    Err(e) => {
                        console!(log, format!("Failed to load help: {}", e));
                        Msg::Ignore
                    }
                }
            });
        let request = Request::get(url).body(Nothing).unwrap();
        self.task = Some(self.fetch_service.fetch(request, callback));
    }

    fn load_example(&mut self, name: String) {
        let location = document().location().unwrap();
        let origin = location.origin().unwrap();
        let pathname = location.pathname().unwrap();
        let url = format!(
            "{}{}/examples/{}/{}.json",
            origin, pathname, self.lang.name, name
        );
        console!(log, format!("Fetching example {}.", url));
        let callback = self
            .link
            .send_back(move |response: Response<Result<String, Error>>| {
                let (_, data) = response.into_parts();
                match data {
                    Ok(data) => {
                        console!(log, format!("Loading example {}.", name));
                        Msg::LoadJson(data)
                    }
                    Err(e) => {
                        console!(log, format!("Failed to load example {}: {}", name, e));
                        Msg::Ignore
                    }
                }
            });
        let request = Request::get(&url).body(Nothing).unwrap();
        self.task = Some(self.fetch_service.fetch(request, callback));
    }
}

impl Component for BayesOMatic {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        BayesOMatic {
            dag: DAG::new(),
            page: Page::Idle,
            load_error: None,
            fetch_service: FetchService::new(),
            task: None,
            link,
            beliefs: None,
            mutual_info: None,
            logodds: true,
            help_contents: None,
            lang: Lang::load("en").unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut redraw = true;
        match msg {
            Msg::Ignore => {}
            Msg::AddNode => {
                let id = self.dag.insert_node();
                self.dag
                    .set_label(id, lang!(self.lang, "default-node-name", id = id));
                self.page = Page::NodeEdit(id);
            }
            Msg::DuplicateNode(id) => {
                let new_id = self.dag.duplicate_node(id);
                self.page = Page::NodeEdit(new_id.unwrap());
            }
            Msg::RemoveNode(id) => {
                self.dag.remove_node(id);
                self.page = Page::Idle;
            }
            Msg::SetLabel { node, label } => {
                self.dag.set_label(node, label);
            }
            Msg::AddValue { node, value } => {
                self.dag.add_value(node, value);
            }
            Msg::DelValue { node, value_id } => {
                self.dag.remove_value(node, value_id);
            }
            Msg::AddParent { node, parent_id } => {
                self.dag.add_edge(node, parent_id).unwrap();
            }
            Msg::DelParent { node, parent_id } => {
                self.dag.remove_edge(node, parent_id);
            }
            Msg::SetDesc { node, desc } => {
                self.dag.set_description(node, desc);
                redraw = false;
            }
            Msg::SetObs { node, obs } => {
                self.dag.set_observation(node, obs);
            }
            Msg::UpdateCredencies {
                node,
                credencies,
                descriptions,
            } => {
                self.dag.set_credencies(node, credencies).unwrap();
                self.dag.set_cred_descriptions(node, descriptions).unwrap();
                redraw = false;
            }
            Msg::MoveToPage(page) => {
                if page == Page::ComputeBeliefs {
                    self.beliefs = self.compute_beliefs();
                } else if page == Page::Help {
                    if self.help_contents.is_none() {
                        self.load_help();
                    }
                } else if let Page::MutualInformation(id) = page {
                    if let Some(id) = id {
                        self.mutual_info = self.compute_mutual_info(id);
                    } else {
                        let id = self
                            .dag
                            .iter_nodes()
                            .filter(|&(_, node)| node.observation.is_none())
                            .map(|(id, _)| id)
                            .next();
                        if let Some(id) = id {
                            self.mutual_info = self.compute_mutual_info(id);
                        } else {
                            self.mutual_info = None;
                        }
                    }
                }
                self.page = page;
                self.load_error = None;
            }
            Msg::Reset => {
                self.dag = DAG::new();
                self.load_error = None;
                self.page = Page::Idle;
            }
            Msg::LoadJson(json) => match DAG::from_json(&json) {
                Ok(dag) => {
                    self.dag = dag;
                    self.page = Page::Idle;
                    self.load_error = None;
                }
                Err(e) => {
                    self.load_error = Some(e);
                }
            },
            Msg::LoadExample(name) => {
                self.load_example(name);
                // only redraw when loading is finished
                redraw = false;
            }
            Msg::ShowHelp(help_contents) => {
                self.help_contents = Some(help_contents);
            }
            Msg::SetLogOdds(logodds) => {
                self.logodds = logodds;
            }
            Msg::SetLang(lang) => {
                self.lang = Lang::load(&lang).unwrap();
                // Invalidate the help & reload if relevant
                self.help_contents = None;
                if self.page == Page::Help {
                    self.load_help();
                }
            }
        }

        redraw
    }
}

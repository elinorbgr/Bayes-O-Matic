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
    pub(crate) logodds: bool,
    pub help_contents: Option<String>,
    pub(crate) lang: Lang,
}

impl BayesOMatic {
    fn compute_beliefs(&mut self) {
        let (mut bayesnet, mapping) = match self.dag.make_bayesnet() {
            Ok(v) => v,
            Err(()) => {
                // beliefs cannnot be computed,
                self.beliefs = None;
                return;
            }
        };

        for _ in 0..100 {
            bayesnet.step();
        }
        let mut beliefs = bayesnet.beliefs();

        for b in &mut beliefs {
            b.renormalize();
        }

        self.beliefs = Some(beliefs.into_iter().zip(mapping.into_iter()).collect());
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
                    self.compute_beliefs();
                } else if page == Page::Help {
                    if self.help_contents.is_none() {
                        self.load_help();
                        // only redraw when help is loaded
                        redraw = false;
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
            }
        }

        redraw
    }
}

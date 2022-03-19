#![recursion_limit = "256"]

mod draw;
mod editor;
mod graph;
mod i18n;
mod js;
mod markdown;
mod model;
mod render;
mod results;
mod ui;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Page {
    Idle,
    NodeEdit(usize),
    ComputeBeliefs,
    MutualInformation(Option<usize>),
    ExportJson,
    LoadJson,
    LoadExample,
    Help,
}

fn main() {
    yew::start_app::<model::BayesOMatic>();
}

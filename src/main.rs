#![recursion_limit = "512"]

mod draw;
mod editor;
mod graph;
mod i18n;
mod markdown;
mod model;
mod render;
mod results;
mod ui;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Page {
    Idle,
    NodeEdit(usize),
    SetObservations,
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

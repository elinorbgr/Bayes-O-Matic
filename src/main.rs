#![recursion_limit = "512"]

mod draw;
mod editor;
mod graph;
mod markdown;
mod model;
mod render;
mod results;
mod ui;

const EXAMPLE_LIST: &[&str] = &["insect_bite", "rain", "flat_earth"];

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Page {
    Idle,
    NodeEdit(usize),
    SetObservations,
    ComputeBeliefs,
    ExportJson,
    LoadJson,
    LoadExample,
    Help,
}

fn main() {
    yew::start_app::<model::BayesOMatic>();
}

#![recursion_limit = "512"]

mod draw;
mod graph;
mod markdown;
mod model;
mod render;

const EXAMPLE_LIST: &[&str] = &["insect_bite", "rain", "flat_earth"];

fn main() {
    yew::start_app::<model::BayesOMatic>();
}

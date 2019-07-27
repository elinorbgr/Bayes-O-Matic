#![recursion_limit = "256"]

#[macro_use]
extern crate stdweb;

use std::{cell::RefCell, rc::Rc};

use stdweb::{
    traits::*,
    web::{document, event::ClickEvent},
};

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

mod draw;
mod editor;
mod graph;
mod handlers;
mod utils;

type State = Rc<RefCell<graph::DAG>>;

fn main() {
    stdweb::initialize();

    let state = Rc::new(RefCell::new(graph::DAG::new()));

    /*
     * setup the main buttons
     */

    document()
        .query_selector("#btn-reset")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                handlers::reset_graph(&state);
        }));

    document()
        .query_selector("#btn-export")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                handlers::export_to_json(&state);
        }));

    document()
        .query_selector("#btn-import")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                handlers::load_from_json(&state);
        }));

    document()
        .query_selector("#btn-examples")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                handlers::select_example(&state);
        }));

    /*
     * setup the editor
     */

    document()
        .query_selector("#btn-addnode")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                editor::add_node(&state);
        }));

    document()
        .query_selector("#btn-observations")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                editor::set_evidence_tab(&state);
        }));

    document()
        .query_selector("#btn-compute")
        .unwrap()
        .unwrap()
        .add_event_listener(enclose!( (state) move |_: ClickEvent| {
                editor::compute_evidences(&state);
        }));
}

use crate::State;
use stdweb::{
    traits::*,
    web::{document, event::ClickEvent, HtmlElement},
};

pub fn reset_graph(state: &State) {
    state.borrow_mut().reset();
    // Clear the node list in the editor
    let list = document().query_selector("#node-list").unwrap().unwrap();
    while let Some(node) = list.first_child() {
        let _ = list.remove_child(&node);
    }
    // Clear the drawing board
    crate::draw::redraw_graph(state);
    // clear the panel
    let panel = document().query_selector("#node-editor").unwrap().unwrap();
    while let Some(node) = panel.first_child() {
        let _ = panel.remove_child(&node);
    }
    let p = document().create_element("p").unwrap();
    p.append_child(&document().create_text_node("Select a node to edit..."));
    panel.append_child(&p);
}

pub fn load_from_json(state: &State) {
    // TODO
}

pub fn export_to_json(state: &State) {
    // TODO
}

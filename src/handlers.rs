use crate::State;
use stdweb::{traits::*, web::document};

pub fn reset_graph(state: &State) {
    state.borrow_mut().reset();
    // Clear the node list in the editor
    let list = document().query_selector("#node-list").unwrap().unwrap();
    crate::utils::clear_children(&list);
    // Clear the drawing board
    crate::draw::redraw_graph(state);
    // clear the buttons
    crate::utils::clear_buttons();
    // clear the panel
    let panel = document().query_selector("#node-editor").unwrap().unwrap();
    crate::utils::clear_children(&panel);
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

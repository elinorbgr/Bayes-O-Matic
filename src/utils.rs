use stdweb::{
    traits::*,
    unstable::TryFrom,
    web::{document, HtmlElement},
};

pub fn set_selected_button(query: &str) {
    clear_buttons();
    let _ = document()
        .query_selector(query)
        .unwrap()
        .unwrap()
        .class_list()
        .add("selected");
}

pub fn clear_buttons() {
    let _ = document()
        .query_selector("#btn-observations")
        .unwrap()
        .unwrap()
        .class_list()
        .remove("selected");
    let _ = document()
        .query_selector("#btn-compute")
        .unwrap()
        .unwrap()
        .class_list()
        .remove("selected");

    let list = document().query_selector("#node-list").unwrap().unwrap();
    for node in list.child_nodes() {
        let _ = HtmlElement::try_from(node.first_child().unwrap())
            .unwrap()
            .class_list()
            .remove("selected");
    }
}

pub fn clear_children<I: INode>(parent: &I) {
    while let Some(node) = parent.first_child() {
        let _ = parent.remove_child(&node);
    }
}

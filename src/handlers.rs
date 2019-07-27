use crate::{
    graph::{DeserError, EdgeError, DAG},
    State,
};
use stdweb::{
    traits::*,
    unstable::TryInto,
    web::{
        document,
        event::{ClickEvent, ProgressLoadEvent},
        html_element::TextAreaElement,
        XmlHttpRequest,
    },
};

pub fn reset_graph(state: &State) {
    hide_popup();
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

pub fn load_json_into_state(state: &State, json: &str) -> Result<(), DeserError> {
    let mut dag = DAG::from_json(json)?;
    reset_graph(&state);
    *(state.borrow_mut()) = dag;
    // refill the node list
    let list = document().query_selector("#node-list").unwrap().unwrap();
    for (i, node) in state.borrow().iter_nodes() {
        let li = document().create_element("li").unwrap();
        let a = document().create_element("a").unwrap();
        a.class_list().add(&format!("node-{}", i)).unwrap();
        a.append_child(&document().create_text_node(&node.label));
        a.set_attribute("href", "#").unwrap();
        a.add_event_listener(enclose!( (state, i) move |_: ClickEvent| {
                crate::editor::node_edit_tab(&state, i);
        }));
        li.append_child(&a);
        list.append_child(&li);
    }
    crate::draw::redraw_graph(&state);
    Ok(())
}

pub fn select_example(state: &State) {
    const EXAMPLE_LIST: &[&str] = &["insect_bite", "rain", "flat_earth"];

    let popup = document().query_selector("#saveload").unwrap().unwrap();
    crate::utils::clear_children(&popup);

    let list = document().create_element("ul").unwrap();

    for example in EXAMPLE_LIST {
        let li = document().create_element("li").unwrap();
        let p = document().create_element("p").unwrap();
        let a = document().create_element("a").unwrap();
        a.set_attribute("href", "#");
        a.append_child(&document().create_text_node(example));
        a.add_event_listener(enclose!( (state, example) move |_: ClickEvent| {
            load_example(&state, example);
        }));
        p.append_child(&a);
        li.append_child(&p);
        list.append_child(&li);
    }

    popup.append_child(&list);
    show_popup();
}

fn load_example(state: &State, example: &str) {
    let req = XmlHttpRequest::new();
    req.open("GET", &format!("examples/{}.json", example));
    req.add_event_listener(enclose!((state, req) move |_: ProgressLoadEvent| {
        let json_data = req.response_text().unwrap().unwrap();
        load_json_into_state(&state, &json_data).unwrap();
    }));
    req.send();
}

pub fn load_from_json(state: &State) {
    let popup = document().query_selector("#saveload").unwrap().unwrap();
    crate::utils::clear_children(&popup);

    let textarea: TextAreaElement = document()
        .create_element("textarea")
        .unwrap()
        .try_into()
        .unwrap();
    textarea.set_attribute("cols", "110");
    textarea.set_attribute("rows", "20");

    let result_p = document().create_element("p").unwrap();

    let close_btn = document().create_element("a").unwrap();
    close_btn.append_child(&document().create_text_node("Close"));
    close_btn.set_attribute("href", "#").unwrap();
    close_btn.add_event_listener(|_: ClickEvent| {
        hide_popup();
    });

    let submit_btn = document().create_element("a").unwrap();
    submit_btn.append_child(&document().create_text_node("Submit"));
    submit_btn.set_attribute("href", "#").unwrap();
    submit_btn.add_event_listener(enclose!( (state, textarea, result_p) move |_: ClickEvent| {
        // try to load the values
        let json = textarea.value();
        if let Err(error) = load_json_into_state(&state, &json) {
            // display the error
            crate::utils::clear_children(&result_p);
            match error {
                DeserError::Json(e) => {
                    result_p.append_child(&document().create_text_node(&format!("The provided input is not valid JSON: {}", e)));
                }
                DeserError::Graph(EdgeError::WouldCycle) => {
                    result_p.append_child(&document().create_text_node("The input graph cannot be loaded as it contains a cycle."));
                }
                DeserError::Graph(EdgeError::BadNode) => {
                    result_p.append_child(&document().create_text_node("The input graph cannot be loaded as it contains references to non-existing nodes."));
                }
                DeserError::Graph(EdgeError::AlreadyExisting) => {
                    result_p.append_child(&document().create_text_node("The input graph cannot be loaded as it contains duplicate edges."));
                }
            }
        }
    }));

    popup.append_child(&textarea);
    popup.append_child(&document().create_element("br").unwrap());
    popup.append_child(&result_p);
    popup.append_child(&document().create_element("br").unwrap());
    popup.append_child(&submit_btn);
    popup.append_child(&close_btn);

    show_popup();
}

pub fn export_to_json(state: &State) {
    let json = state.borrow().to_json();

    let popup = document().query_selector("#saveload").unwrap().unwrap();
    crate::utils::clear_children(&popup);

    let close_btn = document().create_element("a").unwrap();
    close_btn.append_child(&document().create_text_node("Close"));
    close_btn.set_attribute("href", "#").unwrap();
    close_btn.add_event_listener(|_: ClickEvent| {
        hide_popup();
    });

    let textarea: TextAreaElement = document()
        .create_element("textarea")
        .unwrap()
        .try_into()
        .unwrap();
    textarea.set_value(&json);
    textarea.set_attribute("cols", "110");
    textarea.set_attribute("rows", "20");
    textarea.set_attribute("readonly", "");

    popup.append_child(&textarea);
    popup.append_child(&document().create_element("br").unwrap());
    popup.append_child(&close_btn);

    show_popup();
}

fn show_popup() {
    let content = document().query_selector("#content").unwrap().unwrap();
    content.class_list().add("hidden").unwrap();
    content.class_list().remove("flex").unwrap();
    let popup = document().query_selector("#saveload").unwrap().unwrap();
    popup.class_list().remove("hidden").unwrap();
}

fn hide_popup() {
    let popup = document().query_selector("#saveload").unwrap().unwrap();
    popup.class_list().add("hidden").unwrap();
    let content = document().query_selector("#content").unwrap().unwrap();
    content.class_list().add("flex").unwrap();
    content.class_list().remove("hidden").unwrap();
}

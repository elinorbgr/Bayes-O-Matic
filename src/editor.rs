use stdweb::{
    traits::*,
    web::{document, event::ClickEvent},
};

use crate::State;

pub fn add_node(state: &State) {
    let node_id = state.borrow_mut().insert_node();

    let list = document().query_selector("#node-list").unwrap().unwrap();

    let li = document().create_element("li").unwrap();
    let a = document().create_element("a").unwrap();

    a.class_list().add(&format!("node-{}", node_id)).unwrap();
    a.append_child(&document().create_text_node(&format!("Node #{}", node_id)));
    a.set_attribute("href", "#").unwrap();

    a.add_event_listener(enclose!( (state, node_id) move |_: ClickEvent| {
            node_edit_tab(&state, node_id);
    }));

    li.append_child(&a);
    list.append_child(&li);
    crate::draw::redraw_graph(&state);
    // switch the editor to the newly created node
    node_edit_tab(state, node_id);
}

pub fn node_edit_tab(state: &State, id: usize) {
    crate::utils::set_selected_button(&format!(".node-{}", id));
    // TODO
}

pub fn set_evidence_tab(state: &State) {
    crate::utils::set_selected_button("#btn-observations");
    // TODO
}

pub fn compute_evidences(state: &State) {
    crate::utils::set_selected_button("#btn-compute");
    let panel = document().query_selector("#node-editor").unwrap().unwrap();
    crate::utils::clear_children(&panel);
    // Compute the evidence
    let (mut bayesnet, mapping) = match state.borrow().make_bayesnet() {
        Ok(v) => v,
        Err(()) => {
            let p = document().create_element("p").unwrap();
            p.append_child(&document().create_text_node(
                "Inference cannot be performed if a node has 0 possible values.",
            ));
            panel.append_child(&p);
            return;
        }
    };

    for _ in 0..10 {
        bayesnet.step();
    }
    let beliefs = bayesnet.beliefs();

    // Display the output
    let log10 = 10f32.ln();

    let p = document().create_element("p").unwrap();
    p.append_child(&document().create_text_node("Results of the inference:"));
    panel.append_child(&p);
    let ul = document().create_element("ul").unwrap();
    for (i, credencies) in beliefs.iter().enumerate() {
        let li = document().create_element("li").unwrap();
        let inner_ul = document().create_element("ul").unwrap();

        let state = state.borrow();
        let node = state.get(mapping[i]).unwrap();

        for (name, belief) in node
            .values
            .iter()
            .zip(credencies.log_probabilities().iter())
        {
            let inner_li = document().create_element("li").unwrap();
            inner_li.append_child(&document().create_text_node(&format!(
                "{}: {:.2}",
                name,
                belief * log10
            )));
            inner_ul.append_child(&inner_li);
        }

        let p = document().create_element("p").unwrap();
        p.append_child(&document().create_text_node(&format!("Beliefs for node {}:", node.label)));
        li.append_child(&p);
        li.append_child(&inner_ul);
        ul.append_child(&li);
    }

    panel.append_child(&ul);

    // Redraw the graph
    crate::draw::draw_computed_graph(state, &mapping, &beliefs);
}

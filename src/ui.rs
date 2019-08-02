use stdweb::{
    traits::*,
    web::{document, event::ClickEvent, Element},
};

use crate::State;

pub struct Panel {
    elem: Element,
}

impl Panel {
    pub fn get() -> Panel {
        let elem = document().query_selector("#node-editor").unwrap().unwrap();
        Panel { elem }
    }

    pub fn clear(&self) {
        crate::utils::clear_children(&self.elem);
    }

    pub fn element(&self) -> &Element {
        &self.elem
    }
}

pub struct Popup {
    elem: Element,
}

impl Popup {
    pub fn get() -> Popup {
        let elem = document().query_selector("#saveload").unwrap().unwrap();
        Popup { elem }
    }

    pub fn clear(&self) {
        crate::utils::clear_children(&self.elem);
    }

    pub fn element(&self) -> &Element {
        &self.elem
    }

    pub fn show(&self) {
        let content = document().query_selector("#content").unwrap().unwrap();
        content.class_list().add("hidden").unwrap();
        content.class_list().remove("flex").unwrap();
        self.elem.class_list().remove("hidden").unwrap();
    }

    pub fn hide(&self) {
        self.elem.class_list().add("hidden").unwrap();
        let content = document().query_selector("#content").unwrap().unwrap();
        content.class_list().add("flex").unwrap();
        content.class_list().remove("hidden").unwrap();
    }

    pub fn add_close_button(&self) {
        let close_btn = document().create_element("a").unwrap();
        close_btn.append_child(&document().create_text_node("Close"));
        close_btn.set_attribute("href", "#").unwrap();
        close_btn.add_event_listener(|_: ClickEvent| {
            Popup::get().hide();
        });
        self.elem.append_child(&close_btn);
    }
}

pub struct NodeList {
    elem: Element,
}

impl NodeList {
    pub fn get() -> NodeList {
        let elem = document().query_selector("#node-list").unwrap().unwrap();
        NodeList { elem }
    }

    pub fn clear(&self) {
        crate::utils::clear_children(&self.elem);
    }

    pub fn add_node(&self, node_id: usize, state: &State, name: Option<&str>) {
        let li = document().create_element("li").unwrap();
        let a = document().create_element("a").unwrap();

        a.class_list().add(&format!("node-{}", node_id)).unwrap();
        if let Some(name) = name {
            a.append_child(&document().create_text_node(name));
        } else {
            a.append_child(&document().create_text_node(&format!("Node #{}", node_id)));
        }
        a.set_attribute("href", "#").unwrap();

        a.add_event_listener(enclose!( (state, node_id) move |_: ClickEvent| {
                crate::editor::node_edit_tab(&state, node_id);
        }));

        li.append_child(&a);
        self.elem.append_child(&li);
    }
}

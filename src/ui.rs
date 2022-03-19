use yew::{function_component, html, Callback, Properties};

/*
 * A button for selecting a page that has a "selected" state
 */

#[derive(PartialEq, Properties)]
pub struct PushButtonProps {
    pub text: String,
    #[prop_or(false)]
    pub selected: bool,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub title: Option<String>,
}

#[function_component(PushButton)]
pub fn push_button(props: &PushButtonProps) -> Html {
    let onclick = props.onclick.clone();
    if let Some(ref title) = props.title {
        html! {
            <a href="#"
                onclick={ move |_| onclick.emit(()) }
                title={ title.clone() }
                class={ if props.selected { "selected" } else { "" }}
                > { &props.text }</a>
        }
    } else {
        html! {
            <a href="#"
            onclick={ move |_| onclick.emit(()) }
                class={ if props.selected { "selected" } else { "" }}
                > { &props.text }</a>
        }
    }
}

use yew::{html, Callback, Component, ComponentLink, Html, Properties, Renderable, ShouldRender};

/*
 * A button for selecting a page that has a "selected" state
 */

pub struct PushButton {
    text: String,
    selected: bool,
    onclick: Callback<()>,
}

#[derive(Properties)]
pub struct PushButtonProps {
    pub text: String,
    pub selected: bool,
    #[props(required)]
    pub onclick: Callback<()>,
}

impl Component for PushButton {
    type Message = ();
    type Properties = PushButtonProps;

    fn create(props: PushButtonProps, _: ComponentLink<Self>) -> Self {
        PushButton {
            text: props.text,
            selected: props.selected,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, _msg: ()) -> ShouldRender {
        self.onclick.emit(());
        false
    }

    fn change(&mut self, props: PushButtonProps) -> ShouldRender {
        self.text = props.text;
        self.selected = props.selected;
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<PushButton> for PushButton {
    fn view(&self) -> Html<PushButton> {
        html! {
            <a href="#"
               onclick=|_| ()
               class={ if self.selected { "selected" } else { "" }}
               > { &self.text }</a>
        }
    }
}

use yew::{html, Callback, Component, Context, Html, Properties};

/*
 * A button for selecting a page that has a "selected" state
 */

pub struct PushButton;

#[derive(PartialEq, Properties)]
pub struct PushButtonProps {
    pub text: String,
    #[prop_or(false)]
    pub selected: bool,
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub title: Option<String>,
}

impl Component for PushButton {
    type Message = ();
    type Properties = PushButtonProps;

    fn create(_: &Context<Self>) -> Self {
        PushButton
    }

    fn update(&mut self, ctx: &Context<Self>, _msg: ()) -> bool {
        ctx.props().onclick.emit(());
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();
        if let Some(ref title) = props.title {
            html! {
                <a href="#"
                   onclick={ link.callback(|_| ()) }
                   title={ title.clone() }
                   class={ if props.selected { "selected" } else { "" }}
                   > { &props.text }</a>
            }
        } else {
            html! {
                <a href="#"
                   onclick={ link.callback(|_| ()) }
                   class={ if props.selected { "selected" } else { "" }}
                   > { &props.text }</a>
            }
        }
    }
}

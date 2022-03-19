use yew::html::{Scope, TargetCast};
use yew::{html, Html};

use web_sys::{Event, HtmlInputElement, HtmlSelectElement};

use crate::draw::DotCanvas;
use crate::graph::{DeserError, EdgeError};
use crate::lang;
use crate::model::{BayesOMatic, Msg};
use crate::ui::PushButton;
use crate::Page;

impl BayesOMatic {
    pub fn topbar(&self, link: &Scope<Self>) -> Html {
        html! {
            <div id="menu">
            <ul class="blocky">
                <li><PushButton text={ lang!(self.lang, "reset") } onclick={ link.callback(|_| Msg::Reset) } /></li>
                <li><PushButton text={ lang!(self.lang, "export-json") }onclick={ link.callback(|_| Msg::Export) } /></li>
                <li><PushButton text={ lang!(self.lang, "load-json") }
                        onclick={ link.callback(|_| Msg::MoveToPage(Page::LoadJson)) }
                        selected={ self.page == Page::LoadJson }
                    /></li>
                <li><PushButton text={ lang!(self.lang, "load-example") }
                        onclick={ link.callback(|_| Msg::MoveToPage(Page::LoadExample)) }
                        selected={ self.page == Page::LoadExample }
                    /></li>
                <li><PushButton text={ lang!(self.lang, "compute-beliefs") }
                        onclick={ link.callback(|_| Msg::MoveToPage(Page::ComputeBeliefs)) }
                        selected={ self.page == Page::ComputeBeliefs }
                    /></li>
                <li><PushButton text={ lang!(self.lang, "mutual-info") }
                        onclick={ link.callback(|_| Msg::MoveToPage(Page::MutualInformation(None))) }
                        selected={ matches!(&self.page, &Page::MutualInformation(_)) }
                    /></li>
                <li><PushButton text={ lang!(self.lang, "help") }
                        onclick={ link.callback(|_| Msg::MoveToPage(Page::Help)) }
                        selected={ self.page == Page::Help }
                    /></li>
                <li><a href="https://github.com/vberger/Bayes-O-Matic/">{ lang!(self.lang, "github") }</a></li>
                <li>{ lang!(self.lang, "language") }
                    <select onchange={ link.callback(|e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                            Msg::SetLang(select.value())
                        } else {
                            Msg::Ignore
                        }
                    )}>
                        { for crate::i18n::AVAILABLE_LANGS.iter().map(|&lang| {
                            html! {
                                <option selected={ self.lang.name == lang } value={ lang }>{ lang }</option>
                            }
                        })}
                    </select>
                </li>
            </ul>
            </div>
        }
    }

    fn editorbar(&self, link: &Scope<Self>) -> Html {
        html! {
            <div id="meta-editor">
                <ul class="blocky">

                </ul>
                <ul id="node-list" class="blocky">
                    { for self.dag.iter_nodes().map(|(id, node)| { html! {
                        <li><PushButton text={ node.label.clone() }
                               onclick={ link.callback(move |_| Msg::MoveToPage(Page::NodeEdit(id))) }
                               selected={ self.page == Page::NodeEdit(id) }
                            /></li>
                    }})}
                    <li><PushButton text="+" title={ lang!(self.lang, "add-node") } onclick={ link.callback(|_| Msg::AddNode) } /></li>
                </ul>
            </div>
        }
    }

    fn print_error(&self) -> Html {
        if let Some(ref error) = self.load_error {
            let text: String = match error {
                DeserError::Json(ref e) => format!("{}: {}", lang!(self.lang, "invalid-json"), e),
                DeserError::Graph(EdgeError::WouldCycle) => lang!(self.lang, "err-cycle"),
                DeserError::Graph(EdgeError::BadNode) => lang!(self.lang, "err-nodenotfound"),
                DeserError::Graph(EdgeError::AlreadyExisting) => lang!(self.lang, "err-edges"),
            };
            html! {
                <p class="error">{ text }</p>
            }
        } else {
            html! {}
        }
    }

    pub fn content(&self, link: &Scope<Self>) -> Html {
        match self.page {
            Page::LoadJson => {
                html! {
                    <div id="popup">
                        { self.print_error() }
                        <input type="file" id="load-json" accept="application/json" onchange={ link.callback_future(|evt: Event| async move {
                            let fileinput = evt.target_dyn_into::<HtmlInputElement>().unwrap();
                            let file = fileinput.files()?.get(0)?;
                            let text = wasm_bindgen_futures::JsFuture::from(file.text()).await.ok()?;
                            Some(Msg::LoadJson(text.as_string()?))
                        })} />
                        <a href="#" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                    </div>
                }
            }
            Page::LoadExample => {
                html! {
                    <div id="popup">
                        <ul>
                        { for self.lang.examples.iter().cloned().map(|name| {
                            html! { <li><p><a href="#" onclick={ link.callback(move |_| Msg::LoadExample(name.into())) }>{ name }</a></p></li> }
                        })}
                        </ul>
                        <a href="#" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                    </div>
                }
            }
            Page::Help => {
                if let Some(ref help) = self.help_contents {
                    html! {
                        <div id="popup">
                            { crate::markdown::render_markdown(help) }
                            <a href="#" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                        </div>
                    }
                } else {
                    html! {
                        <div id="popup">
                            <p>{ lang!(self.lang, "loading-help") }</p>
                            <a href="#" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                        </div>
                    }
                }
            }
            Page::Idle => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar(link) }
                            <div id="node-editor">
                                <p>{ lang!(self.lang, "select-node") }</p>
                            </div>
                        </div>
                    </div>
                }
            }
            Page::NodeEdit(id) => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar(link) }
                            { self.make_nodeedit_tab(id, link) }
                        </div>
                    </div>
                }
            }
            Page::ComputeBeliefs => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar(link) }
                            { self.make_beliefs_tab(link) }
                        </div>
                    </div>
                }
            }
            Page::MutualInformation(_) => {
                html! {
                    <div id="content">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        <div id="editor">
                            { self.editorbar(link) }
                            { self.make_mutualinfo_tab(link) }
                        </div>
                    </div>
                }
            }
        }
    }
}

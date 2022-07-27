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
            <div class="box">
            <div class="columns">
            <div class="column is-narrow">
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
                </ul>
            </div>
            <div class="column"></div>
            <div class="column is-narrow">
                <ul class="blocky">
                    <li class="field">
                        <div class="select">
                        <select onchange={ link.callback(|e: Event| if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                                Msg::SetLang(select.value())
                            } else {
                                Msg::Ignore
                            }
                        )}>
                            { for crate::i18n::AVAILABLE_LANGS.iter().map(|&lang| {
                                html! {
                                    <option selected={ self.lang.name == lang.0 } value={ lang.0 }>{ format!("{} {}", lang.1, lang.0) }</option>
                                }
                            })}
                        </select>
                        </div>
                    </li>
                    <li><a href="https://github.com/vberger/Bayes-O-Matic/" class="button">{ lang!(self.lang, "github") }</a></li>
                </ul>
            </div>
            </div>
            </div>
        }
    }

    fn editorbar(&self, link: &Scope<Self>) -> Html {
        html! {
            <div class="box">
                <ul class="blocky">
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
                    <div class="columns is-centered">
                    <div class="column is-three-fifths box content">
                        { self.print_error() }
                        <div class="file block is-boxed is-large">
                        <label class="file-label">
                        <input type="file" class="file-input" id="load-json" accept="application/json" onchange={ link.callback_future(|evt: Event| async move {
                            let fileinput = evt.target_dyn_into::<HtmlInputElement>().unwrap();
                            let file = fileinput.files()?.get(0)?;
                            let text = wasm_bindgen_futures::JsFuture::from(file.text()).await.ok()?;
                            Some(Msg::LoadJson(text.as_string()?))
                        })} />
                        <span class="file-cta">
                            <span class="file-icon">
                                <i class="fas fa-upload"></i>
                            </span>
                            <span class="file-label">
                                { lang!(self.lang, "choose-file-lo-load") }
                            </span>
                        </span>
                        </label>

                        </div>
                        <div class="block">
                        <a href="#" class="button" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                        </div>
                    </div>
                    </div>
                }
            }
            Page::LoadExample => {
                html! {
                    <div class="columns is-centered">
                    <div class="column is-three-fifths box content">
                        <ul>
                        { for self.lang.examples.iter().cloned().map(|name| {
                            html! { <li><p><a href="#" onclick={ link.callback(move |_| Msg::LoadExample(name.into())) }>{ name }</a></p></li> }
                        })}
                        </ul>
                        <a href="#" class="button" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                    </div>
                    </div>
                }
            }
            Page::Help => {
                if let Some(ref help) = self.help_contents {
                    html! {
                        <div class="columns is-centered">
                        <div class="column is-three-fifths box content">
                            { crate::markdown::render_markdown(help) }
                            <a href="#" class="button" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                        </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="columns is-centered">
                        <div class="column is-three-fifths box content">
                            <p>{ lang!(self.lang, "loading-help") }</p>
                            <a href="#" class="button" onclick={ link.callback(|_| Msg::MoveToPage(Page::Idle)) }>{ lang!(self.lang, "close") }</a>
                        </div>
                        </div>
                    }
                }
            }
            Page::Idle => {
                html! {
                    <div class="columns">
                        <div class="column">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        </div>
                        <div class="column">
                            { self.editorbar(link) }
                            <div class="box">
                                <p>{ lang!(self.lang, "select-node") }</p>
                            </div>
                        </div>
                    </div>
                }
            }
            Page::NodeEdit(id) => {
                html! {
                    <div class="columns">
                        <div class="column">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        </div>
                        <div class="column">
                            { self.editorbar(link) }
                            { self.make_nodeedit_tab(id, link) }
                        </div>
                    </div>
                }
            }
            Page::ComputeBeliefs => {
                html! {
                    <div class="columns">
                        <div class="column">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        </div>
                        <div class="column">
                            { self.editorbar(link) }
                            { self.make_beliefs_tab(link) }
                        </div>
                    </div>
                }
            }
            Page::MutualInformation(_) => {
                html! {
                    <div class="columns">
                        <div class="column">
                        <DotCanvas dot={ crate::draw::graph_to_dot(&self.dag) } />
                        </div>
                        <div class="column">
                            { self.editorbar(link) }
                            { self.make_mutualinfo_tab(link) }
                        </div>
                    </div>
                }
            }
        }
    }
}

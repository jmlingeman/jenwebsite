use dioxus::prelude::*;
use std::fs;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
pub type Todos = im_rc::HashMap<u32, TodoItem>;

pub fn app(cx: Scope<()>) -> Element {
    cx.render(rsx! {
        section { class: "jenapp",
            style { [include_str!("../src/style.css")] }
            div {
                header { class: "header",
                    h1 {"Title of Website"}
                }

            }
            div {
                class: "textsection",
                header { class: "header",
                    h2 {"Intro"}
                }
                p {
                    "blah blaah blah blah blah"
                }
                p {
                    "blah blaah blah blah blah"
                }
            }
            div { class: "sectionbreak", "Section break image goes here"}
            div {
                class: "textsection",
                header { class: "header",
                    h2 {"About Me"}
                }
                p {
                    "blah blaah blah blah blah"
                }
                p {
                    "blah blaah blah blah blah"
                }
            }
            div { class: "sectionbreak", "Section break image goes here"}
            div {
                class: "textsection",
                header { class: "header",
                    h2 {"Approach"}
                }
                p {
                    "blah blaah blah blah blah"
                }
                p {
                    "blah blaah blah blah blah"
                }
            }
            div { class: "sectionbreak", "Section break image goes here"}
            div {
                class: "textsection",
                header { class: "header",
                    h2 {"Contact Me"}
                }
                button { class: "contact-button",
                    a { href: "+015555555", "Call me maybe?" }
                }
                input {
                        placeholder: "first name"
                }

                button { class: "contact-button",
                    a { href: "mailto:xxx", "Email me maybe?" }
                }
            }

        }
        footer { class: "info",
            p { "Created by ", a {  href: "http://jesseisageek.com", "jesse lingeman" }}
        }
    })
}

#[derive(Props)]
pub struct TodoEntryProps<'a> {
    set_jens: &'a UseState<Todos>,
    id: u32,
}

pub fn jen_entry<'a>(cx: Scope<'a, TodoEntryProps<'a>>) -> Element {
    let (is_editing, set_is_editing) = use_state(&cx, || false);

    let jens = cx.props.set_jens.get();
    let jen = &jens[&cx.props.id];
    let completed = if jen.checked { "completed" } else { "" };
    let editing = if *is_editing { "editing" } else { "" };

    rsx!(cx, li {
        class: "{completed} {editing}",
        onclick: move |_| set_is_editing(true),
        onfocusout: move |_| set_is_editing(false),
        div { class: "view",
            input { class: "toggle", r#type: "checkbox", id: "cbg-{jen.id}", checked: "{jen.checked}",
                onchange: move |evt| {
                    cx.props.set_jens.make_mut()[&cx.props.id].checked = evt.value.parse().unwrap();
                }
            }
            label { r#for: "cbg-{jen.id}", pointer_events: "none", "{jen.contents}" }
        }
        is_editing.then(|| rsx!{
            input {
                class: "edit",
                value: "{jen.contents}",
                oninput: move |evt| cx.props.set_jens.make_mut()[&cx.props.id].contents = evt.value.clone(),
                autofocus: "true",
                onkeydown: move |evt| {
                    match evt.key.as_str() {
                        "Enter" | "Escape" | "Tab" => set_is_editing(false),
                        _ => {}
                    }
                },
            }
        })
    })
}

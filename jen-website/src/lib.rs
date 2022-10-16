use dioxus::prelude::*;

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
                    "blah blahh blah blah blaah blah blah blah"
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
            p { "Website by ", a {  href: "http://jesseisageek.com", "jesse lingeman" }}
        }
    })
}

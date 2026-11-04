use dioxus::prelude::*;
use crate::data::uses;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn Works() -> Element {
    let mut count = use_signal(|| 0);
    let langs: Vec<_> = (0..6).map(uses::call_use).collect();
    let last = langs.len() as i32 - 1;
    let text = move |i: i32| {
        if i == count() { "●" } else { "○" }
    };

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "works",
            div {
                class: "slider",
                onscroll: move |e| {
                    let w = e.client_width() as f64;
                    if w > 0.0 {
                        let i = ((e.scroll_left() as f64 / w).round() as i32).clamp(0, last);
                        if i != count() {
                            count.set(i);
                        }
                    }
                },
                for (i, lan) in langs.iter().enumerate() {
                    div {
                        key: "{i}",
                        id: "slide-{i}",
                        class: "slide",
                        h1 {"{uses::name(lan).to_string()}"}
                        p { "{uses::call_sections(lan)}" }
                    }
                }
            }
            div {
                class: "buttons",
                for i in 0..langs.len() {
                    button {
                        onclick: move |_| {
                            document::eval(&format!(
                                "document.getElementById('slide-{i}').scrollIntoView({{behavior:'smooth',inline:'start',block:'nearest'}})"
                            ));
                        },
                        {text(i.try_into().unwrap())}
                    }
                }
            }
        }
    }
}
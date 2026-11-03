use dioxus::prelude::*;
use crate::data::uses;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Shared navbar component.
#[component]
pub fn Works() -> Element {
    let mut count = use_signal(|| 0);
    let mut lan =  uses::call_use(count());
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "works",
            p { {uses::name(&lan).to_string() + " : " + uses::call_sections(&lan)} }
            div {
                class: "buttons",
                for i in 0..5{
                    button { 
                        onclick: move |_| count.set(i),
                        "●"
                    }
                }
            }
        }
    }
}
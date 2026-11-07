use dioxus::{prelude::*};

use crate::data::{MAIN_CSS, Langs};

/// Home page
#[component]
pub fn WorkDetail(lan: Langs) -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"work",
            div {
                class:"header",
                h1 {"Works " {lan.name()} }
            }
        }
    }
}
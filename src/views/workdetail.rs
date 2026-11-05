use dioxus::{prelude::*};

use crate::data::{MAIN_CSS, Uses};

/// Home page
#[component]
pub fn WorkDetail(lan: Uses) -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"works",
            h1 { "Works " {lan.name()} }
        }
    }
}
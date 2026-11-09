use dioxus::{prelude::*};

use crate::data::{MAIN_CSS, Stack};

/// Home page
#[component]
pub fn WorkDetail(lan: Stack) -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"WorkDetail",
            div {
                class:"header",
                h1 {"Works " {lan.name()} }
            }
            div {
                class:"boxs",
                for p in lan.projects(){
                    div {
                        class:"box",
                        h2 { "{p.title}" }
                        p { "{p.desc}" }
                        a { href: "{p.url}", "見る >" }
                    }
                }
            }
        }
    }
}
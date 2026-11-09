use dioxus::{prelude::*};

use crate::{components::WorkBoxs, data::{MAIN_CSS, Stack}};

/// Home page
#[component]
pub fn Stacks() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"Skills",
            div {
                class:"header",
                h1 {"skills"}
                p {"詳細ページあり"}
            }
            WorkBoxs { items:Stack::LANGS.to_vec(),on_link:true}
            div {
                class:"header",
                h1{"Tools"}
                p {"詳細ページなし"}
            }
            WorkBoxs { items:Stack::TOOLS.to_vec(),on_link:false}
        }
    }
}
use dioxus::{prelude::*};

use crate::{components::WorkBoxs, data::Stack};

/// Home page
#[component]
pub fn Stacks() -> Element {
    rsx! {
        div {
            id:"Skills",
            div {
                class:"header",
                h1 {"Languages"}
                p {"カードを押すと、その言語の作品が見られます」"}
            }
            WorkBoxs { items:Stack::LANGS.to_vec(),on_link:true}
            div {
                class:"header",
                h1{"Experience"}
            }
            WorkBoxs { items:Stack::TOOLS.to_vec(),on_link:false}
        }
    }
}
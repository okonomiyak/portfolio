use dioxus::{prelude::*};

use crate::{components::WorkBoxs, data::{MAIN_CSS, LUNG_IMGS,Langs}};

/// Home page
#[component]
pub fn Work() -> Element {
    let langs: Vec<_> = (0..6).map(Langs::call_use).collect();
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"work",
            div {
                class:"header",
                h1 {"Work"}
            }
            WorkBoxs {
                abouts: langs,
                imgs: LUNG_IMGS.to_vec(),
            }
        }
    }
}
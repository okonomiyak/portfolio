use dioxus::{prelude::*};

use crate::{components::WorkBoxs, data::{MAIN_CSS, LUNG_IMGS,Langs}};

/// Home page
#[component]
pub fn Stack() -> Element {
    let langs: Vec<_> = (0..6).map(Langs::call_use).collect();
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"Skills",
            div {
                class:"header",
                h1 {"skills"}
            }
            WorkBoxs {
                abouts: langs,
                imgs: LUNG_IMGS.to_vec(),
            }
            div {
                class:"header",
                h1{"Use"}
            }
        }
    }
}
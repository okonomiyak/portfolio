use dioxus::prelude::*;
use crate::{Route, data::MAIN_CSS};

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        document::Meta { name: "robots", content: "noindex" }
        div {
            id:"notfound",
            h1 { "404" }
            p { "ページが見つかりません" }
            Link {
                class:"bluebutton", 
                to: Route::Home {}, 
                "ホームに戻る"
            }
        }
    }
}
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/styles/main.scss");

/// Home page
#[component]
pub fn Works() -> Element {
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            h1 {"Works"}
        }
    }

}
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Shared navbar component.
#[component]
pub fn Works() -> Element {
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "works",
            
        }
    }
}
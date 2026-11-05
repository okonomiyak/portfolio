use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/styles/main.scss");

/// Shared navbar component.
#[component]
pub fn Footer() -> Element {
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "footer",
            "このページはGithubにてGPL3.0でソース公開されています。"
        }
    }
}
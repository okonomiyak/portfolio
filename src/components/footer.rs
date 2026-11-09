use dioxus::prelude::*;

use crate::data::MAIN_CSS;

/// Shared navbar component.
#[component]
pub fn Footer() -> Element {
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "footer",
            a {
                href:"https://github.com/okonomiyak/portfolio",
                "このページはGithubにてGPL3.0でソース公開されています。"
            }
        }
    }
}
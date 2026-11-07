use dioxus::prelude::*;
use crate::{Route, data::{MAIN_CSS}};

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            class: "navbar",
            "Iwasite"
            div 
            {
                class: "links",
                Link { to: Route::Home {}, "Home" }
                Link { to: Route::Work {}, "Work" }
            }
        }

        Outlet::<Route> {}
    }
}

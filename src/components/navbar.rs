use dioxus::prelude::*;
use crate::{Route, data::{MAIN_CSS, Uses}};

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
                //Link { to: Route::WorkDetail { lan: Uses::Rust },"Works" }
            }
        }

        Outlet::<Route> {}
    }
}

use dioxus::prelude::*;
use crate::Route;

const MAIN_CSS: Asset = asset!("/assets/main.css");

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
                Link { to: Route::Blog { id: 1 },"Blog" }
            }
        }

        Outlet::<Route> {}
    }
}

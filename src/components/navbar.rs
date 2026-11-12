use dioxus::prelude::*;
use crate::{Route, components::Footer};

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav { class: "navbar",
            Link { class: "logo", to: Route::Home {}, "Iwasite" }
            div { class: "links",
                Link { active_class: "active", to: Route::Home {}, "Home" }
                Link { active_class: "active", to: Route::About {}, "About" }
                Link { active_class: "active", to: Route::Stacks {}, "Stack" }
            }
        }

        main {
            Outlet::<Route> {}
        }
        Footer {}
    }
}
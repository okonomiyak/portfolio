use dioxus::{prelude::*};

use crate::{Route, data::{MAIN_CSS, Uses}};

/// Home page
#[component]
pub fn Work() -> Element {
    rsx! {
        div {
            id:"work",
            h1 { "Work"}
            Link { to: Route::WorkDetail { lan: Uses::Rust },"Works" }
        }
    }
}
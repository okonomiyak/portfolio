use dioxus::prelude::*;
use crate::{Route, Stack, components::Carousel};

#[component]
pub fn Cworks() -> Element {
    rsx! {
        div { id: "cworks",
            Carousel { id: "langs", count: Stack::LANGS.len(),
                for lan in Stack::LANGS {
                    Link {
                        key: "{lan}",
                        class: "slide",
                        to: Route::WorkDetail { lan },
                        h1 { "{lan.name()}" }
                        p { "{lan.desc()}" }
                        img { src: lan.img(), alt: "{lan.name()}" }
                    }
                }
            }
        }
    }
}
use dioxus::{prelude::*};

use crate::{Route, data::{MAIN_CSS, Stack}};

/// Home page
#[component]
pub fn WorkDetail(lan: Stack) -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"Detail",
            nav { class: "crumbs",
                Link { to: Route::Stacks {}, "Stack" }
                " / "
                "{lan.name()}"
            }
            div {
                class:"header",
                h1 {"Works " {lan.name()} }
            }
            div {
                class:"boxs",
                for p in lan.projects(){
                    div {
                        class:"box",
                        h2 { "{p.title}" }
                        p { "{p.desc}" }
                        Link { to: Route::ProjectDetail { lan, slug: p.slug.to_string() }, "見る >" }
                    }
                }
            }
        }
    }
}
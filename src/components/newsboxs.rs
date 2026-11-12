use dioxus::prelude::*;

use crate::{Route, data::{MAIN_CSS, NEWS}};

#[component]
pub fn NewsBoxs(limitnews: usize) -> Element {
    let limit = limitnews.min(NEWS.len());

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "newsboxs",
            for item in NEWS.iter().take(limit) {
                Link {
                    class: "newsbox",
                    to: Route::NewsDetail {slug: item.slug.to_string()},
                    if let Some(imgs) = item.img {
                        img { src: imgs, alt: "..." }
                    }
                    "{item.title}"
                }
            }
        }
    }
}
use dioxus::prelude::*;
use crate::data::MAIN_CSS;
use crate::{Route, data::NEWS};
use crate::components::Carousel;

#[component]
pub fn NewsBoxs(limit: usize) -> Element {
    let items: Vec<_> = NEWS.iter().take(limit).collect();
    let count = items.len();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Carousel { id: "news", count,
            for item in items {
                Link {
                    key: "{item.slug}",
                    class: "slide newsbox",
                    to: Route::NewsDetail { slug: item.slug.to_string() },
                    if let Some(pic) = item.img {
                        img { src: pic, alt: "{item.title}" }
                    }
                    h3{"{item.title}"}
                    p{"{item.desc}"}
                }
            }
        }
    }
}
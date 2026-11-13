use dioxus::prelude::*;

use crate::{Route, data::NEWS};
use crate::components::{Carousel, ImgView};

#[component]
pub fn NewsBoxs(limit: usize) -> Element {
    let items: Vec<_> = NEWS.iter().take(limit).collect();
    let count = items.len();

    rsx! {
        Carousel { id: "news", count,
            for item in items {
                Link {
                    key: "{item.slug}",
                    class: "slide newsbox",
                    to: Route::NewsDetail { slug: item.slug.to_string() },
                    ImgView {imgs: item.img.to_vec()}
                    h3{"{item.title}"}
                    p{"{item.desc}"}
                }
            }
        }
    }
}
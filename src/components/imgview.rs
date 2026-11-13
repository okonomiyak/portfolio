use dioxus::prelude::*;
use crate::components::Carousel;

#[component]
pub fn ImgView(imgs:Vec<Asset>) -> Element {
    rsx! {
        if imgs.len() > 1{
            div { id: "imgview",
                Carousel { id: "img", count: imgs.len(),
                    for item in imgs {
                        img { src: item, alt: "..." }
                    }
                }
            }
        }else{
            if let Some(pic) = imgs.first() {
                img { src: *pic, alt: "..." }
            }
        }

    }
}
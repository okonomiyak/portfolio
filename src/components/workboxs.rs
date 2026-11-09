use dioxus::prelude::*;
use crate::{Route, Stack, data::MAIN_CSS};


// to_vec()で配列からVecに
#[component]
pub fn WorkBoxs(items: Vec<Stack>,on_link: bool) -> Element{
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            class:"workboxs",
            for item in items{
                if on_link {
                    Link { 
                        class:"box",
                        to: Route::WorkDetail { lan: item },
                        img { src: item.img(), alt: "..." },
                        "{item.name()}" 
                    }
                }
                else {
                    div { 
                        class:"box",
                        img { src: item.img(), alt: "..." },
                        "{item.name()}" 
                    }
                }
            }
        }

    }
}
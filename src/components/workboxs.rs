use dioxus::prelude::*;
use crate::{Route, Stack};


// to_vec()で配列からVecに
#[component]
pub fn WorkBoxs(items: Vec<Stack>,on_link: bool) -> Element{
    rsx!{
        div {
            class:"workboxs",
            for item in items{
                if on_link {
                    Link { 
                        class:"linkbox",
                        to: Route::WorkDetail { lan: item },
                        img { src: item.img(), alt: "..." },
                        "{item.name()}" 
                    }
                }
                else {
                    div { 
                        class:"divbox",
                        img { src: item.img(), alt: "..." },
                        "{item.name()}" 
                    }
                }
            }
        }

    }
}
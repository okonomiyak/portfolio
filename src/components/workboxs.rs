use dioxus::prelude::*;
use crate::{Route, Langs, data::MAIN_CSS};


// to_vec()で配列からVecに
#[component]
pub fn WorkBoxs(abouts:Vec<Langs>,imgs:Vec<Asset>) -> Element{
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
        div {
            class:"boxs",
            for (i,lan) in abouts.into_iter().enumerate(){
                Link { 
                    class:"box",
                    to: Route::WorkDetail { lan: lan },
                    img { src: imgs[i], alt: "..." },
                    "{Langs::name(&lan)}" 
                }
            }
        }

    }
}
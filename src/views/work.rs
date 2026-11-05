use dioxus::{prelude::*};

use crate::{Route, data::{MAIN_CSS, Uses}};
const IMGS: [Asset;6] = [
    asset!("/assets/img/rust.svg"),
    asset!("/assets/img/java.svg"),
    asset!("/assets/img/python.svg"),
    asset!("/assets/img/unity.svg"),
    asset!("/assets/img/arduino.svg"),
    asset!("/assets/img/ros2.svg")
    ];
/// Home page
#[component]
pub fn Work() -> Element {
    let langs: Vec<_> = (0..6).map(Uses::call_use).collect();
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id:"work",
            div {
                class:"header",
                h1 {"Work"}
            }
            div {
                class:"boxs",
                for (i,lan) in langs.into_iter().enumerate(){
                    Link { 
                        class:"box",
                        to: Route::WorkDetail { lan: lan },
                        img { src: IMGS[i], alt: "..." },
                        "{Uses::name(&lan)}" 
                    }
                }
            }
        }
    }
}
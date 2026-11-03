use dioxus::prelude::*;

use crate::components::Footer;

const MAIN_CSS: Asset = asset!("/assets/main.css");

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            class:"home",
            section {
                div {
                    id:"Fgb",
                    h2 { "iwaportfolio"}
                    h1 { "いわのポートフォリオ" }
                    h2 { "Rustでいろいろ開発する 高専生2年生"}
                    h2 { "他にも Unity(C#) Java Python などもやってます"}

                    div {
                        class: "button",
                        a { href: "", class:"bluebutton","詳細"}
                        a { href: "", class:"whitebutton" ,"概要を見る >"}
                    }
                }
            }
        }
        Footer {}
    }
}

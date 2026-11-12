use dioxus::prelude::*;

use crate::Route;
use crate::components::Cworks;
use crate::data::MAIN_CSS;

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            class:"home",
            section {
                div {
                    id:"Fhome",
                    div {
                        h2 { "iwaportfolio"}
                        h1 { "いわのポートフォリオ" }
                        h2 { "Rustでいろいろ開発する 高専生2年生"}
                        h2 { "他にも Unity(C#) Java Python などもやってます"}
                        //img{ src: asset!("assets/img/hero-glow.webp"),alt:"hero"}
                    }

                    div {
                        class: "button",
                        Link { 
                            class:"bluebutton",
                            to: Route::About {},
                            "詳細"
                        }
                        a {    
                            class: "whitebutton",
                            href: "#about",
                            onclick: move |evt| {
                                evt.prevent_default();   // 標準のリンク動作を止める
                                document::eval(
                                    r#"document.getElementById("about").scrollIntoView({ behavior: "smooth" });"#,
                                );
                            },
                            "概要を見る >"
                        }
                    }
                }
                div {
                    id:"Shome",
                    h1 { "色々使える"}
                    Cworks {  } 
                }
            }
            section {
                id: "about", class: "about",
                div { class: "inner",
                    h3 { "self-introduction" }
                    h1 { "ものづくりが好き" }
                    p { class: "lead",
                        "高専の2年生です。Rust を中心に、Minecraft の Mod や Discord bot、"
                        "部活のロボット、電子工作まで、作りながら学んでいます。"
                    }
                    dl { class: "facts",
                        div { class: "row",
                            dt { "所属" }
                            dd { "高専 2年" }
                        }
                        div { class: "row",
                            dt { "よく使う言語" }
                            dd { "Rust, Java, C#, Python" }
                        }
                        div { class: "row",
                            dt { "作っているもの" }
                            dd { "Minecraft の Mod、Discord bot、このサイト" }
                        }
                        div { class: "row",
                            dt { "環境" }
                            dd { "Arch Linux (Hyprland) / 自宅サーバー" }
                        }
                    }
            }   }
        }
    }
}

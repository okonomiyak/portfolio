use dioxus::prelude::*;

use crate::data::MAIN_CSS;

#[component]
pub fn About() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "about", class: "about",
            div { class: "inner",
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
                        dt { "好きな食べ物" }
                        dd { "果物全般" }
                    }
                    div { class: "row",
                        dt { "好きな音楽" }
                        dd { "平沢進 / TIMELINEの終わり "}
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
        
            }
        
            // div { class: "stats",
            //     img {
            //         src: "https://raw.githubusercontent.com/okonomiyak/okonomiyak/main/generated/top-langs.svg",
            //         alt: "よく使う言語の割合",
            //         loading: "lazy"
            //     }
            //     img {
            //         src: "https://raw.githubusercontent.com/okonomiyak/okonomiyak/main/generated/stats.svg",
            //         alt: "GitHub の統計",
            //         loading: "lazy"
            //     }
            // }   
        }  
    }
}
use dioxus::prelude::*;
use crate::{Route, Stack, data::MAIN_CSS};
use crate::components::Scroll_to;

#[component]
pub fn Cworks() -> Element {
    let mut count = use_signal(|| 0usize);
    let n = Stack::LANGS.len();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { id: "cworks",
            div {
                class: "slider",
                onscroll: move |e| {
                    let total = e.scroll_width() as f64;
                    if total <= 0.0 {
                        return;   // 幅がまだ0のときは割り算しない
                    }
                    // 今の位置が「何枚目」か(四捨五入して、最後の番号に収める)
                    let i = (e.scroll_left() as f64 / total * n as f64).round() as usize;
                    let i = i.min(n - 1);
                    if i != count() {   // 番号が変わったときだけ更新
                        count.set(i);
                    }
                },
                for lan in Stack::LANGS {
                    Link {
                        key: "{lan}",
                        id: "slide-{lan}",
                        class: "slide",
                        to: Route::WorkDetail { lan },
                        h1 { "{lan.name()}" }
                        p { "{lan.desc()}" }
                        img { src: lan.img(), alt: "{lan.name()}" }
                    }
                }
            }
            div { class: "buttons",
                for (i, lan) in Stack::LANGS.into_iter().enumerate() {
                    button {
                        key: "{lan}",
                        onclick: move |_| Scroll_to(&format!("slide-{lan}")),
                        { if i == count() { "●" } else { "○" } }
                    }
                }
            }
        }
    }
}
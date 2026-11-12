use dioxus::prelude::*;
use crate::components::scroll_to_child;

#[component]
pub fn Carousel(
    #[props(into)] id: String,   // ページ内で重複しない名前
    count: usize,                // スライドの枚数
    children: Element,           // スライド本体
) -> Element {
    let mut current = use_signal(|| 0usize);
    let track = format!("{id}-track");

    rsx! {
        div { id: "{id}", class: "carousel",
            div {
                id: "{track}",
                class: "slider",
                onscroll: move |e| {
                    let total = e.scroll_width() as f64;
                    if total <= 0.0 || count == 0 {
                        return;
                    }
                    let i = (e.scroll_left() as f64 / total * count as f64).round() as usize;
                    let i = i.min(count - 1);
                    if i != current() {
                        current.set(i);
                    }
                },
                {children}
            }
            div { class: "buttons",
                for i in 0..count {
                    button {
                        key: "{i}",
                        onclick: {
                            let track = track.clone();
                            move |_| scroll_to_child(&track, i)
                        },
                        { if i == current() { "●" } else { "○" } }
                    }
                }
            }
        }
    }
}
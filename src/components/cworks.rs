use dioxus::prelude::*;
use crate::data::Uses;

const MAIN_CSS: Asset = asset!("/styles/main.scss");
const IMGS: [Asset;6] = [
    asset!("/assets/img/rust.svg"),
    asset!("/assets/img/java.svg"),
    asset!("/assets/img/python.svg"),
    asset!("/assets/img/unity.svg"),
    asset!("/assets/img/arduino.svg"),
    asset!("/assets/img/ros2.svg")
    ];

#[component]
pub fn Cworks() -> Element {
    let mut count = use_signal(|| 0);
    let langs: Vec<_> = (0..6).map(Uses::call_use).collect();
    let last = langs.len() as i32 - 1;
    let text = move |i: i32| {
        if i == count() { "●" } else { "○" }
    };

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div {
            id: "cworks",
            div {
                class: "slider",
                onscroll: move |e| {
                    let total = e.scroll_width() as f64;
                    if total > 0.0 {                        // 幅がまだ 0 のときは割り算しない
                        let n = last as f64 + 1.0;          // スライドの枚数(last は最後の番号なので +1)
                        let i = ((
                            e.scroll_left() as f64 / total * n   // 今の位置が「何枚目ぶん」か
                        )
                        .round() as i32)                          // 四捨五入して整数にする
                        .clamp(0, last);                         // 0〜最後の範囲に収める
                        if i != count() {                   // 番号が変わったときだけ
                            count.set(i);                   // count を更新(ドットが動く)
                        }
                    }
                },
                for (i, lan) in langs.iter().enumerate() {
                    a {
                        key: "{i}",
                        id: "slide-{i}",
                        class: "slide",
                        href:"",
                        h1 {"{Uses::name(lan).to_string()}"}
                        p { "{Uses::call_sections(lan)}" }
                        img { src: IMGS[i], alt: "..." }
                    }
                }
            }
            div {
                class: "buttons",
                for i in 0..langs.len() {
                    button {
                        onclick: move |_| {
                            document::eval(&format!(
                                "document.getElementById('slide-{i}').scrollIntoView({{behavior:'smooth',inline:'start',block:'nearest'}})"
                            ));
                        },
                        {text(i.try_into().unwrap())}
                    }
                }
            }
        }
    }
}
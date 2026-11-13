use dioxus::prelude::*;

const SUN: Asset = asset!("assets/img/sun.svg");
const MOON: Asset = asset!("assets/img/moon.svg");

// <html data-theme> を切り替えて、選んだ値を localStorage に保存する
const TOGGLE_JS: &str = r#"
const r = document.documentElement;
const cur = r.dataset.theme ||
  (matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
const next = cur === 'dark' ? 'light' : 'dark';
r.dataset.theme = next;
try { localStorage.setItem('theme', next) } catch (e) {}
"#;

#[component]
pub fn ThemeToggle() -> Element {
    rsx! {
        button {
            class: "theme-toggle",
            r#type: "button",
            title: "テーマを切り替える",
            "aria-label": "テーマを切り替える",
            onclick: move |_| {
                let _ = document::eval(TOGGLE_JS);
            },
            // どちらを出すかは CSS(--sun-display / --moon-display)が決める
            img { class: "sun", src: SUN, alt: "" }
            img { class: "moon", src: MOON, alt: "" }
        }
    }
}
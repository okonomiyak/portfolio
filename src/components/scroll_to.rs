// src/utils.rs
use dioxus::prelude::*;

/// ページ内の、idを持つ要素までスクロールする
pub fn scroll_to(id: &str) {
    document::eval(&format!(
        "document.getElementById('{id}')?.scrollIntoView({{behavior:'smooth',inline:'start',block:'nearest'}})"
    ));
}

/// 横スクロールの入れ物の中の、index番目の子要素までスクロールする。
/// ページ全体は動かさず、入れ物の中だけを動かす。
pub fn scroll_to_child(container_id: &str, index: usize) {
    document::eval(&format!(
        "const t = document.getElementById('{container_id}');
         const c = t?.children[{index}];
         if (t && c) {{
             t.scrollTo({{
                 left: t.scrollLeft + c.getBoundingClientRect().left - t.getBoundingClientRect().left,
                 behavior: 'smooth'
             }});
         }}"
    ));
}
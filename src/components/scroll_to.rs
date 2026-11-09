// src/utils.rs
use dioxus::prelude::*;


pub fn Scroll_to(id: &str) {
    document::eval(&format!(
        "document.getElementById('{id}')?.scrollIntoView({{behavior:'smooth',inline:'start',block:'nearest'}})"
    ));
}
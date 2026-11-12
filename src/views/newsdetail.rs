use dioxus::prelude::*;
use crate::{Route, data::{MAIN_CSS, News}, views::PageNotFound};

#[component]
pub fn NewsDetail(slug:String) -> Element {
    let Some(p) = News::find(&slug) else {
        return rsx! { PageNotFound { route: vec![] } };   // return と ; が必要
    };
    rsx!{
        document::Stylesheet { href: MAIN_CSS }
    }
}
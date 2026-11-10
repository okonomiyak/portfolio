use dioxus::prelude::*;

use crate::{Route, data::{Projects, Stack}, views::PageNotFound};

#[component]
pub fn ProjectDetail(lan:Stack,slug:String) -> Element {
    let Some(p) = Projects::find(lan, &slug) else {
        return rsx! { PageNotFound { route: vec![] } };   // return と ; が必要
    };
    rsx! {
        document::Title { "{p.title} | Iwasite" }
        article { id: "ProjectDetail",
            nav { class: "crumbs",
                Link { to: Route::Stacks {}, "Stack" }
                " / "
                Link { to: Route::WorkDetail { lan }, "{lan.name()}" }
                " / {p.title}"
            }
            h1 { "{p.title}" }
            if let Some(pic) = p.img {
                img { src: pic, alt: "{p.title}" }
            }
            p { "{p.body}" }
            h2 { "使った技術" }
            ul {
                for s in p.lang {
                    li { key: "{s}", "{s.name()}" }
                }
            }
            a { class: "bluebutton", href: "{p.url}", "作品 を見る" }
        }   
    }
}
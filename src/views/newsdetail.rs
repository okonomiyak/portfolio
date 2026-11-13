use dioxus::prelude::*;
use crate::{Route, components::ImgView, data::{News,Projects}, views::PageNotFound};

#[component]
pub fn NewsDetail(slug: String) -> Element {
    let Some(p) = News::find(&slug) else {
        return rsx! { PageNotFound { route: vec!["news".into(), slug] } };
    };
    let related: Vec<_> = p 
        .projectslug.iter()
        .filter_map(|s| {
            let proj = Projects::find_by_slug(s)?;
            let lan = *proj.lang.first()?;
            Some((lan,proj))
        }
    ).collect();

    rsx! {
        document::Title { "{p.title} | Iwasite" }
        article { id: "Detail",
            nav { class: "crumbs",
                Link { to: Route::Home {}, "Home" }
                " / {p.title}"
            }
            h1 { "{p.title}" }
            p {"{p.body}" }
            if !related.is_empty() {
                div {
                    class:"projects",
                    h2 { "関連する作品" }
                    ul {
                        for (lan, proj) in related {
                            li { key: "{proj.slug}",
                                Link {
                                    to: Route::ProjectDetail { lan, slug: proj.slug.to_string() },
                                    "{proj.title} を見る >"
                                }
                            }
                        }
                    }
                }
            }
            p { class: "date", "{p.date}" }
            ImgView {imgs: p.img.to_vec() }
            Link { class: "bluebutton", to: Route::Home {}, "ホームに戻る" }
        }
    }
}
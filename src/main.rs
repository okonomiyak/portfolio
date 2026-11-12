mod views;
mod components;
mod data;

use components::{Navbar};
use dioxus::prelude::*;
use views::{Home,WorkDetail,Stacks,PageNotFound,ProjectDetail,About};
use crate::data::Stack;

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/about")]
        About {},
        #[route("/stack")]
        Stacks {},
        #[route("/works/:lan")]
        WorkDetail { lan: Stack },
        #[route("/works/:lan/:slug")]
        ProjectDetail { lan: Stack, slug: String },
        #[route("/:..route")]
        PageNotFound { route: Vec<String> },   // 必ず最後
}

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: data::MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com", crossorigin: "" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=JetBrains+Mono:ital,wght@0,100..800;1,100..800&display=swap" }
        Router::<Route> {}
    }
}



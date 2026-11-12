use dioxus::prelude::*;



const GITHUBICON:Asset = asset!("assets/img/use/github.svg");
const MAIL:Asset = asset!("assets/img/mail.svg");
/// Shared navbar component.
#[component]
pub fn Footer() -> Element {
    rsx!{
        div {
            id: "footer",
            nav {
                class:"contact",
                a { 
                    href:"https://github.com/okonomiyak",
                    alt:"github",
                    "Github"
                    img {
                        src:GITHUBICON,
                        alt:"githubicon"
                    }
                }
                a { 
                    href:"mailto:info@iwaservice.uk",
                    alt:"mail",
                    "mail"
                    img { 
                        src:MAIL,
                        alt:"mail",
                    }
                }
            }
            div {
                class:"legal",
                p { "© 2026 Iwa" }
                a {
                    href:"https://github.com/okonomiyak/portfolio",
                    "このサイトのソースコードは GPL-3.0 で公開しています。"
                }
                p{ "出典:Simple Icons、Devicon" }
            }
        }
    }
}
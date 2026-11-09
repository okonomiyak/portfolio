use dioxus::prelude::*;
use std::{fmt, str::FromStr};

#[derive(Clone, PartialEq)]
pub struct Projects {
    pub title: &'static str,
    pub desc: &'static str,
    pub url: &'static str,
    pub lang: &'static [Stack],
}
#[derive(Clone, Copy, PartialEq)]
pub struct StackInfo {
    pub name: &'static str,
    pub slug: &'static str,
    pub img: Asset,
    pub desc: &'static str,
}

#[derive(Clone, Copy, PartialEq)] 
pub enum Stack {
    //Langs
    Rust,Java,Python,Unity,Arduino,Javascript,
    //Use
    ROS2,Kicad,Git,Github,Proxmox,Docker,Arch,
}

impl Stack {
    pub const LANGS: [Stack; 6] = [
        Stack::Rust, Stack::Java, Stack::Python,
        Stack::Unity, Stack::Arduino, Stack::Javascript,
    ];
    pub const TOOLS: [Stack; 7] = [Stack::ROS2,Stack::Kicad,Stack::Git, Stack::Github, Stack::Proxmox,Stack::Arch,Stack::Docker,];

        pub fn info(self) -> StackInfo {
            match self {
                Stack::Rust => StackInfo { name: "Rust", slug: "rust",
                    img: asset!("/assets/img/lang/rust.svg"), desc: "このサイト、Discordボット" },
                Stack::Java => StackInfo { name: "Java", slug: "java",
                    img: asset!("/assets/img/lang/java.svg"), desc: "Minecraft の Mod・アドオン" },
                Stack::Python => StackInfo { name: "Python", slug: "python",
                    img: asset!("/assets/img/lang/python.svg"), desc: "自作ニューラルネットワーク" },
                Stack::Unity => StackInfo { name: "Unity", slug: "unity",
                    img: asset!("/assets/img/lang/unity.svg"), desc: "(C#) ブラウザで遊べるレースゲーム" },
                Stack::Arduino => StackInfo { name: "Arduino", slug: "arduino",
                    img: asset!("/assets/img/lang/arduino.svg"), desc: "ESP32 + ディスプレイ" },
                Stack::Javascript => StackInfo { name: "Javascript", slug: "javascript",
                    img: asset!("/assets/img/lang/javascript.svg"), desc: "Kubejsやwebサイト" },
                Stack::ROS2 => StackInfo { name: "ROS2", slug: "ros2",
                    img: asset!("/assets/img/use/ros2.svg"), desc: "" },
                Stack::Kicad => StackInfo { name: "Kicad", slug: "kicad",
                    img: asset!("/assets/img/use/kicad.svg"), desc: "" },
                Stack::Git => StackInfo { name: "Git", slug: "git",
                    img: asset!("/assets/img/use/git.svg"), desc: "" },
                Stack::Github => StackInfo { name: "Github", slug: "github",
                    img: asset!("/assets/img/use/github.svg"), desc: "" },
                Stack::Proxmox => StackInfo { name: "Proxmox", slug: "proxmox",
                    img: asset!("/assets/img/use/proxmox.svg"), desc: "" },
                Stack::Arch => StackInfo { name: "Arch", slug: "arch",
                    img: asset!("/assets/img/use/arch.svg"), desc: "" },
                Stack::Docker => StackInfo { name: "Docker", slug: "docker",
                    img: asset!("/assets/img/use/docker.svg"), desc: "" },
            }
        }
        pub fn name(self) -> &'static str { self.info().name }
        pub fn slug(self) -> &'static str { self.info().slug }
        pub fn img(self) -> Asset { self.info().img }
        pub fn desc(self) -> &'static str { self.info().desc }

    
    pub fn projects(self) -> impl Iterator<Item = &'static Projects> {
        PROJECTS.iter().filter(move |p| p.lang.contains(&self))
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.slug())
    }
}

impl FromStr for Stack {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 言語だけを対象にする。/works/git などは404になる
        Stack::LANGS
            .into_iter()
            .find(|v| v.slug() == s)
            .ok_or_else(|| format!("unknown: {s}"))
    }
}
pub const MAIN_CSS: Asset = asset!("/styles/main.scss");

pub const PROJECTS: &[Projects] = &[
    Projects {
        title:"Portfolio",
        desc:"このさいと",
        url: "https://okonomiyak.github.io/portfolio",
        lang:&[Stack::Rust],
    },
    Projects {
        title:"読み上げ&音楽再生bot",
        desc:"DiscordのVCで読み上げと音楽の再生ができるdiscordbot",
        url: "https://github.com/okonomiyak/yomiage",
        lang:&[Stack::Rust],
    },
    Projects {
        title:"LifeGame",
        desc:"Linuxターミナル上で動作するLifeGane",
        url: "https://github.com/okonomiyak/LifeGame-InRust.git",
        lang:&[Stack::Rust],
    },
    Projects {
        title:"discord_task_bot",
        desc:"タスクの管理discordbot Web付き ",
        url: "https://github.com/okonomiyak/discord_task_bot",
        lang:&[Stack::Rust],
    },
    Projects {
        title:"squadtp",
        desc:"BF風FPS用mod 分隊機能",
        url: "https://github.com/okonomiyak/squadtp",
        lang:&[Stack::Java],
    },

    Projects {
        title:"squadtp-conquest",
        desc:"BF風FPS用mod ゲーム用",
        url: "https://github.com/okonomiyak/squadtp-conquest",
        lang:&[Stack::Java],
    },

    Projects {
        title:"classloadout",
        desc:"BF風FPS用mod ロードアウト管理",
        url: "https://github.com/okonomiyak/classloadout",
        lang:&[Stack::Java],
    },
    Projects {
        title:"ModRoster",
        desc:"マルチプレイmod管理用",
        url: "https://github.com/okonomiyak/ModRoster",
        lang:&[Stack::Java],
    },

    Projects {
        title:"SBW-Extra-Armory",
        desc:"SBWアドオンmod",
        url: "https://github.com/okonomiyak/SBW-Extra-Armory",
        lang:&[Stack::Java],
    },
    Projects {
        title:"Campus Sky Attack",
        desc:"高専歳用ゲーム(unity) ランキング付き(rust/api)",
        url: "https://race.iwa.me.uk",
        lang:&[Stack::Unity,Stack::Rust],
    },
    Projects {
        title:"ニューラルネットワーク",
        desc:"MNISというデータセットからフルスクラッチのニューラルネットワークによって分類を行うスクリプト",
        url: "https://colab.research.google.com/drive/1s8RHL4h316pDuKj6tsQbAZ9ivsv1I5xX?usp=sharing&classId=bfb5d42f-49e1-49a9-a762-5922349606fb&assignmentId=070d88ea-a294-43a5-8615-27e77100fdf7&submissionId=de72091b-e1af-8fc8-56cd-1511389bc090",
        lang:&[Stack::Python],
    },
    Projects {
        title:"マインクラフト 投票スクリプト",
        desc:"kubejsで投票を行うスクリプト",
        url: "https://github.com/okonomiyak/minecraft-vote-kubejs",
        lang:&[Stack::Javascript],
    }
];
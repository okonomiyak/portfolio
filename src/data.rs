use dioxus::prelude::*;
use std::{fmt, str::FromStr};

#[derive(Clone, PartialEq)]
pub struct Projects {
    pub slug: &'static str,          // URL用(重複しない英数字とハイフン)
    pub title: &'static str,
    pub desc: &'static str,          // カード用の短い説明
    pub body: &'static str,          // 詳細ページ用の説明
    pub url: &'static str,           // GitHub
    pub img: Option<Asset>,          // 画像がない作品もあるので Option
    pub lang: &'static [Stack],
}

#[derive(Clone, PartialEq)]
pub struct News {
    pub slug: &'static str,
    pub title: &'static str,
    pub desc: &'static str,
    pub date: &'static str,
    pub img: Option<Asset>,
    pub projectslug: &'static [&'static str],
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

impl Projects {
    pub fn find(lan:Stack,slug:&str) -> Option<&'static Projects> {
        PROJECTS.iter().find(|p| p.slug == slug && p.lang.contains(&lan))
    }
    pub fn find_by_slug(slug:&str) -> Option<&'static Projects>  {
       PROJECTS.iter().find(|p| p.slug == slug )
    }
}

impl News {
    pub fn new_sort(mut selfs:Vec<Self>) -> Vec<Self> {
        selfs.sort_by(|a,b| b.date.cmp(&a.date));
        selfs
    }
    pub fn check_projects(self) -> impl Iterator<Item = &'static Projects>{
        let slug = self.projectslug;
        PROJECTS.iter().filter(move | p | slug.contains(&p.slug))
    }
    pub fn find(slug:&str) -> Option<&'static News> {
        NEWS.iter().find(|p| p.slug == slug)
    }
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
        slug:"portfolio",
        title:"Portfolio",
        desc:"このさいと",
        body:"dixousを利用し、GithubPageで公開する自分のポートフォリオ",
        url: "https://okonomiyak.github.io/portfolio",
        img: None,
        lang:&[Stack::Rust],
    },
    Projects {
        slug:"musicyomiagebots",
        title:"読み上げ&音楽再生bot",
        desc:"色々再生discordbot",
        body:"DiscordのVCで読み上げと音楽の再生ができるdiscordbot",
        url: "https://github.com/okonomiyak/yomiage",
        img:None,
        lang:&[Stack::Rust],
    },
    // ...(Portfolio と 読み上げbot も、同じ形で slug / body / img を足してください)
    Projects {
        slug: "lifegame",
        title: "LifeGame",
        desc: "Linuxターミナル上で動作するLifeGame",
        body: "Rustで書いた、ライフゲームです。Linuxのターミナル上で動きます。",
        url: "https://github.com/okonomiyak/LifeGame-InRust",
        img: None,
        lang: &[Stack::Rust],
    },
    Projects {
        slug: "discord-task-bot",
        title: "discord_task_bot",
        desc: "タスクの管理discordbot Web付き",
        body: "Discordからタスクを管理できるbotです。Webからも見られるようにしています。",
        url: "https://github.com/okonomiyak/discord_task_bot",
        img: Some(asset!("assets/img/og.webp")),
        lang: &[Stack::Rust],
    },
    Projects {
        slug: "squadtp",
        title: "squadtp",
        desc: "BF風FPS用mod 分隊機能",
        body: "BF風FPS用modの分隊機能です。最大4人の分隊を組んで、仲間の位置へのテレポートや、仲間の近くへのスポーンができます。JourneyMapと連携しています。",
        url: "https://github.com/okonomiyak/squadtp",
        img: None,
        lang: &[Stack::Java],
    },
    Projects {
        slug: "squadtp-conquest",
        title: "squadtp-conquest",
        desc: "BF風FPS用mod ゲーム用",
        body: "BF風FPS用modのうち、ゲームとして遊ぶための機能です。",
        url: "https://github.com/okonomiyak/squadtp-conquest",
        img: None,
        lang: &[Stack::Java],
    },
    Projects {
        slug: "classloadout",
        title: "classloadout",
        desc: "BF風FPS用mod ロードアウト管理",
        body: "BF風FPS用modの、ロードアウト(装備のセット)を管理する機能です。",
        url: "https://github.com/okonomiyak/classloadout",
        img: None,
        lang: &[Stack::Java],
    },
    Projects {
        slug: "modroster",
        title: "ModRoster",
        desc: "マルチプレイmod管理用",
        body: "マルチプレイで、参加者のmod構成を管理するためのmodです。",
        url: "https://github.com/okonomiyak/ModRoster",
        img: None,
        lang: &[Stack::Java],
    },
    Projects {
        slug: "sbw-extra-armory",
        title: "SBW-Extra-Armory",
        desc: "SBWアドオンmod",
        body: "SuperbWarfare(SBW)の追加武器パックです。グレネードランチャー付きカービン、ナイフ、グレネード、SRAWなどを追加します。",
        url: "https://github.com/okonomiyak/SBW-Extra-Armory",
        img: None,
        lang: &[Stack::Java],
    },
    Projects {
        slug: "campus-sky-attack",
        title: "Campus Sky Attack",
        desc: "高専祭用ゲーム(unity) ランキング付き(rust/api)",
        body: "高専祭向けに作った、ブラウザで遊べるUnityのゲームです。ランキングは、Rust製のAPIで管理しています。",
        url: "https://race.iwa.me.uk",
        img: Some(asset!("assets/img/skyattack.png")),
        lang: &[Stack::Unity, Stack::Rust],
    },
    Projects {
        slug: "neural-network",
        title: "ニューラルネットワーク",
        desc: "MNISTのデータセットを、フルスクラッチのニューラルネットワークで分類するスクリプト",
        body: "MNISTのデータセットを、フルスクラッチで作ったニューラルネットワークで分類するスクリプトです。",
        url: "https://colab.research.google.com/drive/1s8RHL4h316pDuKj6tsQbAZ9ivsv1I5xX?usp=sharing",
        img: None,
        lang: &[Stack::Python],
    },
    Projects {
        slug: "minecraft-vote-kubejs",
        title: "マインクラフト 投票スクリプト",
        desc: "kubejsで投票を行うスクリプト",
        body: "KubeJSで書いた、Minecraft内で投票を行うスクリプトです。",
        url: "https://github.com/okonomiyak/minecraft-vote-kubejs",
        img: None,
        lang: &[Stack::Javascript],
    },
    Projects {
        slug: "lc7981",
        title: "lc7981(ディスプレイ)",
        desc: "lc7981をArudinoで動かすコード",
        body: "Arduinoでlc7981のディスプレイを制御するコードです。",
        url: "https://github.com/okonomiyak/lc7981-board",
        img: Some(asset!("assets/img/IMG_20261107_0217352.webp")),
        lang: &[Stack::Arduino],
    },
];

pub const NEWS: &[News] = &[
    News {
        slug: "kousen2026",
        title: "高専祭(2026)",
        desc:"2026年高専祭 私はレースゲームを作成しました。",
        date:"2026-11-07",
        img: None,
        projectslug: &["campus-sky-attack"],
    }
];
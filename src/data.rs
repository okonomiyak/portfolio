use dioxus::prelude::*;
use std::{fmt, str::FromStr};
#[derive(Clone, PartialEq)]
pub struct Project {
    pub title: &'static str,
    pub desc: &'static str,
    pub url: &'static str,
    pub lang: &'static [Langs],
}

#[derive(Clone, Copy, PartialEq)] 
pub enum Langs {
    Rust,
    Java,
    Python,
    Unity,
    Arduino,
    ROS2,
}

impl Langs {
    pub fn call_sections(&self) -> &str {
        let i = match self {
            Langs::Rust => 0,
            Langs::Java => 1,
            Langs::Python => 2,
            Langs::Unity => 3,
            Langs::Arduino => 4,
            Langs::ROS2 => 5,
        };
        USE_SECTIONS[i]
    }
    pub fn name(&self) -> &str {
        match self {
            Langs::Rust => "Rust",
            Langs::Java => "Java",
            Langs::Python => "Python",
            Langs::Unity => "Unity",
            Langs::Arduino => "Arduino",
            Langs::ROS2 => "ROS2",
        }
    }
    pub fn call_use(i:i32) -> Self{
        match i {
            0 => Langs::Rust,
            1 => Langs::Java,
            2 => Langs::Python,
            3 => Langs::Unity,
            4 => Langs::Arduino,
            _ => Langs::ROS2,
            }
    }
    pub fn projects(self) -> impl Iterator<Item = &'static Project> {
        PROJECTS.iter().filter(move |p| p.lang.contains(&self))
    }
}
impl FromStr for Langs {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Langs::Rust),
            "java" => Ok(Langs::Java),
            "python" => Ok(Langs::Python),
            "unity" => Ok(Langs::Unity),
            "arduino" => Ok(Langs::Arduino),
            "ros2" => Ok(Langs::ROS2),
            _ => Err(format!("unknown: {s}")),
        }
    }
}
impl fmt::Display for Langs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Langs::Rust => "rust",
            Langs::Java => "java",
            Langs::Python => "python",
            Langs::Unity => "unity",
            Langs::Arduino => "arduino",
            Langs::ROS2 => "ros2",
        };
        write!(f, "{s}")
    }
}

pub const MAIN_CSS: Asset = asset!("/styles/main.scss");

pub const LUNG_IMGS: [Asset; 6] = [
    asset!("/assets/img/rust.svg"),
    asset!("/assets/img/java.svg"),
    asset!("/assets/img/python.svg"),
    asset!("/assets/img/unity.svg"),
    asset!("/assets/img/arduino.svg"),
    asset!("/assets/img/ros2.svg"),
];

pub const USE_SECTIONS: &[&str] = &[
    "このサイト、Discordボット",
    "Minecraft の Mod・アドオン",
    "自作ニューラルネットワーク",
    "(C#) ブラウザで遊べるレースゲーム",
    "ESP32 + ディスプレイ",
    "ロボットの制御"
];

pub const PROJECTS: &[Project] = &[
    Project {
        title:"Portfolio",
        desc:"このさいと",
        url: "https://okonomiyak.github.io/portfolio",
        lang:&[Langs::Rust],
    },
    Project {
        title:"読み上げ&音楽再生bot",
        desc:"DiscordのVCで読み上げと音楽の再生ができるdiscordbot",
        url: "https://github.com/okonomiyak/yomiage",
        lang:&[Langs::Rust],
    },
    Project {
        title:"LifeGame",
        desc:"Linuxターミナル上で動作するLifeGane",
        url: "https://github.com/okonomiyak/LifeGame-InRust.git",
        lang:&[Langs::Rust],
    },
    Project {
        title:"discord_task_bot",
        desc:"タスクの管理discordbot Web付き ",
        url: "https://github.com/okonomiyak/discord_task_bot",
        lang:&[Langs::Rust],
    },
    Project {
        title:"squadtp",
        desc:"BF風FPS用mod 分隊機能",
        url: "https://github.com/okonomiyak/squadtp",
        lang:&[Langs::Java],
    },

    Project {
        title:"squadtp-conquest",
        desc:"BF風FPS用mod ゲーム用",
        url: "https://github.com/okonomiyak/squadtp-conquest",
        lang:&[Langs::Java],
    },

    Project {
        title:"classloadout",
        desc:"BF風FPS用mod ロードアウト管理",
        url: "https://github.com/okonomiyak/classloadout",
        lang:&[Langs::Java],
    },
    Project {
        title:"ModRoster",
        desc:"マルチプレイmod管理用",
        url: "https://github.com/okonomiyak/ModRoster",
        lang:&[Langs::Java],
    },

    Project {
        title:"SBW-Extra-Armory",
        desc:"SBWアドオンmod",
        url: "https://github.com/okonomiyak/SBW-Extra-Armory",
        lang:&[Langs::Java],
    },
    Project {
        title:"Campus Sky Attack",
        desc:"高専歳用ゲーム(unity) ランキング付き(rust/api)",
        url: "https://race.iwa.me.uk",
        lang:&[Langs::Unity,Langs::Rust],
    },
    Project {
        title:"ニューラルネットワーク",
        desc:"MNISというデータセットからフルスクラッチのニューラルネットワークによって分類を行うスクリプト",
        url: "https://colab.research.google.com/drive/1s8RHL4h316pDuKj6tsQbAZ9ivsv1I5xX?usp=sharing&classId=bfb5d42f-49e1-49a9-a762-5922349606fb&assignmentId=070d88ea-a294-43a5-8615-27e77100fdf7&submissionId=de72091b-e1af-8fc8-56cd-1511389bc090",
        lang:&[Langs::Python],
    },
];
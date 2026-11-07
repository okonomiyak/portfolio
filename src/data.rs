use dioxus::prelude::*;
use std::{fmt, str::FromStr};

pub const MAIN_CSS: Asset = asset!("/styles/main.scss");
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

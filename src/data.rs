use dioxus::prelude::*;
use std::{fmt, str::FromStr};

pub const MAIN_CSS: Asset = asset!("/styles/main.scss");
#[derive(Clone, Copy, PartialEq)] 
pub enum Uses {
    Rust,
    Java,
    Python,
    Unity,
    Arduino,
    ROS2,
}
impl Uses {
    pub fn call_sections(&self) -> &str {
        let i = match self {
            Uses::Rust => 0,
            Uses::Java => 1,
            Uses::Python => 2,
            Uses::Unity => 3,
            Uses::Arduino => 4,
            Uses::ROS2 => 5,
        };
        USE_SECTIONS[i]
    }
    pub fn name(&self) -> &str {
        match self {
            Uses::Rust => "Rust",
            Uses::Java => "Java",
            Uses::Python => "Python",
            Uses::Unity => "Unity",
            Uses::Arduino => "Arduino",
            Uses::ROS2 => "ROS2",
        }
    }
    pub fn call_use(i:i32) -> Self{
        match i {
            0 => Uses::Rust,
            1 => Uses::Java,
            2 => Uses::Python,
            3 => Uses::Unity,
            4 => Uses::Arduino,
            _ => Uses::ROS2,
            }
    }
}
impl FromStr for Uses {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" => Ok(Uses::Rust),
            "java" => Ok(Uses::Java),
            "python" => Ok(Uses::Python),
            "unity" => Ok(Uses::Unity),
            "arduino" => Ok(Uses::Arduino),
            "ros2" => Ok(Uses::ROS2),
            _ => Err(format!("unknown: {s}")),
        }
    }
}
impl fmt::Display for Uses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Uses::Rust => "rust",
            Uses::Java => "java",
            Uses::Python => "python",
            Uses::Unity => "unity",
            Uses::Arduino => "arduino",
            Uses::ROS2 => "ros2",
        };
        write!(f, "{s}")
    }
}
pub const USE_SECTIONS: &[&str] = &[
    "このサイト、Discordボット",
    "Minecraft の Mod・アドオン",
    "自作ニューラルネットワーク",
    "(C#) ブラウザで遊べるレースゲーム",
    "ESP32 + ディスプレイ",
    "ロボットの制御"
    ];

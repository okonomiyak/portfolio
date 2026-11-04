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

pub const USE_SECTIONS: &[&str] = &[
    "このサイト、Discordボット",
    "Minecraft の Mod・アドオン",
    "自作ニューラルネットワーク",
    "(C#) ブラウザで遊べるレースゲーム",
    "ESP32 + ディスプレイ",
    "ロボットの制御"
    ];

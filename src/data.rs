pub enum uses {
    Rust,
    Java,
    Python,
    Unity,
    Arduino,
    ROS2,
}
impl uses {
    pub fn call_sections(&self) -> &str {
        let i = match self {
            uses::Rust => 0,
            uses::Java => 1,
            uses::Python => 2,
            uses::Unity => 3,
            uses::Arduino => 4,
            uses::ROS2 => 5,
        };
        USE_SECTIONS[i]
    }
    pub fn name(&self) -> &str {
        match self {
            uses::Rust => "Rust",
            uses::Java => "Java",
            uses::Python => "Python",
            uses::Unity => "Unity",
            uses::Arduino => "Arduino",
            uses::ROS2 => "ROS2",
        }
    }
    pub fn call_use(i:i32) -> Self{
        match i {
            0 => uses::Rust,
            1 => uses::Java,
            2 => uses::Python,
            3 => uses::Unity,
            4 => uses::Arduino,
            _ => uses::ROS2,
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

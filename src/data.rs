pub enum uses {
    Rust,
    Java,
    Unity,
    Arduino,
    ROS2,
}
impl uses {
    pub fn call_sections(&self) -> &str {
        let i = match self {
            uses::Rust => 0,
            uses::Java => 1,
            uses::Unity => 2,
            uses::Arduino => 3,
            uses::ROS2 => 4,
        };
        USE_SECTIONS[i]
    }
    pub fn name(&self) -> &str {
        match self {
            uses::Rust => "Rust",
            uses::Java => "Java",
            uses::Unity => "Unity",
            uses::Arduino => "Arduino",
            uses::ROS2 => "ROS2",
        }
    }
    pub fn call_use(i:i32) -> Self{
        match i {
            0 => uses::Rust,
            1 => uses::Java,
            2 => uses::Unity,
            3 => uses::Arduino,
            _ => uses::ROS2,
            }
    }
}

pub const USE_SECTIONS: &[&str] = &[
    "このサイト、Discordボット",
    "Minecraft の Mod・アドオン",
    "(C#) ブラウザで遊べるレースゲーム",
    "ESP32 + ディスプレイ",
    "ロボットの制御"
    ];

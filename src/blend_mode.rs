pub enum BlendMode {
    Normal,
    Additive,
    Multiply,
    Screen,
}

impl BlendMode {
    pub fn values(value: i32) -> Self {
        match value {
            0 => BlendMode::Normal,
            1 => BlendMode::Additive,
            2 => BlendMode::Multiply,
            3 => BlendMode::Screen,
            _ => panic!("Invalid value for BlendMode: {}", value),
        }
    }
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        }
    }
}

impl Color {
    pub fn rgba8888to_color (&mut self, value: u32) {
        self.r = (((value & 0xff000000) >> 24) / 255) as f32;
        self.g = (((value & 0x00ff0000) >> 16) / 255) as f32;
        self.b = (((value & 0x0000ff00) >> 8) / 255) as f32;
        self.a = ((value & 0x000000ff) / 255) as f32;
    }
}

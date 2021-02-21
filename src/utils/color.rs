pub struct Color {
    pub(crate) r: f32,
    pub(crate) g: f32,
    pub(crate) b: f32,
    pub(crate) a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }
    }
}

impl Color {
    pub fn new() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }

    pub fn rgba8888to_color(&mut self, value: i32) {
        self.r = (((value as u32 & 0xff000000) >> 24) / 255) as f32;
        self.g = (((value as u32 & 0x00ff0000) >> 16) / 255) as f32;
        self.b = (((value as u32 & 0x0000ff00) >> 8) / 255) as f32;
        self.a = ((value as u32 & 0x000000ff) / 255) as f32;
    }
}

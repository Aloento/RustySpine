pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

impl Color {
    pub fn rgba8888to_color (&mut self, value: u32) {
        self.r = ((value & 0xff000000) >> 24) / 255f32;
        self.g = ((value & 0x00ff0000) >> 16) / 255f32;
        self.b = ((value & 0x0000ff00) >> 8) / 255f32;
        self.a = (value & 0x000000ff) / 255f32;
    }
}

use crate::attachments::texture::Texture;

pub struct TextureRegion {
    texture: Texture,
    u: f32,
    v: f32,
    u2: f32,
    v2: f32,
    regionWidth: i32,
    regionHeight: i32,
}

impl TextureRegion {
    pub fn new() -> Self {
        Self {
            texture: Texture {},
            u: 0.0,
            v: 0.0,
            u2: 0.0,
            v2: 0.0,
            regionWidth: 0,
            regionHeight: 0,
        }
    }
}

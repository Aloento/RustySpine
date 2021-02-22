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

pub struct Texture {}

pub enum TextureFilter {
    Nearest,
    Linear,
    MipMap,
    MipMapNearestNearest,
    MipMapLinearNearest,
    MipMapNearestLinear,
    MipMapLinearLinear,
}

pub enum TextureWrap {
    MirroredRepeat,
    ClampToEdge,
    Repeat,
}
